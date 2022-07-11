// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod impl_nonfungibles;

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use frame_support::{
        dispatch::{ Weight},
        pallet_prelude::{NMapKey, StorageMap, StorageValue, ValueQuery},
        sp_runtime::traits::{CheckedAdd, One},
        storage::storage_prefix,
        traits::{Get, Hooks},
        Blake2_128Concat, Parameter,
    };
    use frame_system::pallet_prelude::{BlockNumberFor};
    use sp_io::{storage::clear_prefix, KillStorageResult};

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_uniques::Config<ClassId = Self::NftClassId>
        + pallet_assets::Config
    {
        /// Deip class id.
        type DeipNftClassId: Parameter + Copy;

        /// Deip account id.
        type DeipAccountId: Into<Self::AccountId> + Parameter + Clone;

        /// Deip project id.
        type ProjectId: Parameter;

        /// Type of `pallet_uniques::Config::ClassId`.
        type NftClassId: Parameter + CheckedAdd + Default + One + Copy + PartialOrd;

        /// Max class id available for asset creation via origin `pallet_uniques::Call`.
        type MaxOriginClassId: Get<Self::ClassId>;

        type Fungibles: deip_asset_system::FTImplT<
            Account = Self::AccountId,
            FTokenId = Self::AssetId,
            FTokenAmount = Self::Balance,
        >;
    }

    use frame_support::traits::StorageVersion;

    pub const V0: StorageVersion = StorageVersion::new(0);
    pub const V1: StorageVersion = StorageVersion::new(1);
    pub const V2: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(V2)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            fn clear_storage(pallet: &[u8], storage: &[u8]) -> u32 {
                let prefix = storage_prefix(pallet, storage);
                let kill_storage_result = clear_prefix(&prefix, None);
                match kill_storage_result {
                    KillStorageResult::AllRemoved(writes) => writes,
                    KillStorageResult::SomeRemaining(writes) => writes,
                }
            }

            let mut writes = 0u64;
            let reads = 0u64;

            if Pallet::<T>::on_chain_storage_version() == V1 &&
                Pallet::<T>::current_storage_version() == V2
            {
                let pallet_name = b"DeipUniques";

                let storages_to_clear = &[
                    "NftClassIdByDeipNftClassId",
                    "NftClassIdByDeipNftClassIdV1",
                    "DeipNftClassIdByNftClassId",
                    "DeipNftClassIdByNftClassIdV1",
                ];
                for storage in storages_to_clear {
                    writes += clear_storage(pallet_name, storage.as_bytes()) as u64;
                }
            }

            // if Pallet::<T>::on_chain_storage_version() == V0 &&
            //     Pallet::<T>::current_storage_version() == V1
            // {
            //     let mut reads: usize = 0;
            //     for x in &[
            //         "NextNftClassId",
            //         "ProjectIdByDeipNftClassId",
            //         "NftBalanceMap",
            //     ] {
            //         reads += count_items(b"Uniques", x.as_bytes());
            //         move_storage_from_pallet(
            //             x.as_bytes(),
            //             "Uniques".as_bytes(),
            //             "DeipUniques".as_bytes(),
            //         );
            //     }
            //     // reads += ProjectIdByDeipNftClassId::<T>::drain().count();
            //     // reads += NftBalanceMap::<T>::drain().count();

            //     for x in &[
            //         "Class",
            //         "Account",
            //         "Asset",
            //         "ClassMetadataOf",
            //         "InstanceMetadataOf",
            //         "Attribute",
            //     ] {
            //         reads += count_items(b"ParityTechUniques", x.as_bytes());
            //         move_storage_from_pallet(
            //             x.as_bytes(),
            //             "ParityTechUniques".as_bytes(),
            //             "Uniques".as_bytes(),
            //         );
            //     }

            //     let reads: Weight = reads.try_into().unwrap_or(Weight::MAX);
            //     return T::DbWeight::get().reads_writes(reads, reads)
            // }
            T::DbWeight::get().reads_writes(reads, writes)
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T> {
        pub _marker: std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            NextNftClassId::<T>::put(
                T::MaxOriginClassId::get()
                    .checked_add(&One::one())
                    .expect("max origin class id set to max type value"),
            );
        }
    }

    #[cfg(feature = "std")]
    impl<T> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { _marker: std::marker::PhantomData }
        }
    }

    /// Storage for next NFT origin class id.
    #[pallet::storage]
    pub(super) type NextNftClassId<T> = StorageValue<_, <T as Config>::NftClassId, ValueQuery>;

    /////

    #[pallet::storage]
    pub type NextCollectionId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;

    use frame_support::pallet_prelude::*;
    use sp_runtime::app_crypto::sp_core;

    #[pallet::storage]
    pub type CollectionRepo<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        sp_core::H160,
        deip_asset_system::NFTokenCollectionRecord<
            T::AccountId,
            sp_core::H160,
            T::ClassId,
            T::InstanceId,
        >,
    >;

    #[pallet::storage]
    pub type ItemRepo<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        deip_asset_system::NFTokenItemRecord<
            T::AccountId,
            T::Hash,
            T::InstanceId,
            T::ClassId,
            (T::AssetId, T::Balance),
        >,
    >;

    #[pallet::storage]
    pub type FractionRepo<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::Hash,
        Blake2_128Concat,
        T::AccountId,
        deip_asset_system::NFTokenFractionRecord<
            T::AccountId,
            T::Hash,
            (T::AssetId, T::Balance),
            T::Balance,
            u32,
        >,
    >;

    #[pallet::storage]
    pub type FractionalRepo<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, (T::AssetId, T::Balance)>;

    #[pallet::storage]
    pub type FractionHolds<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, T::Hash>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, sp_core::H160>,
            NMapKey<Blake2_128Concat, u32>,
        ),
        (sp_core::H160, u32),
    >;

    /// Storage with fraction FT id - item fingerprint mapping.
    #[pallet::storage]
    pub type FingerprintByFractionTokenId<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AssetId, T::Hash>;

    #[pallet::error]
    pub enum Error<T> {
        DeipNftClassIdExists,
        DeipNftClassIdDoesNotExist,
        NftClassIdOverflow,
        ProjectDoesNotExist,
        ProjectDoesNotBelongToTeam,
        ProjectSecurityTokenCannotBeDestroyed,
        ProjectSecurityTokenCannotBeBurned,
        ProjectSecurityTokenCannotBeFrozen,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::create())]
        // pub fn deip_create(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     admin: T::DeipAccountId,
        // ) -> DispatchResultWithPostInfo {
        //     let call = |class, admin| UniquesCall::<T>::create { class, admin };
        //     Self::create_or_force(origin, class, admin, call)
        // }

        // #[pallet::weight(
        //     pallet_uniques::Call::<T>::destroy{
        //         class: <_>::default(), witness: *witness
        //     }.get_dispatch_info().weight + T::DbWeight::get().reads(2)
        // )]
        // pub fn deip_destroy(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     witness: DestroyWitness,
        // ) -> DispatchResultWithPostInfo {
        // let origin_class_id = Self::deip_to_origin_class_id(class)?;

        // Dispatch destroy call to origin pallet.
        // let call = pallet_uniques::Call::<T>::destroy { class: origin_class_id, witness };
        // let res = call.dispatch_bypass_filter(origin);

        // if res.is_ok() {
        // }

        // res
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::mint())]
        // pub fn deip_mint(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     owner: T::DeipAccountId,
        // ) -> DispatchResultWithPostInfo {
        //     // Convert target to source.
        //     let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());

        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch destroy call to origin pallet.
        //     let call = pallet_uniques::Call::<T>::mint {
        //         class: origin_class_id,
        //         instance,
        //         owner: owner_source,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::burn())]
        // pub fn deip_burn(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     check_owner: Option<T::DeipAccountId>,
        // ) -> DispatchResultWithPostInfo {
        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Convert target to source.
        //     let check_owner_source =
        //         check_owner.map(|owner| <T::Lookup as StaticLookup>::unlookup(owner.into()));

        //     // Dispatch destroy call to origin pallet.
        //     let call = pallet_uniques::Call::<T>::burn {
        //         class: origin_class_id,
        //         instance,
        //         check_owner: check_owner_source,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::transfer())]
        // pub fn deip_transfer(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     dest: T::DeipAccountId,
        // ) -> DispatchResultWithPostInfo {
        //     // Convert target to source.
        //     let dest_source = <T::Lookup as StaticLookup>::unlookup(dest.into());

        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = pallet_uniques::Call::<T>::transfer {
        //         class: origin_class_id,
        //         instance,
        //         dest: dest_source,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::redeposit(instances.len() as u32))]
        // pub fn deip_redeposit(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instances: Vec<T::InstanceId>,
        // ) -> DispatchResult {
        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::redeposit(origin, origin_class_id, instances)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::freeze())]
        // pub fn deip_freeze(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        // ) -> DispatchResultWithPostInfo {
        //     // @TODO
        //     // ensure!(
        //     //     !InvestmentByAssetId::<T>::contains_key(id),
        //     //     Error::<T>::ReservedAssetAccountCannotBeFreezed
        //     // );

        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::freeze { class: origin_class_id, instance };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::thaw())]
        // pub fn deip_thaw(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        // ) -> DispatchResultWithPostInfo {
        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::thaw { class: origin_class_id, instance };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::freeze_class())]
        // pub fn deip_freeze_class(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        // ) -> DispatchResultWithPostInfo {
        // @TODO
        // ensure!(
        //     !InvestmentByAssetId::<T>::contains_key(id),
        //     Error::<T>::ReservedAssetCannotBeFreezed
        // );

        // let origin_class_id = Self::deip_to_origin_class_id(class)?;

        // // Dispatch call to origin pallet.
        // let call = UniquesCall::<T>::freeze_class { class: origin_class_id };
        // call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::thaw_class())]
        // pub fn deip_thaw_class(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        // ) -> DispatchResultWithPostInfo {
        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::thaw_class { class: origin_class_id };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::transfer_ownership())]
        // pub fn deip_transfer_ownership(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     owner: T::DeipAccountId,
        // ) -> DispatchResultWithPostInfo {
        //     // Convert target to source.
        //     let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());

        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::transfer_ownership {
        //         class: origin_class_id,
        //         owner: owner_source,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::set_team())]
        // pub fn deip_set_team(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     issuer: T::DeipAccountId,
        //     admin: T::DeipAccountId,
        //     freezer: T::DeipAccountId,
        // ) -> DispatchResultWithPostInfo {
        //     let issuer_source = <T::Lookup as StaticLookup>::unlookup(issuer.into());
        //     let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
        //     let freezer_source = <T::Lookup as StaticLookup>::unlookup(freezer.into());

        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::set_team {
        //         class: origin_class_id,
        //         issuer: issuer_source,
        //         admin: admin_source,
        //         freezer: freezer_source,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // Approve an instance to be transferred by a delegated third-party account.
        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::approve_transfer())]
        // pub fn deip_approve_transfer(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     delegate: T::DeipAccountId,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     let delegate = <T::Lookup as StaticLookup>::unlookup(delegate.into());
        //     // ??? @TODO update inner storages.
        //     UniquesPallet::<T>::approve_transfer(origin, class, instance, delegate)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::cancel_approval())]
        // pub fn deip_cancel_approval(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     maybe_check_delegate: Option<T::DeipAccountId>,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     let maybe_check_delegate =
        //         maybe_check_delegate.map(|d| <T::Lookup as StaticLookup>::unlookup(d.into()));
        //     // ??? @TODO update inner storages.
        //     UniquesPallet::<T>::cancel_approval(origin, class, instance, maybe_check_delegate)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::set_attribute())]
        // pub fn deip_set_attribute(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     maybe_instance: Option<T::InstanceId>,
        //     key: BoundedVec<u8, T::KeyLimit>,
        //     value: BoundedVec<u8, T::ValueLimit>,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::set_attribute(origin, class, maybe_instance, key, value)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::clear_attribute())]
        // pub fn deip_clear_attribute(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     maybe_instance: Option<T::InstanceId>,
        //     key: BoundedVec<u8, T::KeyLimit>,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::clear_attribute(origin, class, maybe_instance, key)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::set_metadata())]
        // pub fn deip_set_metadata(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        //     data: BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
        //     is_frozen: bool,
        // ) -> DispatchResultWithPostInfo {
        //     let origin_class_id = Self::deip_to_origin_class_id(class)?;

        //     // Dispatch call to origin pallet.
        //     let call = UniquesCall::<T>::set_metadata {
        //         class: origin_class_id,
        //         instance,
        //         data,
        //         is_frozen,
        //     };
        //     call.dispatch_bypass_filter(origin)
        // }

        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::clear_metadata())]
        // pub fn deip_clear_metadata(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     instance: T::InstanceId,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::clear_metadata(origin, class, instance)
        // }

        // /// Set the metadata for an asset class.
        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::set_class_metadata())]
        // pub fn deip_set_class_metadata(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        //     data: BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
        //     is_frozen: bool,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::set_class_metadata(origin, class, data, is_frozen)
        // }

        // /// Clear the metadata for an asset class.
        // #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::clear_class_metadata())]
        // pub fn deip_clear_class_metadata(
        //     origin: OriginFor<T>,
        //     class: DeipNftClassIdOf<T>,
        // ) -> DispatchResult {
        //     let class = Self::deip_to_origin_class_id(class)?;
        //     UniquesPallet::<T>::clear_class_metadata(origin, class)
        // }
    }

    // impl<T: Config> Pallet<T> {
        // Convert DeipNftClassId to origin class id.
        // fn deip_to_origin_class_id(class: DeipNftClassIdOf<T>) -> Result<T::NftClassId, Error<T>> {
        //         .ok_or(Error::<T>::DeipNftClassIdDoesNotExist)
        // }

    //     fn create_or_force(
    //         origin: OriginFor<T>,
    //         admin: T::DeipAccountId,
    //         call: impl FnOnce(T::NftClassId, <T::Lookup as StaticLookup>::Source) -> UniquesCall<T>,
    //     ) -> DispatchResultWithPostInfo {
    //         // Check if NFT with this deip id exist.
    //         // ensure!(
    //         //     Error::<T>::DeipNftClassIdExists
    //         // );

    //         // Get next origin class id.
    //         let new_class_id = NextNftClassId::<T>::get();

    //         // Dispatch call to origin uniques pallet.
    //         let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
    //         let call = call(new_class_id, admin_source);
    //         let post_dispatch_info = call.dispatch_bypass_filter(origin)?;

    //         // Save next class id.
    //         let next_class_id =
    //             new_class_id.checked_add(&One::one()).ok_or(Error::<T>::NftClassIdOverflow)?;
    //         NextNftClassId::<T>::put(next_class_id);

    //         Ok(post_dispatch_info)
    //     }
    // }
}
