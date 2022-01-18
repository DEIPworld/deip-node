use frame_support::{
    dispatch::DispatchResult,
    traits::{
        fungibles::{Inspect, Mutate},
        tokens::{DepositConsequence, WithdrawConsequence},
    },
};
use frame_system::Config as SystemConfig;
use sp_runtime::DispatchError;

use crate::{Config, Pallet};

impl<T: Config> Inspect<<T as SystemConfig>::AccountId> for Pallet<T> {
    type AssetId = <T as Config>::AssetId;

    type Balance = <T as pallet_assets::Config>::Balance;

    fn total_issuance(asset: Self::AssetId) -> Self::Balance {
        todo!()
    }

    fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
        todo!()
    }

    fn balance(asset: Self::AssetId, who: &<T as SystemConfig>::AccountId) -> Self::Balance {
        todo!()
    }

    fn reducible_balance(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        todo!()
    }

    fn can_deposit(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        todo!()
    }

    fn can_withdraw(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        todo!()
    }
}

impl<T: Config> Mutate<<T as SystemConfig>::AccountId> for Pallet<T> {
    fn mint_into(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> DispatchResult {
        todo!()
    }

    fn burn_from(
        asset: Self::AssetId,
        who: &<T as SystemConfig>::AccountId,
        amount: Self::Balance,
    ) -> Result<Self::Balance, DispatchError> {
        todo!()
    }
}
