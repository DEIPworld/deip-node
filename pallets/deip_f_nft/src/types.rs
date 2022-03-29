use codec::{Decode, Encode};
use scale_info::TypeInfo;

use crate::traits::GetToken;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct PayloadDetails<AccountId> {
    pub owner: AccountId,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum PayloadAssetId<AssetId, ClassId> {
    Ft(AssetId),
    Nft(ClassId),
}

impl<AssetId, ClassId> GetToken<AssetId, ClassId> for PayloadAssetId<AssetId, ClassId> {
    fn nft_class_id(&self) -> Option<&ClassId> {
        if let Self::Nft(id) = self {
            Some(id)
        } else {
            None
        }
    }

    fn ft_asset_id(&self) -> Option<&AssetId> {
        if let Self::Ft(id) = self {
            Some(id)
        } else {
            None
        }
    }
}
