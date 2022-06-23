use deip_asset_system::{
    error::Error as NftError, NFTImplT, NFTokenCollectionRecord, NFTokenFractionRecord,
    NFTokenItemRecord,
};
use frame_support::sp_runtime::app_crypto::sp_core::H160;

use crate::{
    AssetIdOf, CollectionRepo, Config, Error, FractionHolds, FractionRepo, FractionalRepo,
    ItemRepo, NextCollectionId, Pallet,
};

impl<T: Config> NFTImplT for Pallet<T> {
    type Fungibles = T::Fungibles;

    type Fingerprint = T::Hash;

    type Hasher = T::Hashing;

    type CollectionId = T::CollectionId;

    type ItemId = T::ItemId;

    type FTokenId = AssetIdOf<T>;

    type FractionAmount = T::Balance;

    type Account = T::AccountId;

    type Fractional = (Self::FTokenId, Self::FractionAmount);

    type CollectionRecord =
        NFTokenCollectionRecord<Self::Account, Self::CollectionId, Self::ItemId>;

    type ItemRecord = NFTokenItemRecord<
        Self::Account,
        Self::Fingerprint,
        Self::ItemId,
        Self::CollectionId,
        Self::Fractional,
    >;

    type FractionRecord = NFTokenFractionRecord<
        Self::Account,
        Self::Fingerprint,
        Self::Fractional,
        Self::FractionAmount,
        Self::FractionHoldGuard,
    >;

    type CollectionRepo = CollectionRepo<T>;

    type ItemRepo = ItemRepo<T>;

    type FractionRepo = FractionRepo<T>;

    type FractionalRepo = FractionalRepo<T>;

    type FractionHolderId = H160;

    type FractionHoldGuard = u32;

    type FractionHolds = FractionHolds<T>;

    type NextCollectionId = NextCollectionId<T>;

    type Nonfungibles = pallet_uniques::Pallet<T>;

    type Error = Error<T>;
}

impl<T> NftError for Error<T> {
    fn other() -> Self {
        // Self::Other
        todo!()
    }

    fn bad_value() -> Self {
        // Self::BadValue
        todo!()
    }

    fn bad_target() -> Self {
        todo!()
    }

    fn wrong_owner() -> Self {
        todo!()
    }

    fn unknown_collection() -> Self {
        // Self::UnknownCollection
        todo!()
    }

    fn unknown_f_token_id() -> Self {
        todo!()
    }

    fn unknown_item() -> Self {
        todo!()
    }

    fn overflow() -> Self {
        todo!()
    }

    fn insufficient_balance() -> Self {
        todo!()
    }

    fn no_permission() -> Self {
        todo!()
    }

    fn not_fractionalized() -> Self {
        todo!()
    }
}
