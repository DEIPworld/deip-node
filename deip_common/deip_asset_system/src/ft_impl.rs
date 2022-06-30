use sp_runtime::traits::{AtLeast32BitUnsigned, One, Zero, CheckedAdd};
use frame_support::storage::{StorageValue};
use frame_support::traits::tokens::{fungibles::{self, Create, Mutate, Inspect, Transfer}};
use frame_support::pallet_prelude::*;

use crate::{Seal, error::Error};

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
    
    type Error: Error + Into<DispatchError>;

    fn _obtain_ft_id(_: Seal) -> Option<Self::FTokenId> {
        let id = Self::NextFTokenId::try_get()
            .unwrap_or(Self::FTokenId::zero());
        Self::NextFTokenId::put(id.checked_add(&Self::FTokenId::one())?);
        Some(id)
    }

    fn create_ft(
        account: Self::Account,
        minimum_balance: Self::FTokenAmount,
        _: Seal
    ) -> Result<Self::FTokenId, DispatchError>
    {
        let id = Self::_obtain_ft_id(Seal(()))
            .ok_or_else(|| Self::Error::unknown_f_token_id().into())?;
        Self::Fungibles::create(
            id,
            account,
            true,
            minimum_balance
        )?;
        Ok(id)
    }

    fn can_mint(
        _id: Self::FTokenId,
        _account: &Self::Account,
        _: Seal,
    ) -> bool
    {
        // @TODO Self::Fungibles::is_lock_mint()
        true
    }

    fn can_burn(
        _: Self::FTokenId,
        _: &Self::Account,
        _: Seal,
    ) -> bool {
        // @TODO add checks

        true
    }

    fn mint_ft(
        id: Self::FTokenId,
        account: &Self::Account,
        amount: Self::FTokenAmount,
        _: Seal
    ) -> DispatchResult
    {
       ensure!(Self::can_mint(id, account, Seal(())), Self::Error::no_permission());
        let minimum_balance = Self::Fungibles::minimum_balance(id);

        ensure!(amount >= minimum_balance, Self::Error::insufficient_balance());

        Self::Fungibles::mint_into(
            id,
            account,
            amount
        )
    }

    fn burn_ft(
        id: Self::FTokenId,
        account: &Self::Account,
        amount: Self::FTokenAmount,
        _: Seal
    ) -> Result<Self::FTokenAmount, DispatchError> {
        todo!()
    }

    fn lock_minting(
        _id: Self::FTokenId,
        _account: &Self::Account,
        _: Seal
    ) -> DispatchResult
    {
        // Self::Fungibles::lock_mint()
        Ok(())
    }

    fn balance(
        id: Self::FTokenId,
        account: &Self::Account,
        _: Seal
    ) -> Self::FTokenAmount
    {
        Self::Fungibles::balance(
            id,
            account
        )
    }

    fn total_issuance(
        id: Self::FTokenId,
        _: Seal
    ) -> Self::FTokenAmount
    {
        Self::Fungibles::total_issuance(id)
    }

    fn _can_transfer(
        _id: Self::FTokenId,
        _account: &Self::Account,
        _: Seal
    ) -> DispatchResult
    {
        // Self::Fungibles::is_lock_transfer()
        Ok(())
    }

    fn transfer(
        id: Self::FTokenId,
        from: &Self::Account,
        to: &Self::Account,
        amount: Self::FTokenAmount,
        _: Seal
    ) -> DispatchResult
    {
        Self::_can_transfer(id, from, Seal(()))?;

        ensure!(!amount.is_zero(), Self::Error::bad_value());
        
        ensure!(from != to, Self::Error::bad_target());

        Self::Fungibles::transfer(
            id,
            from,
            to,
            amount,
            true
        )?;

        Ok(())
    }

    fn lock_transfer(
        _id: Self::FTokenId,
        _account: &Self::Account,
        _: Seal
    ) -> Result<(), ()>
    {
        // Self::Fungibles::lock_transfer()
        Ok(())
    }
}
