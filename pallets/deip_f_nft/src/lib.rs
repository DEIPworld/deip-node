// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod traits;
pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{traits::GetToken, types::PayloadDetails};
    use codec::HasCompact;
    use deip_asset_lock::Result as LockResult;
    use frame_support::{
        dispatch::DispatchResult,
        ensure,
        log::error,
        pallet_prelude::{IsType, Member, StorageMap},
        sp_runtime::traits::StaticLookup,
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use sp_std::vec::Vec;

    type Uniques<T> = pallet_deip_uniques::Pallet<T>;
    type Assets<T> = pallet_deip_assets::pallet_assets::Pallet<T>;
    type AssetIdOf<T> = <T as pallet_deip_assets::pallet_assets::Config>::AssetId;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_deip_assets::Config + pallet_deip_uniques::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the F-NFT asset.
        type FNftId: Member + Parameter + Default + Copy + HasCompact;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        Fractionalized { token: AssetIdOf<T>, amount: T::Balance },
        Fused,
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub fn fractionalize(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            token: AssetIdOf<T>,
            min_balance: T::Balance,
            amount: T::Balance,
        ) -> DispatchResult {
            let account = ensure_signed(origin.clone())?;

            Uniques::<T>::lock(account.clone(), class, instance)?;

            let admin = <T::Lookup as StaticLookup>::unlookup(account);
            Assets::<T>::create(origin.clone(), token, admin.clone(), min_balance)?;

            Assets::<T>::mint(origin, token, admin, amount)?;

            error!("❗️❗️❗️ fractionalize @TODO ❗️❗️❗️");

            let event = Event::Fractionalized { token, amount };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
        pub fn fuse(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            test_id: T::FNftId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            error!("❗️❗️❗️ fuse @TODO ❗️❗️❗️");

            let event = Event::Fused;
            Self::deposit_event(event);
            Ok(())
        }
    }
}
