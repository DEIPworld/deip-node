use sp_std::collections::btree_map::BTreeMap;
use sp_std::prelude::*;

use frame_support::pallet_prelude::*;
use frame_support::Hashable;

use deip_transaction_ctx::{TransactionCtxId, TransactionCtxT};

use crate::storage::{StorageOpsT, StorageOps};

use super::{Config, Event, Error, ProposalRepository};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};


pub type ProposalId = sp_core::H160;

pub type ProposalBatchX<Item> = Vec<Item>;

#[allow(type_alias_bounds)]
pub type ProposalBatch<T: Config> = Vec<ProposalBatchItemOf<T>>;

#[allow(type_alias_bounds)]
pub type InputProposalBatch<T: Config> = Vec<InputProposalBatchItem<T>>;

/// Specialized version of [`BatchItem`]
#[allow(type_alias_bounds)]
pub type ProposalBatchItemOf<T: Config> = BatchItem<
    <T as frame_system::Config>::AccountId,
    <T as Config>::Call
>;

#[allow(type_alias_bounds)]
pub type InputProposalBatchItem<T: Config> = BatchItem<
    T::DeipAccountId,
    <T as Config>::Call
>;

/// Batch item generic container
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BatchItem<Account, CallT> {
    pub account: Account,
    pub call: CallT,
}

/// Proposal object
#[derive(Debug, Encode, Decode, Clone, Eq, PartialEq)]
pub struct DeipProposal<T: Config> {
    /// Proposal ID
    pub(super) id: ProposalId,
    /// Batch-transaction items
    pub(super) batch: ProposalBatch<T>,
    /// Member decisions mapping
    pub(super) decisions: BTreeMap<T::AccountId, ProposalMemberDecision>,
    /// Proposal state
    pub(super) state: ProposalState,
    /// Proposal author
    pub(super) author: T::AccountId,
    pub(super) created_at: T::Moment,
    /// Created with context
    pub created_ctx: TransactionCtxId<T::TransactionCtx>,
}

/// Proposal state
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProposalState {
    /// Pending proposal
    Pending,
    /// Rejected proposal
    Rejected,
    /// Batch transaction executed successfully
    Done,
    /// Batch transaction execution failed
    Failed(sp_runtime::DispatchError)
}

/// A global extrinsic index, formed as the extrinsic index within a block, together with that
/// block's height. This allows a transaction in which a multisig operation of a particular
/// composite was created to be uniquely identified.
#[derive(Copy, Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug)]
struct Timepoint<BlockNumber> {
    /// The height of the chain at the point in time.
    height: BlockNumber,
    /// The index of the extrinsic at the point in time.
    index: u32,
}

/// Proposal member decision
#[derive(Debug, Clone, Copy, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ProposalMemberDecision {
    /// Pending state
    Pending,
    /// Approved state
    Approve,
    /// Rejected state
    Reject
}
impl ProposalMemberDecision {
    /// Make decision state transition.
    /// 
    /// Except of transitions from `Reject` current state all another transitions are allowed.
    /// `Ok(None)` result means transition to the same state.
    /// 
    /// This function must stay private to disallow state transitions from code outsides
    /// of this Pallet.
    /// You should prefer to use [`DeipProposal`] object as a pallet logic's main interface
    /// 
    fn decide(&mut self, decision: Self) -> Result<Option<Self>, Self> {
        let cur = self;
        let new = &decision;
        match (&cur, new) {
            (Self::Reject, _) => Err(*cur),
            _ => {
                let transition = cur != new;
                *cur = *new;
                if transition { Ok(Some(*cur)) } else { Ok(None) }
            },
        }
    }
}

impl<T: Config> DeipProposal<T> {
    /// Generate "Timepoint" aka unique proposal ID.
    /// Implemented as hash-value of Timepoint from `pallet_multisig`   
    fn timepoint() -> ProposalId {
        let timepoint = Timepoint::<T::BlockNumber> {
            height: <frame_system::Pallet<T>>::block_number(),
            index: <frame_system::Pallet::<T>>::extrinsic_index().unwrap_or_default(),
        }.twox_256();
        ProposalId::from_slice(&timepoint[..20])
    }
    
    /// Create proposal object.
    /// Fail if input arguments violates proposal assertions (See [proposal_assertions](./Pallet.proposal_assertions))
    pub fn create(
        batch: InputProposalBatch<T>,
        author: T::AccountId,
        external_id: Option<ProposalId>,
        storage_ops: &mut StorageOpsT<T>,
        created_at: T::Moment
    )
        -> Result<(), Error<T>>
    {
        let id = external_id.unwrap_or_else(Self::timepoint);
        ensure!(
            !ProposalRepository::<T>::contains_key(&id),
            Error::<T>::AlreadyExist
        );
        match crate::batch_assertions::assert_proposal::<T, _>(&batch, &id, 2) {
            Some(crate::batch_assertions::ProposalAssertions::DepthLimit) => {
                return Err(Error::<T>::ReachDepthLimit)
            },
            Some(crate::batch_assertions::ProposalAssertions::SelfReference) => {
                return Err(Error::<T>::SelfReferential)
            },
            None => (),
        }
        
        let batch: ProposalBatch<T> = batch
            .into_iter()
            .map(|x| {
                let BatchItem { account, call } = x;
                BatchItem {
                    account: account.into(),
                    call
                }
            })
            .collect();
        
        use sp_std::iter::FromIterator;
        let decisions = BTreeMap::from_iter(
            batch.iter().map(|x| (
                x.account.clone(),
                ProposalMemberDecision::Pending
            ))
        );
        
        let proposal = Self {
            id,
            batch,
            decisions,
            state: ProposalState::Pending,
            author,
            created_at,
            created_ctx: T::TransactionCtx::current().id()
        };
        storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::Proposed {
            author: proposal.author.clone(),
            batch: proposal.batch.clone(),
            proposal_id: proposal.id,
        }));
        storage_ops.push_op(StorageOps::CreateProposal(proposal));
        Ok(())
    }
    
    /// 
    pub fn decide<BatchExec>(
        mut self,
        member: &T::AccountId,
        decision: ProposalMemberDecision,
        batch_exec: BatchExec,
        storage_ops: &mut StorageOpsT<T>
    )
        -> Result<Option<BatchExec::Output>, super::Error<T>>
        where
            BatchExec: FnOnce(ProposalBatch<T>) -> frame_support::dispatch::DispatchResultWithPostInfo
    {
        let member_decision = self.decisions.get_mut(member).ok_or(Error::<T>::NotAMember)?;
        
        ensure!(matches!(self.state, ProposalState::Pending), Error::<T>::AlreadyResolved);

        match member_decision.decide(decision) {
            Err(_) => return Err(Error::<T>::AlreadyResolved),
            Ok(None) => Ok(None),
            Ok(Some(ProposalMemberDecision::Pending)) => {
                storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::RevokedApproval {
                    member: member.clone(),
                    proposal_id: self.id
                }));
                storage_ops.push_op(StorageOps::UpdateProposal(self));
                Ok(None)
            },
            Ok(Some(ProposalMemberDecision::Reject)) => {
                self.state = ProposalState::Rejected;
                storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::Resolved {
                    member: member.clone(),
                    proposal_id: self.id,
                    state: self.state
                }));
                storage_ops.push_op(StorageOps::DeleteProposal(self));
                Ok(None)
            },
            Ok(Some(ProposalMemberDecision::Approve)) => {
                if self.ready_to_exec() {
                    let batch_exec_result = batch_exec(self.batch.clone());
                    self.state = if let Err(ref err) = batch_exec_result { 
                        ProposalState::Failed(err.error.clone())
                    } else {
                        ProposalState::Done
                    };
                    storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::Resolved {
                        member: member.clone(),
                        proposal_id: self.id,
                        state: self.state
                    }));
                    storage_ops.push_op(StorageOps::DeleteProposal(self));
                    Ok(Some(batch_exec_result))
                } else {
                    storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::Approved {
                        member: member.clone(),
                        proposal_id: self.id,
                    }));
                    storage_ops.push_op(StorageOps::UpdateProposal(self));
                    Ok(None)
                }
            },
        }
    }
    
    fn ready_to_exec(&self) -> bool {
        let approved = self.decisions.values()
            .all(|x: &ProposalMemberDecision| {
                matches!(x, ProposalMemberDecision::Approve)
            });
        approved && matches!(self.state, ProposalState::Pending)
    }
    
    pub fn expired(&self, now: T::Moment) -> bool {
        (self.created_at + T::Ttl::get()) <= now 
            && matches!(self.state, ProposalState::Pending)
    }
    
    pub fn expire(self, now: T::Moment, storage_ops: &mut StorageOpsT<T>) -> Result<(), Error<T>>{
        ensure!(self.expired(now), Error::<T>::NotExpired);
        storage_ops.push_op(StorageOps::DepositEvent(Event::<T>::Expired { proposal_id: self.id }));
        storage_ops.push_op(StorageOps::DeleteProposal(self));
        Ok(())
    }
}
