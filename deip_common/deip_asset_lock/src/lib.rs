#![cfg_attr(not(feature = "std"), no_std)]

use core::marker::PhantomData;

pub struct LockableAsset<T, Id> {
    runtime: PhantomData<T>,
    id: PhantomData<Id>,
}

pub trait Lockable<T, Id> {
    fn lock(id: impl Into<Id>) -> Result;
    fn unlock(id: Id) -> Result;
    fn is_locked(id: Id) -> bool;
}

impl<T, Id> Lockable<T, Id> for LockableAsset<T, Id> {
    fn lock(id: impl Into<Id>) -> Result {
        todo!()
    }

    fn unlock(id: Id) -> Result {
        todo!()
    }

    fn is_locked(id: Id) -> bool {
        todo!()
    }
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
