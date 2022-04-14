// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::{ChestDetails, DepositBalanceOf};
    use codec::HasCompact;
    use frame_support::{
        pallet_prelude::{
            ensure, Blake2_128Concat, DispatchResult, Get, IsType, Member, Parameter, StorageMap,
        },
        sp_runtime::traits::StaticLookup,
        traits::ReservableCurrency,
    };
    use frame_system::pallet_prelude::{ensure_signed, OriginFor};

    #[pallet::config]
    pub trait Config<I: 'static = ()>: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the chest with assets.
        type ChestId: Member + Parameter + Default + Copy + HasCompact;

        /// The currency mechanism, used for paying for reserves.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The basic amount of funds that must be reserved for an asset chest.
        #[pallet::constant]
        type ChestDeposit: Get<DepositBalanceOf<Self, I>>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config<I>, I: 'static = ()> {
        /// An asset chest was created.
        Created { chest: T::ChestId, creator: T::AccountId, owner: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T, I = ()> {
        /// The `ChestId` is already taken.
        InUse,
    }

    #[pallet::storage]
    pub(super) type Chest<T: Config<I>, I: 'static = ()> = StorageMap<
        _,
        Blake2_128Concat,
        T::ChestId,
        ChestDetails<T::AccountId, DepositBalanceOf<T, I>>,
    >;

    #[pallet::pallet]
    pub struct Pallet<T, I = ()>(_);

    #[pallet::call]
    impl<T: Config<I>, I: 'static> Pallet<T, I> {
        #[pallet::weight(1)]
        pub fn create(
            origin: OriginFor<T>,
            #[pallet::compact] chest: T::ChestId,
            admin: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;
            let admin = T::Lookup::lookup(admin)?;

            ensure!(!Chest::<T, I>::contains_key(chest), Error::<T, I>::InUse);

            let deposit = T::ChestDeposit::get();
            T::Currency::reserve(&owner, deposit)?;

            let details = ChestDetails {
                owner: owner.clone(),
                issuer: admin.clone(),
                admin: admin.clone(),
                freezer: admin,
                total_deposit: deposit,
                is_frozen: false,
            };
            Chest::<T, I>::insert(chest, details);

            Self::deposit_event(Event::Created { chest, creator: owner.clone(), owner });
            Ok(())
        }
    }
}
