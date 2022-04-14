use codec::{Decode, Encode};
use frame_support::traits::Currency;
use scale_info::TypeInfo;

use crate::Config;

pub(super) type DepositBalanceOf<T, I = ()> =
    <<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct ChestDetails<AccountId, DepositBalance> {
    /// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
    pub(super) owner: AccountId,
    /// Can mint tokens.
    pub(super) issuer: AccountId,
    /// Can thaw tokens, force transfers and burn tokens from any account.
    pub(super) admin: AccountId,
    /// Can freeze tokens.
    pub(super) freezer: AccountId,
    /// The total balance deposited for the all storage associated with this asset class.
    /// Used by `destroy`.
    pub(super) total_deposit: DepositBalance,
    /// Whether the asset is frozen for non-admin transfers.
    pub(super) is_frozen: bool,
}
