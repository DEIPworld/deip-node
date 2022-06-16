use deip_asset_system::{
    error::NftError, NFTImplT, NFTokenCollectionRecord, NFTokenFractionRecord, NFTokenItemRecord,
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

    type FTokenAmount = T::FungiblesBalance;

    type Account = T::AccountId;

    /// @TODO bad name.
    type NFTokenCollectionId = (Self::Fingerprint, Self::CollectionId);

    /// @TODO bad name.
    type NFTokenItemId = (Self::Fingerprint, Self::ItemId);

    /// @TODO bad name.
    type Fractional = (Self::FTokenId, Self::FTokenAmount);

    type CollectionRecord =
        NFTokenCollectionRecord<Self::Account, Self::NFTokenCollectionId, Self::ItemId>;

    type ItemRecord =
        NFTokenItemRecord<Self::Account, Self::NFTokenItemId, Self::CollectionId, Self::Fractional>;

    type FractionRecord = NFTokenFractionRecord<
        Self::Account,
        Self::NFTokenItemId,
        Self::Fractional,
        Self::FTokenAmount,
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
