#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;

pub mod asset;
pub mod nft_impl;
pub mod fnft;
pub mod ft_impl;
pub mod error;

pub use asset::*;
pub use nft_impl::*;
pub use fnft::*;
pub use ft_impl::*;

pub use deip_assets_error::{ReserveError, UnreserveError};
use sp_std::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Seal(());

pub trait AssetIdInitT<AssetId> {
    fn asset_id(raw: &[u8]) -> AssetId;
}
