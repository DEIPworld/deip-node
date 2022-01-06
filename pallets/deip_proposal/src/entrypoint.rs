/// Pallet's business-logic public interface

use crate::proposal::{InputProposalBatch, DeipProposal, ProposalId};
use crate::storage::StorageWrite;

use super::{Config};

use frame_support::dispatch::DispatchResultWithPostInfo;

/// Create proposal
pub fn propose<T: Config>(
    author: T::AccountId,
    batch: InputProposalBatch<T>,
    external_id: Option<ProposalId>,
)
    -> DispatchResultWithPostInfo
{
    StorageWrite::<T>::new()
        .commit(move |ops| {
            DeipProposal::<T>::create(
                batch,
                author,
                external_id,
                ops,
                pallet_timestamp::Pallet::<T>::get()
            )
        })
}
