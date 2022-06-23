// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_asset_system;
mod impl_nonfungibles;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_asset_system::{
        create_collection, fractionalize_item, mint_item, transfer_item, FTImplT, NFTImplT,
        NFTokenCollectionRecord, NFTokenFractionRecord, NFTokenItemRecord, OpaqueUnique,
    };
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::{
            Member, NMapKey, StorageDoubleMap, StorageMap, StorageNMap, StorageValue, ValueQuery,
        },
        sp_runtime::{
            app_crypto::sp_core::H160,
            traits::{AtLeast32BitUnsigned, StaticLookup},
        },
        traits::IsType,
        transactional, Blake2_128Concat, Parameter, Twox64Concat,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};

    type ItemIdOf<T> = <Pallet<T> as NFTImplT>::ItemId;
    pub(crate) type AssetIdOf<T> = <T as Config>::AssetId;
    type FractionAmountOf<T> = <Pallet<T> as NFTImplT>::FractionAmount;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_assets::Config<AssetId = <Self as Config>::AssetId>
        + pallet_uniques::Config<ClassId = Self::CollectionId, InstanceId = Self::ItemId>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Id of the NFT collection.
        type CollectionId: Member + Parameter + AtLeast32BitUnsigned + Copy + Default;

        /// Id of the NFT in the collection.
        type ItemId: Member + Parameter + AtLeast32BitUnsigned + Copy;

        /// Id of a fungible token asset.
        type AssetId: Member + Parameter + AtLeast32BitUnsigned + Copy;

        /// Pallet with low level control over fungible tokens.
        type Fungibles: FTImplT<
            FTokenId = AssetIdOf<Self>,
            Account = Self::AccountId,
            FTokenAmount = Self::Balance,
        >;
    }

    /// Records of an  NFT collection by (account & fingerprint).
    #[pallet::storage]
    pub type CollectionRepo<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::CollectionId,
        NFTokenCollectionRecord<T::AccountId, T::CollectionId, T::ItemId>,
    >;

    /// Records of an NFT by fingerprint, account and NFT id.
    #[pallet::storage]
    pub type ItemRepo<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        NFTokenItemRecord<
            T::AccountId,
            T::Hash,
            T::ItemId,
            T::CollectionId,
            (AssetIdOf<T>, T::Balance),
        >,
    >;

    /// Records of a NFT fractions by fingerpring, account and NFT id.
    #[pallet::storage]
    pub type FractionRepo<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::Hash,
        Blake2_128Concat,
        T::AccountId,
        NFTokenFractionRecord<T::AccountId, T::Hash, (AssetIdOf<T>, T::Balance), T::Balance, u32>,
    >;

    /// Records of fraction asset id and balance by item fingerprint.
    #[pallet::storage]
    pub type FractionalRepo<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, (AssetIdOf<T>, T::Balance)>;

    /// @TODO Documentation
    #[pallet::storage]
    pub type FractionHolds<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, H160>,
            NMapKey<Blake2_128Concat, u32>,
        ),
        (H160, u32),
    >;

    /// Id of the next collection to be created.
    #[pallet::storage]
    pub type NextCollectionId<T: Config> = StorageValue<_, T::CollectionId, ValueQuery>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CollectionCreated {
            issuer: T::AccountId,
            collection: T::CollectionId,
            max_items: ItemIdOf<T>,
        },
        ItemMinted {
            collection: T::CollectionId,
            item: T::Hash,
            owner: T::AccountId,
        },
        ItemFractionalized {
            item: T::Hash,
            issuer: T::AccountId,
            total_amount: FractionAmountOf<T>,
        },
        ItemTransferred {
            item: T::Hash,
            from: T::AccountId,
            to: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        Other,
        BadValue,
        UnknownCollection,
        UnknownItem,
        BadTarget,
        WrongOwner,
        UnknownFTokenId,
        Overflow,
        InsufficientBalance,
        NoPermission,
        NotFractionalized,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates new collection. Returns collection id in event.
        ///
        /// Parameters
        /// - `max_items`: Max number of items in the collection.
        ///
        /// Emits:
        ///     [`Event::CollectionCreated`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn create(origin: OriginFor<T>, max_items: ItemIdOf<T>) -> DispatchResult {
            let issuer = ensure_signed(origin.clone())?;

            let collection = create_collection::<Self>(&issuer, max_items)?;

            Self::deposit_event(Event::CollectionCreated { issuer, collection, max_items });
            Ok(())
        }

        /// Mints item into collection.
        ///
        /// Parameters
        /// - `collection`: Id of the collection to be minted.
        /// - `item`: Unique item identifier, eg hash.
        ///
        /// Emits:
        ///     [`Event::ItemMinted`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn mint(
            origin: OriginFor<T>,
            collection: T::CollectionId,
            item: T::Hash,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            mint_item(collection, &owner, OpaqueUnique::<Self>(item)).unwrap();

            Self::deposit_event(Event::ItemMinted { collection, item, owner });
            Ok(())
        }

        /// Transfers ownership of the item to another account.
        ///
        /// Parameters
        /// - `item`: Unique identifier of the item to be transferred.
        /// - `to`: Destination account.
        ///
        /// Emits:
        ///     [`Event::ItemTransferred`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn transfer(
            origin: OriginFor<T>,
            item: T::Hash,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(to)?;

            transfer_item::<Self>(item, &from, &to)?;

            Self::deposit_event(Event::ItemTransferred { item, from, to });
            Ok(())
        }

        /// Fractionalizes NFT.
        ///
        /// Parameters
        /// - `item`: Unique id of the item to be fractionalized.
        /// - `total_amount`: Amount of the fractions.
        ///
        /// Emits:
        ///     [`Event::ItemFractionalized`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn fractionalize(
            origin: OriginFor<T>,
            item: T::Hash,
            total_amount: FractionAmountOf<T>,
        ) -> DispatchResult {
            let issuer = ensure_signed(origin)?;

            fractionalize_item::<Self>(item, &issuer, total_amount)?;

            Self::deposit_event(Event::ItemFractionalized { item, issuer, total_amount });
            Ok(())
        }
    }
}
