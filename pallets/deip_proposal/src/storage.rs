/// Module contains abstractions over pallet storage operations

pub use deip_storage_ops::*;

use crate::proposal::DeipProposal;

use super::{Config, Event, ProposalRepository, Pallet};


pub type StorageWrite<T> = StorageOpsTransaction<StorageOps<T>>;
pub type StorageOpsT<T> = StorageOpsQueue<StorageOps<T>>;

/// Storage operations
pub enum StorageOps<T: Config> {
    /// Deposit event
    DepositEvent(Event<T>),
    /// Create proposal
    CreateProposal(DeipProposal<T>),
    /// Update proposal
    UpdateProposal(DeipProposal<T>),
    /// Delete proposal
    DeleteProposal(DeipProposal<T>),
}
impl<T: Config> StorageOp for StorageOps<T> {
    fn exec(self) {
        match self {
            StorageOps::DepositEvent(event) => {
                <Pallet<T>>::deposit_event(event);
            },
            StorageOps::CreateProposal(proposal) => {
                <ProposalRepository<T>>::insert(proposal.id, proposal);
            },
            StorageOps::UpdateProposal(proposal) => {
                <ProposalRepository<T>>::insert(proposal.id, proposal)
            },
            StorageOps::DeleteProposal(proposal) => {
                <ProposalRepository<T>>::remove(proposal.id);
            },
        }
    }
}
