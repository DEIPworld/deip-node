/// Module contains abstractions over pallet storage operations

use sp_std::prelude::*;

pub use deip_storage_ops::*;

use crate::proposal::DeipProposal;

use super::{Config, Event, ProposalRepository, Pallet, PendingProposals, ProposalIdByAccountId};


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
                let members = proposal.decisions.keys().cloned();
                for m in members {
                    PendingProposals::<T>::mutate(m, |x| {
                        x.insert(proposal.id, proposal.author.clone());
                    });
                }

                let id = proposal.id;
                let author = proposal.author.clone();
                <ProposalRepository<T>>::insert(id, proposal);
                ProposalIdByAccountId::<T>::insert(author, id, ());
            },
            StorageOps::UpdateProposal(proposal) => {
                <ProposalRepository<T>>::insert(proposal.id, proposal)
            },
            StorageOps::DeleteProposal(proposal) => {
                let DeipProposal::<T> {
                    id: proposal_id,
                    decisions,
                    author,
                    .. 
                } = proposal;
                let members = decisions.keys();
                for m in members {
                    PendingProposals::<T>::mutate(m, |x| {
                        x.remove(&proposal_id);
                    });
                }

                ProposalIdByAccountId::<T>::remove(author, proposal_id);
                <ProposalRepository<T>>::remove(proposal_id);
            },
        }
    }
}
