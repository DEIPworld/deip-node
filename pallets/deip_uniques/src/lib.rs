// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use frame_support::{dispatch::DispatchResult, sp_runtime::traits::StaticLookup};
    use frame_system::pallet_prelude::OriginFor;
    use pallet_uniques::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_uniques::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    pub struct GenesisConfig<T> {
        pub _marker: std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }

    #[cfg(feature = "std")]
    impl<T> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { _marker: std::marker::PhantomData }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            class: <T as pallet_uniques::Config>::ClassId,
            admin: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            pallet_uniques::Pallet::<T>::create(origin, class, admin)
        }
    }
}
