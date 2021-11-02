use super::TransactionCtxT;
use super::TransactionCtxId;

use sp_runtime::{DispatchResultWithInfo, traits::Dispatchable};

pub trait PortalCtxT<LocalCall>: TransactionCtxT {
    type PortalId;
    
    fn submit_transaction(call: LocalCall, ctx: TransactionCtxId<Self>) -> Result<(), ()>;
    
    fn portal_id(ctx: &TransactionCtxId<Self>) -> Self::PortalId;
    
    /// Dispatch with the Portal context.
    fn with_ctx<D: Dispatchable>(portal_id: Self::PortalId, call: D, origin: D::Origin) -> DispatchResultWithInfo<D::PostInfo>;
}
