use deip_transaction_ctx::{TransactionCtxT, TransactionCtxId, PortalCtxT, ctx_t, TransactionCtx};
use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};

use crate::{PortalInfo, PortalTagOfTransaction};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::prelude::*;

use sp_runtime::{DispatchResultWithInfo, traits::Dispatchable};

ctx_t!(PortalCtx);

#[allow(type_alias_bounds)]
pub type PortalCtxOf<T: frame_system::Config> = PortalCtx<TransactionCtx<T>>;

impl<T, LocalCall> PortalCtxT<LocalCall> for PortalCtxOf<T>
    where T: crate::Config,
          T: SendTransactionTypes<crate::Call<T>>,
          <T as crate::Config>::Call: From<LocalCall>,
          T::PortalId: Copy + Default
{
    type PortalId = T::PortalId;

    fn submit_transaction(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()> {
        let call = crate::Call::on_behalf(Self::portal_id(&ctx), Box::new(call.into()));
        SubmitTransaction::<T, crate::Call<T>>::submit_unsigned_transaction(call.into())
    }

    fn portal_id(ctx: &TransactionCtxId<Self>) -> Self::PortalId {
        let portal_info: PortalInfo<Self::PortalId> = PortalTagOfTransaction::<T>::iter_prefix(&ctx.block_number).collect();
        let map = crate::transpose::<BTreeMap<&Self::ExtrinsicId, &Self::PortalId>, _, Self::PortalId>(portal_info.iter());
        map.get(&ctx.extrinsic_id).map(|x| **x).unwrap_or_else(Default::default)
    }
    
    fn with_ctx<D: Dispatchable>(
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin
    )
        -> DispatchResultWithInfo<D::PostInfo>
    {
        let cur = Self::current();
        PortalTagOfTransaction::<T>::append(cur.block_number(), portal_id, cur.extrinsic_id());
        call.dispatch(origin)
    }
}
