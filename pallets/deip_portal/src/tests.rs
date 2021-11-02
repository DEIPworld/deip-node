#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate as pallet_deip_dao;
use super::{*, Event as RawEvent, Call as RawCall};

use sp_std::prelude::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        DeipDao: pallet_deip_dao::{Module, Call, Storage, Event<T>, Config},
    }
);

frame_support::parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
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
}

impl crate::Config for TestRuntime {
    type Event = Event;
    type Call = Call;
}

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
        sp_io::TestExternalities::from(storage)
    }
}

fn with_test_ext<R>(t: impl FnOnce() -> R) -> R {
    ExtBuilder::build().execute_with(t)
}

use frame_support::{assert_noop, assert_ok};
use crate::dao::*;
use sp_std::str::FromStr;
use frame_system::RawOrigin;

fn last_event() -> Event {
    frame_system::Module::<TestRuntime>::events().pop().map(|e| e.event).expect("Event expected")
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
