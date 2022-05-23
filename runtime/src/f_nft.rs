use crate::{currency, Balance, Balances, Event, FNftId, Runtime};

use frame_support::parameter_types;
use pallet_assets::Pallet as Assets;
use pallet_uniques::Pallet as Uniques;

parameter_types! {
    pub const FNftDeposit: Balance = 10 * currency::UNITS;
}

impl pallet_deip_f_nft::Config for Runtime {
    type Event = Event;
    type AssetId = <Self as pallet_assets::Config>::AssetId;
    type ClassId = <Self as pallet_uniques::Config>::ClassId;
    type InstanceId = <Self as pallet_uniques::Config>::InstanceId;
    type Currency = Balances;
    type DestroyWitness = pallet_assets::DestroyWitness;
    type FNftDeposit = FNftDeposit;
    type FNftId = FNftId;
    type Fungible = Assets<Self>;
    type FungibleBalance = <Self as pallet_assets::Config>::Balance;
    type NonFungible = Uniques<Self>;
    type WeightInfo = pallet_deip_f_nft::SubstrateWeight<Self>;
}
