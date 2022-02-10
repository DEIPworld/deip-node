// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::MaybeSerializeDeserialize,
    traits::{Currency, LockableCurrency, ReservableCurrency},
};
use sp_std::fmt::Debug;

pub use pallet::*;
pub use pallet_balances;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
    use frame_system::pallet_prelude::OriginFor;
    use pallet_balances::WeightInfo;
    use sp_runtime::traits::StaticLookup;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_balances::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResultWithPostInfo {
            pallet_balances::Pallet::<T>::transfer(origin, dest, value)
        }

        #[pallet::weight(T::WeightInfo::transfer_all())]
        pub fn transfer_all(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            keep_alive: bool,
        ) -> DispatchResult {
            pallet_balances::Pallet::<T>::transfer_all(origin, dest, keep_alive)
        }

        #[pallet::weight(
            T::WeightInfo::set_balance_creating() // Creates a new account.
                .max(T::WeightInfo::set_balance_killing()) // Kills an existing account.
        )]
        pub fn set_balance(
            origin: OriginFor<T>,
            who: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] new_free: T::Balance,
            #[pallet::compact] new_reserved: T::Balance,
        ) -> DispatchResultWithPostInfo {
            pallet_balances::Pallet::<T>::set_balance(origin, who, new_free, new_reserved)
        }

        #[pallet::weight(T::WeightInfo::transfer_keep_alive())]
        pub fn transfer_keep_alive(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResultWithPostInfo {
            pallet_balances::Pallet::<T>::transfer_keep_alive(origin, dest, value)
        }
    }
}

impl<T: Config> ReservableCurrency<T::AccountId> for Pallet<T>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    fn can_reserve(who: &T::AccountId, value: Self::Balance) -> bool {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::can_reserve(who, value)
    }

    fn slash_reserved(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> (Self::NegativeImbalance, Self::Balance) {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::slash_reserved(who, value)
    }

    fn reserved_balance(who: &T::AccountId) -> Self::Balance {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::reserved_balance(who)
    }

    fn reserve(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::reserve(who, value)
    }

    fn unreserve(who: &T::AccountId, value: Self::Balance) -> Self::Balance {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::unreserve(who, value)
    }

    fn repatriate_reserved(
        slashed: &T::AccountId,
        beneficiary: &T::AccountId,
        value: Self::Balance,
        status: frame_support::traits::BalanceStatus,
    ) -> Result<Self::Balance, frame_support::dispatch::DispatchError> {
        <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::repatriate_reserved(
            slashed,
            beneficiary,
            value,
            status,
        )
    }
}

impl<T: Config> LockableCurrency<T::AccountId> for Pallet<T>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type Moment = <pallet_balances::Pallet<T> as LockableCurrency<T::AccountId>>::Moment;

    type MaxLocks = <pallet_balances::Pallet<T> as LockableCurrency<T::AccountId>>::MaxLocks;

    fn set_lock(
        id: frame_support::traits::LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
    ) {
        <pallet_balances::Pallet<T> as LockableCurrency<T::AccountId>>::set_lock(
            id, who, amount, reasons,
        )
    }

    fn extend_lock(
        id: frame_support::traits::LockIdentifier,
        who: &T::AccountId,
        amount: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
    ) {
        <pallet_balances::Pallet<T> as LockableCurrency<T::AccountId>>::extend_lock(
            id, who, amount, reasons,
        )
    }

    fn remove_lock(id: frame_support::traits::LockIdentifier, who: &T::AccountId) {
        <pallet_balances::Pallet<T> as LockableCurrency<T::AccountId>>::remove_lock(id, who)
    }
}

impl<T: Config> Currency<T::AccountId> for Pallet<T>
where
    T::Balance: MaybeSerializeDeserialize + Debug,
{
    type Balance = <pallet_balances::Pallet<T> as Currency<T::AccountId>>::Balance;

    type PositiveImbalance =
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::PositiveImbalance;

    type NegativeImbalance =
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::NegativeImbalance;

    fn total_balance(who: &T::AccountId) -> Self::Balance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::total_balance(who)
    }

    fn can_slash(who: &T::AccountId, value: Self::Balance) -> bool {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::can_slash(who, value)
    }

    fn total_issuance() -> Self::Balance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::total_issuance()
    }

    fn minimum_balance() -> Self::Balance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::minimum_balance()
    }

    fn burn(amount: Self::Balance) -> Self::PositiveImbalance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::burn(amount)
    }

    fn issue(amount: Self::Balance) -> Self::NegativeImbalance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::issue(amount)
    }

    fn free_balance(who: &T::AccountId) -> Self::Balance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::free_balance(who)
    }

    fn ensure_can_withdraw(
        who: &T::AccountId,
        amount: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
        new_balance: Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::ensure_can_withdraw(
            who,
            amount,
            reasons,
            new_balance,
        )
    }

    fn transfer(
        source: &T::AccountId,
        dest: &T::AccountId,
        value: Self::Balance,
        existence_requirement: frame_support::traits::ExistenceRequirement,
    ) -> frame_support::dispatch::DispatchResult {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::transfer(
            source,
            dest,
            value,
            existence_requirement,
        )
    }

    fn slash(who: &T::AccountId, value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::slash(who, value)
    }

    fn deposit_into_existing(
        who: &T::AccountId,
        value: Self::Balance,
    ) -> Result<Self::PositiveImbalance, frame_support::dispatch::DispatchError> {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::deposit_into_existing(who, value)
    }

    fn deposit_creating(who: &T::AccountId, value: Self::Balance) -> Self::PositiveImbalance {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::deposit_creating(who, value)
    }

    fn withdraw(
        who: &T::AccountId,
        value: Self::Balance,
        reasons: frame_support::traits::WithdrawReasons,
        liveness: frame_support::traits::ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, frame_support::dispatch::DispatchError> {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::withdraw(
            who, value, reasons, liveness,
        )
    }

    fn make_free_balance_be(
        who: &T::AccountId,
        balance: Self::Balance,
    ) -> frame_support::traits::SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        <pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(who, balance)
    }
}
