#![allow(type_alias_bounds)]

pub use deip_asset_system::investment_opportunity::*;
pub use deip_asset_system::asset::*;

use deip_asset_system::DeipAssetSystem;

use deip_transaction_ctx::TransactionCtxId;

pub type DeipAssetId<T: crate::Config> =
<T as DeipAssetSystem<T::AccountId, crate::ProjectId, InvestmentId>>::AssetId;
pub type DeipAssetBalance<T: crate::Config> =
<T as DeipAssetSystem<T::AccountId, crate::ProjectId, InvestmentId>>::Balance;

pub type DeipAsset<T: crate::Config> = Asset<DeipAssetId<T>, DeipAssetBalance<T>>;
pub type FundingModelOf<T: crate::Config> = FundingModel<T::Moment, DeipAsset<T>>;

pub type SimpleCrowdfundingOf<T: crate::Config> = SimpleCrowdfunding<
    T::Moment,
    DeipAssetId<T>,
    DeipAssetBalance<T>,
    TransactionCtxId<T::TransactionCtx>,
>;

pub type Investment<T: crate::Config> = Contribution<
    T::AccountId,
    DeipAssetBalance<T>,
    T::Moment
>;
