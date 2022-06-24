use deip_asset_system::total_fraction;
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

    fn total_issuance(asset: Self::AssetId) -> Self::Balance {
        //@TODO rename total_fraction -> total_fraction_issuance.
        total_fraction::<Self>(asset).unwrap_or_else(Zero::zero)
    }

    fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
        let asset = todo!();
        // @TODO how to get asset_id.
        <pallet_assets::Pallet<T> as Inspect<T::AccountId>>::minimum_balance(asset)
    }

    fn balance(asset: Self::AssetId, who: &T::AccountId) -> Self::Balance {
        // @TODO balance method
        todo!()
    }

    fn reducible_balance(
        asset: Self::AssetId,
        who: &T::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        // @TODO
        todo!()
    }

    fn can_deposit(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        // @TODO
        todo!()
    }

    fn can_withdraw(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        // @TODO
        todo!()
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
    fn mint_into(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        todo!()
    }

    fn burn_from(
        asset: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        todo!()
    }
}
