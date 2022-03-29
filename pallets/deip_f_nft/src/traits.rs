use deip_asset_lock::Result as LockResult;

pub trait GetToken<AssetId, ClassId> {
    fn ft_asset_id(&self) -> Option<&AssetId>;

    fn nft_class_id(&self) -> Option<&ClassId>;
}

pub trait Lock {
    fn lock<T>(&self) -> LockResult
    where
        T: pallet_deip_assets::Config + pallet_deip_uniques::Config;
}
