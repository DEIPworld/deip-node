#![cfg_attr(not(feature = "std"), no_std)]

pub enum ReserveError<AssetId> {
    NotEnoughBalance,
    AlreadyReserved,
    AssetTransferFailed(AssetId),
}

pub enum UnreserveError<AssetId> {
    NoSuchInvestment,
    AssetTransferFailed(AssetId),
}
