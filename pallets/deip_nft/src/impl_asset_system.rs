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
        Self::Other
    }

    fn bad_value() -> Self {
        Self::BadValue
    }

    fn bad_target() -> Self {
        Self::BadTarget
    }

    fn wrong_owner() -> Self {
        Self::WrongOwner
    }

    fn unknown_collection() -> Self {
        Self::UnknownCollection
    }

    fn unknown_f_token_id() -> Self {
        Self::UnknownFTokenId
    }

    fn unknown_item() -> Self {
        Self::UnknownItem
    }

    fn overflow() -> Self {
        Self::Overflow
    }

    fn insufficient_balance() -> Self {
        Self::InsufficientBalance
    }

    fn no_permission() -> Self {
        Self::NoPermission
    }

    fn not_fractionalized() -> Self {
        Self::NotFractionalized
    }
}
