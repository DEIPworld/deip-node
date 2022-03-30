pub trait GetToken<AssetId, ClassId, InstanceId> {
    fn ft_asset_id(&self) -> Option<&AssetId>;

    fn nft_class_id(&self) -> Option<(&ClassId, &InstanceId)>;
}
