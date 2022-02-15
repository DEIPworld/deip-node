#![cfg(feature = "runtime-benchmarks")]

use super::{ *};
use core::convert::TryInto;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_support::{ensure, traits::Get};
use frame_system::{Config as Sys, EventRecord, RawOrigin};
use sp_std::prelude::*;

use crate::Pallet;
use frame_support::weights::Weight;

const SEED: u32 = 0;

// fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
//     let events = frame_system::Pallet::<T>::events();
//     let system_event: <T as frame_system::Config>::Event = generic_event.into();
//     // compare to the last event record
//     let EventRecord { event, .. } = &events[events.len() - 1];
//     assert_eq!(event, &system_event);
// }

fn init_member<T: Config>(index: u32) -> T::AccountId {
    let member = account::<T::AccountId>("member", index, SEED);
    whitelist_account!(member);
    member
}

// benchmarks! {
// }