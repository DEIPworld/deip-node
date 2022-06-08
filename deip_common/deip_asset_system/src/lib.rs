#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;

pub mod asset;
pub mod nft_impl;
pub mod nft;
pub mod ft_impl;
pub mod ft;

pub use asset::*;
pub use nft_impl::*;
pub use nft::*;
pub use ft_impl::*;
pub use ft::*;

pub use deip_assets_error::{ReserveError, UnreserveError};
use sp_std::prelude::*;

pub trait AssetIdInitT<AssetId> {
    fn asset_id(raw: &[u8]) -> AssetId;
}
