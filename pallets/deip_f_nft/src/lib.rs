// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{DepositBalanceOf, FNftDetails};
    use codec::HasCompact;
    use frame_support::{
        dispatch::DispatchResult,
        ensure,
        log::error,
        pallet_prelude::{Get, IsType, Member, StorageMap},
        sp_runtime::traits::StaticLookup,
        traits::ReservableCurrency,
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    type Uniques<T> = pallet_deip_uniques::Pallet<T>;
    type Assets<T> = pallet_deip_assets::pallet_assets::Pallet<T>;
    type AssetIdOf<T> = <T as pallet_deip_assets::pallet_assets::Config>::AssetId;
    type CurrencyOf<T, I> = <T as Config<I>>::Currency;

    #[pallet::config]
    pub trait Config<I: 'static = ()>:
        frame_system::Config + pallet_deip_assets::Config + pallet_deip_uniques::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the F-NFT asset.
        type FNftId: Member + Parameter + Default + Copy + HasCompact;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The basic amount of funds that must be reserved for an FNft asset.
        #[pallet::constant]
        type FNftDeposit: Get<DepositBalanceOf<Self, I>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        Fractionalized { token: AssetIdOf<T>, amount: T::Balance },
        Fused,
    }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// `FNftId` not found.
        FNftIdNotFound,
        /// Operation can be performed only by admin.
        WrongAdmin,
        /// `FNftId` is already taken.
        InUse,
    }

    #[pallet::storage]
    pub(super) type FNft<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::FNftId,
        FNftDetails<
            T::AccountId,
            DepositBalanceOf<T, I>,
            T::NftClassId,
            T::InstanceId,
            T::AssetsAssetId,
        >,
    >;

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(_);

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::weight(1)]
        pub fn fractionalize(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::FNftId,
            class: T::ClassId,
            instance: T::InstanceId,
            token: AssetIdOf<T>,
            min_balance: T::Balance,
            amount: T::Balance,
        ) -> DispatchResult {
            let account = ensure_signed(origin.clone())?;

            ensure!(!FNft::<T, I>::contains_key(id), Error::<T, I>::InUse);

            let deposit = T::FNftDeposit::get();
            CurrencyOf::<T, I>::reserve(&account, deposit)?;

            Uniques::<T>::lock(account.clone(), class, instance)?;

            let admin = <T::Lookup as StaticLookup>::unlookup(account.clone());
            Assets::<T>::create(origin.clone(), token, admin.clone(), min_balance)?;

            Assets::<T>::mint(origin, token, admin, amount)?;

            FNft::<T, I>::insert(
                id,
                FNftDetails {
                    owner: account.clone(),
                    issuer: account.clone(),
                    admin: account.clone(),
                    freezer: account,
                    total_deposit: deposit,
                    is_frozen: false,
                    class,
                    instance,
                    token,
                },
            );

            error!("❗️❗️❗️ fractionalize @TODO ❗️❗️❗️");

            let event = Event::Fractionalized { token, amount };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
        pub fn fuse(origin: OriginFor<T>, #[pallet::compact] id: T::FNftId) -> DispatchResult {
            let account = ensure_signed(origin)?;

            let details = FNft::<T, I>::get(id).ok_or(Error::<T, I>::FNftIdNotFound)?;

            ensure!(account == details.admin, Error::<T, I>::WrongAdmin);

            error!("❗️❗️❗️ check that origin is admin @TODO ❗️❗️❗️");
            error!("❗️❗️❗️ check that origin has all tokens free on its balance @TODO ❗️❗️❗️");

            error!("❗️❗️❗️ burn all tokens @TODO ❗️❗️❗️");
            error!("❗️❗️❗️ destroy token AssetId @TODO ❗️❗️❗️");

            FNft::<T, I>::remove(id);

            Uniques::<T>::unlock(account, details.class, details.instance)?;

            let event = Event::Fused;
            Self::deposit_event(event);
            Ok(())
        }
    }
}
