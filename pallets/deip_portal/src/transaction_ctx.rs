pub use deip_tenant::*;

use deip_transaction_ctx::{ctx_t, PortalCtxT, TransactionCtx, TransactionCtxId, TransactionCtxT};
use frame_support::{
    ensure,
    log::debug,
    traits::{ExtrinsicCall, IsSubType},
    Hashable,
};
use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

use crate::{DelegateLookup, PendingTx, PortalInfo, PortalT, PortalTagOfTransaction, ScheduledTx};
use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, prelude::*};

use sp_runtime::{
    traits::{Dispatchable, Extrinsic},
    DispatchResultWithInfo,
};

use codec::{Decode, Encode, EncodeLike};

use crate::Error::*;

ctx_t!(PortalCtx);

#[allow(type_alias_bounds)]
pub type PortalCtxOf<T: frame_system::Config> = PortalCtx<TransactionCtx<T>>;

impl<T, LocalCall> PortalCtxT<LocalCall> for PortalCtxOf<T>
where
    T: crate::Config,
    T: SendTransactionTypes<crate::Call<T>>,
    <T as crate::Config>::Call: From<LocalCall>,
    T::PortalId: Copy + Default + Debug,
    LocalCall: Debug,
{
    type PortalId = T::PortalId;
    type Extrinsic = T::UncheckedExtrinsic;
    type Error = crate::Error<T>;
    type Delegate = crate::PortalDelegate<T>;

    fn portal_id(ctx: &TransactionCtxId<Self>) -> Self::PortalId {
        let portal_info: PortalInfo<Self::PortalId> =
            PortalTagOfTransaction::<T>::iter_prefix(&ctx.block_number).collect();
        let map =
            crate::transpose::<BTreeMap<&Self::ExtrinsicId, &Self::PortalId>, _, Self::PortalId>(
                portal_info.iter(),
            );
        map.get(&ctx.extrinsic_id).map(|x| **x).unwrap_or_else(Default::default)
    }

    fn dispatch<D: Dispatchable>(
        &self,
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin,
    ) -> DispatchResultWithInfo<D::PostInfo> {
        let extrinsic_id = self.extrinsic_id();
        let block_number = self.block_number();
        PortalTagOfTransaction::<T>::append(block_number, portal_id, extrinsic_id);
        call.dispatch(origin)
    }

    fn schedule_extrinsic(
        &self,
        xt: Self::Extrinsic,
        delegate: Self::Delegate,
    ) -> Result<(), Self::Error> {
        if let Some(crate::Call::exec { portal_id, call: _ }) =
            <T as crate::Config>::Call::is_sub_type(ExtrinsicCall::call(&xt))
        {
            ensure!(T::Portal::lookup_delegate(portal_id)? == delegate, DelegateMismatch);
            let xt_hash = xt.twox_256();
            ensure!(!ScheduledTx::<T>::contains_key(xt_hash), AlreadyScheduled);
            ScheduledTx::<T>::insert(xt_hash, portal_id);
            PendingTx::<T>::insert(self.block_number(), xt_hash, xt);
            return Ok(())
        }
        Err(UnproperCall)?
    }

    fn submit_scheduled(at: Self::BlockNumber) -> Result<Vec<()>, ()> {
        debug!("Submit scheduled extrinsics at #{:?}", at);
        PendingTx::<T>::drain_prefix(at)
            .into_iter()
            .map(|(xt_hash, xt)| {
                // frame_support::debug(&xt);
                debug!("Submit scheduled extrinsic with hash {:?}", xt_hash);
                sp_io::offchain::submit_transaction(xt.encode())
            })
            .collect()
    }

    fn dispatch_scheduled<D: Dispatchable>(
        &self,
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin,
    ) -> Result<DispatchResultWithInfo<D::PostInfo>, Self::Error> {
        let xt = self.extrinsic_data(true);
        let xt_hash = sp_io::hashing::twox_256(&mut &xt[..]);
        debug!("{}: {:?}", "Dispatch scheduled", &xt_hash);
        let expected_portal_id = ScheduledTx::<T>::take(xt_hash);
        ensure!(expected_portal_id.is_some(), NotScheduled);
        ensure!(portal_id == expected_portal_id.unwrap(), PortalMismatch);
        Ok(self.dispatch(portal_id, call, origin))
    }

    fn submit_postponed(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()> {
        debug!("{}: {:?}", "Submit postponed", &call);
        let call = crate::Call::exec_postponed {
            portal_id: Self::portal_id(&ctx),
            call: Box::new(call.into()),
        };
        SubmitTransaction::<T, crate::Call<T>>::submit_unsigned_transaction(call.into())
    }
}
