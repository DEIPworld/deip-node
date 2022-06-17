use crate::{CollectionRecordT, FractionalT, FractionRecordT, NFTImplT, Seal};


// Fraction ops:

pub fn total_fraction<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint,
) -> Option<Impl::FractionAmount>
{
    Some(*Impl::find_fractional(fingerprint, Seal(()))?.total())
}

pub fn hold_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    fingerprint: Impl::Fingerprint,
    holder_id: Impl::FractionHolderId,
    guard: Impl::FractionHoldGuard
) -> Result<(), ()>
{
    pick_fraction::<Impl>(account, fingerprint)?
        .hold(holder_id, guard)
}

pub fn unhold_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    fingerprint: Impl::Fingerprint,
    holder_id: Impl::FractionHolderId,
    guard: Impl::FractionHoldGuard
) -> Result<(), ()>
{
    pick_fraction::<Impl>(account, fingerprint)?
        .unhold(holder_id, guard)
}

pub fn transfer_fraction<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint,
    from: &Impl::Account,
    to: &Impl::Account,
    amount: Impl::FractionAmount
) -> Result<(), ()>
{
    pick_fraction::<Impl>(from, fingerprint)?
        .transfer_amount(to, amount)
}

pub fn transfer_fraction_full<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint,
    from: &Impl::Account,
    to: &Impl::Account,
) -> Result<(), ()>
{
    pick_fraction::<Impl>(from, fingerprint)?
        .check_account(from)?
        .transfer_all(to)

}

pub fn pick_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    fingerprint: Impl::Fingerprint,
) -> Result<impl NFTokenFractionT<Impl>, ()>
{
    NFTokenFraction::<Impl>::pick_fraction(fingerprint, account)
        .ok_or_else(|| ())
}

// Item ops:

pub fn transfer_item<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint,
    from: &Impl::Account,
    to: &Impl::Account,
) -> Result<(), ()>
{
    pick_item::<Impl>(fingerprint)?
        .check_account(from)?
        .transfer_item(to)
}

pub fn fractionalize_item<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint,
    account: &Impl::Account,
    total: Impl::FractionAmount
) -> Result<(), ()>
{
    pick_item::<Impl>(fingerprint)?
        .check_account(account)?
        .fractionalize(total)
}

pub fn pick_item<Impl: NFTImplT>(
    fingerprint: Impl::Fingerprint
) -> Result<impl NFTokenItemT<Impl>, ()>
{
    NFTokenItem::<Impl>::pick_item(fingerprint)
        .ok_or_else(|| ())
}

// Collection ops:

pub fn create_collection<Impl: NFTImplT>(
    account: &Impl::Account,
    max_items: Impl::ItemId
) -> Result<(), ()>
{
    NFTokenCollection::<Impl>::create_collection(
        account,
        max_items
    )
}

pub fn transfer_collection<Impl: NFTImplT>(
    id: Impl::CollectionId,
    from: &Impl::Account,
    to: &Impl::Account
) -> Result<(), ()>
{
    pick_collection::<Impl>(id)?
        .check_account(from)?
        .transfer_collection(to)
}

pub fn mint_item<Impl: NFTImplT>(
    id: Impl::CollectionId,
    account: &Impl::Account,
    unique: impl Unique<Impl>
) -> Result<(), ()>
{
    pick_collection::<Impl>(id)?
        .check_account(account)?
        .mint_item(unique)
}

pub fn pick_collection<Impl: NFTImplT>(
    id: Impl::CollectionId
) -> Result<impl NFTokenCollectionT<Impl>, ()>
{
    NFTokenCollection::<Impl>::pick_collection(id)
        .ok_or_else(|| ())
}

//

pub trait Unique<Impl: NFTImplT> {
    fn fingerprint(self) -> Impl::Fingerprint;
}

pub struct OpaqueUnique<Impl: NFTImplT>(pub Impl::Fingerprint);

impl<Impl: NFTImplT> Unique<Impl> for OpaqueUnique<Impl> {
    fn fingerprint(self) -> Impl::Fingerprint {
        self.0
    }
}

pub mod unique_demo {
    use super::Unique;
    use crate::NFTImplT;
    use sp_runtime::traits::Hash;

    pub struct Dao {
        pub id: u32,
    }

    pub struct UniqueDao<'a>(pub &'a Dao);

    impl<Impl: NFTImplT> Unique<Impl> for UniqueDao<'_> {
        fn fingerprint(self) -> Impl::Fingerprint {
            Impl::Hasher::hash_of(&("DAO", self.0.id))
        }
    }
}

//

pub struct NFTokenCollection<Impl: NFTImplT>(Impl::CollectionRecord)
where Self: NFTokenCollectionT<Impl>;

pub struct NFTokenItem<Impl: NFTImplT>(Impl::ItemRecord)
where Self: NFTokenItemT<Impl>;

pub struct NFTokenFraction<Impl: NFTImplT>(Impl::FractionRecord)
where Self: NFTokenFractionT<Impl>;

//

pub trait NFTokenCollectionT<Impl: NFTImplT>: Sized {
    fn create_collection(
        account: &Impl::Account,
        max_items: Impl::ItemId,
    ) -> Result<(), ()>;

    fn pick_collection(
        id: Impl::CollectionId
    ) -> Option<Self>;

    fn account(&self) -> &Impl::Account;

    fn check_account(
        self,
        account: &Impl::Account
    ) -> Result<Self, ()>
    {
        if self.account() == account {
            Ok(self)
        } else {
            Err(())
        }
    }

    fn transfer_collection(self, to: &Impl::Account) -> Result<(), ()>;

    fn mint_item(self, unique: impl Unique<Impl>) -> Result<(), ()>;
}

pub trait NFTokenItemT<Impl: NFTImplT>: Sized
{
    fn pick_item(
    ) -> Option<Self>;

    fn check_account(
        self,
        account: &Impl::Account
    ) -> Result<Self, ()>
    {
        if self.account() == account {
            Ok(self)
        } else {
            Err(())
        }
    }

    fn transfer_item(self, to: &Impl::Account) -> Result<(), ()>;

    fn fractionalize(self, total: Impl::FractionAmount) -> Result<(), ()>;
}

pub trait NFTokenFractionT<Impl: NFTImplT>: Sized
{
    fn pick_fraction(
        fingerprint: Impl::Fingerprint,
        account: &Impl::Account,
    ) -> Option<Self>;

    fn account(&self) -> &Impl::Account;

    fn amount(&self) -> &Impl::FractionAmount;

    fn fractional(&self) -> &Impl::Fractional;

    fn check_account(
        self,
        account: &Impl::Account
    ) -> Result<Self, ()>
    {
        if self.account() == account {
            Ok(self)
        } else {
            Err(())
        }
    }

    fn transfer_amount(
        self,
        to: &Impl::Account,
        amount: Impl::FractionAmount
    ) -> Result<(), ()>;

    fn transfer_all(
        self,
        to: &Impl::Account,
    ) -> Result<(), ()>
    {
        let amount = *self.amount();
        self.transfer_amount(to, amount)
    }

    fn on_hold(&self) -> bool;

    fn hold(
        self,
        holder_id: Impl::FractionHolderId,
        guard: Impl::FractionHoldGuard
    ) -> Result<(), ()>;

    fn unhold(
        self,
        holder_id: Impl::FractionHolderId,
        guard: Impl::FractionHoldGuard
    ) -> Result<(), ()>;
}

impl<Impl: NFTImplT> NFTokenCollectionT<Impl> for NFTokenCollection<Impl>
{
    fn create_collection(
        account: &Impl::Account,
        max_items: Impl::ItemId
    ) -> Result<(), ()>
    {
        Impl::create_collection(
            account,
            max_items,
            Seal(())
        )
    }

    fn pick_collection(
        id: Impl::CollectionId
    ) -> Option<Self>
    {
        Some(Self(Impl::find_collection(id, Seal(()))?))
    }

    fn account(&self) -> &Impl::Account {
        self.0.account()
    }

    fn transfer_collection(self, to: &Impl::Account) -> Result<(), ()> {
        let Self(collection) = self;
        Impl::transfer_collection(collection, to, Seal(()))
    }

    fn mint_item(self, unique: impl Unique<Impl>) -> Result<(), ()> {
        let Self(collection) = self;
        Impl::mint_item(collection, unique.fingerprint(), Seal(()))
    }
}

//

use crate::ItemRecordT;

impl<Impl: NFTImplT> NFTokenItemT<Impl> for NFTokenItem<Impl>
{
    fn pick_item(
        fingerprint: Impl::Fingerprint
    ) -> Option<Self>
    {
        Some(Self(Impl::find_item(fingerprint, Seal(()))?))
    }

    fn is_fractional(&self) -> bool {
        self.0.is_fractional()
    }

    fn account(&self) -> &Impl::Account {
        self.0.account()
    }

    fn transfer_item(self, to: &Impl::Account) -> Result<(), ()> {
        let Self(item) = self;
        Impl::transfer_item(item, to, Seal(()))
    }

    fn fractionalize(self, total: Impl::FractionAmount) -> Result<(), ()> {
        let Self(item) = self;
        Impl::fractionalize(item, total, Seal(()))
    }
}

//

impl<Impl: NFTImplT> NFTokenFractionT<Impl> for NFTokenFraction<Impl>
{
    fn pick_fraction(
        fingerprint: Impl::Fingerprint,
        account: &Impl::Account,
    ) -> Option<Self>
    {
        Some(Self(Impl::find_fraction(fingerprint, account, Seal(()))?))
    }

    fn account(&self) -> &Impl::Account {
        self.0.account()
    }

    fn amount(&self) -> &Impl::FractionAmount {
        self.0.amount()
    }

    fn fractional(&self) -> &Impl::Fractional {
        self.0.fractional()
    }

    fn transfer_amount(
        self,
        to: &Impl::Account,
        amount: Impl::FractionAmount
    ) -> Result<(), ()>
    {
        let Self(fraction) = self;
        Impl::transfer_fraction(fraction, to, amount, Seal(()))
    }

    fn on_hold(&self) -> bool {
        self.0.on_hold()
    }

    fn hold(
        self,
        holder_id: Impl::FractionHolderId,
        guard: Impl::FractionHoldGuard
    ) -> Result<(), ()>
    {
        let Self(fraction) = self;
        Impl::hold_fraction(fraction, holder_id, guard, Seal(()))
    }

    fn unhold(
        self,
        holder_id: Impl::FractionHolderId,
        guard: Impl::FractionHoldGuard
    ) -> Result<(), ()>
    {
        let Self(fraction) = self;
        Impl::unhold_fraction(fraction, holder_id, guard, Seal(()))
    }
}
