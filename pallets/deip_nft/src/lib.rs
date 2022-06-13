// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_asset_system;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_asset_system::FTImplT;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::Member,
        sp_runtime::traits::{AtLeast32BitUnsigned, StaticLookup},
        traits::IsType,
        transactional, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_uniques::Config<ClassId = Self::CollectionId>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Id of the NFT collection.
        type CollectionId: Member + Parameter + AtLeast32BitUnsigned + Copy;

        /// Id of the NFT in the collection.
        type ItemId: Member + Parameter + AtLeast32BitUnsigned + Copy;

        /// @TODO shouldn't be here.
        type Fungibles: FTImplT<
            FTokenId = Self::AssetId,
            FTokenAmount = Self::Balance,
            Account = Self::AccountId,
        >;
        type AssetId: Member + Parameter + AtLeast32BitUnsigned + Copy;
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Copy;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CollectionCreated { issuer: T::AccountId, collection: T::CollectionId },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn create_collection(
            origin: OriginFor<T>,
            collection: T::CollectionId,
        ) -> DispatchResult {
            let issuer = ensure_signed(origin.clone())?;
            let admin = <T::Lookup as StaticLookup>::unlookup(issuer.clone());

            pallet_uniques::Pallet::<T>::create(origin, collection.clone(), admin)?;

            Self::deposit_event(Event::CollectionCreated { issuer, collection });
            Ok(())
        }
    }
}
