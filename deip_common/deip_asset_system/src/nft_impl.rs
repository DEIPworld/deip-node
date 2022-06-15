use sp_runtime::traits::{Hash, AtLeast32BitUnsigned, One, Zero, CheckedAdd, Saturating, CheckedSub};
use frame_support::storage::{StorageNMap, StorageValue};
use frame_support::pallet_prelude::*;
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use frame_support::traits::tokens::nonfungibles::{
    self,
    Create,
    Mutate,
    Transfer
};
use crate::FTImplT;


pub type CollectionRecord<T> = <T as NFTImplT>::CollectionRecord;
pub type ItemRecord<T> = <T as NFTImplT>::ItemRecord;
pub type FractionRecord<T> = <T as NFTImplT>::FractionRecord;

//

pub type NFTokenCollectionId<T> = <T as NFTImplT>::NFTokenCollectionId;
pub type NFTokenItemId<T> = <T as NFTImplT>::NFTokenItemId;

//

pub trait NFTImplT
{
    type Fungibles: FTImplT<
        Account=Self::Account,
        FTokenId=Self::FTokenId,
        FTokenAmount=Self::FTokenAmount
    >;

    type Fingerprint: Copy + Parameter + 'static;

    type Hasher: Hash<Output=Self::Fingerprint>;

    type CollectionId: AtLeast32BitUnsigned + Copy + Parameter;
    type ItemId: AtLeast32BitUnsigned + Copy + Parameter + 'static;
    type FTokenId: AtLeast32BitUnsigned + Copy + Parameter;
    type FTokenAmount: AtLeast32BitUnsigned + Copy + Parameter;

    type Account: Clone + Parameter + 'static;

    type NFTokenCollectionId: NFTokenCollectionIdT<Self> + Copy; // (Self::Fingerprint, Self::CollectionId)
    type NFTokenItemId: NFTokenItemIdT<Self> + Copy;             // (Self::Fingerprint, Self::ItemId)

    type Fractional: FractionalT<Self> + Copy;                   // (Self::FTokenId, Self::FTokenAmount)

    type CollectionRecord: CollectionRecordT<Self> + Parameter;
    type ItemRecord: ItemRecordT<Self> + Parameter;
    type FractionRecord: FractionRecordT<Self> + Parameter;

    type CollectionRepo: StorageNMap<
        (
            NMapKey<Blake2_128Concat, Self::Account>,
            NMapKey<Blake2_128Concat, Self::Fingerprint>
        ),
        Self::CollectionRecord
    >;

    type ItemRepo: StorageNMap<
        (
            NMapKey<Blake2_128Concat, Self::Fingerprint>,
            NMapKey<Blake2_128Concat, Self::Account>,
            NMapKey<Twox64Concat, Self::ItemId>
        ),
        Self::ItemRecord
    >;

    type FractionRepo: StorageNMap<
        (
            NMapKey<Blake2_128Concat, Self::Fingerprint>,
            NMapKey<Blake2_128Concat, Self::Account>,
            NMapKey<Twox64Concat, Self::ItemId>
        ),
        Self::FractionRecord
    >;

    type FractionHolderId: From<sp_core::H160> +  Copy + Parameter + 'static;
    type FractionHoldGuard: AtLeast32BitUnsigned + Copy + Parameter + 'static;
    type FractionHolds: StorageNMap<
        (
            NMapKey<Blake2_128Concat, Self::Fingerprint>,
            NMapKey<Blake2_128Concat, Self::Account>,
            NMapKey<Blake2_128Concat, Self::FractionHolderId>,
            NMapKey<Blake2_128Concat, Self::FractionHoldGuard>,
        ),
        (Self::FractionHolderId, Self::FractionHoldGuard)
    >;

    type NextCollectionId: StorageValue<Self::CollectionId>;

    type Nonfungibles:
        nonfungibles::Inspect<
            Self::Account,
            ClassId=Self::CollectionId,
            InstanceId=Self::ItemId
        > +
        nonfungibles::Transfer<Self::Account> +
        nonfungibles::Create<Self::Account> +
        nonfungibles::Mutate<Self::Account>;

    fn _obtain_collection_id() -> Option<Self::CollectionId> {
        let id = Self::NextCollectionId::try_get()
            .unwrap_or(Self::CollectionId::zero());
        Self::NextCollectionId::put(id.checked_add(&Self::CollectionId::one())?);
        Some(id)
    }

    fn find_collection(
        account: Self::Account,
        fingerprint: Self::Fingerprint
    ) -> Option<Self::CollectionRecord>
    {
        Self::CollectionRepo::try_get((account, fingerprint)).ok()
    }

    fn find_item(
        account: Self::Account,
        id: Self::NFTokenItemId
    ) -> Option<Self::ItemRecord>
    {
        let (fingerprint, item_id) = id.split();
        Self::ItemRepo::try_get((fingerprint, account, item_id)).ok()
    }

    fn find_fraction(
        account: &Self::Account,
        id: Self::NFTokenItemId
    ) -> Option<Self::FractionRecord>
    {
        let (fingerprint, item_id) = id.split();
        Self::FractionRepo::try_get((
            fingerprint,
            account.clone(),
            item_id
        )).ok()
    }

    fn _insert_collection(
        collection: Self::CollectionRecord
    ) {
        Self::CollectionRepo::insert(
            (
                collection.account().clone(),
                *collection.id().fingerprint()
            ),
            collection
        );
    }

    fn _insert_item(
        item: Self::ItemRecord
    ) {
        Self::ItemRepo::insert(
            (
                *item.id().fingerprint(),
                item.account().clone(),
                *item.id().item_id()
            ),
            item
        );
    }

    fn _insert_fraction(
        fraction: Self::FractionRecord
    ) {
        Self::FractionRepo::insert(
            (
                *fraction.id().fingerprint(),
                fraction.account().clone(),
                *fraction.id().item_id()
            ),
            fraction
        );
    }

    fn _remove_fraction(
        fraction: &Self::FractionRecord
    ) {
        Self::FractionRepo::remove((
            *fraction.id().fingerprint(),
            fraction.account().clone(),
            *fraction.id().item_id()
        ));
    }

    fn _fraction_hold_key(
        fraction: &Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard
    ) -> (Self::Fingerprint,
          Self::Account,
          Self::FractionHolderId,
          Self::FractionHoldGuard)
    {
        (
            *fraction.id().fingerprint(),
            fraction.account().clone(),
            holder_id,
            guard
        )
    }

    fn create_collection(
        fingerprint: Self::Fingerprint,
        account: &Self::Account,
        max_items: Self::ItemId
    ) -> Result<(), ()>
    {
        if max_items.is_zero() { return Err(()) }

        if Self::find_collection(account.clone(), fingerprint).is_some() {
            return Err(())
        }

        let id = Self::_obtain_collection_id().ok_or(())?;

        Self::Nonfungibles::create_class(
            &id,
            account,
            account
        ).map_err(|_| ())?;

        let collection = Self::CollectionRecord::new(
            account,
            Self::NFTokenCollectionId::new(fingerprint, id),
            max_items,
            Self::ItemId::zero()
        );

        Self::_insert_collection(collection);

        Ok(())
    }

    fn mint_item(
        mut collection: Self::CollectionRecord,
    ) -> Result<(), ()>
    {
        let id = collection.obtain_item_id().ok_or(())?;

        let item = Self::ItemRecord::new(
            collection.account(),
            id,
            *collection.id().collection_id(),
            None
        );

        Self::_insert_collection(collection);

        Self::Nonfungibles::mint_into(
            item.collection_id(),
            item.id().item_id(),
            item.account()
        ).map_err(|_| ())?;

        Self::_insert_item(item);

        Ok(())
    }

    fn fractionalize(
        mut item: Self::ItemRecord,
        total: Self::FTokenAmount,
    ) -> Result<(), ()>
    {
        if item.is_fractional() { return Err(()) }

        if total.is_zero() { return Err(()) }

        let minimum_balance = One::one();

        let ft_id = Self::Fungibles::create_ft(
            item.account().clone(),
            minimum_balance
        )?;

        Self::Fungibles::mint_ft(
            ft_id,
            item.account(),
            total
        )?;

        Self::Fungibles::lock_minting(ft_id, item.account())?;

        let fractional = Self::Fractional::new(ft_id, total);

        Self::_insert_fraction(Self::FractionRecord::new(
            item.account(),
            *item.id(),
            fractional,
            total,
            <Self::FractionHoldGuard>::zero(),
        ));

        item.fractionalize(fractional);

        Self::_insert_item(item);

        Ok(())
    }

    fn transfer_collection(
        mut collection: Self::CollectionRecord,
        to: &Self::Account
    ) {
        collection.transfer_collection(to);
        Self::_insert_collection(collection);
    }

    fn transfer_item(
        mut item: Self::ItemRecord,
        to: &Self::Account,
    ) -> Result<(), ()>
    {
        if item.is_fractional() { return Err(()) }

        Self::Nonfungibles::transfer(
            item.collection_id(),
            item.id().item_id(),
            to
        ).map_err(|_| ())?;

        item.transfer_item(to);

        Self::_insert_item(item);

        Ok(())
    }

    fn transfer_fraction(
        mut donor: Self::FractionRecord,
        to: &Self::Account,
        amount: Self::FTokenAmount
    ) -> Result<(), ()>
    {
        if donor.on_hold() { return Err(()) }

        if amount.is_zero() { return Err(()) }

        if &amount > donor.amount() { return Err(()) }

        if donor.account() == to { return Err(()) }

        let maybe_fraction = Self::find_fraction(to, *donor.id());

        let mut fraction = maybe_fraction.unwrap_or(
            Self::FractionRecord::new(
                to,
                *donor.id(),
                *donor.fractional(),
                <Self::FTokenAmount>::zero(),
                <Self::FractionHoldGuard>::zero(),
            )
        );

        if fraction.on_hold() { return Err(()) }

        Self::Fungibles::transfer(
            *donor.fractional().ft_id(),
            donor.account(),
            to,
            amount,
        )?;

        fraction.increase_amount(amount)?;

        Self::_insert_fraction(fraction);

        donor.decrease_amount(amount)?;

        if donor.amount().is_zero() {
            Self::_remove_fraction(&donor);
        } else {
            Self::_insert_fraction(donor);
        }

        Ok(())
    }

    fn hold_fraction(
        mut fraction: Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard
    ) -> Result<(), ()>
    {
        let key = Self::_fraction_hold_key(&fraction, holder_id, guard);

        if Self::FractionHolds::contains_key(&key) { return Err(()) }

        Self::FractionHolds::insert(key, (holder_id, guard));

        fraction.inc_holds()?;

        Self::_insert_fraction(fraction);

        Ok(())
    }

    fn unhold_fraction(
        mut fraction: Self::FractionRecord,
        holder_id: Self::FractionHolderId,
        guard: Self::FractionHoldGuard
    ) -> Result<(), ()>
    {
        let key = Self::_fraction_hold_key(&fraction, holder_id, guard);

        if !Self::FractionHolds::contains_key(&key) { return Err(()) }

        Self::FractionHolds::remove(key);

        fraction.dec_holds()?;

        Self::_insert_fraction(fraction);

        Ok(())
    }
}

//

pub trait NFTokenCollectionIdT<Impl: NFTImplT + ?Sized>: Sized
{
    fn fingerprint(&self) -> &Impl::Fingerprint;

    fn collection_id(&self) -> &Impl::CollectionId;

    fn split(self) -> (Impl::Fingerprint, Impl::CollectionId);

    fn new(
        fingerprint: Impl::Fingerprint,
        collection_id: Impl::CollectionId
    ) -> Self;
}

impl<Impl: NFTImplT + ?Sized> NFTokenCollectionIdT<Impl> for (Impl::Fingerprint, Impl::CollectionId)
{
    fn fingerprint(&self) -> &Impl::Fingerprint {
        &self.0
    }

    fn collection_id(&self) -> &Impl::CollectionId {
        &self.1
    }

    fn split(self) -> (Impl::Fingerprint, Impl::CollectionId) {
        self
    }

    fn new(
        fingerprint: Impl::Fingerprint,
        collection_id: Impl::CollectionId
    ) -> Self
    {
        (fingerprint, collection_id)
    }
}

//

pub trait NFTokenItemIdT<Impl: NFTImplT + ?Sized>: Sized
{
    fn fingerprint(&self) -> &Impl::Fingerprint;

    fn item_id(&self) -> &Impl::ItemId;

    fn split(self) -> (Impl::Fingerprint, Impl::ItemId);

    fn new(
        fingerprint: Impl::Fingerprint,
        item_id: Impl::ItemId
    ) -> Self;
}

impl<Impl: NFTImplT + ?Sized> NFTokenItemIdT<Impl> for (Impl::Fingerprint, Impl::ItemId)
{
    fn fingerprint(&self) -> &Impl::Fingerprint {
        &self.0
    }

    fn item_id(&self) -> &Impl::ItemId {
        &self.1
    }

    fn split(self) -> (Impl::Fingerprint, Impl::ItemId) {
        self
    }

    fn new(
        fingerprint: Impl::Fingerprint,
        item_id: Impl::ItemId
    ) -> Self
    {
        (fingerprint, item_id)
    }
}

//

pub trait CollectionRecordT<Impl: NFTImplT + ?Sized>: Sized
{
    fn account(&self) -> &Impl::Account;

    fn id(&self) -> &Impl::NFTokenCollectionId;

    fn max_items(&self) -> &Impl::ItemId;

    fn items(&self) -> &Impl::ItemId;

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenCollectionId,
        max_items: Impl::ItemId,
        items: Impl::ItemId
    ) -> Self;

    fn _inc_items(&mut self);

    fn _mut_account(&mut self) -> &mut Impl::Account;

    fn obtain_item_id(&mut self) -> Option<Impl::NFTokenItemId>
    {
        if self.items() < self.max_items() {
            let id = *self.items();
            self._inc_items();
            return Some(
                Impl::NFTokenItemId::new(
                    *self.id().fingerprint(),
                    id)
            )
        }
        None
    }

    fn transfer_collection(&mut self, to: &Impl::Account) {
        *self._mut_account() = to.clone();
    }
}

//

pub trait ItemRecordT<Impl: NFTImplT + ?Sized>: Sized
{
    fn account(&self) -> &Impl::Account;

    fn id(&self) -> &Impl::NFTokenItemId;

    fn collection_id(&self) -> &Impl::CollectionId;

    fn fractional(&self) -> Option<&Impl::Fractional>;

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
        collection_id: Impl::CollectionId,
        fractional: Option<Impl::Fractional>,
    ) -> Self;

    fn is_fractional(&self) -> bool {
        self.fractional().is_some()
    }

    fn _mut_account(&mut self) -> &mut Impl::Account;

    fn _mut_fractional(&mut self) -> &mut Option<Impl::Fractional>;

    fn transfer_item(&mut self, to: &Impl::Account) {
        *self._mut_account() = to.clone();
    }

    fn fractionalize(&mut self, fractional: Impl::Fractional) {
        self._mut_fractional().replace(fractional);
    }

    fn fuse(&mut self) {
        *self._mut_fractional() = None;
    }
}

//

pub trait FractionalT<Impl: NFTImplT + ?Sized>: Sized
{
    fn ft_id(&self) -> &Impl::FTokenId;

    fn total(&self) -> &Impl::FTokenAmount;

    fn new(
        ft_id: Impl::FTokenId,
        total: Impl::FTokenAmount
    ) -> Self;
}

impl<Impl: NFTImplT + ?Sized> FractionalT<Impl> for (Impl::FTokenId, Impl::FTokenAmount)
{
    fn ft_id(&self) -> &Impl::FTokenId {
        &self.0
    }

    fn total(&self) -> &Impl::FTokenAmount {
        &self.1
    }

    fn new(
        ft_id: Impl::FTokenId,
        total: Impl::FTokenAmount
    ) -> Self
    {
        (ft_id, total)
    }
}

//

pub trait FractionRecordT<Impl: NFTImplT + ?Sized>: Sized
{
    fn account(&self) -> &Impl::Account;

    fn id(&self) -> &Impl::NFTokenItemId;

    fn fractional(&self) -> &Impl::Fractional;

    fn amount(&self) -> &Impl::FTokenAmount;

    fn holds(&self) -> &Impl::FractionHoldGuard;

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
        fractional: Impl::Fractional,
        amount: Impl::FTokenAmount,
        holds: Impl::FractionHoldGuard
    ) -> Self;

    fn _mut_amount(&mut self) -> &mut Impl::FTokenAmount;

    fn _mut_holds(&mut self) -> &mut Impl::FractionHoldGuard;

    fn can_fuse(&self) -> bool {
        self.amount() == self.fractional().total()
    }

    fn on_hold(&self) -> bool {
        !self.holds().is_zero()
    }

    fn increase_amount(&mut self, by: Impl::FTokenAmount) -> Result<(), ()>
    {
        *self._mut_amount() = self.amount().checked_add(&by).ok_or(())?;
        Ok(())
    }

    fn decrease_amount(&mut self, by: Impl::FTokenAmount) -> Result<(), ()>
    {
        *self._mut_amount() = self.amount().checked_sub(&by).ok_or(())?;
        Ok(())
    }

    fn inc_holds(&mut self) -> Result<(), ()> {
        *self._mut_holds() = self.holds().checked_add(&One::one()).ok_or(())?;
        Ok(())
    }

    fn dec_holds(&mut self) -> Result<(), ()> {
        *self._mut_holds() = self.holds().checked_sub(&One::one()).ok_or(())?;
        Ok(())
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenCollectionRecord<Account, Id, ItemId> {
    pub account: Account,
    pub id: Id,
    pub max_items: ItemId,
    pub items: ItemId
}

impl<Impl: NFTImplT + ?Sized> CollectionRecordT<Impl> for
    NFTokenCollectionRecord<
        Impl::Account,
        Impl::NFTokenCollectionId,
        Impl::ItemId
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn id(&self) -> &Impl::NFTokenCollectionId {
        &self.id
    }

    fn max_items(&self) -> &Impl::ItemId {
        &self.max_items
    }

    fn items(&self) -> &Impl::ItemId {
        &self.items
    }

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenCollectionId,
        max_items: Impl::ItemId,
        items: Impl::ItemId
    ) -> Self
    {
        Self {
            account: account.clone(),
            id,
            max_items,
            items
        }
    }

    fn _inc_items(&mut self) {
        self.items.saturating_inc();
    }

    fn _mut_account(&mut self) -> &mut Impl::Account {
        &mut self.account
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenItemRecord<Account, Id, CollectionId, Fractional> {
    pub account: Account,
    pub id: Id,
    pub collection_id: CollectionId,
    pub fractional: Option<Fractional>
}

impl<Impl: NFTImplT + ?Sized> ItemRecordT<Impl> for
    NFTokenItemRecord<
        Impl::Account,
        Impl::NFTokenItemId,
        Impl::CollectionId,
        Impl::Fractional
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn id(&self) -> &Impl::NFTokenItemId {
        &self.id
    }

    fn collection_id(&self) -> &Impl::CollectionId {
        &self.collection_id
    }

    fn fractional(&self) -> Option<&Impl::Fractional> {
        self.fractional.as_ref()
    }

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
        collection_id: Impl::CollectionId,
        fractional: Option<Impl::Fractional>
    ) -> Self
    {
        Self {
            account: account.clone(),
            id,
            collection_id,
            fractional
        }
    }

    fn _mut_account(&mut self) -> &mut Impl::Account {
        &mut self.account
    }

    fn _mut_fractional(&mut self) -> &mut Option<Impl::Fractional> {
        &mut self.fractional
    }
}

//

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub struct NFTokenFractionRecord<Account, Id, Fractional, Amount, HoldGuard> {
    account: Account,
    id: Id,
    fractional: Fractional,
    amount: Amount,
    holds: HoldGuard,
}

impl<Impl: NFTImplT + ?Sized> FractionRecordT<Impl> for
    NFTokenFractionRecord<
        Impl::Account,
        Impl::NFTokenItemId,
        Impl::Fractional,
        Impl::FTokenAmount,
        Impl::FractionHoldGuard
    >
{
    fn account(&self) -> &Impl::Account {
        &self.account
    }

    fn id(&self) -> &Impl::NFTokenItemId {
        &self.id
    }

    fn fractional(&self) -> &Impl::Fractional {
        &self.fractional
    }

    fn amount(&self) -> &Impl::FTokenAmount {
        &self.amount
    }

    fn holds(&self) -> &Impl::FractionHoldGuard {
        &self.holds
    }

    fn new(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
        fractional: Impl::Fractional,
        amount: Impl::FTokenAmount,
        holds: Impl::FractionHoldGuard
    ) -> Self
    {
        Self {
            account: account.clone(),
            id,
            fractional,
            amount,
            holds
        }
    }

    fn _mut_amount(&mut self) -> &mut Impl::FTokenAmount {
        &mut self.amount
    }

    fn _mut_holds(&mut self) -> &mut Impl::FractionHoldGuard {
        &mut self.holds
    }
}
