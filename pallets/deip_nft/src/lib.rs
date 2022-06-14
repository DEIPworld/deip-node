// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_asset_system;
mod impl_nonfungibles;
mod types;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_asset_system::{FTImplT, NFTokenCollectionRecord};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::{Member, NMapKey, StorageMap, StorageNMap, StorageValue, ValueQuery},
        sp_runtime::traits::{AtLeast32BitUnsigned, StaticLookup},
        traits::IsType,
        transactional, Blake2_128Concat, Parameter, Twox64Concat,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    use crate::types::CollectionDetails;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_uniques::Config<ClassId = Self::CollectionId>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Id of the NFT collection.
        type CollectionId: Member + Parameter + AtLeast32BitUnsigned + Copy + Default;

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

    #[pallet::storage]
    /// Details of a NFT collection.
    pub type Collection<T: Config> =
        StorageMap<_, Blake2_128Concat, T::CollectionId, CollectionDetails>;

    #[pallet::storage]
    /// Records of an  NFT collection by (account & fingerprint).
    pub type CollectionByAccount<T: Config> = StorageNMap<
        _,
        (NMapKey<Blake2_128Concat, T::AccountId>, NMapKey<Blake2_128Concat, T::Hash>),
        NFTokenCollectionRecord<T::AccountId, (T::Hash, T::CollectionId), T::ItemId>,
    >;
    #[pallet::storage]
    pub type ItemByAccount<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Twox64Concat, T::ItemId>,
        ),
        deip_asset_system::NFTokenItemRecord<
            T::AccountId,
            (T::Hash, T::ItemId),
            T::CollectionId,
            (T::AssetId, T::Balance),
        >,
    >;
    #[pallet::storage]
    pub type FractionByAccount<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Twox64Concat, T::ItemId>,
        ),
        deip_asset_system::NFTokenFractionRecord<
            T::AccountId,
            (T::Hash, T::ItemId),
            (T::AssetId, T::Balance),
            T::Balance,
        >,
    >;
    #[pallet::storage]
    pub type NextCollectionId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;

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
