use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{self, Serialize, Deserialize};
use sp_runtime::traits::{AtLeast32BitUnsigned};
use frame_support::{RuntimeDebug};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use crate::asset::Asset;

/// Unique InvestmentOpportunity ID reference
pub type InvestmentId = sp_core::H160;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum SimpleCrowdfundingStatus {
    Active,
    Finished,
    Expired,
    Inactive,
}

impl Default for SimpleCrowdfundingStatus {
    fn default() -> Self {
        SimpleCrowdfundingStatus::Inactive
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum FundingModel<Moment, Asset> {
    SimpleCrowdfunding {
        /// a moment when the crowdfunding starts. Must be later than current moment.
        start_time: Moment,
        /// a moment when the crowdfunding ends. Must be later than `start_time`.
        end_time: Moment,
        /// amount of units to raise.
        soft_cap: Asset,
        /// amount upper limit of units to raise. Must be greater or equal to `soft_cap`.
        hard_cap: Asset,
    },
}

/// The object represents a sale of tokens with various parameters.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct SimpleCrowdfunding<Moment, AssetId, AssetBalance: Clone + AtLeast32BitUnsigned, CtxId> {
    #[cfg_attr(feature = "std", serde(skip))]
    pub created_ctx: CtxId,
    /// Reference for external world and uniques control
    pub external_id: InvestmentId,
    /// When the sale starts
    pub start_time: Moment,
    /// When it supposed to end
    pub end_time: Moment,
    pub status: SimpleCrowdfundingStatus,
    pub asset_id: AssetId,
    /// How many contributions already reserved
    pub total_amount: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub soft_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub hard_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    /// How many and what tokens supposed to sale
    pub shares: Vec<Asset<AssetId, AssetBalance>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Contribution<AccountId, Balance, Moment> {
    pub sale_id: InvestmentId,
    pub owner: AccountId,
    pub amount: Balance,
    pub time: Moment,
}
