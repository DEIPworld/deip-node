use crate::Config;
use codec::{Decode, Encode};
use frame_support::traits::Currency;
use scale_info::TypeInfo;

pub(super) type DepositBalanceOf<T, I = ()> =
    <<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct FNftDetails<AccountId, DepositBalance, NftClassId, InstanceId, AssetId, TokenBalance> {
    /// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
    pub(super) owner: AccountId,
    /// @TODO
    pub(super) issuer: AccountId,
    /// @TODO
    pub(super) admin: AccountId,
    /// @TODO
    pub(super) freezer: AccountId,
    /// @TODO
    pub(super) total_deposit: DepositBalance,
    /// @TODO
    pub(super) is_frozen: bool,
    /// Fractionalized NFT class.
    pub(super) class: NftClassId,
    /// Fractionalized NFT class instance.
    pub(super) instance: InstanceId,
    /// Fractionalized NFT corresponding token id.
    pub(super) token: Option<AssetId>,
    /// Fractionalized NFT corresponding token minted amount.
    pub(super) amount: TokenBalance,
    /// Is NFT fractionalized.
    pub(super) is_fractionalized: bool,
}
