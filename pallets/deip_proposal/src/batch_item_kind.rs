/// Module contains classification of the proposal batch item

use frame_support::traits::IsSubType;

use crate::proposal::{ProposalId, ProposalBatchX, BatchItem};

use super::{Config, Call};


pub trait BatchItemKindT<T: Config>: Sized {
    fn kind(&self) -> BatchItemKind<'_, Self>;
}

/// Batch item kinds
pub enum BatchItemKind<'a, Item> {
    /// Batch item contains `propose` dispatchable
    Propose(&'a ProposalBatchX<Item>),
    /// Batch item contains `decide` dispatchable
    Decide(&'a ProposalId),
    Other
}

impl<T: Config> BatchItemKindT<T> for BatchItem<T::DeipAccountId, <T as Config>::Call> {

    fn kind(&self) -> BatchItemKind<'_, Self> {
        match self.call.is_sub_type() {
            Some(Call::propose(batch, _)) => {
                BatchItemKind::Propose(batch)
            },
            Some(Call::decide(proposal_id, _decision)) => {
                BatchItemKind::Decide(proposal_id)
            },
            _ => BatchItemKind::Other
        }
    }
}
