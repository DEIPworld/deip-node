use deip_asset_system::{
    error::Error as NftError, NFTImplT, NFTokenCollectionRecord, NFTokenFractionRecord,
    NFTokenItemRecord,
};

use crate::{
    CollectionByAccount, Config, Error, FractionByAccount, ItemByAccount, NextCollectionId, Pallet,
};

impl<T: Config> NFTImplT for Pallet<T> {
    type Fungibles = T::Fungibles;

    type Fingerprint = T::Hash;

    type Hasher = T::Hashing;

    type CollectionId = T::CollectionId;

    type ItemId = T::ItemId;

    type FTokenId = T::AssetId;

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

    type CollectionRepo = CollectionByAccount<T>;

    type ItemRepo = ItemByAccount<T>;

    type FractionRepo = FractionByAccount<T>;

    type NextCollectionId = NextCollectionId<T>;

    type Nonfungibles = pallet_uniques::Pallet<T>;

    type Error = Error<T>;
}

impl<T> NftError for Error<T> {
    fn bad_value() -> Self {
        Self::BadValue
    }

    fn unknown_collection() -> Self {
        Self::UnknownCollection
    }

    fn other() -> Self {
        Self::Other
    }
}
