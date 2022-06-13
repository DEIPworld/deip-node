use deip_asset_system::NFTImplT;

use crate::{Config, Pallet};

impl<T: Config> NFTImplT for Pallet<T> {
    type Fungibles = T::Fungibles;

    type Fingerprint = T::Hash;

    type Hasher = T::Hashing;

    type CollectionId = T::CollectionId;

    type ItemId = T::ItemId;

    type FTokenId = T::AssetId;

    type FTokenAmount = T::Balance;

    type Account = T::AccountId;

    /// @TODO bad name.
    type NFTokenCollectionId = (Self::Fingerprint, Self::CollectionId);

    /// @TODO bad name.
    type NFTokenItemId = (Self::Fingerprint, Self::ItemId);

    /// @TODO bad name.
    type Fractional = (Self::FTokenId, Self::FTokenAmount);

    type CollectionRecord;

    type ItemRecord;

    type FractionRecord;

    type CollectionRepo;

    type ItemRepo;

    type FractionRepo;

    type NextCollectionId;

    type Nonfungibles;
}
