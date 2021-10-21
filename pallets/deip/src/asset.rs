use crate::*;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Asset<AssetId, AssetBalance> {
    pub id: AssetId,
    pub amount: AssetBalance,
}

impl<AssetId, AssetBalance> Asset<AssetId, AssetBalance> {
    pub fn new(id: AssetId, amount: AssetBalance) -> Self {
        Self { id, amount }
    }
}
