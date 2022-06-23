#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_system::{RawOrigin, Call as SystemCall};
use sp_runtime::traits::{Bounded, StaticLookup};

use crate::Pallet as StakeVoting;
use deip_asset_system::*;
use core::ops::Range;
use pallet_deip_uniques::Pallet as DeipUniques;
use pallet_deip_uniques::Config as DeipUniquesConfig;
use frame_system::Config as SystemConfig;

const SEED: u32 = 1;

/*fn assert_last_event<T: Config>(event: <T as Config>::Event) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::Event = event.into();
	// compare to the last event record
	let EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}*/

fn setup_accounts<T: Config>(
	users: u32,
) -> Result<Vec<T::AccountId>, &'static str> {
	let mut holders: Vec<T::AccountId> = Vec::new();
	for i in 0..users {
		let holder = account::<T::AccountId>("user", i, SEED);
		whitelist_account!(holder);
		// Give them some balance for a possible deposit
		let balance = BalanceOf::<T>::max_value();
		T::Currency::make_free_balance_be(&holder, balance);
		holders.push(holder);
	}
	Ok(holders)
}

fn gen_call<T: Config>(
	size: u32,
) -> Result<OpaqueCall<T>, &'static str> {
	let call: <T as Config>::Call =
		SystemCall::<T>::remark { remark: vec![0; size as usize] }.into();
	let call_data = OpaqueCall::<T>::from_encoded(call.encode());
	Ok(call_data)
}

fn create_asset<T>(_min: T::AssetBalance) -> (<T as Config>::AssetId, T::AccountId)
where
	T: Config + DeipUniquesConfig, 
	<T as Config>::AssetId: From<u32>
{
	let caller: T::AccountId = whitelisted_caller();
	let source = T::Lookup::unlookup(caller.clone());
	let balance = BalanceOf::<T>::max_value();
	<T as Config>::Currency::make_free_balance_be(&caller, balance);
	let asset = Default::default();
	let collection = create_collection::<DeipUniques<T>>(&caller, 1).unwrap();
	let asset = 1u32.into();
	let unique = OpaqueUnique(asset);
	mint_item::<DeipUniques<T>>(collection, &caller, unique).unwrap();
	(asset, caller)
}

fn distribute_asset<T>(
	admin: T::AccountId,
	asset: <T as Config>::AssetId,
	amount: T::AssetBalance, 
	accounts: &[T::AccountId],
)
where
	T: Config<
			AssetId = <T as SystemConfig>::Hash,
			AssetBalance = T::Balance,
		>
		+ DeipUniquesConfig<
		>,
{
	assert!(!accounts.is_empty());
	assert!(!amount.is_zero());
	let total = amount * (accounts.len() as u32).into();
	fractionalize_item::<DeipUniques<T>>(asset, &admin, total).unwrap();
	for u in accounts {
		let source = T::Lookup::unlookup(u.clone());
		transfer_fraction::<DeipUniques<T>>(asset, &admin, u, amount).unwrap();
	}
}

fn create_voting<T: Config>(
	author: T::AccountId,
	asset: T::AssetId,
	start: TimeOf<T>,
	end: Option<TimeOf<T>>,
	threshold: ThresholdOf<T>,
	call_size: u32,
) -> (VotingId, VotingOf<T>) {
	let call = gen_call::<T>(call_size).unwrap();
	let call_hash = blake2_256(call.encoded());
	let (id, voting) = StakeVoting::<T>::new_voting(
		author.clone(),
		asset,
		start,
		end,
		threshold,
		call_hash
	);
	StakeVoting::<T>::create(
		RawOrigin::Signed(author).into(),
		asset,
		Some(start),
		end,
		threshold,
		call,
		max_weight(),
	).unwrap();
	(id, voting)
}

fn now<T: Config>() -> TimeOf<T> {
	StakeVoting::<T>::timepoint()
}

fn random_range(r: Range<u32>) -> u32 {
	// TODO
	r.start
}

fn max_weight() -> Weight {
	1000000u64
}

benchmarks! {
	where_clause {
		where T: Config + DeipUniquesConfig,
			<T as Config>::AssetId: From<T::Hash>,
	}
	create {
		let z in 0 .. 1000000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let asset = asset.into();
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let end = None;
		let call = gen_call::<T>(z)?;
		let call_hash = blake2_256(call.encoded());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = StakeVoting::<T>::new_voting(caller.clone(), asset, time, end, threshold, call_hash);
	}: create(RawOrigin::Signed(caller.clone()), asset, Some(time), end, threshold, call.clone(), max_weight())
	verify {
		let value: T::AssetBalance = value.into();
		let fr = T::Asset::pick_fraction(asset, &caller).unwrap();
		assert_eq!(*fr.amount(), value);
		assert!(<T as Config>::Currency::reserved_balance(&caller) > 0u32.into());
		assert_eq!(Calls::<T>::get(&voting.call_hash).map(|t| t.0), Some(call));
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let state = State { votes: 1, value, fullness: value };
		assert_eq!(States::<T>::get(&id), Some(state));
		let v = Votes::<T>::get(&(caller, asset), &id);
		assert!(v.is_some());
		assert_eq!(v.unwrap().0, Sign::Positive);
	}
	create_and_execute {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let asset = asset.into();
		let threshold = Threshold::Absolute(value.into());
		let end = None;
		let call = gen_call::<T>(z)?;
		let call_hash = blake2_256(call.encoded());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = StakeVoting::<T>::new_voting(caller.clone(), asset, time, end, threshold, call_hash);
	}: create(RawOrigin::Signed(caller.clone()), asset, Some(time), end, threshold, call, max_weight())
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		assert!(Calls::<T>::get(&call_hash).is_none());
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	vote {
		let z in 0 .. 10000;
		let n = random_range(3 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
	}: vote(RawOrigin::Signed(caller.clone()), id, Sign::Positive, max_weight())
	verify {
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let value = (value + value).into();
		let state = State { votes: 2, value, fullness: value };
		assert_eq!(States::<T>::get(&id), Some(state));
		let asset: <T as Config>::AssetId = asset.into();
		let author_vote = Votes::<T>::get(&(author, asset), &id);
		assert!(author_vote.is_some());
		assert_eq!(author_vote.unwrap().0, Sign::Positive);
		let caller_vote = Votes::<T>::get(&(caller, asset), &id);
		assert!(caller_vote.is_some());
		assert_eq!(caller_vote.unwrap().0, Sign::Positive);
	}
	vote_and_execute {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
	}: vote(RawOrigin::Signed(caller.clone()), id, Sign::Positive, max_weight())
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		let author_vote = Votes::<T>::get(&(author, asset), &id);
		assert!(author_vote.is_some());
		assert_eq!(author_vote.unwrap().0, Sign::Positive);
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote {
		let z in 0 .. 10000;
		let n = random_range(3 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(caller.clone()).into(), id, Sign::Positive, max_weight()).unwrap();
	}: cancel(RawOrigin::Signed(caller.clone()), id, max_weight())
	verify {
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let state = State { votes: 1, value: value.into(), fullness: value.into() };
		assert_eq!(States::<T>::get(&id), Some(state));
		let asset: <T as Config>::AssetId = asset.into();
		let author_vote = Votes::<T>::get(&(author, asset), &id);
		assert!(author_vote.is_some());
		assert_eq!(author_vote.unwrap().0, Sign::Positive);
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote_and_execute {
		let z in 0 .. 10000;
		let n = random_range(4 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(caller.clone()).into(), id, Sign::Negative, max_weight()).unwrap();
		let approver = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(approver.clone()).into(), id, Sign::Positive, max_weight()).unwrap();
	}: cancel(RawOrigin::Signed(caller.clone()), id, max_weight())
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		assert!(Calls::<T>::get(&voting.call_hash).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		let author_vote = Votes::<T>::get(&(author, asset), &id);
		assert!(author_vote.is_some());
		assert_eq!(author_vote.unwrap().0, Sign::Positive);
		let approver_vote = Votes::<T>::get(&(approver, asset), &id);
		assert!(approver_vote.is_some());
		assert_eq!(approver_vote.unwrap().0, Sign::Positive);
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote_and_cancel {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(caller.clone(), asset.into(), time, None, threshold, z);
	}: cancel(RawOrigin::Signed(caller.clone()), id, max_weight())
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		assert!(Calls::<T>::get(&voting.call_hash).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	retain_asset {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = 100u32.into();
		distribute_asset::<T>(admin, asset, value, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(caller.clone(), asset.into(), time, None, threshold, z);
		let approver = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(approver.clone()).into(), id, Sign::Positive, max_weight()).unwrap();
		// should be executed (or cancelled)
	}: retain_asset(RawOrigin::Signed(caller.clone()), asset.into())
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		assert!(Votes::<T>::get(&(approver, asset), &id).is_none());
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}

	impl_benchmark_test_suite!(StakeVoting, crate::tests::new_test_ext(), crate::tests::Test);
}
