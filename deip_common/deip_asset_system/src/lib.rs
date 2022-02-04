#![cfg_attr(not(feature = "std"), no_std)]

pub trait AssetIdInitT<AssetId> {
    fn asset_id(raw: &[u8]) -> AssetId;
}
