use deip_asset_system::{
    error::Error as NftError, FTImplT, NFTImplT, NFTokenCollectionRecord, NFTokenFractionRecord,
    NFTokenItemRecord,
};
use sp_core::H160;

use crate::{
    CollectionRepo, Config, Error, FingerprintByFractionTokenId, FractionHolds, FractionRepo,
    FractionalRepo, ItemRepo, NextCollectionId, NextFTokenId, Pallet,
};

impl<T: Config> FTImplT for Pallet<T> {
    type Account = T::AccountId;

    type FTokenId = T::InternalFTokenId;

    type FTokenAmount = T::NFTFractionAmount;

    type NextFTokenId = NextFTokenId<T>;

    type Fungibles = pallet_assets::Pallet<T>;

    type Error = Error<T>;
}

impl<T: Config> NFTImplT for Pallet<T> {
    type Fungibles = T::Fungibles;

    type Fingerprint = T::NFTItemId;

    type CollectionId = T::NFTCollectionId;

    type Hasher = T::Hashing;

    type InternalCollectionId = T::InternalCollectionId;

    type ItemId = T::NFTCollectionSize;

    type FTokenId = T::InternalFTokenId;

    type FractionAmount = T::NFTFractionAmount;

    type Account = T::AccountId;

    type Fractional = (Self::FTokenId, Self::FractionAmount);

    type CollectionRecord = NFTokenCollectionRecord<
        Self::Account,
        Self::CollectionId,
        Self::InternalCollectionId,
        Self::ItemId,
    >;

    type ItemRecord = NFTokenItemRecord<
        Self::Account,
        Self::Fingerprint,
        Self::ItemId,
        Self::InternalCollectionId,
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

    type FingerprintByFractionTokenId = FingerprintByFractionTokenId<T>;

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
