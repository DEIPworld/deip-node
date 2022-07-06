//! # Stake voting pallet
//! A pallet for doing multisig dispatch by asset's holders
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! TODO
//! This pallet contains functionality for multi-signature dispatch, a stateful
//! operation, allowing multiple stakeholders/shareholders to coordinate and dispatch
//! a call from a well-known origin linked with specified assets
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create voting for asset's holders and vote for a call if possible dispatch a call from a composite origin
//! * `vote` - Vote for a call if possible dispatch a call from a composite origin
//! * `unvote` - Delete previously sent vote if possible dispatch a call from a composite origin
//! * `cancel` - Cancel voting by its author
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
pub(crate) mod tests;
pub mod weights;

use codec::{Decode, Encode};
use deip_asset_system::*;
use frame_support::dispatch::{
    Codec, DispatchError, DispatchResult, DispatchResultWithPostInfo, PostDispatchInfo,
};
use frame_support::traits::{
    Currency, Get, PalletInfoAccess, ReservableCurrency, WrapperKeepOpaque,
};
use frame_support::weights::{GetDispatchInfo, Weight};
use frame_support::RuntimeDebug;
use frame_support::{ensure, transactional};
use frame_system::{Config as SystemConfig, RawOrigin};
use scale_info::TypeInfo;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{AtLeast32Bit, Dispatchable, TrailingZeroInput, Zero};
use sp_std::prelude::*;
pub use weights::WeightInfo;

pub use pallet::*;

type TimeOf<T> = Timepoint<<T as SystemConfig>::BlockNumber>;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type ThresholdOf<T> = Threshold<<T as Config>::AssetBalance>;

type VotingOf<T> =
    Voting<<T as SystemConfig>::AccountId, <T as Config>::AssetId, TimeOf<T>, ThresholdOf<T>>;

type HoldGuardOf<T> = <<T as Config>::Assets as NFTImplT>::FractionHoldGuard;

type Guard = u32;

// A global extrinsic index, formed as the extrinsic index within a block, together with that
/// block's height. This allows a transaction in which a multisig operation of a particular
/// composite was created to be uniquely identified.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Default, RuntimeDebug, TypeInfo,
)]
pub struct Timepoint<Height> {
    /// The height of the chain at the point in time.
    height: Height,
    /// The index of the extrinsic at the point in time.
    index: u32,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Voting<Account, Asset, Time, Threshold> {
    author: Account,
    asset: Asset,
    start: Time,
    end: Option<Time>,
    threshold: Threshold,
    delegate: Account,
    call_hash: CallHash,
}

#[derive(Clone, Default, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct State<Value> {
    votes: u32,
    yas: Value,
    nos: Value,
    sum: Value,
}

impl<Value: AtLeast32Bit + Copy> State<Value> {
    pub(crate) fn add(&mut self, value: Value, sign: Sign) {
        self.votes += 1;
        self.sum += value;
        match sign {
            Sign::Positive => self.yas += value,
            Sign::Negative => self.nos -= value,
            _ => (),
        }
    }

    pub(crate) fn remove(&mut self, value: Value, sign: Sign) {
        self.votes -= 1;
        self.sum -= value;
        match sign {
            Sign::Positive => self.yas -= value,
            Sign::Negative => self.nos += value,
            _ => (),
        }
    }

    pub(crate) fn value(&self) -> Value {
        if self.yas < self.nos {
            Value::zero()
        } else {
            self.yas - self.nos
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sum.is_zero() && self.yas.is_zero() && self.nos.is_zero() && self.votes == 0
    }

    pub(crate) fn is_reached(&self, threshold: Threshold<Value>, total: Value, limit: Value) -> bool {
        let v = self.value();
        use Threshold::*;
        match threshold {
            Absolute(x) => v >= x,
            Relative(x) => v >= get_relative_balance(x, total, limit),
            RelativeExcept(x) => v > get_relative_balance(x, total, limit),
        }
    }

    pub(crate) fn is_fullfilled(&self, threshold: Threshold<Value>, total: Value, limit: Value) -> bool {
        let s = total - self.sum;
        let v = self.value() + s; // max attainable
        use Threshold::*;
        match threshold {
            Absolute(x) => v < x,
            Relative(x) => v < get_relative_balance(x, total, limit),
            RelativeExcept(x) => v <= get_relative_balance(x, total, limit),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Sign {
    Positive,
    Negative,
    Neutral,
}

#[derive(Clone, Copy, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Threshold<Value> {
    Absolute(Value),
    Relative(Value),
    RelativeExcept(Value),
}

impl<Account, Asset, Time: PartialOrd, Value> Voting<Account, Asset, Time, Value> {
    pub fn is_actual(&self, time: &Time) -> bool {
        time >= &self.start && self.end.as_ref().map_or(true, |end| time <= end)
    }
}

type OpaqueCall<T> = WrapperKeepOpaque<<T as Config>::Call>;

type CallHash = [u8; 32];

type VotingId = [u8; 32];

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::Call as SystemCall;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The overarching call type.
        type Call: Parameter
            + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + Codec
            + From<SystemCall<Self>>;

        /// The currency mechanism.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The base amount of currency needed to reserve for creating a multisig execution or to
        /// store a dispatch call for later.
        ///
        /// This is held for an additional storage item whose value size is
        /// `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and whose key size is
        /// `32 + sizeof(AccountId)` bytes.
        type DepositBase: Get<BalanceOf<Self>>;

        /// Asset identifier
        type AssetId: Member + Parameter + Default + Copy;

        /// Asset balance value
        type AssetBalance: Member + Parameter + AtLeast32Bit + Default + Copy;

        /// Assets storage/provider
        type Assets: NFTImplT<
            Account = Self::AccountId,
            Fingerprint = Self::AssetId,
            FractionAmount = Self::AssetBalance,
        >;

        /// Asset system object
        //type Asset: NFTokenFractionT<Self::AssetImpl>;

        #[pallet::constant]
        /// Max value for relative threshold, it's equivalent to 100%
        type RelativeThresholdLimit: Get<Self::AssetBalance>;

        #[pallet::constant]
        /// Max value for relative threshold, it's equivalent to 100%
        type MaxVotesPerAccountAsset: Get<u16>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// The set of open voting operations.
    #[pallet::storage]
    pub type Votings<T: Config> = StorageMap<_, Blake2_128Concat, VotingId, VotingOf<T>>;

    /// The set of open voting operation states.
    #[pallet::storage]
    pub type States<T: Config> = StorageMap<_, Blake2_128Concat, VotingId, State<T::AssetBalance>>;

    /// The set of votes. [Need to unlock holders' assets]
    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        (T::AccountId, T::AssetId),
        Blake2_128Concat,
        VotingId,
        (Sign, Guard),
    >;

    /// The set of call data to be executed and reserved balance for it
    #[pallet::storage]
    pub type Calls<T: Config> =
        StorageMap<_, Identity, CallHash, (OpaqueCall<T>, T::AccountId, BalanceOf<T>)>;

    #[pallet::storage]
    pub type NextGuard<T: Config> = StorageValue<_, Guard, ValueQuery>;

    #[pallet::storage]
    pub type Guards<T: Config> = StorageMap<_, Blake2_128Concat, Guard, HoldGuardOf<T>>;

    #[pallet::error]
    pub enum Error<T> {
        /// Voting for the call is already exists and pending
        AlreadyExists,
        /// Call is already voted by this signatory
        AlreadyVoted,
        /// Voting exists and pending
        StillProcessing,
        /// Bad voting state
        BadState,
        /// Call isn't voted by this signatory
        NotVoted,
        /// Unknown asset or unexpected total issuance
        BadAsset,
        /// Unknown depositoraccount
        UnknownDepositor,
        /// Insufficent asset minimum balance for an account
        InsufficientAssetBalance,
        /// Voting operation wasn't found
        NotFound,
        /// Voting state wasn't found in storage
        StateNotFound,
        /// Origin hasn't access to call the operation
        PermissionDenied,
        /// A different timepoint was given to the voting operation that is underway.
        BadTimepoint,
        /// Call data wasn't found
        NoCall,
        /// Threshold value is out of bounds
        BadThresholdValue,
        /// Reserved balance has unexpected low value
        UnexpectedLowReservedBalance,
        /// Unexpected call data or unknown encoding format
        BadCallEncoding,
        /// Too much votings with the asset are running
        LimitVotingsPerAsset,
        /// The maximum weight information provided was too low.
        MaxWeightTooLow,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new voting operation has begun.
        Created { id: VotingId, voting: VotingOf<T> },
        /// The asset's holder has made voting update (voted/unvoted).
        Updated { id: VotingId, author: T::AccountId },
        /// A voting operation has been finished, its call has been executed.
        Executed { id: VotingId, voting: VotingOf<T>, result: DispatchResult },
        /// A voting has been closed by its author.
        Closed { id: VotingId, voting: VotingOf<T> },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new voting operation
        ///
        /// Payment: `DepositBase` will be reserved.
        /// It is returned once this dispatch happens or is cancelled.
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `id`: Voting unique identifier
        /// - `asset`: Asset identifier to restrict a voting around it
        /// - `start`: Voting activation timepoint (optional); initialized with the extrinsic call timepoint if it's empty
        /// - `end`: Voting deactivation timepoint (optional); permanent voting if it's empty
        /// - `threshold`: Absolute or relative asset balance threshold; minimum sum of asset holders' balances for operation to be executed
        /// - `call`: The call to be executed
        #[pallet::weight({
			let z = call.encoded_len() as u32;
			(
				T::WeightInfo::create(z),
				DispatchClass::Normal
			)
		})]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            id: VotingId,
            asset: T::AssetId,
            start: Option<TimeOf<T>>,
            end: Option<TimeOf<T>>,
            threshold: ThresholdOf<T>,
            call: OpaqueCall<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let zero = T::AssetBalance::zero();
            let max_balance = Self::total(asset)?;
            match threshold {
                Threshold::Absolute(v) => {
                    ensure!(v > zero && v <= max_balance, Error::<T>::BadThresholdValue);
                },
                Threshold::Relative(v) => {
                    let limit = T::RelativeThresholdLimit::get();
                    ensure!(v > zero && v <= limit, Error::<T>::BadThresholdValue);
                },
                Threshold::RelativeExcept(v) => {
                    let limit = T::RelativeThresholdLimit::get();
                    ensure!(v >= zero && v < limit, Error::<T>::BadThresholdValue);
                },
            }
            ensure!(!Votings::<T>::contains_key(&id), Error::<T>::AlreadyExists);
            ensure!(Self::is_valid_stakeholder(&who, asset), Error::<T>::PermissionDenied);
            let encoded_call = call.encoded();
            let call_len = encoded_call.len();
            let call_hash = blake2_256(encoded_call);
            let start = start.unwrap_or_else(|| Self::timepoint());
            ensure!(end.map(|t| t > start).unwrap_or(true), Error::<T>::BadTimepoint);
            let v = Self::new_voting(who.clone(), asset, start, end, threshold, call_hash);
            let deposit = T::DepositBase::get() + BalanceOf::<T>::from((call_len + 31 / 32) as u32);
            T::Currency::reserve(&who, deposit)?;
            Calls::<T>::insert(&call_hash, (call, who, deposit));
            Votings::<T>::insert(id, v.clone());
            Self::deposit_event(Event::<T>::Created { id, voting: v });
            Ok(())
        }

        /// Add a new vote into the active voting operation
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `id`: Voting unique identifier
        /// - `sign`: Vote value (sign: positive (yes) | neutral | negative (no))
        /// - `max_weight`: Maximum call execution weight
        #[pallet::weight({
			(
				T::WeightInfo::vote()
				.max(T::WeightInfo::vote_and_execute())
				.saturating_add(*max_weight),
				DispatchClass::Normal
			)
		})]
        #[transactional]
        pub fn vote(
            origin: OriginFor<T>,
            id: VotingId,
            sign: Sign,
            max_weight: Weight,
        ) -> DispatchResultWithPostInfo {
            let voter = ensure_signed(origin)?;
            let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
            let asset = v.asset;
            let key = &(voter.clone(), asset);
            ensure!(!Votes::<T>::contains_key(key, id), Error::<T>::AlreadyVoted);
            ensure!(Self::is_valid_stakeholder(&voter, asset), Error::<T>::PermissionDenied);
            let time = Self::timepoint();
            ensure!(v.is_actual(&time), Error::<T>::BadTimepoint);
            let state = Self::put_vote(&voter, asset, id, sign)?;
            let total = Self::total(asset)?;
            if state.is_reached(v.threshold, total, T::RelativeThresholdLimit::get()) {
                let res = Self::execute_call(id, v, max_weight)?.actual_weight;
                let _ = Self::remove_vote(&voter, asset, id)?;
                Ok(res.map(|w| T::WeightInfo::vote_and_execute().saturating_add(w)).into())
            } else {
                Self::deposit_event(Event::<T>::Updated { id, author: voter });
                // TODO cancel if it's fulfilled
                // if threshold - balance > total - completeness
                Ok(Some(T::WeightInfo::vote()).into())
            }
        }

        /// Remove vote from the active voting operation
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `id`: Voting unique identifier
        /// - `max_weight`: Maximum call execution weight
        #[pallet::weight({
			(
				T::WeightInfo::unvote()
				.max(T::WeightInfo::unvote_last())
				.max(T::WeightInfo::unvote_and_execute())
				.saturating_add(*max_weight),
				DispatchClass::Normal
			)
		})]
        #[transactional]
        pub fn unvote(
            origin: OriginFor<T>,
            id: VotingId,
            max_weight: Weight,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
            let asset = v.asset;
            let state = Self::pop_vote(&who, asset, id)?;
            let w = if state.votes == 0 {
                T::WeightInfo::unvote_last()
            } else {
                let time = Self::timepoint();
                if v.is_actual(&time) {
                    let total = Self::total(asset)?;
                    if state.is_reached(v.threshold, total, T::RelativeThresholdLimit::get()) {
                        let res = Self::execute_call(id, v, max_weight)?.actual_weight;
                        return Ok(res
                            .map(|w| T::WeightInfo::unvote_and_execute().saturating_add(w))
                            .into());
                    } else {
                        Self::deposit_event(Event::<T>::Updated { id, author: who.clone() });
                    }
                }
                T::WeightInfo::unvote()
            };
            Ok(Some(w).into())
        }

        /// Manually execute the voting when the threshold is reached
        /// by some specific reasons like burning or minting the asset fractions
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `id`: Voting unique identifier
        /// - `max_weight`: Maximum call execution weight
        #[pallet::weight({
			(
				T::WeightInfo::execute()
				.saturating_add(*max_weight),
				DispatchClass::Normal
			)
		})]
        #[transactional]
        pub fn execute(
            origin: OriginFor<T>,
            id: VotingId,
            max_weight: Weight,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
            let asset = v.asset;
            ensure!(Self::is_valid_stakeholder(&who, asset), Error::<T>::PermissionDenied);
            let time = Self::timepoint();
            ensure!(v.is_actual(&time), Error::<T>::BadTimepoint);
            let state = States::<T>::get(&id).ok_or_else(|| Error::<T>::StateNotFound)?;
            let total = Self::total(asset)?;
            let limit = T::RelativeThresholdLimit::get();
            ensure!(state.is_reached(v.threshold, total, limit), Error::<T>::BadState);
            let res = Self::execute_call(id, v, max_weight)?.actual_weight;
            let key = (who.clone(), asset);
            if Votes::<T>::contains_key(&key, &id) {
                let _ = Self::remove_vote(&who, asset, id)?;
            }
            Ok(res.map(|w| T::WeightInfo::execute().saturating_add(w)).into())
        }

        /// Close voting if there is no votes or it's fullfilled
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `id`: Voting unique identifier
        #[pallet::weight({
            (T::WeightInfo::close(), DispatchClass::Normal)
        })]
        #[transactional]
        pub fn close(
            origin: OriginFor<T>,
            id: VotingId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
            ensure!(who == v.author, Error::<T>::PermissionDenied);
            if let Some(state) = States::<T>::get(&id) {
                let total = Self::total(v.asset)?;
                let limit = T::RelativeThresholdLimit::get();
                ensure!(!state.is_reached(v.threshold, total, limit), Error::<T>::BadState);
                let can_close = state.votes == 0
                || state.is_fullfilled(v.threshold, total, limit)
                || !v.is_actual(&Self::timepoint());
                ensure!(can_close, Error::<T>::StillProcessing);
            }
            Self::close_voting(id, &v)?;
            Self::deposit_event(Event::<T>::Closed { id, voting: v.clone() });
            Ok(())
        }

        /// Return control on the asset to its holder
        ///
        /// The dispatch origin for this call must be _Signed_.
        ///
        /// - `asset`: Asset identifier to be unlocked for the holder (caller)
        #[pallet::weight((T::WeightInfo::retain_asset(T::MaxVotesPerAccountAsset::get() as u32), DispatchClass::Normal))]
        #[transactional]
        pub fn retain_asset(origin: OriginFor<T>, asset: T::AssetId) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let key = (who.clone(), asset);
            let ids: Vec<_> = Votes::<T>::iter_prefix(&key).map(|p| p.0).collect();
            let num = ids.len() as u32;
            for id in ids {
                ensure!(!Votings::<T>::contains_key(&id), Error::<T>::StillProcessing);
                let _ = Self::remove_vote(&who, asset, id)?;
            }
            Ok(Some(T::WeightInfo::retain_asset(num)).into())
        }

        /*
        // TODO
        #[pallet::weight((T::WeightInfo::cancel_all(), DispatchClass::Normal))]
        pub fn cancel_all(
            origin: OriginFor<T>,
            asset: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            todo!()
        }
        */
    }
}

impl<T: Config> Pallet<T> {
    // by analogy with substrate's frames proxy and multisig
    pub fn voting_account_id(asset: T::AssetId, threshold: ThresholdOf<T>) -> T::AccountId {
        let entropy = (b"modeip/stakevoting", asset, threshold).using_encoded(blake2_256);
        Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }

    pub(crate) fn new_voting(
        author: T::AccountId,
        asset: T::AssetId,
        start: TimeOf<T>,
        end: Option<TimeOf<T>>,
        threshold: ThresholdOf<T>,
        call_hash: CallHash,
    ) -> VotingOf<T> {
        let delegate = Self::voting_account_id(asset, threshold);
        VotingOf::<T> { author, asset, start, end, threshold, delegate, call_hash }
    }

    #[inline]
    pub fn get_voting(id: &VotingId) -> Option<VotingOf<T>> {
        Votings::<T>::get(id)
    }

    fn is_valid_stakeholder(account: &T::AccountId, asset: T::AssetId) -> bool {
        let zero = T::AssetBalance::zero();
        Self::balance(asset, account).map(|v| v > zero).unwrap_or_default()
    }

    #[inline]
    fn balance(asset: T::AssetId, account: &T::AccountId) -> Option<T::AssetBalance> {
        pick_fraction::<T::Assets>(account, asset).map(|v| *v.amount()).ok()
    }

    #[inline]
    fn total(asset: T::AssetId) -> Result<T::AssetBalance, Error<T>> {
        total_fraction::<T::Assets>(asset).ok_or_else(|| Error::<T>::BadAsset)
    }

    fn take_guard() -> u32 {
        for (g, _) in Guards::<T>::drain() {
            return g.into();
        }
        NextGuard::<T>::mutate(|i| {
            *i = *i + 1;
            *i
        })
        .into()
    }

    #[inline]
    fn release_guard(guard: u32) {
        Guards::<T>::insert::<u32, HoldGuardOf<T>>(guard, guard.into())
    }

    fn hold(asset: T::AssetId, account: &T::AccountId) -> Result<u32, Error<T>> {
        let guard = Self::take_guard();
        hold_fraction::<T::Assets>(account, asset, Self::holder_id(), guard.into())
            .map_err(|_| Error::<T>::PermissionDenied)?;
        Ok(guard)
    }

    fn release(asset: T::AssetId, account: &T::AccountId, guard: u32) -> Result<(), Error<T>> {
        unhold_fraction::<T::Assets>(account, asset, Self::holder_id(), guard.into())
            .map_err(|_| Error::<T>::PermissionDenied)?;
        Self::release_guard(guard);
        Ok(())
    }

    fn holder_id() -> <T::Assets as NFTImplT>::FractionHolderId {
        let entropy = (b"modl/", Self::name().as_bytes()).using_encoded(blake2_256);
        Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
            .expect("infinite length input; no invalid inputs for type; qed")
    }

    fn close_voting(id: VotingId, v: &VotingOf<T>) -> DispatchResult {
        Votings::<T>::remove(&id);
        States::<T>::remove(&id);
        let (_call, depositor, deposit) =
            Calls::<T>::take(&v.call_hash).ok_or_else(|| Error::<T>::NoCall)?;
        ensure!(depositor == v.author, Error::<T>::UnknownDepositor);
        let reserved = T::Currency::reserved_balance(&depositor);
        ensure!(reserved >= deposit, Error::<T>::UnexpectedLowReservedBalance);
        T::Currency::unreserve(&depositor, deposit); // should be reserved within `create` call
        Ok(())
    }

    fn put_vote(
        voter: &T::AccountId,
        asset: T::AssetId,
        id: VotingId,
        sign: Sign,
    ) -> Result<State<T::AssetBalance>, Error<T>> {
        let value =
            Self::balance(asset, voter).ok_or_else(|| Error::<T>::InsufficientAssetBalance)?;
        let key = (voter.clone(), asset);
        ensure!(!Votes::<T>::contains_key(&key, &id), Error::<T>::AlreadyVoted);
        let n = Votes::<T>::iter_prefix_values(&key).count() as u16;
        ensure!(n < T::MaxVotesPerAccountAsset::get(), Error::<T>::LimitVotingsPerAsset);
        let mut state = States::<T>::get(&id).unwrap_or_default();
        state.add(value, sign);
        let guard = Self::hold(asset, voter)?;
        Votes::<T>::insert(key, id, (sign, guard));
        States::<T>::insert(id, state.clone());
        Ok(state)
    }

    fn pop_vote(
        voter: &T::AccountId,
        asset: T::AssetId,
        id: VotingId,
    ) -> Result<State<T::AssetBalance>, DispatchError> {
        let mut state = States::<T>::take(id).ok_or_else(|| Error::<T>::StateNotFound)?;
        ensure!(state.votes > 0, Error::<T>::BadState);
        let value =
            Self::balance(asset, voter).ok_or_else(|| Error::<T>::InsufficientAssetBalance)?;
        let sign = Self::remove_vote(voter, asset, id)?;
        state.remove(value, sign);
        if state.votes > 0 {
            ensure!(!state.is_empty(), Error::<T>::BadState);
            States::<T>::insert(id, state.clone());
        } else {
            ensure!(state.is_empty(), Error::<T>::BadState);
        }
        Ok(state)
    }

    fn remove_vote(
        voter: &T::AccountId,
        asset: T::AssetId,
        id: VotingId,
    ) -> Result<Sign, DispatchError> {
        let key = (voter.clone(), asset);
        let (sign, guard) = Votes::<T>::get(&key, &id).ok_or_else(|| Error::<T>::NotVoted)?;
        Self::release(asset, voter, guard)?;
        Votes::<T>::remove(&key, &id);
        Ok(sign)
    }

    pub fn execute_call(
        id: VotingId,
        voting: VotingOf<T>,
        max_weight: Weight,
    ) -> DispatchResultWithPostInfo {
        let (data, author, balance) =
            Calls::<T>::get(&voting.call_hash).ok_or_else(|| Error::<T>::NoCall)?;
        let call = data.try_decode().ok_or_else(|| Error::<T>::BadCallEncoding)?;
        let dispatch_info = call.get_dispatch_info();
        ensure!(max_weight >= dispatch_info.weight, Error::<T>::MaxWeightTooLow);
        Votings::<T>::remove(id);
        States::<T>::remove(id);
        Calls::<T>::remove(&voting.call_hash);
        ensure!(
            T::Currency::reserved_balance(&author) >= balance,
            Error::<T>::UnexpectedLowReservedBalance
        );
        T::Currency::unreserve(&author, balance); // should be reserved within `create` call
        let call_res = call.dispatch(RawOrigin::Signed(voting.delegate.clone()).into());
        let result = call_res.map(|_| ()).map_err(|e| e.error);
        Self::deposit_event(Event::<T>::Executed { id, voting, result });
        Ok(get_result_weight(call_res).into())
    }

    /// The current `Timepoint`.
    #[inline]
    pub fn timepoint() -> Timepoint<T::BlockNumber> {
        Timepoint {
            height: <frame_system::Pallet<T>>::block_number(),
            index: <frame_system::Pallet<T>>::extrinsic_index().unwrap_or_default(),
        }
    }
}

/// Convert value relative to the limit into real balance.
#[inline]
fn get_relative_balance<B: AtLeast32Bit + Copy>(value: B, total: B, limit: B) -> B {
    (total / limit) * value + (((total % limit) * value) / limit)
}

/// Return the weight of a dispatch call result as an `Option`.
///
/// Will return the weight regardless of what the state of the result is.
#[inline]
fn get_result_weight(result: DispatchResultWithPostInfo) -> Option<Weight> {
    match result {
        Ok(post_info) => post_info.actual_weight,
        Err(err) => err.post_info.actual_weight,
    }
}
