use deip_asset_system::{pick_fraction, pick_item, total_fraction, NFTokenFractionT};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    sp_runtime::traits::Zero,
    traits::{
        fungibles::{Inspect, Mutate},
        tokens::{DepositConsequence, WithdrawConsequence},
    },
};

use crate::{AssetIdOf, Config, FractionAmountOf, Pallet};

type Assets<T> = pallet_assets::Pallet<T>;

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
    type AssetId = AssetIdOf<T>;

    type Balance = FractionAmountOf<T>;

    fn total_issuance(asset: Self::AssetId) -> Self::Balance {
        Assets::<T>::total_issuance(asset)
    }

    fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
        Assets::<T>::minimum_balance(asset)
    }

    fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
        Assets::<T>::balance(asset, who)
    }

    fn reducible_balance(
        asset: Self::AssetId,
        who: &T::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        Assets::<T>::reducible_balance(asset, who, keep_alive)
    }

    fn can_deposit(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        Assets::<T>::can_deposit(asset, who, amount)
    }

    fn can_withdraw(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        Assets::<T>::can_withdraw(asset, who, amount)
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
    fn mint_into(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        // @TODO add mint_fraction to api.
        todo!()
    }

    fn burn_from(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        // @TODO add burn_fraction to api.
        todo!()
    }
}
