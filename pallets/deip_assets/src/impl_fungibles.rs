use frame_support::{
    dispatch::DispatchResult,
    traits::{
        fungibles::{Inspect, Mutate, Create, Transfer},
        tokens::{DepositConsequence, WithdrawConsequence},
    },
};
use frame_system::Config as SystemConfig;
use sp_runtime::DispatchError;
use sp_runtime::traits::AtLeast32BitUnsigned;
use deip_asset_system::error::Error as NftError;

use crate::{Config, Pallet, Error};

use deip_asset_system::FTImplT;

impl<T: Config> FTImplT for Pallet<T>
    where
        <T as pallet_assets::Config>::AssetId: AtLeast32BitUnsigned
{
    type Account = T::AccountId;
    type FTokenId = <T as pallet_assets::Config>::AssetId;
    type FTokenAmount = T::Balance;
    type NextFTokenId = crate::NextFTokenId<T>;
    type Fungibles = Self;
    type Error = Error<T>;
}

impl<T: Config> Inspect<<T as SystemConfig>::AccountId> for Pallet<T> {
    type AssetId = <T as pallet_assets::Config>::AssetId;

    type Balance = T::Balance;

    fn total_issuance(asset: Self::AssetId) -> Self::Balance {
        pallet_assets::Pallet::<T>::total_issuance(asset)
    }

    fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
        pallet_assets::Pallet::<T>::minimum_balance(asset)
    }

    fn balance(asset: Self::AssetId, who: &<T as SystemConfig>::AccountId) -> Self::Balance {
        pallet_assets::Pallet::<T>::balance(asset, who)
    }

    fn reducible_balance(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        pallet_assets::Pallet::<T>::reducible_balance(asset, who, keep_alive)
    }

    fn can_deposit(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        pallet_assets::Pallet::<T>::can_deposit(asset, who, amount)
    }

    fn can_withdraw(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        pallet_assets::Pallet::<T>::can_withdraw(asset, who, amount)
    }
}

impl<T: Config> Mutate<<T as SystemConfig>::AccountId> for Pallet<T> {
    fn mint_into(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        pallet_assets::Pallet::<T>::mint_into(asset, who, amount)
    }

    fn burn_from(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        pallet_assets::Pallet::<T>::burn_from(asset, who, amount)
    }
}

impl<T: Config> Create<<T as SystemConfig>::AccountId> for Pallet<T> {
    fn create(id: Self::AssetId, admin: <T as SystemConfig>::AccountId, is_sufficient: bool, min_balance: Self::Balance) -> DispatchResult {
        <pallet_assets::Pallet::<T> as Create<T::AccountId>>::create(id, admin, is_sufficient, min_balance)
    }
}

impl<T: Config> Transfer<<T as SystemConfig>::AccountId> for Pallet<T> {
    fn transfer(asset: Self::AssetId, source: &<T as SystemConfig>::AccountId, dest: &<T as SystemConfig>::AccountId, amount: Self::Balance, keep_alive: bool) -> Result<Self::Balance, DispatchError> {
        <pallet_assets::Pallet::<T> as Transfer<T::AccountId>>::transfer(asset, source, dest, amount, keep_alive)
    }
}

impl<T> NftError for Error<T> {
    fn bad_value() -> Self {
        todo!()
    }

    fn bad_target() -> Self {
        todo!()
    }

    fn unknown_collection() -> Self {
        todo!()
    }

    fn other() -> Self {
        todo!()
    }

    fn overflow() -> Self {
        todo!()
    }

    fn insufficient_balance() -> Self {
        todo!()
    }

    fn forbidden_for_fractionalized() -> Self {
        todo!()
    }
}