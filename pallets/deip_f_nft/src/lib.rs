// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_asset_system;
mod impl_fungibles;
mod impl_nonfungibles;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_asset_system::{
        burn_fraction, create_collection, fractionalize_item, mint_fraction, mint_item,
        transfer_fraction, transfer_item, FTImplT, NFTokenCollectionRecord, NFTokenFractionRecord,
        NFTokenItemRecord, OpaqueUnique,
    };
    use frame_support::{
        dispatch::DispatchResult,
        migration::storage_key_iter,
        pallet_prelude::{
            Member, NMapKey, StorageDoubleMap, StorageMap, StorageNMap, StorageValue, ValueQuery,
            Weight,
        },
        sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, One, StaticLookup, Zero},
        traits::{tokens::currency::Currency, Get, Hooks, IsType},
        transactional, Blake2_128Concat, Parameter,
    };
    use frame_system::{
        ensure_signed,
        pallet_prelude::{BlockNumberFor, OriginFor},
    };
    use sp_core::H160;

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            let reads = 0;
            let writes = 0;

            // NextFTokenId
            let pallet = b"Assets";
            let storage = b"Asset";
            type DepositBalanceOf<T> = <<T as pallet_assets::Config>::Currency as Currency<
                <T as frame_system::Config>::AccountId,
            >>::Balance;
            let iterator = storage_key_iter::<
                T::AssetId,
                pallet_assets::AssetDetails<T::Balance, T::AccountId, DepositBalanceOf<T>>,
                Blake2_128Concat,
            >(pallet, storage);
            let next_id = iterator
                .map(|(id, _)| id)
                .max()
                .and_then(|id| id.checked_add(&One::one()))
                .unwrap_or_else(Zero::zero);
            NextFTokenId::<T>::put(next_id);

            // CollectionRepo

            // ItemRepo

            // FingerprintByFractionTokenId

            // FractionRepo

            // FractionalRepo

            // FractionHolds

            // NextCollectionId

            T::DbWeight::get().reads_writes(reads, writes)
        }
    }

    #[pallet::config]
    pub trait Config:
        frame_system::Config<Hash = Self::NFTItemId>
        + pallet_assets::Config<AssetId = Self::InternalFTokenId, Balance = Self::NFTFractionAmount>
        + pallet_uniques::Config<
            ClassId = Self::InternalCollectionId,
            InstanceId = Self::NFTCollectionSize,
        >
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        // /// Id of the NFT collection.
        type NFTCollectionId: From<sp_core::H160> + Member + Parameter + Copy + Default;
        type NFTCollectionSize: Member + Parameter + AtLeast32BitUnsigned + Copy + Default;
        type NFTItemId: Member + Parameter + Copy + Default;
        type NFTFractionAmount: Member + Parameter + AtLeast32BitUnsigned + Copy + Default;

        type InternalCollectionId: Member + Parameter + AtLeast32BitUnsigned + Copy + Default;
        type InternalFTokenId: Member + Parameter + AtLeast32BitUnsigned + Copy + Default + One;

        /// Pallet with low level control over fungible tokens.
        type Fungibles: FTImplT<
            FTokenId = Self::InternalFTokenId,
            Account = Self::AccountId,
            FTokenAmount = Self::Balance,
        >;
    }

    /// Records of an  NFT collection by (account & fingerprint).
    #[pallet::storage]
    pub type CollectionRepo<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::NFTCollectionId,
        NFTokenCollectionRecord<
            T::AccountId,
            T::NFTCollectionId,
            T::InternalCollectionId,
            T::NFTCollectionSize,
        >,
    >;

    /// Records of an NFT by fingerprint, account and NFT id.
    #[pallet::storage]
    pub type ItemRepo<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::NFTItemId,
        NFTokenItemRecord<
            T::AccountId,
            T::NFTItemId,
            T::NFTCollectionSize,
            T::InternalCollectionId,
            (T::InternalFTokenId, T::NFTFractionAmount),
        >,
    >;

    /// Records of a NFT fractions by fingerpring, account and NFT id.
    #[pallet::storage]
    pub type FractionRepo<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::NFTItemId,
        Blake2_128Concat,
        T::AccountId,
        NFTokenFractionRecord<
            T::AccountId,
            T::NFTItemId,
            (T::InternalFTokenId, T::NFTFractionAmount),
            T::NFTFractionAmount,
            u32,
        >,
    >;

    /// Records of fraction asset id and balance by item fingerprint.
    #[pallet::storage]
    pub type FractionalRepo<T: Config> =
        StorageMap<_, Blake2_128Concat, T::NFTItemId, (T::InternalFTokenId, T::NFTFractionAmount)>;

    /// @TODO Documentation
    #[pallet::storage]
    pub type FractionHolds<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::NFTItemId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, H160>,
            NMapKey<Blake2_128Concat, u32>,
        ),
        (H160, u32),
    >;

    /// Id of the next collection to be created.
    #[pallet::storage]
    pub type NextCollectionId<T: Config> = StorageValue<_, T::InternalCollectionId, ValueQuery>;

    /// Id of the next fraction to be created.
    #[pallet::storage]
    pub type NextFTokenId<T: Config> = StorageValue<_, T::InternalFTokenId, ValueQuery>;

    /// Storage with fraction FT id - item fingerprint mapping.
    #[pallet::storage]
    pub type FingerprintByFractionTokenId<T: Config> =
        StorageMap<_, Blake2_128Concat, T::InternalFTokenId, T::NFTItemId>;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CollectionCreated {
            issuer: T::AccountId,
            collection: T::NFTCollectionId,
            max_items: T::NFTCollectionSize,
        },
        ItemMinted {
            collection: T::NFTCollectionId,
            item: T::NFTItemId,
            owner: T::AccountId,
        },
        ItemFractionalized {
            item: T::NFTItemId,
            issuer: T::AccountId,
            total_amount: T::NFTFractionAmount,
            limited: bool,
        },
        ItemTransferred {
            item: T::NFTItemId,
            from: T::AccountId,
            to: T::AccountId,
        },
        FractionMinted {
            item: T::Hash,
            owner: T::AccountId,
            amount: T::NFTFractionAmount,
        },
        FractionBurned {
            item: T::Hash,
            owner: T::AccountId,
            amount: T::NFTFractionAmount,
        },
        FractionTransferred {
            item: T::NFTItemId,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::NFTFractionAmount,
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
        pub fn create_collection(
            origin: OriginFor<T>,
            id: T::NFTCollectionId,
            max_items: Option<T::NFTCollectionSize>,
        ) -> DispatchResult {
            let issuer = ensure_signed(origin.clone())?;

            let max_items = max_items.unwrap_or_else(T::NFTCollectionSize::max_value);

            create_collection::<Self>(&issuer, id, max_items)?;

            Self::deposit_event(Event::CollectionCreated { issuer, collection: id, max_items });
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
        pub fn mint_item(
            origin: OriginFor<T>,
            collection: T::NFTCollectionId,
            item: T::NFTItemId,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            mint_item(collection, &owner, OpaqueUnique::<Self>(item))?;

            Self::deposit_event(Event::ItemMinted { collection, item, owner });
            Ok(())
        }

        /// Transfers item to another account.
        ///
        /// Parameters
        /// - `item`: Unique identifier of the item to be transferred.
        /// - `to`: Destination account.
        ///
        /// Emits:
        ///     [`Event::ItemTransferred`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn transfer_item(
            origin: OriginFor<T>,
            item: T::NFTItemId,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(to)?;

            transfer_item::<Self>(item, &from, &to)?;

            Self::deposit_event(Event::ItemTransferred { item, from, to });
            Ok(())
        }

        /// Mints additional fungible tokens, fractions for an NFT or coins.
        /// Fails if issuance of the fractions was limited on fractionalization
        /// stage.
        ///
        /// Parameters
        /// - `item`: Unique identifier of the fractionalized item.
        /// - `amount`: Amount of fractions to be minted.
        ///
        /// Emits:
        ///     [`Event::FractionMinted`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn mint_fraction(
            origin: OriginFor<T>,
            item: T::Hash,
            amount: T::NFTFractionAmount,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            mint_fraction::<Self>(item, &owner, amount)?;

            Self::deposit_event(Event::FractionMinted { item, owner, amount });
            Ok(())
        }

        /// Burns fractions from item.
        ///
        /// Parameters
        /// - `item`: Unique identifier of the fractionalized item.
        /// - `amount`: Amount of fractions to be burned.
        ///
        /// Emits:
        ///     [`Event::FractionBurned`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn burn_fraction(
            origin: OriginFor<T>,
            item: T::Hash,
            amount: T::NFTFractionAmount,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let amount = burn_fraction::<Self>(item, &owner, amount)?;

            Self::deposit_event(Event::FractionBurned { item, owner, amount });
            Ok(())
        }

        /// Transfers fraction (fungible token) to another account.
        ///
        /// Parameters
        /// - `item`: Unique id of the fractionalized item.
        /// - `to`: Destination account.
        /// - `amount`: Amount of fractions to be transferred.
        ///
        /// Emits:
        ///     [`Event::FractionTransferred`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn transfer_fraction(
            origin: OriginFor<T>,
            item: T::NFTItemId,
            to: <T::Lookup as StaticLookup>::Source,
            amount: T::NFTFractionAmount,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;
            let to = T::Lookup::lookup(to)?;

            transfer_fraction::<Self>(item, &from, &to, amount)?;

            Self::deposit_event(Event::FractionTransferred { item, from, to, amount });
            Ok(())
        }

        /// Fractionalizes NFT.
        ///
        /// Parameters
        /// - `item`: Unique id of the item to be fractionalized.
        /// - `total_amount`: Amount of the fractions.
        /// - `limited`: If set to true, further minting will be locked.
        ///
        /// Emits:
        ///     [`Event::ItemFractionalized`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn fractionalize_item(
            origin: OriginFor<T>,
            item: T::NFTItemId,
            total_amount: T::NFTFractionAmount,
            limited: bool,
        ) -> DispatchResult {
            let issuer = ensure_signed(origin)?;

            fractionalize_item::<Self>(item, &issuer, total_amount, limited)?;

            Self::deposit_event(Event::ItemFractionalized { item, issuer, total_amount, limited });
            Ok(())
        }
    }
}
