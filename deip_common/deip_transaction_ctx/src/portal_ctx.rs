use super::{TransactionCtxId, TransactionCtxT};

use scale_info::TypeInfo;
use sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};

pub trait PortalCtxT<LocalCall>: TransactionCtxT + TypeInfo {
    type PortalId;

    fn portal_id(ctx: &TransactionCtxId<Self>) -> Self::PortalId;

    /// Dispatch within Portal context.
    fn dispatch<D: Dispatchable>(
        &self,
        portal_id: Self::PortalId,
        call: D,
        origin: D::Origin,
    ) -> DispatchResultWithInfo<D::PostInfo>;

    fn submit_postponed(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()>;
}
