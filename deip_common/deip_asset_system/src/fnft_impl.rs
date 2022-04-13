use core::marker::PhantomData;
use sp_std::prelude::*;

use frame_support::dispatch::{DispatchResult, DispatchError};
use frame_support::traits::tokens::{fungibles, nonfungibles};
use frame_support::traits::tokens::{DepositConsequence, WithdrawConsequence};
use sp_runtime::traits::Hash;
use codec::{Encode, Decode};

use super::NFTImplT;


pub trait NFTokenClassIdObtainT<Id> {
    fn obtain_nft_class_id() -> Option<Id>;
}

pub trait NFTokenInstanceIdObtainT<Id, ClassId> {
    fn obtain_nft_instance_id(class_id: &ClassId) -> Option<Id>;
}

pub trait FTokenIdObtainT<Id> {
    fn obtain_ft_id() -> Option<Id>;
}

pub trait AssetIdLookupT<Id> {
    type Source;
    fn lookup(source: &Self::Source) -> Option<Id>;
}

pub struct DummyDeipAssetsPallet<
    NFT,
    FT,
    Account,
    Hasher,
>(PhantomData<(NFT, FT, Account, Hasher)>);

impl
<
    N: nonfungibles::Inspect<Account>,
    F: fungibles::Inspect<Account>,
    Account,
    Hasher: Hash,
>
    NFTImplT<Account> for
    DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    type Hasher = Hasher;

    type NFTokenId = (N::ClassId, N::InstanceId, F::AssetId);
    type Fingerprint = Hasher::Output;

    fn lookup(f: &Self::Fingerprint) -> Option<Self::NFTokenId> {
        // Pallet::lookup(f)
        None
    }

    fn obtain_id() -> Option<Self::NFTokenId> {
        // Some((Self::obtain()?, Self::obtain()?, Self::obtain()?))
        None
    }

    fn bind(source: Self::Fingerprint, id: Self::NFTokenId) -> Result<(), ()> {
        Ok(())
    }
}


impl<N: nonfungibles::Inspect<Account>, F, Account, Hasher> nonfungibles::Inspect<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    type InstanceId = N::InstanceId;
    type ClassId = N::ClassId;

    fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<Account> {
        N::owner(class, instance)
    }

    fn class_owner(class: &Self::ClassId) -> Option<Account> {
        N::class_owner(class)
    }

    fn attribute(class: &Self::ClassId, instance: &Self::InstanceId, key: &[u8]) -> Option<Vec<u8>> {
        N::attribute(class, instance, key)
    }

    fn typed_attribute<K: Encode, V: Decode>(class: &Self::ClassId, instance: &Self::InstanceId, key: &K) -> Option<V> {
        N::typed_attribute(class, instance, key)
    }

    fn class_attribute(class: &Self::ClassId, key: &[u8]) -> Option<Vec<u8>> {
        N::class_attribute(class, key)
    }

    fn typed_class_attribute<K: Encode, V: Decode>(class: &Self::ClassId, key: &K) -> Option<V> {
        N::typed_class_attribute(class, key)
    }

    fn can_transfer(class: &Self::ClassId, instance: &Self::InstanceId) -> bool {
        N::can_transfer(class, instance)
    }
}

impl<N: nonfungibles::Transfer<Account>, F, Account, Hasher> nonfungibles::Transfer<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn transfer(class: &Self::ClassId, instance: &Self::InstanceId, destination: &Account) -> DispatchResult {
        N::transfer(class, instance, destination)
    }
}

impl<N: nonfungibles::Create<Account>, F, Account, Hasher> nonfungibles::Create<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn create_class(class: &Self::ClassId, who: &Account, admin: &Account) -> DispatchResult {
        N::create_class(class, who, admin)
    }
}

impl<N: nonfungibles::Mutate<Account>, F, Account, Hasher> nonfungibles::Mutate<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn mint_into(class: &Self::ClassId, instance: &Self::InstanceId, who: &Account) -> DispatchResult {
        N::mint_into(class, instance, who)
    }

    fn burn_from(class: &Self::ClassId, instance: &Self::InstanceId) -> DispatchResult {
        N::burn_from(class, instance)
    }

    fn set_attribute(class: &Self::ClassId, instance: &Self::InstanceId, key: &[u8], value: &[u8]) -> DispatchResult {
        N::set_attribute(class, instance, key, value)
    }

    fn set_typed_attribute<K: Encode, V: Encode>(class: &Self::ClassId, instance: &Self::InstanceId, key: &K, value: &V) -> DispatchResult {
        N::set_typed_attribute(class, instance, key, value)
    }

    fn set_class_attribute(class: &Self::ClassId, key: &[u8], value: &[u8]) -> DispatchResult {
        N::set_class_attribute(class, key, value)
    }

    fn set_typed_class_attribute<K: Encode, V: Encode>(class: &Self::ClassId, key: &K, value: &V) -> DispatchResult {
        N::set_typed_class_attribute(class, key, value)
    }
}

impl<N, F: fungibles::Inspect<Account>, Account, Hasher> fungibles::Inspect<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    type AssetId = F::AssetId;
    type Balance = F::Balance;

    fn total_issuance(asset: Self::AssetId) -> Self::Balance {
        F::total_issuance(asset)
    }

    fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
        F::minimum_balance(asset)
    }

    fn balance(asset: Self::AssetId, who: &Account) -> Self::Balance {
        F::balance(asset, who)
    }

    fn reducible_balance(asset: Self::AssetId, who: &Account, keep_alive: bool) -> Self::Balance {
        F::reducible_balance(asset, who, keep_alive)
    }

    fn can_deposit(asset: Self::AssetId, who: &Account, amount: Self::Balance) -> DepositConsequence {
        F::can_deposit(asset, who, amount)
    }

    fn can_withdraw(asset: Self::AssetId, who: &Account, amount: Self::Balance) -> WithdrawConsequence<Self::Balance> {
        F::can_withdraw(asset, who, amount)
    }
}

impl<N, F: fungibles::Transfer<Account>, Account, Hasher> fungibles::Transfer<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn transfer(asset: Self::AssetId, source: &Account, dest: &Account, amount: Self::Balance, keep_alive: bool) -> Result<Self::Balance, DispatchError> {
        F::transfer(asset, source, dest, amount, keep_alive)
    }
}

impl<N, F: fungibles::Create<Account>, Account, Hasher> fungibles::Create<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn create(id: Self::AssetId, admin: Account, is_sufficient: bool, min_balance: Self::Balance) -> DispatchResult {
        F::create(id, admin, is_sufficient, min_balance)
    }
}

impl<N, F: fungibles::Mutate<Account>, Account, Hasher> fungibles::Mutate<Account>
    for DummyDeipAssetsPallet<N, F, Account, Hasher>
{
    fn mint_into(asset: Self::AssetId, who: &Account, amount: Self::Balance) -> DispatchResult {
        F::mint_into(asset, who, amount)
    }

    fn burn_from(asset: Self::AssetId, who: &Account, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
        F::burn_from(asset, who, amount)
    }

    fn slash(asset: Self::AssetId, who: &Account, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
        F::slash(asset, who, amount)
    }

    fn teleport(asset: Self::AssetId, source: &Account, dest: &Account, amount: Self::Balance) -> Result<Self::Balance, DispatchError> {
        F::teleport(asset, source, dest, amount)
    }
}
