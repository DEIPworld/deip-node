use crate as pallet_deip;
use frame_support::{parameter_types, traits::Get};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup},
};

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub const DEFAULT_ACCOUNT_ID: <Test as system::Config>::AccountId = 123;
pub const ALICE_ACCOUNT_ID: <Test as system::Config>::AccountId = 124;
pub const BOB_ACCOUNT_ID: <Test as system::Config>::AccountId = 125;

pub const INIT_TIMESTAMP: u64 = 30_000;
pub const BLOCK_TIME: u64 = 1_000;

pub type Extrinsic = TestXt<Call, ()>;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type AccountId = u64;
type AssetId = u32;

#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DeipAssetId(pub AssetId);

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Deip: pallet_deip::{Pallet, Call, Storage, Event<T>, Config},
        Assets: pallet_assets::{Pallet, Storage, Event<T>},
        DeipAssets: pallet_deip_assets::{Pallet, Storage, Call},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 1024;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl pallet_deip::Config for Test {
    type Event = Event;
    type DeipAccountId = Self::AccountId;
    type Currency = Balances;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const AssetDeposit: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
    type Event = Event;
    type Balance = u64;
    type AssetId = AssetId;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type AssetDeposit = AssetDeposit;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub const WipePeriod: u64 = 10;
}

impl DeipProjectsInfo<AccountId> for Test {
    type ProjectId = pallet_deip::ProjectId;
    type InvestmentId = pallet_deip::InvestmentId;

    fn try_get_project_team(id: &Self::ProjectId) -> Option<AccountId> {
        Deip::try_get_project_team(id)
    }
}

impl pallet_deip_assets::Config for Test {
    type ProjectsInfo = Self;
    type DeipAccountId = Self::AccountId;
    type AssetsAssetId = AssetId;
    type AssetId = DeipAssetId;
    type WipePeriod = WipePeriod;
}

impl<LocalCall> system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub fn new_test_ext2() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (DEFAULT_ACCOUNT_ID, (1000 * <ExistentialDeposit as Get<u64>>::get()).into()),
            (ALICE_ACCOUNT_ID, (2000 * <ExistentialDeposit as Get<u64>>::get()).into()),
            (BOB_ACCOUNT_ID, (2500 * <ExistentialDeposit as Get<u64>>::get()).into()),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        Timestamp::set_timestamp(System::block_number() * BLOCK_TIME + INIT_TIMESTAMP);
    });
    ext
}
