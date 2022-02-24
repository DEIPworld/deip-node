pub use deip_tenant::*;

use deip_transaction_ctx::{ctx_t, PortalCtxT, TransactionCtx, TransactionCtxId, TransactionCtxT};
use frame_support::{
    ensure,
    log::{debug, error},
};
use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

use crate::{PortalInfo, PortalT, PortalTagOfTransaction};
use sp_std::{collections::btree_map::BTreeMap, fmt::Debug, prelude::*};

use sp_runtime::{
    traits::{Dispatchable, Extrinsic, Hash},
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

    fn submit_postponed(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()> {
        debug!("{}: {:?}", "Submit postponed", &call);
        let call = crate::Call::exec_postponed {
            portal_id: Self::portal_id(&ctx),
            call: Box::new(call.into()),
        };
        SubmitTransaction::<T, crate::Call<T>>::submit_unsigned_transaction(call.into())
    }
}
