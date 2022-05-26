use crate::Config;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::Currency;
use scale_info::TypeInfo;

pub(super) type DepositBalanceOf<T, I = ()> =
    <<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct FNftDetails<AccountId, DepositBalance, NftClassId, InstanceId, AssetId, TokenBalance> {
    /// Can perform operations with this asset.
    pub(super) owner: AccountId,
    /// The balance deposited for this asset. This pays for the data stored here.
    pub(super) deposit: DepositBalance,
    /// Class of the fractionalized NFT.
    pub(super) class: NftClassId,
    /// Instance of the fractionalized NFT.
    pub(super) instance: InstanceId,
    /// FT `AssetId` of F-NFT shares.
    pub(super) token: Option<AssetId>,
    /// Amount of shares minted for this F-NFT.
    pub(super) amount: TokenBalance,
    /// Is NFT fractionalized.
    pub(super) is_fractionalized: bool,
}
