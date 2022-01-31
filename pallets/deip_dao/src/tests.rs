#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]

use super::{Call as RawCall, Event as RawEvent, *};
use crate as pallet_deip_dao;

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
        DeipDao: pallet_deip_dao::{Pallet, Call, Storage, Event<T>, Config},
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
    type Event = Event;
    type Call = Call;
    type DaoId = ();
    type DeipDaoWeightInfo = weights::Weights<Self>;
    type MaxSignatories = ();
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

use crate::dao::*;
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

fn plain_key_source(who: u64) -> InputAuthority<u64> {
    InputAuthority { signatories: vec![who], threshold: 0 }
}

#[test]
#[ignore]
fn fake_test_example() {
    with_test_ext(|| {
        // ...test conditions...
    })
}

#[test]
fn dao_create() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        assert_ok!(DeipDao::create(Origin::signed(who), id, plain_key_source(who), None));
        todo!()
        // assert!(matches!(
        //     last_event(),
        //     Event::pallet_deip_dao(RawEvent::DaoCreate(dao))
        //     if dao.dao_key() == &who && dao.id() == &id
        // ));
    })
}

#[test]
fn dao_create_exists() {
    with_test_ext(|| {
        let who = 1;
        let name = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        DeipDao::create(Origin::signed(who), name, plain_key_source(who), None).expect("create OK");
        assert_noop!(
            DeipDao::create(Origin::signed(who), name, plain_key_source(who), None),
            Error::<TestRuntime>::Exists,
        );
    })
}

#[test]
fn dao_transfer_ownership() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        DeipDao::create(Origin::signed(who), id, plain_key_source(who), None).expect("create OK");
        let transfer_to = 2;
        todo!()
        // assert_ok!(DeipDao::transfer_ownership(
        //     Origin::signed(who),
        //     id,
        //     transfer_to,
        //     plain_key_source(transfer_to)
        // ));
        // assert!(matches!(
        //     last_event(),
        //     Event::pallet_deip_dao(RawEvent::DaoTransferOwnership(dao))
        //     if dao.dao_key() == &transfer_to && dao.id() == &id
        // ));
    })
}

#[test]
fn dao_transfer_ownership_not_found() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        let transfer_to = 2;
        // assert_noop!(
        //     DeipDao::transfer_ownership(
        //         Origin::signed(who),
        //         id,
        //         transfer_to,
        //         plain_key_source(who)
        //     ),
        //     Error::<TestRuntime>::NotFound,
        // );
        todo!()
    })
}

#[test]
fn dao_transfer_ownership_forbidden() {
    with_test_ext(|| {
        System::set_block_number(1);
        let owner = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        DeipDao::create(Origin::signed(owner), id, plain_key_source(owner), None)
            .expect("create OK");
        let transfer_to = 2;
        let other = 3;
        // assert_noop!(
        //     DeipDao::transfer_ownership(
        //         Origin::signed(transfer_to),
        //         id,
        //         transfer_to,
        //         plain_key_source(transfer_to)
        //     ),
        //     Error::<TestRuntime>::Forbidden,
        // );
        // assert_noop!(
        //     DeipDao::transfer_ownership(
        //         Origin::signed(other),
        //         id,
        //         transfer_to,
        //         plain_key_source(transfer_to)
        //     ),
        //     Error::<TestRuntime>::Forbidden,
        // );
        todo!()
    })
}

#[test]
fn dao_on_behalf() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        DeipDao::create(Origin::signed(who), id, plain_key_source(who), None).expect("create OK");
        let transfer_to = 2;
        // assert_ok!(DeipDao::on_behalf(
        //     Origin::signed(who),
        //     id,
        //     Box::new(Call::DeipDao(RawCall::transfer_ownership(
        //         id,
        //         transfer_to,
        //         plain_key_source(transfer_to)
        //     )))
        // ));
        todo!()
    })
}

#[test]
fn dao_on_behalf_not_found() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let id = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        let transfer_to = 2;
        assert_noop!(
            DeipDao::on_behalf(
                Origin::signed(who),
                id,
                Box::new(Call::DeipDao(RawCall::transfer_ownership(
                    id,
                    transfer_to,
                    plain_key_source(transfer_to)
                )))
            ),
            Error::<TestRuntime>::NotFound,
        );
    })
}

#[test]
fn dao_on_behalf_forbidden() {
    with_test_ext(|| {
        System::set_block_number(1);
        let who = 1;
        let name = DaoId::from_slice("test_dao\0\0\0\0\0\0\0\0\0\0\0\0".as_bytes());
        DeipDao::create(Origin::signed(who), name, plain_key_source(who), None).expect("create OK");
        let transfer_to = 2;
        assert_noop!(
            DeipDao::on_behalf(
                Origin::signed(transfer_to),
                name,
                Box::new(Call::DeipDao(RawCall::transfer_ownership(
                    name,
                    transfer_to,
                    plain_key_source(transfer_to)
                )))
            ),
            Error::<TestRuntime>::Forbidden,
        );
    })
}
