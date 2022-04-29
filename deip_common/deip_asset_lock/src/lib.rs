#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
use pallet_assets::Pallet as Assets;
use pallet_uniques::Pallet as Uniques;

pub trait LockableAsset<AccountId> {
    type AssetId;

    fn lock(who: &AccountId, id: Self::AssetId) -> DispatchResult;
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

impl<T: pallet_assets::Config<I>, I: 'static> LockableAsset<<T as frame_system::Config>::AccountId>
    for Assets<T, I>
{
    type AssetId = ();

    fn lock(who: &<T as frame_system::Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::lock_asset();
        Ok(())
    }

    fn unlock() -> Result {
        Ok(())
    }

    fn is_locked() -> bool {
        true
    }
}

impl<T: pallet_uniques::Config<I>, I: 'static> LockableAsset<<T as frame_system::Config>::AccountId>
    for Uniques<T, I>
{
    type AssetId = (T::ClassId, T::InstanceId);

    fn lock(who: &<T as frame_system::Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::lock_asset(who, id.0, id.1)
    }

    fn unlock() -> Result {
        Ok(())
    }

    fn is_locked() -> bool {
        true
    }
}
