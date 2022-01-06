#![cfg(feature = "runtime-benchmarks")]

use super::{*, proposal::*};
use frame_system::{RawOrigin, EventRecord};
use frame_system::Config as Sys;
use frame_support::{ensure, traits::Get};
use frame_benchmarking::{benchmarks, account, whitelisted_caller};
use sp_std::prelude::*;
use core::convert::TryInto;

use crate::proposal::BATCH_MAX_SIZE;
use crate::Pallet as Proposal;
use frame_support::weights::Weight;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = generic_event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

type Author<T> = <T as Sys>::AccountId;
type Member<T> = <T as Sys>::AccountId;

fn pre_decide<T: Config>(
    batch: Vec<InputProposalBatchItem<T>>
)
    -> (Author<T>, ProposalId)
{
    let author: Author<T> = whitelisted_caller();
    let proposal_id = DeipProposal::<T>::timepoint();
    Proposal::<T>::propose(
        RawOrigin::Signed(author.clone()).into(),
        batch,
        Some(proposal_id))
        .unwrap();
    (author, proposal_id)
}

/// Batch with 2 depth levels with n items on each depth level.
/// AccountId of an each batch item counts as 0..n.
fn init_batch<T: Config>(c: usize) -> (Vec<InputProposalBatchItem<T>>, Weight) {
    let mut nested = vec![];
    while nested.len() < c {
        nested.push(InputProposalBatchItem::<T> {
            account: init_member::<T>(nested.len() as u32).into(),
            call: frame_system::Call::<T>::remark(vec![]).into(),
        });
    }
    let mut batch = vec![InputProposalBatchItem::<T> {
        account: init_member::<T>(0).into(),
        call: Call::<T>::propose(nested, None).into(),
    }];
    while batch.len() < c {
        batch.push(InputProposalBatchItem::<T> {
            account: init_member::<T>(batch.len() as u32).into(),
            call: frame_system::Call::<T>::remark(vec![]).into()
        });
    }
    let batch_weight = batch_weight::<T>(batch.as_slice());
    (batch, batch_weight)
}

fn init_member<T: Config>(index: u32) -> T::AccountId {
    let member = account::<T::AccountId>("member", index, SEED);
    let member_key = frame_system::Account::<T>::hashed_key_for(&member);
    frame_benchmarking::benchmarking::add_to_whitelist(member_key.into());
    member
}

fn pre_decide_final_approval<T: Config>()
    -> (Member<T>, ProposalId, Weight)
{
    let member = init_member::<T>(0);
    let batch = vec![InputProposalBatchItem::<T> {
        account: member.clone().into(),
        call: frame_system::Call::<T>::remark(vec![]).into(),
    }];
    let batch_weight = batch_weight::<T>(batch.as_slice());
    let (_author, proposal_id) = pre_decide::<T>(batch);
    (member, proposal_id, batch_weight)
}

benchmarks! {
    propose {
        let c in 0 .. BATCH_MAX_SIZE.try_into().unwrap();
        
        let caller: T::AccountId = whitelisted_caller();
        let (batch, _) = init_batch::<T>(c as usize);
        
        let proposal_id = DeipProposal::<T>::timepoint();
        let external_id: Option<ProposalId> = Some(proposal_id);
    }: _(RawOrigin::Signed(caller), batch, external_id)
    verify {
        ensure!(ProposalRepository::<T>::contains_key(proposal_id), "proposal not created")
    }
    
    decide_reject {
        let (batch, batch_weight) = init_batch::<T>(BATCH_MAX_SIZE);
        let (_author, proposal_id) = pre_decide::<T>(batch);
        let member = init_member::<T>(0);
        let decision = ProposalMemberDecision::Reject;
    }: decide(RawOrigin::Signed(member.clone()), proposal_id, decision, batch_weight)
    verify {
        assert_last_event::<T>(Event::Resolved {
            member,
            proposal_id,
            state: ProposalState::Rejected
        }.into())
    }
    
    decide_approve {
        let (batch, batch_weight) = init_batch::<T>(BATCH_MAX_SIZE);
        let (_author, proposal_id) = pre_decide::<T>(batch);
        let member = init_member::<T>(0);
        let decision = ProposalMemberDecision::Approve;
    }: decide(RawOrigin::Signed(member.clone()), proposal_id, decision, batch_weight)
    verify {
        assert_last_event::<T>(Event::Approved {
            member,
            proposal_id,
        }.into())
    }
    
    decide_revoke_approval {
        let (batch, batch_weight) = init_batch::<T>(BATCH_MAX_SIZE);
        let (_author, proposal_id) = pre_decide::<T>(batch);
        let member = init_member::<T>(0);
        Proposal::<T>::decide(
            RawOrigin::Signed(member.clone()).into(),
            proposal_id,
            ProposalMemberDecision::Approve,
            batch_weight
        )?;
        let decision = ProposalMemberDecision::Pending;
    }: decide(RawOrigin::Signed(member.clone()), proposal_id, decision, batch_weight)
    verify {
        assert_last_event::<T>(Event::RevokedApproval {
            member,
            proposal_id,
        }.into())
    }
    
    decide_final_approve {
        let (member, proposal_id, batch_weight) = pre_decide_final_approval::<T>();
        let decision = ProposalMemberDecision::Approve;
    }: decide(RawOrigin::Signed(member.clone()), proposal_id, decision, batch_weight)
    verify {
        assert_last_event::<T>(Event::Resolved {
            member,
            proposal_id,
            state: ProposalState::Done
        }.into())
    }
    
    expire {
        let (batch, _batch_weight) = init_batch::<T>(BATCH_MAX_SIZE);
        let (_author, proposal_id) = pre_decide::<T>(batch);
        ProposalRepository::<T>::mutate(proposal_id, |x| {
            x.as_mut().unwrap().created_at -= T::Ttl::get();
        });
    }: _(RawOrigin::None, proposal_id)
    verify {
        assert_last_event::<T>(Event::Expired {
            proposal_id,
        }.into())
    }
}
