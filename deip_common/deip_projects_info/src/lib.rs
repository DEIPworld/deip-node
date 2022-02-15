// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::TypeInfo;

pub trait DeipProjectsInfo<AccountId>: TypeInfo {
    type ProjectId: sp_std::fmt::Debug
        + Clone
        + frame_support::codec::FullCodec
        + PartialEq
        + AsRef<[u8]>
        + TypeInfo;
    type InvestmentId: sp_std::fmt::Debug
        + Clone
        + frame_support::codec::FullCodec
        + PartialEq
        + AsRef<[u8]>
        + TypeInfo;

    fn try_get_project_team(id: &Self::ProjectId) -> Option<AccountId>;
    
    fn project_id(source: &[u8]) -> Self::ProjectId;
}
