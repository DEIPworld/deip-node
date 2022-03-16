#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Call as RawCall, *};
use crate as pallet_deip_investment_opportunity;

use sp_std::prelude::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        DeipDao: pallet_deip_investment_opportunity::{Pallet, Call, Storage, Config},
    }
);

frame_support::parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = sp_core::H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = u64;
    type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
    type Header = sp_runtime::testing::Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl crate::Config for TestRuntime {
    // type Event = Event;
    // type Call = Call;
}

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> sp_io::TestExternalities {
        let storage =
            frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
        sp_io::TestExternalities::from(storage)
    }
}

fn with_test_ext<R>(t: impl FnOnce() -> R) -> R {
    ExtBuilder::build().execute_with(t)
}

use frame_support::{assert_noop, assert_ok, traits::Everything};
use frame_system::RawOrigin;
use sp_std::str::FromStr;

fn last_event() -> Event {
    frame_system::Pallet::<TestRuntime>::events()
        .pop()
        .map(|e| e.event)
        .expect("Event expected")
}

fn expect_event<E: Into<Event>>(e: E) {
    assert_eq!(last_event(), e.into());
}

#[test]
#[ignore]
fn fake_test_example() {
    with_test_ext(|| {
        // ...test conditions...
    })
}
