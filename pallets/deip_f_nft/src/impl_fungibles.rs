use frame_support::{
    dispatch::DispatchResult,
    sp_runtime::traits::Zero,
    traits::{
        fungibles::{Create, Inspect},
        tokens::{DepositConsequence, WithdrawConsequence},
    },
};

use crate::{Config, Pallet};

impl<T: Config<I>, I: 'static> Inspect<T::AccountId> for Pallet<T, I> {
    type AssetId = (T::ClassId, T::InstanceId);

    type Balance = T::FungibleBalance;

    fn total_issuance(nft: Self::AssetId) -> Self::Balance {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::total_issuance(ft)
        } else {
            Zero::zero()
        }
    }

    fn minimum_balance(nft: Self::AssetId) -> Self::Balance {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::minimum_balance(ft)
        } else {
            Zero::zero()
        }
    }

    fn balance(nft: Self::AssetId, who: &T::AccountId) -> Self::Balance {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::balance(ft, who)
        } else {
            Zero::zero()
        }
    }

    fn reducible_balance(
        nft: Self::AssetId,
        who: &T::AccountId,
        keep_alive: bool,
    ) -> Self::Balance {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::reducible_balance(ft, who, keep_alive)
        } else {
            Zero::zero()
        }
    }

    fn can_deposit(
        nft: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> DepositConsequence {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::can_deposit(ft, who, amount)
        } else {
            DepositConsequence::UnknownAsset
        }
    }

    fn can_withdraw(
        nft: Self::AssetId,
        who: &T::AccountId,
        amount: Self::Balance,
    ) -> WithdrawConsequence<Self::Balance> {
        if let Some(ft) = Pallet::<T, I>::class_instance_to_ft_id(nft.0, nft.1) {
            T::Fungible::can_withdraw(ft, who, amount)
        } else {
            WithdrawConsequence::UnknownAsset
        }
    }
}

impl<T: Config<I>, I: 'static> Create<T::AccountId> for Pallet<T, I> {
    fn create(
        id: Self::AssetId,
        admin: T::AccountId,
        is_sufficient: bool,
        min_balance: Self::Balance,
    ) -> DispatchResult {
        todo!()
    }
}
