/// Pallet's business-logic public interface

use crate::proposal::{InputProposalBatch, DeipProposal, ProposalId};
use crate::storage::StorageWrite;

use super::{Config, Error};

/// Create proposal
pub fn propose<T: Config>(
    author: T::AccountId,
    batch: InputProposalBatch<T>,
    external_id: Option<ProposalId>,
)
    -> Result<(), Error<T>>
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
