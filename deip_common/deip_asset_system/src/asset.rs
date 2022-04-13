#![allow(type_alias_bounds)]
use crate::*;
use sp_runtime::traits::{AtLeast32BitUnsigned, Hash, Zero};
use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use codec::{Encode, Decode};
use frame_support::{RuntimeDebug};
use frame_support::dispatch::DispatchResult;
#[cfg(feature = "std")]
use serde::{self, Serialize, Deserialize};
use scale_info::TypeInfo;

use frame_support::traits::{fungibles, tokens::nonfungibles};
use frame_support::traits::tokens::{DepositConsequence, WithdrawConsequence};
use sp_std::marker::PhantomData;
use sp_std::default::Default;
use frame_system::Config as System;
use sp_runtime::DispatchError;

pub trait TransferUnitT<Account, Amount, Impl>
{
    fn transfer(self, to: &Account);

    fn amount(&self) -> &Amount;
}

pub trait GenericAssetT<Id, Payload, Account, Amount, Impl>:
    TransferUnitT<Account, Amount, Impl> +
    Sized
{
    fn new(id: Id, payload: Payload, account: Account, amount: Amount) -> Self;

    fn pick(id: Id, account: &Account, amount: Amount) -> Option<Self>;

    fn balance(id: &Id, account: &Account) -> Option<Amount>;

    fn id(&self) -> &Id;

    fn payload(&self) -> &Payload;

    fn account(&self) -> &Account;
}

pub trait FTokenT<Id, Payload, Account, Amount, Impl>:
    GenericAssetT<Id, Payload, Account, Amount, Impl> {

    fn fungible(_account: Account, _amount: Amount) -> Self {
        todo!()
    }
}

pub trait Unique<Hasher: NFTImplT<Account>, Account> {
    fn fingerprint(self) -> Hasher::Fingerprint;
}

pub struct OpaqueUnique<H: NFTImplT<Account>, Account>(pub H::Fingerprint);

impl<H: NFTImplT<Account>, Account> Unique<H, Account> for OpaqueUnique<H, Account> {
    fn fingerprint(self) -> H::Fingerprint {
        self.0
    }
}

pub trait NFTokenT<Id, Payload, Account, Amount, Impl: NFTImplT<Account>>:
    GenericAssetT<Id, Payload, Account, Amount, Impl>
{
    fn unique<U: Unique<Impl, Account>>(
        unique: U,
        account: &Account,
    ) -> Result<Self, ()>;
}

pub trait CNFTokenT<Id, Payload, Account, Amount, Impl: NFTImplT<Account>>:
    GenericAssetT<Id, Payload, Account, Amount, Impl>
{
    fn collectable<U: Unique<Impl, Account>>(
        unique: U,
        account: &Account,
        instances: Amount
    ) -> Result<Self, ()>;
}

pub trait FNFTokenT<Id, Payload, Account, Amount, Impl: NFTImplT<Account>>:
    GenericAssetT<Id, Payload, Account, Amount, Impl>
{
    fn fractional<U: Unique<Impl, Account>>(
        unique: U,
        account: &Account,
        fraction: Amount
    ) -> Result<Self, ()>;
}

pub struct GenericAsset
    <Id, Payload, Account, Amount, Impl>
    (Id, Payload, Account, Amount, PhantomData<Impl>)
    where Self: GenericAssetT<Id, Payload, Account, Amount, Impl>;

pub struct GenericFToken // type name
    <Id, Payload, Account, Amount, Impl> // type template
    (GenericAsset<Id, Payload, Account, Amount, Impl>) // type structure
    where Self: FTokenT<Id, Payload, Account, Amount, Impl>; // type class/signature

pub struct GenericNFToken
    <Id, Payload, Account, Impl: NFTImplT<Account>>
    (GenericAsset<Id, Payload, Account, (), Impl>)
    where Self: NFTokenT<Id, Payload, Account, (), Impl>;

pub struct GenericFNFToken
    <Id, Payload, Account, Amount, Impl: NFTImplT<Account>>
    (GenericAsset<Id, Payload, Account, Amount, Impl>)
    where Self: FNFTokenT<Id, Payload, Account, Amount, Impl>;

pub trait Fungibles<Account>:
        fungibles::Inspect<Account> +
        fungibles::Transfer<Account> +
        fungibles::Create<Account> +
        fungibles::Mutate<Account> {}

impl<T: fungibles::Inspect<Account> +
        fungibles::Transfer<Account> +
        fungibles::Create<Account> +
        fungibles::Mutate<Account>, Account> Fungibles<Account> for T {}

pub trait Nonfungibles<Account>:
        nonfungibles::Inspect<Account> +
        nonfungibles::Transfer<Account> +
        nonfungibles::Create<Account> +
        nonfungibles::Mutate<Account> {}

impl<T: nonfungibles::Inspect<Account> +
        nonfungibles::Transfer<Account> +
        nonfungibles::Create<Account> +
        nonfungibles::Mutate<Account>, Account> Nonfungibles<Account> for T {}

// FNFToken:

impl<
    Account: Clone,
    Impl:
        Nonfungibles<Account> +
        Fungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId, Impl::AssetId),
        >,
>
    TransferUnitT<Account, Impl::Balance, Impl> for
    GenericFNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl::Balance,
        Impl,
    >
    where
        Impl::Fingerprint: Copy,
        Impl::NFTokenId: Copy,
{
    fn transfer(self, to: &Account) {
        GenericFToken::<
            Impl::AssetId,
            (),
            Account,
            Impl::Balance,
            Impl
        >::new(
            self.payload().2,
            (),
            self.account().clone(),
            *self.amount()
        ).transfer(
            to
        );
    }

    fn amount(&self) -> &Impl::Balance {
        self.0.amount()
    }
}

impl<
    Account: Clone,
    Impl:
        Nonfungibles<Account> +
        Fungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId, Impl::AssetId)
        >
>
    GenericAssetT<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl::Balance,
        Impl
    > for
    GenericFNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl::Balance,
        Impl,
    >
    where
        Impl::Fingerprint: Copy,
        Impl::NFTokenId: Copy,
{
    fn new(
        id: Impl::Fingerprint,
        payload: Impl::NFTokenId,
        account: Account,
        amount: Impl::Balance,
    ) -> Self
    {
        Self(GenericAsset::new(id, payload, account, amount))
    }

    fn pick(
        id: Impl::Fingerprint,
        account: &Account,
        amount: Impl::Balance,
    ) -> Option<Self>
    {
        let (class_id, instance_id, ft_id) = Impl::lookup(&id)?;
        if amount.is_zero() || amount > Impl::balance(ft_id, account) {
            return None
        }
        Some(Self::new(id, (class_id, instance_id, ft_id), account.clone(), amount))
    }

    fn balance(id: &Impl::Fingerprint, account: &Account) -> Option<Impl::Balance> {
        let (_, _, ft_id) = Impl::lookup(id)?;
        Some(Impl::balance(ft_id, account))
    }

    fn id(&self) -> &Impl::Fingerprint {
        self.0.id()
    }
    fn payload(&self) -> &Impl::NFTokenId {
        self.0.payload()
    }

    fn account(&self) -> &Account {
        self.0.account()
    }
}

pub trait FTImpl<Account>:
{
    type FTokenId;

    fn obtain_id() -> Self::FTokenId;
}

pub trait NFTImplT<Account>:
{
    type Hasher: Hash<Output=Self::Fingerprint>;

    type NFTokenId;
    type Fingerprint;

    fn lookup(source: &Self::Fingerprint) -> Option<Self::NFTokenId>;

    fn obtain_id() -> Option<Self::NFTokenId>;

    fn bind(source: Self::Fingerprint, id: Self::NFTokenId) -> Result<(), ()>;
}

type DaoId = u32;

pub struct Dao {
    pub id: DaoId,
}

pub struct UniqueDao<'a>(pub &'a Dao);

impl<H: NFTImplT<Account>, Account> Unique<H, Account> for UniqueDao<'_> {
    fn fingerprint(self) -> H::Fingerprint {
        H::Hasher::hash_of(&("DAO", self.0.id))
    }
}

impl
<
    Account: Clone,
    Impl:
        Nonfungibles<Account> +
        Fungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId, Impl::AssetId),
        >
>
    FNFTokenT<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl::Balance,
        Impl,
    > for
    GenericFNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl::Balance,
        Impl,
    >
    where
        Impl::Fingerprint: Copy,
        Impl::NFTokenId: Copy,
{
    fn fractional<U: Unique<Impl, Account>>(
        unique: U,
        account: &Account,
        fraction: Impl::Balance
    ) -> Result<Self, ()>
    {
        let fingerprint = unique.fingerprint();
        if Impl::lookup(&fingerprint).is_some() {
            return Err(())
        }
        let id = Impl::obtain_id().ok_or(())?;
        Impl::bind(fingerprint, id)?;
        let (nft_class_id, nft_instance_id, ft_id) = id;
        let _ = Impl::create_class(
            &nft_class_id,
            account,
            account
        );
        let _ = <Impl as nonfungibles::Mutate<Account>>::mint_into(
            &nft_class_id,
            &nft_instance_id,
            account
        );
        let _ = <Impl as fungibles::Create<Account>>::create(
            ft_id,
            account.clone(),
            true,
            <Impl as fungibles::Inspect<Account>>::minimum_balance(ft_id)
        );
        let _ = <Impl as fungibles::Mutate<Account>>::mint_into(
            ft_id,
            account,
            fraction,
        );
        Ok(Self::new(fingerprint, (nft_class_id, nft_instance_id, ft_id), account.clone(), fraction))
    }
}

// GenericAsset:

impl<Id, Payload, Account, Amount, Impl>
    TransferUnitT<Account, Amount, Impl>
    for GenericAsset<Id, Payload, Account, Amount, Impl>
{
    fn transfer(self, _to: &Account) {}

    fn amount(&self) -> &Amount {
        &self.3
    }
}

impl<Id, Payload, Account, Amount, Impl>
    GenericAssetT<Id, Payload, Account, Amount, Impl>
    for GenericAsset<Id, Payload, Account, Amount, Impl>
{
    fn new(id: Id, payload: Payload, account: Account, amount: Amount) -> Self {
        Self(id, payload, account, amount, <_>::default())
    }

    fn pick(_id: Id, _account: &Account, _amount: Amount) -> Option<Self> {
        None
    }

    fn balance(_id: &Id, _account: &Account) -> Option<Amount> {
        None
    }

    fn id(&self) -> &Id {
        &self.0
    }
    fn payload(&self) -> &Payload {
        &self.1
    }

    fn account(&self) -> &Account {
        &self.2
    }
}

// FToken:

impl<Account: Clone, Impl: Fungibles<Account>>
    TransferUnitT<Account, Impl::Balance, Impl>
    for GenericFToken<Impl::AssetId, (), Account, Impl::Balance, Impl>
{
    fn transfer(self, to: &Account) {
        Impl::transfer(
            *self.id(),
            self.account(),
            to,
            *self.amount(),
            true
        ).unwrap();
    }

    fn amount(&self) -> &Impl::Balance {
        self.0.amount()
    }
}

impl<Account: Clone, Impl: Fungibles<Account>>
    GenericAssetT<Impl::AssetId, (), Account, Impl::Balance, Impl>
    for GenericFToken<Impl::AssetId, (), Account, Impl::Balance, Impl>
{
    fn new(id: Impl::AssetId, payload: (), account: Account, amount: Impl::Balance) -> Self {
        Self(GenericAsset::new(id, payload, account, amount))
    }

    fn pick(id: Impl::AssetId, account: &Account, amount: Impl::Balance) -> Option<Self>
    {
        if amount.is_zero() || amount > Impl::balance(id, account) {
            return None
        }
        Some(Self::new(id, (), account.clone(), amount))
    }

    fn balance(id: &Impl::AssetId, account: &Account) -> Option<Impl::Balance> {
        let balance = Impl::balance(*id, account);
        if balance.is_zero() {
            return None
        }
        Some(balance)
    }

    fn id(&self) -> &Impl::AssetId {
        self.0.id()
    }

    fn payload(&self) -> &() {
        self.0.payload()
    }

    fn account(&self) -> &Account {
        self.0.account()
    }
}

impl<Account: Clone, Impl: Fungibles<Account>>
    FTokenT<Impl::AssetId, (), Account, Impl::Balance, Impl>
    for GenericFToken<Impl::AssetId, (), Account, Impl::Balance, Impl>
{
}

// NFToken:

impl<
    Account: Clone + PartialEq,
    Impl:
        Nonfungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId),
        >
>
    TransferUnitT<Account, (), Impl> for
    GenericNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl
    >
{
    fn transfer(self, to: &Account) {
        Impl::transfer(
            &self.payload().0,
            &self.payload().1,
            to,
        ).unwrap();
    }

    fn amount(&self) -> &() {
        self.0.amount()
    }
}

impl<
    Account: Clone + PartialEq,
    Impl:
        Nonfungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId),
        >
>
    GenericAssetT<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        (),
        Impl
    > for
    GenericNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl
    >
{
    fn new(
        id: Impl::Fingerprint,
        payload: Impl::NFTokenId,
        account: Account,
        amount: ()
    ) -> Self
    {
        Self(GenericAsset::new(id, payload, account, amount))
    }

    fn pick(id: Impl::Fingerprint, account: &Account, amount: ()) -> Option<Self> {
        let (class_id, instance_id) = Impl::lookup(&id)?;
        if account != &Impl::owner(&class_id, &instance_id)? {
            return None
        }
        Some(Self::new(id, (class_id, instance_id), account.clone(), amount))
    }

    fn balance(_id: &Impl::Fingerprint, _account: &Account) -> Option<()> {
        None
    }

    fn id(&self) -> &Impl::Fingerprint {
        self.0.id()
    }
    fn payload(&self) -> &Impl::NFTokenId {
        self.0.payload()
    }
    fn account(&self) -> &Account {
        self.0.account()
    }
}

impl<
    Account: Clone + PartialEq,
    Impl:
        Nonfungibles<Account> +
        NFTImplT<
            Account,
            NFTokenId=(Impl::ClassId, Impl::InstanceId),
        >
>
    NFTokenT<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        (),
        Impl
    > for
    GenericNFToken<
        Impl::Fingerprint,
        Impl::NFTokenId,
        Account,
        Impl
    >
{
    fn unique<U: Unique<Impl, Account>>(unique: U, owner: &Account) -> Result<Self, ()>
    {
        let fingerprint = unique.fingerprint();
        if Impl::lookup(&fingerprint).is_some() {
            return Err(())
        }
        let (nft_class_id, nft_instance_id) = Impl::obtain_id().ok_or(())?;
        let _ = Impl::create_class(
            &nft_class_id,
            owner,
            owner
        );
        let _ = Impl::mint_into(
            &nft_class_id,
            &nft_instance_id,
            owner
        );
        Ok(Self::new(fingerprint, (nft_class_id, nft_instance_id), owner.clone(), ()))
    }
}

// Legacy asset:

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Asset<AssetId, AssetBalance: Clone + AtLeast32BitUnsigned> {
    id: AssetId,
    amount: SerializableAtLeast32BitUnsigned<AssetBalance>,
}

impl<AssetId, AssetBalance: Clone + AtLeast32BitUnsigned> Asset<AssetId, AssetBalance> {
    pub fn new(id: AssetId, amount: AssetBalance) -> Self {
        Self { id, amount: SerializableAtLeast32BitUnsigned(amount) }
    }

    pub fn id(&self) -> &AssetId {
        &self.id
    }

    pub fn amount(&self) -> &AssetBalance {
        &self.amount.0
    }
}
