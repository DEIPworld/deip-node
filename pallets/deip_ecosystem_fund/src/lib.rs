//! # DEIP Ecosystem Funds module
//! A module for manage DAO and perform actions on behalf of it
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests;

pub mod benchmarking;
pub mod weights;

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::{pallet_prelude::*, RawOrigin};

    use frame_support::{
        pallet_prelude::*,
        weights::{GetDispatchInfo, PostDispatchInfo},
        Hashable,
    };

    use frame_support::traits::{
        Get,
        IsSubType,
        UnfilteredDispatchable,
        StoredMap
    };

    use sp_std::{collections::btree_map::BTreeMap, iter::FromIterator, prelude::*};

    use frame_support::dispatch::DispatchResult;
    use sp_runtime::{
        traits::{Dispatchable, IdentifyAccount},
        MultiSigner,
    };

    use sp_core::H256;

    use crate::weights::WeightInfo;
    use frame_support::traits::{OnUnbalanced, Currency, Imbalance};

    pub struct DontBurnFee<X>(PhantomData<X>);
    impl<T, C> OnUnbalanced<C::NegativeImbalance> for DontBurnFee<(T, C)>
        where
            T: crate::Config,
            C: Currency<T::AccountId, Balance=T::Balance>,
    {
        fn on_nonzero_unbalanced(amount: C::NegativeImbalance) {
            let fee_recipient = FeeRecipient::<T>::get();
            let _ = T::AccountStore::mutate(&fee_recipient, |x| {
                x.free += amount.peek();
            });
        }
    }

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_balances::Config {}

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::error]
    pub enum Error<T> {}

    // #[pallet::event]
    // #[pallet::generate_deposit(pub(super) fn deposit_event)]
    // pub enum Event<T: Config> {}

    #[doc(hidden)]
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub fee_recipient: T::AccountId
    }
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { fee_recipient: <_>::default(), }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            FeeRecipient::<T>::put(self.fee_recipient.clone());
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    // ==== Storage ====:

    #[pallet::storage]
    #[pallet::getter(fn fee_recipient)]
    pub(super) type FeeRecipient<T: Config> =
        StorageValue<_, T::AccountId, ValueQuery>;
}
