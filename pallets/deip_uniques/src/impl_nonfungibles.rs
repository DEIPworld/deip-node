use sp_std::prelude::*;
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::sp_runtime;
use frame_support::traits::tokens::nonfungibles::{Inspect, Create, Mutate, Transfer};
use deip_asset_system::{NFTImplT, NFTokenCollectionRecord, NFTokenItemRecord, NFTokenFractionRecord};
use crate::{Config, Pallet};
use sp_runtime::traits::AtLeast32BitUnsigned;

impl<T: Config> NFTImplT for Pallet<T>
    where
        T::ClassId: AtLeast32BitUnsigned,
        T::InstanceId: AtLeast32BitUnsigned,
        T::AssetId: AtLeast32BitUnsigned,
{
    type Fungibles = T::Fungibles;

    type Fingerprint = T::Hash;

    type Hasher = T::Hashing;

    type CollectionId = T::ClassId;
    type ItemId = T::InstanceId;
    type FTokenId = T::AssetId;
    type FTokenAmount = T::Balance;

    type Account = T::AccountId;

    type NFTokenCollectionId = (Self::Fingerprint, Self::CollectionId);
    type NFTokenItemId = (Self::Fingerprint, Self::ItemId);

    type Fractional = (Self::FTokenId, Self::FTokenAmount);

    type CollectionRecord = NFTokenCollectionRecord<
        Self::Account,
        Self::NFTokenCollectionId,
        Self::ItemId
    >;
    type ItemRecord = NFTokenItemRecord<
        Self::Account,
        Self::NFTokenItemId,
        Self::CollectionId,
        Self::Fractional
    >;
    type FractionRecord = NFTokenFractionRecord<
        Self::Account,
        Self::NFTokenItemId,
        Self::Fractional,
        Self::FTokenAmount
    >;

    type CollectionRepo = crate::CollectionRepo<T>;
    type ItemRepo = crate::ItemRepo<T>;
    type FractionRepo = crate::FractionRepo<T>;

    type NextCollectionId = crate::NextCollectionId<T>;

    type Nonfungibles = Self;
}

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
    type InstanceId = T::InstanceId;
    type ClassId = T::ClassId;

    fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<T::AccountId> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::owner(
            class, instance
        )
    }

    fn class_owner(class: &Self::ClassId) -> Option<T::AccountId> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::class_owner(
            class
        )
    }

    fn attribute(class: &Self::ClassId, instance: &Self::InstanceId, key: &[u8]) -> Option<Vec<u8>> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::attribute(
            class, instance, key
        )
    }

    fn typed_attribute<K: Encode, V: Decode>(class: &Self::ClassId, instance: &Self::InstanceId, key: &K) -> Option<V> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::typed_attribute(
            class, instance, key
        )
    }

    fn class_attribute(class: &Self::ClassId, key: &[u8]) -> Option<Vec<u8>> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::class_attribute(
            class, key
        )
    }

    fn typed_class_attribute<K: Encode, V: Decode>(class: &Self::ClassId, key: &K) -> Option<V> {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::typed_class_attribute(
            class, key
        )
    }

    fn can_transfer(class: &Self::ClassId, instance: &Self::InstanceId) -> bool {
        <pallet_uniques::Pallet<T> as Inspect<T::AccountId>>::can_transfer(
            class, instance
        )
    }
}

impl<T: Config> Create<T::AccountId> for Pallet<T> {
    fn create_class(class: &Self::ClassId, who: &T::AccountId, admin: &T::AccountId) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Create<T::AccountId>>::create_class(
            class, who, admin,
        )
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {
    fn mint_into(class: &Self::ClassId, instance: &Self::InstanceId, who: &T::AccountId) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::mint_into(
            class, instance, who
        )
    }

    fn burn_from(class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::burn_from(
            class, instance
        )
    }

    fn set_attribute(class: &Self::ClassId, instance: &Self::InstanceId, key: &[u8], value: &[u8]) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::set_attribute(
            class, instance, key, value
        )
    }

    fn set_typed_attribute<K: Encode, V: Encode>(class: &Self::ClassId, instance: &Self::InstanceId, key: &K, value: &V) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::set_typed_attribute(
            class, instance, key, value
        )
    }

    fn set_class_attribute(class: &Self::ClassId, key: &[u8], value: &[u8]) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::set_class_attribute(
            class, key, value
        )
    }

    fn set_typed_class_attribute<K: Encode, V: Encode>(class: &Self::ClassId, key: &K, value: &V) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Mutate<T::AccountId>>::set_typed_class_attribute(
            class, key, value
        )
    }
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
    fn transfer(class: &Self::ClassId, instance: &Self::InstanceId, destination: &T::AccountId) -> DispatchResult {
        <pallet_uniques::Pallet<T> as Transfer<T::AccountId>>::transfer(
            class, instance, destination
        )
    }
}