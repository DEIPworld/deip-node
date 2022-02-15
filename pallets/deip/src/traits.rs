use crate::*;

use deip_assets_error::*;
use sp_runtime::traits::AtLeast32BitUnsigned;
use deip_asset_system::AssetIdInitT;

pub trait DeipAssetSystem<AccountId>: AssetIdInitT<Self::AssetId> {
    /// The units in which asset balances are recorded.
    type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

    /// The arithmetic type of asset identifier.
    type AssetId: Member + Parameter + Default + Copy + AsRef<[u8]>;

    fn try_get_tokenized_project(id: &Self::AssetId) -> Option<ProjectId>;

    fn account_balance(account: &AccountId, asset: &Self::AssetId) -> Self::Balance;

    fn total_supply(asset: &Self::AssetId) -> Self::Balance;

    fn get_project_fts(id: &ProjectId) -> Vec<Self::AssetId>;

    fn get_ft_balances(id: &Self::AssetId) -> Option<Vec<AccountId>>;

    fn transactionally_transfer(
        from: &AccountId,
        asset: Self::AssetId,
        transfers: &[(Self::Balance, AccountId)],
    ) -> Result<(), ()>;

    /// Tries to transfer assets specified by `shares` from
    /// `account` to a specific balance identified by `id`.
    /// Some collateral fee may be locked from `account`.
    fn transactionally_reserve(
        account: &AccountId,
        id: InvestmentId,
        shares: &[(Self::AssetId, Self::Balance)],
        asset: Self::AssetId,
    ) -> Result<(), ReserveError<Self::AssetId>>;

    /// Transfers all assets currently owned by `id` to the account, used in
    /// transactionally_reserve, in a transactional way.
    fn transactionally_unreserve(id: InvestmentId) -> Result<(), UnreserveError<Self::AssetId>>;

    /// Transfers `amount` of assets `id` owned by account specified with `id` to `who`.
    fn transfer_from_reserved(
        id: InvestmentId,
        who: &AccountId,
        asset: Self::AssetId,
        amount: Self::Balance,
    ) -> Result<(), UnreserveError<Self::AssetId>>;

    /// Transfers `amount` of assets from `who` to account specified by `id`.
    /// Assets should be specified in call to `transactionally_reserve`.
    fn transfer_to_reserved(
        who: &AccountId,
        id: InvestmentId,
        amount: Self::Balance,
    ) -> Result<(), UnreserveError<Self::AssetId>>;
}
