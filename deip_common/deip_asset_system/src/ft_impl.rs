use sp_runtime::traits::{AtLeast32BitUnsigned, One, Zero, CheckedAdd};
use frame_support::storage::{StorageValue};
use frame_support::traits::tokens::{fungibles::{self, Create, Mutate, Inspect, Transfer}};
use frame_support::pallet_prelude::*;

pub trait FTImplT:
{
    type Account: Clone + Parameter;

    type FTokenId: AtLeast32BitUnsigned + Copy + Parameter;
    type FTokenAmount: AtLeast32BitUnsigned + Copy;

    type NextFTokenId: StorageValue<Self::FTokenId>;

    type Fungibles:
        fungibles::Inspect<
            Self::Account,
            AssetId=Self::FTokenId,
            Balance=Self::FTokenAmount
        > +
        fungibles::Transfer<Self::Account> +
        fungibles::Create<Self::Account> +
        fungibles::Mutate<Self::Account>
        // + LockableAsset<Self::AccountId>
        ;

    fn _obtain_ft_id() -> Option<Self::FTokenId> {
        let id = Self::NextFTokenId::try_get()
            .unwrap_or(Self::FTokenId::zero());
        Self::NextFTokenId::put(id.checked_add(&Self::FTokenId::one())?);
        Some(id)
    }

    fn create_ft(
        account: Self::Account,
        minimum_balance: Self::FTokenAmount
    ) -> Result<Self::FTokenId, ()>
    {
        let id = Self::_obtain_ft_id().ok_or(())?;
        Self::Fungibles::create(
            id,
            account,
            true,
            minimum_balance
        ).map_err(|_| ())?;
        Ok(id)
    }

    fn _can_mint(
        _id: Self::FTokenId,
        _account: &Self::Account
    ) -> Result<(), ()>
    {
        // Self::Fungibles::is_lock_mint()
        Ok(())
    }

    fn mint_ft(
        id: Self::FTokenId,
        account: &Self::Account,
        amount: Self::FTokenAmount
    ) -> Result<(), ()>
    {
        Self::_can_mint(id, account)?;
        let minimum_balance = Self::Fungibles::minimum_balance(id);
        if amount < minimum_balance { return Err(()) }
        Self::Fungibles::mint_into(
            id,
            account,
            amount
        ).map_err(|_| ())
    }

    fn lock_minting(
        _id: Self::FTokenId,
        _account: &Self::Account
    ) -> Result<(), ()>
    {
        // Self::Fungibles::lock_mint()
        Ok(())
    }

    fn balance(
        id: Self::FTokenId,
        account: &Self::Account
    ) -> Self::FTokenAmount
    {
        Self::Fungibles::balance(
            id,
            account
        )
    }

    fn total_issuance(
        id: Self::FTokenId,
    ) -> Self::FTokenAmount
    {
        Self::Fungibles::total_issuance(id)
    }

    fn _can_transfer(
        _id: Self::FTokenId,
        _account: &Self::Account
    ) -> Result<(), ()>
    {
        // Self::Fungibles::is_lock_transfer()
        Ok(())
    }

    fn transfer(
        id: Self::FTokenId,
        from: &Self::Account,
        to: &Self::Account,
        amount: Self::FTokenAmount
    ) -> Result<(), ()>
    {
        Self::_can_transfer(id, from)?;

        if amount.is_zero() { return Err(()) }

        if from == to { return Err(()) }

        Self::Fungibles::transfer(
            id,
            from,
            to,
            amount,
            true
        ).map_err(|_| ())?;

        Ok(())
    }

    fn lock_transfer(
        _id: Self::FTokenId,
        _account: &Self::Account
    ) -> Result<(), ()>
    {
        // Self::Fungibles::lock_transfer()
        Ok(())
    }
}
