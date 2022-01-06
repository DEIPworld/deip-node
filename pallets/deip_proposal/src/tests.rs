#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate as pallet_deip_proposal;
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
        // Utility: pallet_utility::{Module, Call, Event},
        // RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        // Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        // Aura: pallet_aura::{Module, Config<T>},
        // Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event},
        // Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        // TransactionPayment: pallet_transaction_payment::{Module, Storage},
        // Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
        // // Include the custom logic from the template pallet in the runtime.
        // TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
        // Deip: pallet_deip::{Module, Call, Storage, Event<T>, Config},
        Proposal: pallet_deip_proposal::{Module, Call, Storage, Event<T>, Config},
        // Multisig: pallet_multisig::{Module, Call, Storage, Event<T>, Config},
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

#[test]
fn decide_on_not_exist_proposal() {
    with_test_ext(|| {
        assert_noop!(
            Proposal::decide(Origin::signed(1), ProposalId::default(), ProposalMemberDecision::Pending),
            Error::<TestRuntime>::NotFound,
        );
    })
}

#[test]
fn create_proposal_emits_event() {
    with_test_ext(|| {
        System::set_block_number(1);
        assert_ok!(Proposal::propose(Origin::signed(0), Vec::new()));
        match last_event() {
            self::Event::pallet_deip_proposal(
                RawEvent::Proposed {
                    author: _,
                    batch: _,
                    proposal_id: _,
                    ..
                }) => {},
            _ => { unreachable!() }
        }
    })
}

#[test]
fn assert_nested_proposals_limit() {
    with_test_ext(|| {
        let author = 0;
        let batch = vec![
            ProposalBatchItemOf::<TestRuntime> {
                account: author,
                call: Call::Proposal(RawCall::propose(vec![
                    ProposalBatchItemOf::<TestRuntime> {
                        account: author,
                        call: Call::Proposal(RawCall::propose(vec![
                            ProposalBatchItemOf::<TestRuntime> {
                                account: author,
                                call: Call::Proposal(RawCall::propose(vec![])),
                            }
                        ])),
                    }
                ])),
            }
        ];
        // System::set_block_number(1);
        let origin = Origin::signed(0);
        assert_noop!(
            Proposal::propose(origin, batch),
            Error::<TestRuntime>::ReachDepthLimit
        );
    })
}

// #[test]
// fn create_proposal {
//     with_test_ext(|| {
//         assert_noop!(
//             Proposal::decide(Origin::signed(1), ProposalId::default(), ProposalMemberDecision::Pending),
//             Error::<TestRuntime>::NotFound,
//         );
//         let author = Origin::signed(0);
//         let batch = Vec::new();
//         System::set_block_number(1);
//         assert_ok!(Proposal::propose(author, batch));
//         match last_event() {
//             self::Event::pallet_deip_proposal(
//                 RawEvent::Proposed {
//                     author,
//                     batch,
//                     proposal_id
//                 }) => {},
//             _ => { unreachable!() }
//         }
//     })
// }
