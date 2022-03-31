use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

use crate::traits::GetToken;

#[derive(Clone, Encode, Decode, TypeInfo)]
pub struct PayloadDetails<AccountId, PayloadAssetId> {
    pub owner: AccountId,
    pub assets: Vec<PayloadAssetId>,
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum PayloadAssetId<AssetId, ClassId, InstanceId> {
    Ft(AssetId),
    Nft { class: ClassId, instance: InstanceId },
}

impl<AssetId, ClassId, InstanceId> GetToken<AssetId, ClassId, InstanceId>
    for PayloadAssetId<AssetId, ClassId, InstanceId>
{
    fn ft_asset_id(&self) -> Option<&AssetId> {
        if let Self::Ft(id) = self {
            Some(id)
        } else {
            None
        }
    }

    fn nft_class_id(&self) -> Option<(&ClassId, &InstanceId)> {
        if let Self::Nft { class, instance } = self {
            Some((class, instance))
        } else {
            None
        }
    }
}
