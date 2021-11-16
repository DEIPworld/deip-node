//! # Proposal Pallet
//! A module for doing a propose of transactions composed of an arbitrary set of operations
//! that requires approvals from multiple accounts to been executed.
//! 
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! This module contains functionality to create a postponed transaction (a proposal)
//! where members of it are proposed to make decision on execution of of corresponding operations.
//! When all members of proposal add them approvals then the bunch of operations (a batch)
//! that is a list of dispatchables with their signature origins (accounts)
//! will be executed as a single transaction.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `propose` - Propose a postponed transaction.
//! * `decide` - Make decision on a proposed transaction being a member of it.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;
mod batch_tree;
pub mod proposal;
mod batch_item_kind;
mod storage;
pub mod entrypoint;
mod batch_assertions;

#[doc(inline)]
pub use pallet::*;

/// Re-exports deip_storage_ops.
pub use deip_storage_ops;

const NON_LOCAL: u8 = 99;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    use frame_system::offchain::{SendTransactionTypes};
    
    use frame_support::pallet_prelude::*;
    use frame_support::weights::{PostDispatchInfo, GetDispatchInfo};
    // use frame_support::log::RuntimeLogger;
    use frame_support::log::debug;
    
    use frame_support::traits::{UnfilteredDispatchable, IsSubType};
    
    use sp_std::prelude::*;
    use sp_std::collections::{btree_map::BTreeMap};
    
    use sp_runtime::traits::{Dispatchable, Zero};
    
    use crate::proposal::{
        ProposalId, DeipProposal,
        ProposalMemberDecision, ProposalState,
        ProposalBatch, ProposalBatchItemOf,
        InputProposalBatchItem
    };
    use crate::storage::StorageWrite;
    
    use deip_transaction_ctx::{PortalCtxT};

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config + SendTransactionTypes<Call<Self>> {
        /// Context of extrinsic currently being in execution
        type TransactionCtx: PortalCtxT<Call<Self>>;
        /// Type represents events
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Type represents particular call from batch-transaction 
        type Call: Parameter +
             Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo> +
             GetDispatchInfo +
             From<frame_system::pallet::Call<Self>> +
             UnfilteredDispatchable<Origin = Self::Origin> +
             frame_support::dispatch::Codec + 
             IsSubType<Call<Self>>;
        
        type DeipAccountId: Into<Self::AccountId> + Parameter + Member;
        
        /// Pending proposal's time-to-live
        #[pallet::constant]
        type Ttl: Get<Self::Moment>;
        
        /// Period of check for expired proposals
        #[pallet::constant]
        type ExpirePeriod: Get<Self::BlockNumber>;
    }
    
    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(n: T::BlockNumber) {
            // RuntimeLogger::init();
            if !sp_io::offchain::is_validator() {
                debug!("{}", "not a validator");
                return
            }
            if n % T::ExpirePeriod::get() != Zero::zero() {
                debug!("skip expire proposals at {:?}", n);
                return
            }
            debug!("expire proposals at {:?}", n);
            let now = pallet_timestamp::Pallet::<T>::get();
            for (id, obj) in ProposalRepository::<T>::iter() {
                if !obj.expired(now) {
                    continue
                }
                let call = Call::expire(id);
                
                let submit = T::TransactionCtx::submit_postponed(call, obj.created_ctx);
                
                if submit.is_err() {
                    debug!("{}", "error on submit unsigned transaction");
                } else {
                    debug!("{}", "submit unsigned transaction");
                }
            }
        }
    }
    
    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// Validate unsigned call to this module.
        ///
        /// By default unsigned transactions are disallowed, but implementing the validator
        /// here we make sure that some particular calls (the ones produced by offchain worker)
        /// are being whitelisted and marked as valid.
        fn validate_unsigned(
            source: TransactionSource,
            call: &Self::Call,
        )
            -> TransactionValidity
        {
            // Firstly let's check that we get the local transaction.
            if !matches!(source, TransactionSource::Local | TransactionSource::InBlock) {
                return InvalidTransaction::Custom(super::NON_LOCAL).into()
            }
            // Check that we call the right function.
            if let Call::expire(ref proposal_id) = call {
                let proposal = ProposalRepository::<T>::get(proposal_id);
                let now = pallet_timestamp::Pallet::<T>::get();
                if proposal.is_none() { return InvalidTransaction::Stale.into() }
                else if !proposal.as_ref().unwrap().expired(now) { return InvalidTransaction::Future.into() }
                ValidTransaction::with_tag_prefix("DeipProposalOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides((*proposal_id, proposal.unwrap().created_at))
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }
    
    #[pallet::error]
    pub enum Error<T> {
        /// Proposal not found
        NotFound,
        /// Proposal already exist
        AlreadyExist,
        /// Current origin is not a member of Proposal
        NotAMember,
        /// Proposal already resolved (done, failed or rejected)
        AlreadyResolved,
        /// Reach depth limit of nested proposals
        ReachDepthLimit,
        /// Self-referential proposal
        SelfReferential,
        /// Not expired yet
        NotExpired
    }
    
    #[pallet::event]
    #[pallet::metadata(u32 = "SpecialU32")]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emits when proposal created
        Proposed {
            author: T::AccountId,
            batch: ProposalBatch<T>,
            proposal_id: ProposalId
        },
        /// Emits when proposal approved by it's member
        Approved {
            member: T::AccountId,
            proposal_id: ProposalId
        },
        /// Emits when member revokes his approval
        RevokedApproval {
            member: T::AccountId,
            proposal_id: ProposalId
        },
        /// Emits when proposal resolved (rejected / done / failed)
        Resolved {
            member: T::AccountId,
            proposal_id: ProposalId,
            state: ProposalState
        },
        /// Expired
        Expired {
            proposal_id: ProposalId
        }
    }
    
    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}
    
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10)]
        pub fn propose(
            origin: OriginFor<T>,
            batch: Vec<InputProposalBatchItem<T>>,
            external_id: Option<ProposalId>
        )
            -> DispatchResultWithPostInfo
        {
            let author = ensure_signed(origin)?;
            
            // frame_support::debug::RuntimeLogger::init();

            crate::entrypoint::propose::<T>(author, batch, external_id)?;
            
            Ok(Some(0).into())
        }

        #[pallet::weight(10)]
        pub fn decide(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
            decision: ProposalMemberDecision,
        )
            -> DispatchResultWithPostInfo
        {
            let member = ensure_signed(origin)?;
            let proposal = ProposalRepository::<T>::get(&proposal_id).ok_or(Error::<T>::NotFound)?;
            let maybe_batch_exec_result: Option<DispatchResultWithPostInfo> =
                StorageWrite::<T>::new()
                    .commit(|ops| {
                        proposal.decide(
                            &member,
                            decision,
                            Self::exec_batch,
                            ops,
                        )
                    })?;
            if let Some(batch_exec_result) = maybe_batch_exec_result {
                let _batch_exec_ok = batch_exec_result?;
            }
            Ok(Some(0).into())
        }
        
        #[pallet::weight(10_000)]
        pub fn expire(
            origin: OriginFor<T>,
            proposal_id: ProposalId,
        )
            -> DispatchResultWithPostInfo
        {
            ensure_none(origin)?;
            
            let proposal = ProposalRepository::<T>::get(proposal_id)
                .ok_or_else(|| Error::<T>::NotFound)?;
            
            StorageWrite::<T>::new().commit(move |ops| {
                let now = pallet_timestamp::Pallet::<T>::get();
                proposal.expire(now, ops)
            })?;
            
            Ok(Some(0).into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Execute batch as atomic transaction
        #[frame_support::transactional]
        fn exec_batch(batch: ProposalBatch<T>) -> DispatchResultWithPostInfo
        {
            // frame_support::debug::RuntimeLogger::init();
            for x in batch {
                let ProposalBatchItemOf::<T> { account, call } = x;
                frame_support::log::debug!("{:?}; {:?}", &account, &call);
                call.dispatch(RawOrigin::Signed(account).into())?;
            }
            Ok(Some(0).into())
        }
    }
    
    #[pallet::storage]
    pub(super) type ProposalRepository<T: Config> = StorageMap<_,
        Blake2_128Concat,
        ProposalId,
        DeipProposal<T>,
        OptionQuery
    >;

    #[pallet::storage]
    pub(super) type ProposalIdByAccountId<T: Config> = StorageDoubleMap<_,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        ProposalId,
        (),
        OptionQuery
    >;

    #[allow(type_alias_bounds)]
    pub type PendingProposalsMap<T: Config> = BTreeMap<ProposalId, T::AccountId>;
    
    #[pallet::storage]
    #[pallet::getter(fn pending_proposals)]
    pub(super) type PendingProposals<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::AccountId,
        PendingProposalsMap<T>,
        ValueQuery,
        PendingProposalsMapDefault<T>
    >;
    
    #[pallet::type_value]
    pub(super) fn PendingProposalsMapDefault<T: Config>() -> PendingProposalsMap<T> { Default::default() }
}
