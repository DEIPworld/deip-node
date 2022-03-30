#![cfg_attr(not(feature = "std"), no_std)]

pub trait Lockable<Id> {
    fn lock(id: impl Into<Id>) -> Result;
    fn unlock(id: Id) -> Result;
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
