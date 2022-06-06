#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_system::{RawOrigin, Call as SystemCall};
use sp_runtime::traits::{Bounded, StaticLookup};

use crate::Pallet as StakeVoting;
use pallet_assets::Pallet as Assets;
use pallet_assets::Config as AssetsConfig;
use core::ops::Range;

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

fn create_asset<T: Config + AssetsConfig> (
	min_balance: <T as AssetsConfig>::Balance,
) -> (<T as AssetsConfig>::AssetId, T::AccountId) {
	let caller: T::AccountId = whitelisted_caller();
	let source = T::Lookup::unlookup(caller.clone());
	let balance = BalanceOf::<T>::max_value();
	<T as Config>::Currency::make_free_balance_be(&caller, balance);
	let asset = Default::default();
    assert!(Assets::<T>::create(
        RawOrigin::Signed(caller.clone()).into(),
        asset,
        source.clone(),
        min_balance,
    )
    .is_ok());
	(asset, caller)
}

fn distribute_asset<T: Config + AssetsConfig>(
	admin: T::AccountId,
	asset: <T as AssetsConfig>::AssetId,
	total: <T as AssetsConfig>::Balance,
	accounts: &[T::AccountId]
) -> <T as AssetsConfig>::Balance {
	assert!(!accounts.is_empty());
	assert!(!total.is_zero());
	let amount = total / (accounts.len() as u32).into();
	for u in accounts {
		let source = T::Lookup::unlookup(u.clone());
		Assets::<T>::mint(
			RawOrigin::Signed(admin.clone()).into(),
			asset,
			source,
			amount,
		).unwrap();
	}
	amount
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
	let (id, voting) = StakeVoting::<T>::create_voting(
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

benchmarks! {
	where_clause {
		where T: Config
			+ pallet_assets::Config,
			<T as Config>::AssetId: From<<T as AssetsConfig>::AssetId>,
			<T as Config>::AssetBalance: From<<T as AssetsConfig>::Balance>,
	}
	create {
		let z in 0 .. 1000000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let asset = asset.into();
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let end = None;
		let call = gen_call::<T>(z)?;
		let call_hash = blake2_256(call.encoded());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = StakeVoting::<T>::create_voting(caller.clone(), asset, time, end, threshold, call_hash);
	}: create(RawOrigin::Signed(caller.clone()), asset, Some(time), end, threshold, call.clone())
	verify {
		let value: T::AssetBalance = value.into();
		assert_eq!(T::Assets::balance(asset, &caller), value);
		assert!(<T as Config>::Currency::reserved_balance(&caller) > 0u32.into());
		assert_eq!(Calls::<T>::get(&voting.call_hash).map(|t| t.0), Some(call));
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let state = State { votes: 1, value, fullness: value };
		assert_eq!(States::<T>::get(&id), Some(state));
		assert_eq!(Votes::<T>::get(&(caller, asset), &id), Some(Sign::Positive));
	}
	create_and_execute {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let asset = asset.into();
		let threshold = Threshold::Absolute(value.into());
		let end = None;
		let call = gen_call::<T>(z)?;
		let call_hash = blake2_256(call.encoded());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = StakeVoting::<T>::create_voting(caller.clone(), asset, time, end, threshold, call_hash);
	}: create(RawOrigin::Signed(caller.clone()), asset, Some(time), end, threshold, call)
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
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
	}: vote(RawOrigin::Signed(caller.clone()), id, Sign::Positive)
	verify {
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let value = (value + value).into();
		let state = State { votes: 2, value, fullness: value };
		assert_eq!(States::<T>::get(&id), Some(state));
		let asset: <T as Config>::AssetId = asset.into();
		assert_eq!(Votes::<T>::get((author, asset), &id), Some(Sign::Positive));
		assert_eq!(Votes::<T>::get((caller, asset), &id), Some(Sign::Positive));
	}
	vote_and_execute {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
	}: vote(RawOrigin::Signed(caller.clone()), id, Sign::Positive)
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		assert_eq!(Votes::<T>::get(&(author, asset), &id), Some(Sign::Positive));
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote {
		let z in 0 .. 10000;
		let n = random_range(3 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(caller.clone()).into(), id, Sign::Positive).unwrap();
	}: cancel(RawOrigin::Signed(caller.clone()), id)
	verify {
		assert_eq!(Votings::<T>::get(&id), Some(voting));
		let state = State { votes: 1, value: value.into(), fullness: value.into() };
		assert_eq!(States::<T>::get(&id), Some(state));
		let asset: <T as Config>::AssetId = asset.into();
		assert_eq!(Votes::<T>::get(&(author, asset), &id), Some(Sign::Positive));
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote_and_execute {
		let z in 0 .. 10000;
		let n = random_range(4 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let author = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(author.clone(), asset.into(), time, None, threshold, z);
		let caller = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(caller.clone()).into(), id, Sign::Negative).unwrap();
		let approver = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(approver.clone()).into(), id, Sign::Positive).unwrap();
	}: cancel(RawOrigin::Signed(caller.clone()), id)
	verify {
		assert!(Votings::<T>::get(&id).is_none());
		assert!(States::<T>::get(&id).is_none());
		assert!(Calls::<T>::get(&voting.call_hash).is_none());
		let asset: <T as Config>::AssetId = asset.into();
		assert_eq!(Votes::<T>::get(&(author, asset), &id), Some(Sign::Positive));
		assert_eq!(Votes::<T>::get(&(approver, asset), &id), Some(Sign::Positive));
		assert!(Votes::<T>::get(&(caller, asset), &id).is_none());
	}
	unvote_and_cancel {
		let z in 0 .. 10000;
		let n = random_range(2 .. 100);
		let mut holders = setup_accounts::<T>(n)?;
		let min = 1u32.into();
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let threshold = Threshold::Relative(T::RelativeThresholdLimit::get());
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(caller.clone(), asset.into(), time, None, threshold, z);
	}: cancel(RawOrigin::Signed(caller.clone()), id)
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
		let total = 1000u32.into();
		let (asset, admin) = create_asset::<T>(min);
		let value = distribute_asset::<T>(admin, asset, total, &holders);
		let thr_value = (value + value).into();
		let threshold = Threshold::Absolute(thr_value);
		let caller = holders.pop().unwrap();
		let time = now::<T>();
		let (id, voting) = create_voting::<T>(caller.clone(), asset.into(), time, None, threshold, z);
		let approver = holders.pop().unwrap();
		StakeVoting::<T>::vote(RawOrigin::Signed(approver.clone()).into(), id, Sign::Positive).unwrap();
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
