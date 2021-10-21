use crate as pallet_deip;
use frame_support::{parameter_types, traits::Get};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup},
};

use deip_assets_error::{ReserveError, UnreserveError};

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

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Deip: pallet_deip::{Module, Call, Storage, Event<T>, Config},
        Assets: pallet_assets::{Module, Storage, Event<T>},
        // DeipAssets: pallet_deip_assets::{Module, Storage, Call},
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
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 1024;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl pallet_deip::traits::DeipAssetSystem<u64> for Test {
    type Balance = u64;
    type AssetId = u32;

    fn try_get_tokenized_project(id: &Self::AssetId) -> Option<super::ProjectId> {
        DeipAssets::try_get_tokenized_project(id)
    }

    fn account_balance(account: &AccountId, asset: &Self::AssetId) -> Self::Balance {
        DeipAssets::account_balance(account, asset)
    }

    fn total_supply(asset: &Self::AssetId) -> Self::Balance {
        DeipAssets::total_supply(asset)
    }

    fn get_project_nfts(id: &super::ProjectId) -> Vec<Self::AssetId> {
        DeipAssets::get_project_nfts(id)
    }

    fn get_nft_balances(id: &Self::AssetId) -> Option<Vec<AccountId>> {
        DeipAssets::get_nft_balances(id)
    }

    fn transactionally_transfer(
        from: &AccountId,
        asset: Self::AssetId,
        transfers: &[(Self::Balance, AccountId)],
    ) -> Result<(), ()> {
        DeipAssets::transactionally_transfer(from, asset, transfers)
    }

    fn transactionally_reserve(
        account: &u64,
        id: super::InvestmentId,
        security_tokens_on_sale: &[(Self::AssetId, Self::Balance)],
        asset: Self::AssetId,
    ) -> Result<(), ReserveError<Self::AssetId>> {
        DeipAssets::transactionally_reserve(account, id, security_tokens_on_sale, asset)
    }

    fn transactionally_unreserve(id: super::InvestmentId) -> Result<(), UnreserveError<Self::AssetId>> {
        DeipAssets::transactionally_unreserve(id)
    }

    fn transfer_from_reserved(
        id: super::InvestmentId,
        who: &u64,
        asset: Self::AssetId,
        amount: Self::Balance,
    ) -> Result<(), UnreserveError<Self::AssetId>> {
        DeipAssets::transfer_from_reserved(id, who, asset, amount)
    }

    fn transfer_to_reserved(
        who: &AccountId,
        id: super::InvestmentId,
        amount: Self::Balance,
    ) -> Result<(), UnreserveError<Self::AssetId>> {
        DeipAssets::transfer_to_reserved(who, id, amount)
    }
}

impl pallet_deip::Config for Test {
    type Event = Event;
    type DeipAccountId = Self::AccountId;
    type Currency = Balances;
    type AssetSystem = Self;
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
    pub const AssetDepositBase: Balance = 0;
    pub const AssetDepositPerZombie: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
    type Event = Event;
    type Balance = u64;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type AssetDepositBase = AssetDepositBase;
    type AssetDepositPerZombie = AssetDepositPerZombie;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
}

parameter_types! {
	pub const WipePeriod: u64 = 10;
}

// impl pallet_deip_assets::traits::DeipProjectsInfo<AccountId> for Test {
//     type ProjectId = pallet_deip::ProjectId;
//     type InvestmentId = pallet_deip::InvestmentId;
// 
//     fn try_get_project_team(id: &Self::ProjectId) -> Option<AccountId> {
//         Deip::try_get_project_team(id)
//     }
// }

// impl pallet_deip_assets::Config for Test {
//     type ProjectsInfo = Self;
//     type DeipAccountId = Self::AccountId;
//     type WipePeriod = WipePeriod;
// }

impl<LocalCall> system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = Extrinsic;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn new_test_ext2() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (
                DEFAULT_ACCOUNT_ID,
                (1000 * <ExistentialDeposit as Get<u64>>::get()).into(),
            ),
            (
                ALICE_ACCOUNT_ID,
                (2000 * <ExistentialDeposit as Get<u64>>::get()).into(),
            ),
            (
                BOB_ACCOUNT_ID,
                (2500 * <ExistentialDeposit as Get<u64>>::get()).into(),
            ),
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
