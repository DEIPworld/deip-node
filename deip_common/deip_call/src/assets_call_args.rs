#![allow(dead_code)]

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use serde::Serialize;
use sp_runtime::{AccountId32, MultiAddress as SpMultiAddress};

#[derive(Serialize)]
pub(crate) struct AssetsCreateCallArgs {
    id: u32,
    admin: MultiAddress<AccountId32, ()>,
    min_balance: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsCreateCallArgs {
    pub(crate) fn new(id: u32, admin: &SpMultiAddress<AccountId32, ()>, min_balance: u128) -> Self {
        let admin = admin.into();
        Self { id, admin, min_balance: SerializableAtLeast32BitUnsigned(min_balance) }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsDestroyCallArgs {
    id: u32,
}

impl AssetsDestroyCallArgs {
    pub(crate) fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsMintCallArgs {
    id: u32,
    beneficiary: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsMintCallArgs {
    pub(crate) fn new(
        id: u32,
        beneficiary: &SpMultiAddress<AccountId32, ()>,
        amount: u128,
    ) -> Self {
        let amount = SerializableAtLeast32BitUnsigned(amount);
        let beneficiary = beneficiary.into();
        Self { id, beneficiary, amount }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsBurnCallArgs {
    id: u32,
    who: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsBurnCallArgs {
    pub(crate) fn new(id: u32, who: &SpMultiAddress<AccountId32, ()>, amount: u128) -> Self {
        let amount = SerializableAtLeast32BitUnsigned(amount);
        let who = who.into();
        Self { id, who, amount }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsTransferCallArgs {
    id: u32,
    target: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsTransferCallArgs {
    pub(crate) fn new(id: u32, target: &SpMultiAddress<AccountId32, ()>, amount: u128) -> Self {
        let amount = SerializableAtLeast32BitUnsigned(amount);
        let target = target.into();
        Self { id, target, amount }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsTransferKeepAliveCallArgs {
    id: u32,
    target: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsTransferKeepAliveCallArgs {
    pub(crate) fn new(id: u32, target: &SpMultiAddress<AccountId32, ()>, amount: u128) -> Self {
        let amount = SerializableAtLeast32BitUnsigned(amount);
        let target = target.into();
        Self { id, target, amount }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsFreezeCallArgs {
    id: u32,
    who: MultiAddress<AccountId32, ()>,
}

impl AssetsFreezeCallArgs {
    pub(crate) fn new(id: u32, who: &SpMultiAddress<AccountId32, ()>) -> Self {
        let who = who.into();
        Self { id, who }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsThawCallArgs {
    id: u32,
    who: MultiAddress<AccountId32, ()>,
}

impl AssetsThawCallArgs {
    pub(crate) fn new(id: u32, who: &SpMultiAddress<AccountId32, ()>) -> Self {
        let who = who.into();
        Self { id, who }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsFreezeAssetCallArgs {
    id: u32,
}

impl AssetsFreezeAssetCallArgs {
    pub(crate) fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsTransferOwnershipCallArgs {
    id: u32,
    who: MultiAddress<AccountId32, ()>,
}

impl AssetsTransferOwnershipCallArgs {
    pub(crate) fn new(id: u32, who: &SpMultiAddress<AccountId32, ()>) -> Self {
        let who = who.into();
        Self { id, who }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsSetTeamCallArgs {
    id: u32,
    issuer: MultiAddress<AccountId32, ()>,
    admin: MultiAddress<AccountId32, ()>,
    freezer: MultiAddress<AccountId32, ()>,
}

impl AssetsSetTeamCallArgs {
    pub(crate) fn new(
        id: u32,
        issuer: &SpMultiAddress<AccountId32, ()>,
        admin: &SpMultiAddress<AccountId32, ()>,
        freezer: &SpMultiAddress<AccountId32, ()>,
    ) -> Self {
        let issuer = issuer.into();
        let admin = admin.into();
        let freezer = freezer.into();
        Self { id, issuer, admin, freezer }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsSetMetadataCallArgs<N, S>
where
    N: Serialize,
    S: Serialize,
{
    id: u32,
    name: N,
    symbol: S,
    decimals: u8,
}

impl<N, S> AssetsSetMetadataCallArgs<N, S>
where
    N: Serialize,
    S: Serialize,
{
    pub(crate) fn new(id: u32, name: N, symbol: S, decimals: u8) -> Self {
        Self { id, name, symbol, decimals }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsClearMetadataCallArgs {
    id: u32,
}

impl AssetsClearMetadataCallArgs {
    pub(crate) fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsApproveTransferCallArgs {
    id: u32,
    delegate: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsApproveTransferCallArgs {
    pub(crate) fn new(id: u32, delegate: &SpMultiAddress<AccountId32, ()>, amount: u128) -> Self {
        let delegate = delegate.into();
        let amount = SerializableAtLeast32BitUnsigned(amount);
        Self { id, delegate, amount }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsCancelApprovalCallArgs {
    id: u32,
    delegate: MultiAddress<AccountId32, ()>,
}

impl AssetsCancelApprovalCallArgs {
    pub(crate) fn new(id: u32, delegate: &SpMultiAddress<AccountId32, ()>) -> Self {
        let delegate = delegate.into();
        Self { id, delegate }
    }
}

#[derive(Serialize)]
pub(crate) struct AssetsTransferApprovedCallArgs {
    id: u32,
    owner: MultiAddress<AccountId32, ()>,
    destination: MultiAddress<AccountId32, ()>,
    amount: SerializableAtLeast32BitUnsigned<u128>,
}

impl AssetsTransferApprovedCallArgs {
    pub(crate) fn new(
        id: u32,
        owner: &SpMultiAddress<AccountId32, ()>,
        destination: &SpMultiAddress<AccountId32, ()>,
        amount: u128,
    ) -> Self {
        let owner = owner.into();
        let destination = destination.into();
        let amount = SerializableAtLeast32BitUnsigned(amount);
        Self { id, owner, destination, amount }
    }
}

#[derive(Serialize)]
enum MultiAddress<AccountId, AccountIndex> {
    /// It's an account ID (pubkey).
    Id(AccountId),
    /// It's an account index.
    Index(AccountIndex),
    /// It's some arbitrary raw bytes.
    Raw(Vec<u8>),
    /// It's a 32 byte representation.
    Address32([u8; 32]),
    /// Its a 20 byte representation.
    Address20([u8; 20]),
}

impl<AccountId, AccountIndex> From<&SpMultiAddress<AccountId, AccountIndex>>
    for MultiAddress<AccountId, AccountIndex>
where
    AccountId: Clone,
    AccountIndex: Clone,
{
    fn from(src: &SpMultiAddress<AccountId, AccountIndex>) -> Self {
        match src {
            SpMultiAddress::Id(v) => MultiAddress::Id(v.clone()),
            SpMultiAddress::Index(v) => MultiAddress::Index(v.clone()),
            SpMultiAddress::Raw(v) => MultiAddress::Raw(v.to_owned()),
            SpMultiAddress::Address32(v) => MultiAddress::Address32(v.to_owned()),
            SpMultiAddress::Address20(v) => MultiAddress::Address20(v.to_owned()),
        }
    }
}
