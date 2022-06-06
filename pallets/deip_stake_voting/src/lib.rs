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

mod tests;
mod benchmarking;
pub mod weights;

use codec::{Decode, Encode};
use frame_support::dispatch::{DispatchResult, DispatchError, DispatchResultWithPostInfo, PostDispatchInfo, Codec};
use frame_support::ensure;
use frame_support::traits::{Currency, Get, ReservableCurrency, WrapperKeepOpaque};
use frame_support::traits::fungibles::Inspect;
use frame_support::weights::{GetDispatchInfo, Weight};
use frame_support::RuntimeDebug;
use frame_system::{Config as SystemConfig, RawOrigin};
use scale_info::TypeInfo;
use sp_io::hashing::blake2_256;
use sp_runtime::traits::{AtLeast32BitUnsigned, AtLeast32Bit, TrailingZeroInput, Dispatchable, Zero};
use sp_std::prelude::*;
pub use weights::WeightInfo;

pub use pallet::*;

type TimeOf<T> = Timepoint<<T as SystemConfig>::BlockNumber>;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type ThresholdOf<T> = Threshold<<T as Config>::AssetBalance>;

type VotingOf<T> = Voting<<T as SystemConfig>::AccountId, <T as Config>::AssetId, TimeOf<T>, ThresholdOf<T>>;

// A global extrinsic index, formed as the extrinsic index within a block, together with that
/// block's height. This allows a transaction in which a multisig operation of a particular
/// composite was created to be uniquely identified.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
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
	value: Value,
	fullness: Value,
}

impl<Value: AtLeast32Bit + Copy> State<Value> {
	pub(crate) fn add(&mut self, value: Value, sign: Sign) {
		self.votes += 1;
		self.fullness += value;
		match sign {
			Sign::Positive => self.value += value,
			Sign::Negative => self.value -= value,
			_ => (),
		}
	}

	pub(crate) fn remove(&mut self, value: Value, sign: Sign) {
		self.votes -= 1;
		self.fullness -= value;
		match sign {
			Sign::Positive => self.value -= value,
			Sign::Negative => self.value += value,
			_ => (),
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
        type AssetId: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

		/// Asset balance
		type AssetBalance: Member + Parameter + AtLeast32Bit + Default + Copy;

		/// Assets storage/provider
		type Assets: Inspect<
				Self::AccountId,
				AssetId=Self::AssetId,
				Balance=Self::AssetBalance,
			>;

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
	pub type Votings<T: Config> =
		StorageMap<_, Identity, VotingId, VotingOf<T>>;

	#[pallet::storage]
	pub type States<T: Config> =
		StorageMap<_, Identity, VotingId, State<T::AssetBalance>>;

	#[pallet::storage]
	pub type Votes<T: Config> =
		StorageDoubleMap<_, Identity, (T::AccountId, T::AssetId), Identity, VotingId, Sign>;

	#[pallet::storage]
	pub type Calls<T: Config> =
		StorageMap<_, Identity, CallHash, (OpaqueCall<T>, T::AccountId, BalanceOf<T>)>;

	#[pallet::error]
	pub enum Error<T> {
		/// Voting for the call is already exists and pending
		AlreadyExists,
		/// Call is already voted by this signatory
		AlreadyVoted,
		/// Voting exists and pending
		StillProcessing,
		/// Call isn't voted by this signatory
		NotVoted,
		/// Unknown asset or unexpected total issuance
		BadAsset,
		/// Unexpected voter account
		UnexpectedVoter,
		/// Unexpected author account
		UnexpectedAuthor,
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
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new voting has begun.
		Created {
			id: VotingId,
			voting: VotingOf<T>,
		},
		/// Some asset's holder has made voting update (vote/unvote).
		Updated {
			id: VotingId,
			author: T::AccountId,
		},
		/// A voting has been executed.
		Executed {
			id: VotingId,
			voting: VotingOf<T>,
			result: DispatchResult,
		},
		/// A voting has been cancelled.
		Cancelled {
			id: VotingId,
			voting: VotingOf<T>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight((T::WeightInfo::create(call.encoded_len() as u32), DispatchClass::Normal))]
		pub fn create(
			origin: OriginFor<T>,
			asset: T::AssetId,
			start: Option<TimeOf<T>>,
			end: Option<TimeOf<T>>,
			threshold: ThresholdOf<T>,
			call: OpaqueCall<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let zero = T::AssetBalance::zero();
			let max_balance = T::Assets::total_issuance(asset);
			match threshold {
				Threshold::Absolute(v) => {
					ensure!(v > zero && v <= max_balance, Error::<T>::BadThresholdValue);
				}
				Threshold::Relative(v) => {
					let limit = T::RelativeThresholdLimit::get();
					ensure!(v > zero && v <= limit, Error::<T>::BadThresholdValue);
				}
				Threshold::RelativeExcept(v) => {
					let limit = T::RelativeThresholdLimit::get();
					ensure!(v >= zero && v < limit, Error::<T>::BadThresholdValue);
				}
			}
			ensure!(max_balance >= T::Assets::minimum_balance(asset), Error::<T>::BadAsset);
			ensure!(Self::is_valid_stakeholder(&who, asset), Error::<T>::PermissionDenied);
			let encoded_call = call.encoded();
			let call_len = encoded_call.len();
			let call_hash = blake2_256(encoded_call);
			let start = start.unwrap_or_else(|| Self::timepoint());
			ensure!(end.map(|t| t > start).unwrap_or(true), Error::<T>::BadTimepoint);
			let (id, v) = Self::create_voting(who.clone(), asset, start, end, threshold, call_hash);
			ensure!(!Votings::<T>::contains_key(&id), Error::<T>::AlreadyExists);
			let deposit = T::DepositBase::get()
				+ BalanceOf::<T>::from((call_len + 31 / 32) as u32);
			T::Currency::reserve(&who, deposit)?;
			// TODO optimize: don't touch storage if author's asset balance is enough to execute
			let state = Self::put_vote(&who, asset, id, Sign::Positive)?;
			Calls::<T>::insert(&call_hash, (call, who.clone(), deposit));
			Votings::<T>::insert(id, v.clone());
			Self::deposit_event(Event::<T>::Created { id, voting: v.clone() });
			let approved = match v.threshold {
				Threshold::Absolute(x) => state.value >= x,
				Threshold::Relative(x) => {
					state.value >= get_relative_balance(x, max_balance, T::RelativeThresholdLimit::get())
				}
				Threshold::RelativeExcept(x) => {
					state.value > get_relative_balance(x, max_balance, T::RelativeThresholdLimit::get())
				}
			};
			if approved {
				let res = Self::execute(id, v)?.actual_weight;
				Votes::<T>::remove(&(who.clone(), asset), &id);
				Self::try_return_asset(&who, asset);
				Ok(res.map(|w| {
					T::WeightInfo::create_and_execute(call_len as u32).saturating_add(w)
				}).into())
			} else {
				Ok(Some(T::WeightInfo::create(call_len as u32)).into())
			}
		}

		#[pallet::weight((T::WeightInfo::vote(), DispatchClass::Operational))]
		pub fn vote(
			origin: OriginFor<T>,
			id: VotingId,
			sign: Sign,
		) -> DispatchResultWithPostInfo {
			let voter = ensure_signed(origin)?;
			let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
			let asset = v.asset;
			ensure!(!Votes::<T>::contains_key(&(voter.clone(), asset), id), Error::<T>::AlreadyVoted);
			ensure!(Self::is_valid_stakeholder(&voter, asset), Error::<T>::PermissionDenied);
			let time = Self::timepoint();
			ensure!(v.is_actual(&time), Error::<T>::BadTimepoint);
			let state = Self::put_vote(&voter, asset, id, sign)?;
			let total = T::Assets::total_issuance(asset);
			let approved = match v.threshold {
				Threshold::Absolute(x) => state.value >= x,
				Threshold::Relative(x) => {
					state.value >= get_relative_balance(x, total, T::RelativeThresholdLimit::get())
				}
				Threshold::RelativeExcept(x) => {
					state.value > get_relative_balance(x, total, T::RelativeThresholdLimit::get())
				}
			};
			if approved {
				let res = Self::execute(id, v)?.actual_weight;
				Votes::<T>::remove(&(voter.clone(), asset), &id);
				Self::try_return_asset(&voter, asset);
				Ok(res.map(|w| {
					T::WeightInfo::vote_and_execute(0).saturating_add(w)
				}).into())
			} else {
				Self::deposit_event(Event::<T>::Updated { id, author: voter });
				// TODO cancel if it's fulfilled
				// if threshold - balance > total - completeness
				Ok(Some(T::WeightInfo::vote()).into())
			}
		}

		#[pallet::weight((T::WeightInfo::unvote(), DispatchClass::Normal))]
		pub fn cancel(
			origin: OriginFor<T>,
			id: VotingId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let v = Votings::<T>::get(&id).ok_or_else(|| Error::<T>::NotFound)?;
			let asset = v.asset;
			let state = Self::pop_vote(&who, asset, id)?;
			let w = if state.votes == 0 {
				Self::close_voting(id, &v)?;
				Self::deposit_event(Event::<T>::Cancelled { id, voting: v.clone() });
				T::WeightInfo::unvote_and_cancel()
			} else {
				let time = Self::timepoint();
				if v.is_actual(&time) {
					let total = T::Assets::total_issuance(asset);
					let approved = match v.threshold {
						Threshold::Absolute(x) => state.value >= x,
						Threshold::Relative(x) => {
							state.value >= get_relative_balance(x, total, T::RelativeThresholdLimit::get())
						}
						Threshold::RelativeExcept(x) => {
							state.value > get_relative_balance(x, total, T::RelativeThresholdLimit::get())
						}
					};
					if approved {
						let res = Self::execute(id, v)?.actual_weight;
						Votes::<T>::remove(&(who, asset), &id);
						return Ok(res.map(|w| {
							T::WeightInfo::unvote_and_execute(0).saturating_add(w)
						}).into())
					} else {
						Self::deposit_event(Event::<T>::Updated { id, author: who.clone() });
					}
				}
				T::WeightInfo::unvote()
			};
			Self::try_return_asset(&who, asset);
			Ok(Some(w).into())
		}

		#[pallet::weight((T::WeightInfo::retain_asset(), DispatchClass::Normal))]
		pub fn retain_asset(
			origin: OriginFor<T>,
			asset: T::AssetId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = (who, asset);
			let ids: Vec<_> = Votes::<T>::iter_prefix(&key).map(|p| p.0).collect();
			for id in ids {
				ensure!(!Votings::<T>::contains_key(&id), Error::<T>::StillProcessing);
				Votes::<T>::remove(&key, &id);
			}
			Self::try_return_asset(&key.0, asset);
			Ok(())
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

	pub(crate) fn create_voting(
		author: T::AccountId,
		asset: T::AssetId,
		start: TimeOf<T>,
		end: Option<TimeOf<T>>,
		threshold: ThresholdOf<T>,
		call_hash: CallHash,
	) -> (VotingId, VotingOf<T>) {
		let delegate = Self::voting_account_id(asset, threshold);
		let v = VotingOf::<T> { author, asset, start, end, threshold, delegate, call_hash };
		let id = blake2_256(&v.encode());
		(id, v)
	}

	pub fn get_voting(id: &VotingId) -> Option<VotingOf<T>> {
		Votings::<T>::get(id)
	}

	fn is_valid_stakeholder(account: &T::AccountId, asset: T::AssetId) -> bool {
		let balance = T::Assets::balance(asset, account);
		balance > T::AssetBalance::zero() && balance >= T::Assets::minimum_balance(asset)
	}

	fn close_voting(id: VotingId, v: &VotingOf<T>) -> DispatchResult {
		Votings::<T>::remove(&id);
		States::<T>::remove(&id);
		let (_call, depositor, deposit) = Calls::<T>::take(&v.call_hash).ok_or_else(|| Error::<T>::NoCall)?;
		ensure!(depositor == v.author, Error::<T>::UnexpectedAuthor);
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
		let n = Votes::<T>::iter_prefix_values((voter.clone(), asset)).count() as u16;
		ensure!(n < T::MaxVotesPerAccountAsset::get(), Error::<T>::LimitVotingsPerAsset);
		ensure!(!Votes::<T>::contains_key((voter.clone(), asset), id), Error::<T>::AlreadyVoted);
		let value = T::Assets::balance(asset, voter);
		let min = T::Assets::minimum_balance(asset);
		ensure!(value >= min, Error::<T>::InsufficientAssetBalance);
		let mut state = States::<T>::get(id).unwrap_or_default();
		state.add(value, sign);
		Votes::<T>::insert((voter.clone(), asset), id, sign);
		States::<T>::insert(id, state.clone());
		Ok(state)
	}

	fn pop_vote(
		voter: &T::AccountId,
		asset: T::AssetId,
		id: VotingId,
	) -> Result<State<T::AssetBalance>, DispatchError> {
		let mut state = States::<T>::get(id).ok_or_else(|| Error::<T>::StateNotFound)?;
		let value = T::Assets::balance(asset, voter);
		let sign = Votes::<T>::take((voter.clone(), asset), id).ok_or_else(|| Error::<T>::NotVoted)?;
		state.remove(value, sign);
		States::<T>::insert(id, state.clone());
		Ok(state)
	}

	fn try_return_asset(account: &T::AccountId, asset: T::AssetId) {
		for _ in Votes::<T>::iter_prefix(&(account.clone(), asset)) {
			return;
		}
		// TODO unlock account's asset
	}

	pub fn execute(id: VotingId, voting: VotingOf<T>) -> DispatchResultWithPostInfo {
		Votings::<T>::remove(id);
		States::<T>::remove(id);
		let call_hash = voting.call_hash;
		let call_info = Calls::<T>::take(&call_hash);
		ensure!(call_info.is_some(), Error::<T>::NoCall);
		let (data, author, balance) = call_info.unwrap();
		let call = data.try_decode().ok_or_else(|| Error::<T>::BadCallEncoding)?;
		ensure!(T::Currency::reserved_balance(&author) >= balance, Error::<T>::UnexpectedLowReservedBalance);
		T::Currency::unreserve(&author, balance); // should be reserved within `create` call
		let result = call.dispatch(RawOrigin::Signed(voting.delegate.clone()).into());
		Self::deposit_event(Event::<T>::Executed {
			id,
			voting,
			result: result.map(|_| ()).map_err(|e| e.error),
		});
		Ok(get_result_weight(result).into())
	}

	/// The current `Timepoint`.
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
