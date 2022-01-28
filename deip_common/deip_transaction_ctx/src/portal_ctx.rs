use super::{TransactionCtxId, TransactionCtxT};

use scale_info::TypeInfo;
use sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};
use sp_std::prelude::Vec;

pub trait PortalCtxT<LocalCall>: TransactionCtxT + TypeInfo {
    type PortalId;
    type Extrinsic;
    type Error;
    type Delegate;

    fn portal_id(ctx: &TransactionCtxId<Self>) -> Self::PortalId;

    /// Dispatch within Portal context.
    fn dispatch<D: Dispatchable>(
        &self,
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin,
    ) -> DispatchResultWithInfo<D::PostInfo>;

    /// Schedule extrinsic to be dispatched within Portal context.
    fn schedule_extrinsic(
        &self,
        xt: Self::Extrinsic,
        delegate: Self::Delegate,
    ) -> Result<(), Self::Error>;

    fn submit_scheduled(at: Self::BlockNumber) -> Result<Vec<()>, ()>;

    fn dispatch_scheduled<D: Dispatchable>(
        &self,
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin,
    ) -> Result<DispatchResultWithInfo<D::PostInfo>, Self::Error>;

    fn submit_postponed(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()>;
}
