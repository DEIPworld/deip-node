#![cfg_attr(not(feature = "std"), no_std)]

use pallet_assets::Pallet as Assets;
use pallet_uniques::Pallet as Uniques;

pub trait LockableAsset {
    fn lock() -> Result;
    fn unlock() -> Result;
    fn is_locked() -> bool;
}

#[derive(Debug, Clone)]
pub enum Error {
    AlreadyLocked,
    NotLocked,
    AssetNotFound,
    AccountDoesNotHaveAsset,
    Unknown,
}

pub type Result = core::result::Result<(), Error>;

impl<T, I: 'static> LockableAsset for Assets<T, I> {
    fn lock() -> Result {
        Self::lock_asset();
        Ok(())
    }

    fn unlock() -> Result {
        todo!()
    }

    fn is_locked() -> bool {
        todo!()
    }
}

impl<T, I: 'static> LockableAsset for Uniques<T, I> {
    fn lock() -> Result {
        Self::lock_asset();
        Ok(())
    }

    fn unlock() -> Result {
        todo!()
    }

    fn is_locked() -> bool {
        todo!()
    }
}
