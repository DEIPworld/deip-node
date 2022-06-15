use crate::{FractionalT, FractionRecordT, NFTImplT, FTImplT};


pub fn hold_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    id: Impl::NFTokenItemId,
    holder_id: Impl::FractionHolderId,
    guard: Impl::FractionHoldGuard
) -> Result<(), ()>
{
    NFTokenFraction::<Impl>::pick_fraction(account, id)
        .ok_or(())?
        .hold(holder_id, guard)
}

pub fn unhold_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    id: Impl::NFTokenItemId,
    holder_id: Impl::FractionHolderId,
    guard: Impl::FractionHoldGuard
) -> Result<(), ()>
{
    NFTokenFraction::<Impl>::pick_fraction(account, id)
        .ok_or(())?
        .unhold(holder_id, guard)
}

pub fn pick_fraction<Impl: NFTImplT>(
    account: &Impl::Account,
    id: Impl::NFTokenItemId,
) -> Option<impl NFTokenFractionT<Impl>>
{
    NFTokenFraction::<Impl>::pick_fraction(account, id)
}

//

pub trait Unique<Impl: NFTImplT> {
    fn fingerprint(self) -> Impl::Fingerprint;
}

pub struct OpaqueUnique<Impl: NFTImplT>(pub Impl::Fingerprint);

impl<Impl: NFTImplT> Unique<Impl> for OpaqueUnique<Impl>
{
    fn fingerprint(self) -> Impl::Fingerprint {
        self.0
    }
}

pub mod unique_demo {
    use crate::NFTImplT;
    use super::Unique;
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

pub trait NFTokenCollectionT<Impl: NFTImplT>: Sized
{
    fn create_collection(
        unique: impl Unique<Impl>,
        account: &Impl::Account,
        max_items: Impl::ItemId
    ) -> Result<(), ()>;

    fn pick_collection(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint
    ) -> Option<Self>;

    fn transfer_collection(self, to: &Impl::Account);

    fn mint_item(self) -> Result<(), ()>;
}

pub trait NFTokenItemT<Impl: NFTImplT>: Sized
{
    fn pick_item(
        account: &Impl::Account,
        id: Impl::NFTokenItemId
    ) -> Option<Self>;

    fn is_fractional(&self) -> bool;

    fn transfer_item(self, to: &Impl::Account) -> Result<(), ()>;

    fn fractionalize(self, total: Impl::FTokenAmount) -> Result<(), ()>;
}

pub trait NFTokenFractionT<Impl: NFTImplT>: Sized
{
    fn pick_fraction(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
    ) -> Option<Self>;

    fn total_issuance(&self) -> Impl::FTokenAmount;

    fn transfer_amount(
        self,
        to: &Impl::Account,
        amount: Impl::FTokenAmount
    ) -> Result<(), ()>;

    fn transfer_all(
        self,
        to: &Impl::Account,
    ) -> Result<(), ()>
    {
        let amount = *self.amount();
        self.transfer_amount(to, amount)
    }

    fn account(&self) -> &Impl::Account;

    fn amount(&self) -> &Impl::FTokenAmount;

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

//

impl<Impl: NFTImplT> NFTokenCollectionT<Impl> for NFTokenCollection<Impl>
{
    fn create_collection(
        unique: impl Unique<Impl>,
        account: &Impl::Account,
        max_items: Impl::ItemId
    ) -> Result<(), ()>
    {
        Impl::create_collection(
            unique.fingerprint(),
            account,
            max_items
        )
    }

    fn pick_collection(
        account: &Impl::Account,
        fingerprint: Impl::Fingerprint
    ) -> Option<Self>
    {
        Some(Self(Impl::find_collection(account.clone(), fingerprint)?))
    }

    fn transfer_collection(self, to: &Impl::Account) {
        let Self(collection) = self;
        Impl::transfer_collection(collection, to);
    }

    fn mint_item(self) -> Result<(), ()> {
        let Self(collection) = self;
        Impl::mint_item(collection)
    }
}

//

use crate::ItemRecordT;

impl<Impl: NFTImplT> NFTokenItemT<Impl> for NFTokenItem<Impl>
{
    fn pick_item(
        account: &Impl::Account,
        id: Impl::NFTokenItemId
    ) -> Option<Self>
    {
        Some(Self(Impl::find_item(account.clone(), id)?))
    }

    fn is_fractional(&self) -> bool {
        self.0.is_fractional()
    }

    fn transfer_item(self, to: &Impl::Account) -> Result<(), ()> {
        let Self(item) = self;
        Impl::transfer_item(item, to)
    }

    fn fractionalize(self, total: Impl::FTokenAmount) -> Result<(), ()> {
        let Self(item) = self;
        Impl::fractionalize(item, total)
    }
}

//

impl<Impl: NFTImplT> NFTokenFractionT<Impl> for NFTokenFraction<Impl>
{
    fn pick_fraction(
        account: &Impl::Account,
        id: Impl::NFTokenItemId,
    ) -> Option<Self>
    {
        Some(Self(Impl::find_fraction(account, id)?))
    }

    fn total_issuance(&self) -> Impl::FTokenAmount {
        Impl::Fungibles::total_issuance(*self.0.fractional().ft_id())
    }

    fn transfer_amount(
        self,
        to: &Impl::Account,
        amount: Impl::FTokenAmount
    ) -> Result<(), ()>
    {
        let Self(fraction) = self;
        Impl::transfer_fraction(fraction, to, amount)
    }

    fn account(&self) -> &Impl::Account {
        self.0.account()
    }

    fn amount(&self) -> &Impl::FTokenAmount {
        self.0.amount()
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
        Impl::hold_fraction(fraction, holder_id, guard)
    }

    fn unhold(
        self,
        holder_id: Impl::FractionHolderId,
        guard: Impl::FractionHoldGuard
    ) -> Result<(), ()>
    {
        let Self(fraction) = self;
        Impl::unhold_fraction(fraction, holder_id, guard)
    }
}
