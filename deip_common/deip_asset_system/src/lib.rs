#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;

pub mod asset;
pub mod fnft_impl;

pub use asset::*;
pub use fnft_impl::*;

pub use deip_assets_error::{ReserveError, UnreserveError};
use frame_support::dispatch::Parameter;
use sp_runtime::traits::{AtLeast32BitUnsigned, Member};
use sp_std::prelude::*;

pub trait AssetIdInitT<AssetId> {
    fn asset_id(raw: &[u8]) -> AssetId;
}
