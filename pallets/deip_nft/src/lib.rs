// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_asset_system;
mod impl_nonfungibles;
mod types;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_asset_system::{
        FTImplT, NFTImplT, NFTokenCollectionRecord, NFTokenFractionRecord, NFTokenItemRecord,
    };
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        ensure,
        pallet_prelude::{Member, NMapKey, StorageMap, StorageNMap, StorageValue, ValueQuery},
        sp_runtime::traits::{AtLeast32BitUnsigned, One, StaticLookup},
        traits::{fungible::ItemOf, IsType},
        transactional, Blake2_128Concat, Parameter, Twox64Concat,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use pallet_uniques::{DestroyWitness, Pallet as Uniques};

    use crate::types::CollectionDetails;

    type ItemIdOf<T> = <Pallet<T> as NFTImplT>::ItemId;
    type CollectionRecordOf<T> = <Pallet<T> as NFTImplT>::CollectionRecord;
    type FTokenAmountOf<T> = <Pallet<T> as NFTImplT>::FTokenAmount;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
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

        /// Balance of a fungible token asset.
        type FungiblesBalance: Member + Parameter + AtLeast32BitUnsigned + Copy;

        /// Pallet with low level control over fungible tokens.
        type Fungibles: FTImplT<
            FTokenId = Self::AssetId,
            FTokenAmount = Self::FungiblesBalance,
            Account = Self::AccountId,
        >;
    }

    #[pallet::storage]
    pub(crate) type Collection<T: Config> =
        StorageMap<_, Blake2_128Concat, T::CollectionId, CollectionDetails>;

    /// Records of an  NFT collection by (account & fingerprint).
    #[pallet::storage]
    pub type CollectionByAccount<T: Config> = StorageNMap<
        _,
        (NMapKey<Blake2_128Concat, T::AccountId>, NMapKey<Blake2_128Concat, T::Hash>),
        NFTokenCollectionRecord<T::AccountId, (T::Hash, T::CollectionId), T::ItemId>,
    >;

    /// Records of an NFT by fingerprint, account and NFT id.
    #[pallet::storage]
    pub type ItemByAccount<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Twox64Concat, T::ItemId>,
        ),
        NFTokenItemRecord<
            T::AccountId,
            (T::Hash, T::ItemId),
            T::CollectionId,
            (T::AssetId, T::FungiblesBalance),
        >,
    >;

    /// Records of a NFT fractions by fingerpring, account and NFT id.
    #[pallet::storage]
    pub type FractionByAccount<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Twox64Concat, T::ItemId>,
        ),
        NFTokenFractionRecord<
            T::AccountId,
            (T::Hash, T::ItemId),
            (T::AssetId, T::FungiblesBalance),
            T::FungiblesBalance,
        >,
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
            collection: T::Hash,
            max_items: ItemIdOf<T>,
        },
        ItemMinted {
            collection: T::Hash,
            // item: T::Hash,
            owner: T::AccountId,
        },
        ItemBurned {
            collection: T::CollectionId,
            item: T::ItemId,
        },
        ItemFractionalized {
            collection: T::Hash,
            item: ItemIdOf<T>,
            total_amount: FTokenAmountOf<T>,
        },
        ItemTransferred {
            collection: T::Hash,
            item: ItemIdOf<T>,
            to: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        CollectionIdInUse,
        UnknownCollection,
        UnknownItem,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates new collection. Returns collection id in event.
        ///
        /// Parameters
        /// - @TODO
        ///
        /// Emits:
        ///     [`Event::CollectionCreated`] when successful.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            collection: T::Hash,
            max_items: ItemIdOf<T>,
        ) -> DispatchResult {
            let issuer = ensure_signed(origin.clone())?;

            Self::create_collection(collection, &issuer, max_items)?;

            Self::deposit_event(Event::CollectionCreated { issuer, collection, max_items });
            Ok(())
        }

        /// Destroys collection.
        ///
        /// Parameters:
        /// - @TODO
        ///
        /// Emits:
        ///     [`Event::CollectionDestroyed`] when successful.
        // #[pallet::weight(1_000_000)]
        // #[transactional] @TODO
        // pub fn destroy(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     witness: DestroyWitness,
        // ) -> DispatchResultWithPostInfo {
        //     let origin_id = ensure_signed(origin)?;

        //     todo!();

        // Self::deposit_event(Event::CollectionDestroyed { collection });
        // Ok(())
        // }

        /// Mints itme into collection.
        #[pallet::weight(1_000_000)]
        // #[transactional] @TODO
        pub fn mint(
            origin: OriginFor<T>,
            collection: T::Hash,
            //  item: T::Hash @TODO does item has hash???
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let record =
                Self::find_collection(&owner, &collection).ok_or(Error::<T>::UnknownCollection)?;

            frame_support::log::error!("before mint");
            Self::mint_item(record)?;
            frame_support::log::error!("after mint");

            Self::deposit_event(Event::ItemMinted {
                collection,
                // item, @TODO
                owner,
            });
            Ok(())
        }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn burn(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     check_owner: Option<<T::Lookup as StaticLookup>::Source>,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;

        //     Uniques::<T>::burn(origin, collection, item, check_owner)?;

        //     Self::deposit_event(Event::ItemBurned { collection, item });
        //     todo!()
        // }

        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn transfer(
            origin: OriginFor<T>,
            collection: T::Hash,
            item: T::ItemId,
            to: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            let origin_id = ensure_signed(origin)?;
			let to = T::Lookup::lookup(to)?;

            let record =
                Self::find_item(collection, origin_id, item).ok_or(Error::<T>::UnknownItem)?;

            Self::transfer_item(record, &to)?;

            Self::deposit_event(Event::<T>::ItemTransferred { collection, item, to });
            Ok(())
        }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn redeposit(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn freeze(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn thaw(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn freeze_collection(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn thaw_collection(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn transfer_ownership(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_team(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn approve_transfer(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn cancel_approval(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_attribute(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn clear_attribute(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_metadata(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn clear_metadata(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_collection_metadata(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn clear_collection_metadata(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_accept_ownership(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        // #[pallet::weight(1_000_000)]
        // #[transactional]
        // pub fn set_collection_max_supply(
        //     origin: OriginFor<T>,
        //     collection: T::CollectionId,
        //     item: T::ItemId,
        //     owner: <T::Lookup as StaticLookup>::Source,
        // ) -> DispatchResult {
        //     let origin_id = ensure_signed(origin)?;
        //     todo!()
        // }

        /// Fractionalizes NFT.
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn fractionalize(
            origin: OriginFor<T>,
            collection: T::Hash,
            item: ItemIdOf<T>,
            total_amount: FTokenAmountOf<T>,
        ) -> DispatchResult {
            let origin_id = ensure_signed(origin)?;

            let record =
                Self::find_item(collection, origin_id, item).ok_or(Error::<T>::UnknownItem)?;

            <Self as NFTImplT>::fractionalize(record, total_amount)?;

            Self::deposit_event(Event::<T>::ItemFractionalized { collection, item, total_amount });
            Ok(())
        }
    }
}
