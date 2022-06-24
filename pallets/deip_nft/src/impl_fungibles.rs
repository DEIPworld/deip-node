use deip_asset_system::{pick_fraction, pick_item, total_fraction, NFTokenFractionT};
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    sp_runtime::traits::Zero,
    traits::{
        fungibles::{Inspect, Mutate},
        tokens::{DepositConsequence, WithdrawConsequence},
    },
};

use crate::{Config, FractionAmountOf, Pallet};

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
    type AssetId = T::Hash;

    type Balance = FractionAmountOf<T>;

    fn total_issuance(item: Self::AssetId) -> Self::Balance {
        //@TODO rename total_fraction -> total_fraction_issuance.
        let asset = todo!();
        total_fraction::<Self>(asset).unwrap_or_else(Zero::zero)
    }

    fn minimum_balance(item: Self::AssetId) -> Self::Balance {
        let asset = todo!();
        // @TODO add getter for ft id to public api.
        <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::minimum_balance(asset)
    }

    fn balance(item: Self::AssetId, who: &T::AccountId) -> Self::Balance {
        let fraction = pick_fraction::<Self>(who, item).unwrap();
        let fractional_whatever_that_means = fraction.fractional();
        fractional_whatever_that_means.1
    }

    fn reducible_balance(
        item: Self::AssetId,
        who: &T::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        let asset = todo!();
        // @TODO add getter for ft id to public api.
        <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::reducible_balance(
            asset, who, keep_alive,
        )
    }

    fn can_deposit(
        item: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        let asset = todo!();
        // @TODO add getter for ft id to public api.
        <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::can_deposit(asset, who, amount)
    }

    fn can_withdraw(
        item: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        let asset = todo!();
        // @TODO add getter for ft id to public api.
        <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::can_withdraw(asset, who, amount)
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
    fn mint_into(item: Self::AssetId, who: &T::AccountId, amount: Self::Balance) -> DispatchResult {
        // @TODO add mint_fraction to api.
        todo!()
    }

    fn burn_from(
        item: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        // @TODO add burn_fraction to api.
        todo!()
    }
}
