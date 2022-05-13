#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
use frame_system::Config;
use pallet_assets::Pallet as Assets;
use pallet_uniques::Pallet as Uniques;

pub trait LockableAsset<AccountId> {
    type AssetId;

    fn lock(who: &AccountId, id: Self::AssetId) -> DispatchResult;

    fn unlock(who: &AccountId, id: Self::AssetId) -> DispatchResult;
    fn unlock_mint(who: &AccountId, id: Self::AssetId) -> DispatchResult;
    fn unlock_transfer(who: &AccountId, id: Self::AssetId) -> DispatchResult;

    fn is_locked(id: Self::AssetId) -> bool;
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

impl<T: pallet_assets::Config<I>, I: 'static> LockableAsset<<T as Config>::AccountId>
    for Assets<T, I>
{
    type AssetId = T::AssetId;

    fn lock(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::lock_asset(who, id)
    }

    fn unlock(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::unlock_asset(who, id)
    }

    fn unlock_mint(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::unlock_asset_mint(who, id)
    }

    fn unlock_transfer(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::unlock_asset_transfer(who, id)
    }

    fn is_locked(id: Self::AssetId) -> bool {
        Self::is_asset_locked(id)
    }
}

impl<T: pallet_uniques::Config<I>, I: 'static> LockableAsset<<T as Config>::AccountId>
    for Uniques<T, I>
{
    type AssetId = (T::ClassId, T::InstanceId);

    fn lock(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::lock_asset(who, id.0, id.1)
    }

    fn unlock(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::unlock_asset(who, id.0, id.1)
    }

    /// Lock sets on pair (ClassId, InstanceId), so (un)locking mint separatly doesn't make
    /// sense.
    fn unlock_mint(_: &<T as Config>::AccountId, _: Self::AssetId) -> DispatchResult {
        Ok(())
    }

    fn unlock_transfer(who: &<T as Config>::AccountId, id: Self::AssetId) -> DispatchResult {
        Self::unlock_asset_transfer(who, id.0, id.1)
    }

    fn is_locked(id: Self::AssetId) -> bool {
        Self::is_asset_locked(id.0, id.1)
    }
}
