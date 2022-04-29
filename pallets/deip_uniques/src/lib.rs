// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use deip_projects_info::DeipProjectsInfo;
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo, UnfilteredDispatchable, Weight},
        ensure,
        pallet_prelude::{OptionQuery, StorageMap, StorageValue, ValueQuery},
        sp_runtime::traits::{CheckedAdd, One, StaticLookup},
        traits::{Get, Hooks},
        Blake2_128Concat, BoundedVec, Identity, Parameter,
    };
    use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
    use pallet_uniques::{
        Call as UniquesCall, DestroyWitness, Pallet as UniquesPallet, WeightInfo,
    };
    use sp_std::vec::Vec;

    // Helper types.
    type DeipNftClassIdOf<T> = <T as Config>::DeipNftClassId;
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type DeipProjectIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::ProjectId;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_uniques::Config<ClassId = Self::NftClassId>
    {
        /// Deip class id.
        type DeipNftClassId: Parameter + Copy;

        /// Deip account id.
        type DeipAccountId: Into<Self::AccountId> + Parameter + Clone;

        /// Deip project id.
        type ProjectId: Parameter;

        /// Type of `pallet_uniques::Config::ClassId`.
        type NftClassId: Parameter + CheckedAdd + Default + One + Copy + PartialOrd;

        /// Additional project info.
        type ProjectsInfo: DeipProjectsInfo<Self::AccountId>;

        /// Max class id available for asset creation via origin `pallet_uniques::Call`.
        type MaxOriginClassId: Get<Self::ClassId>;
    }

    use frame_support::traits::{GetStorageVersion, StorageVersion};

    pub const V0: StorageVersion = StorageVersion::new(0);
    pub const V1: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(V1)]
    pub struct Pallet<T>(_);

    fn count_items(pallet_name: &[u8], storage_name: &[u8]) -> usize {
        use frame_support::storage::{storage_prefix, PrefixIterator};
        let prefix = storage_prefix(pallet_name, storage_name);
        PrefixIterator::<()>::new(prefix.to_vec(), prefix.to_vec(), |_key, _value| Ok(())).count()
    }

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            use core::convert::TryInto;
            use frame_support::storage::migration::move_storage_from_pallet;

            if Pallet::<T>::on_chain_storage_version() == V0 &&
                Pallet::<T>::current_storage_version() == V1
            {
                let mut reads: usize = 0;
                for x in &[
                    "NftClassIdByDeipNftClassId",
                    "DeipNftClassIdByNftClassId",
                    "NextNftClassId",
                    "ProjectIdByDeipNftClassId",
                    "NftBalanceMap",
                ] {
                    reads += count_items(b"Uniques", x.as_bytes());
                    move_storage_from_pallet(
                        x.as_bytes(),
                        "Uniques".as_bytes(),
                        "DeipUniques".as_bytes(),
                    );
                }
                NftClassIdByDeipNftClassId::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, v)| NftClassIdByDeipNftClassIdV1::<T>::insert(k, v));
                DeipNftClassIdByNftClassId::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, v)| DeipNftClassIdByNftClassIdV1::<T>::insert(k, v));
                reads += ProjectIdByDeipNftClassId::<T>::drain().count();
                reads += NftBalanceMap::<T>::drain().count();

                for x in &[
                    "Class",
                    "Account",
                    "Asset",
                    "ClassMetadataOf",
                    "InstanceMetadataOf",
                    "Attribute",
                ] {
                    reads += count_items(b"ParityTechUniques", x.as_bytes());
                    move_storage_from_pallet(
                        x.as_bytes(),
                        "ParityTechUniques".as_bytes(),
                        "Uniques".as_bytes(),
                    );
                }

                let reads: Weight = reads.try_into().unwrap_or(Weight::MAX);
                return T::DbWeight::get().reads_writes(reads, reads)
            }
            0
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

    /// Storage for matching Deip class id and origin `pallet_uniques` class id.
    #[pallet::storage]
    pub type NftClassIdByDeipNftClassId<T: Config> = StorageMap<
        _,
        Identity,
        DeipNftClassIdOf<T>,
        <T as pallet_uniques::Config>::ClassId,
        OptionQuery,
    >;
    // Migrate key hasher:
    #[pallet::storage]
    pub type NftClassIdByDeipNftClassIdV1<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        DeipNftClassIdOf<T>,
        <T as pallet_uniques::Config>::ClassId,
        OptionQuery,
    >;

    /// Storage for matching Deip class id and origin `pallet_uniques` class id.
    #[pallet::storage]
    pub type DeipNftClassIdByNftClassId<T: Config> = StorageMap<
        _,
        Identity,
        <T as pallet_uniques::Config>::ClassId,
        DeipNftClassIdOf<T>,
        OptionQuery,
    >;
    // Migrate key hasher:
    #[pallet::storage]
    pub type DeipNftClassIdByNftClassIdV1<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        <T as pallet_uniques::Config>::ClassId,
        DeipNftClassIdOf<T>,
        OptionQuery,
    >;

    /// Storage for next NFT origin class id.
    #[pallet::storage]
    pub(super) type NextNftClassId<T> = StorageValue<_, <T as Config>::NftClassId, ValueQuery>;

    /// Storage with projects ids.
    /// Deprecated
    #[pallet::storage]
    pub(super) type ProjectIdByDeipNftClassId<T> =
        StorageMap<_, Identity, DeipNftClassIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    /// Storage with assets classes ant accounts which hold corresponding asset.
    /// Deprecated
    #[pallet::storage]
    pub(super) type NftBalanceMap<T: Config> =
        StorageMap<_, Identity, DeipNftClassIdOf<T>, Vec<AccountIdOf<T>>, OptionQuery>;

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
        NftClassHasLockedInstances,
        NftIsLocked,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create())]
        pub fn deip_create(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            admin: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let call = |class, admin| UniquesCall::<T>::create { class, admin };
            Self::create_or_force(origin, class, admin, call)
        }

        #[pallet::weight(
            pallet_uniques::Call::<T>::destroy{
                class: <_>::default(), witness: *witness
            }.get_dispatch_info().weight + T::DbWeight::get().reads(2)
        )]
        pub fn deip_destroy(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            witness: DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch destroy call to origin pallet.
            let call = pallet_uniques::Call::<T>::destroy { class: origin_class_id, witness };
            let res = call.dispatch_bypass_filter(origin);

            if res.is_ok() {
                DeipNftClassIdByNftClassIdV1::<T>::mutate_exists(origin_class_id, |v| *v = None);
                NftClassIdByDeipNftClassIdV1::<T>::mutate_exists(class, |v| *v = None);
            }

            res
        }

        #[pallet::weight(T::WeightInfo::mint())]
        pub fn deip_mint(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            // Convert target to source.
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch destroy call to origin pallet.
            let call = pallet_uniques::Call::<T>::mint {
                class: origin_class_id,
                instance,
                owner: owner_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::burn())]
        pub fn deip_burn(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            check_owner: Option<T::DeipAccountId>,
        ) -> DispatchResultWithPostInfo {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Convert target to source.
            let check_owner_source =
                check_owner.map(|owner| <T::Lookup as StaticLookup>::unlookup(owner.into()));

            // Dispatch destroy call to origin pallet.
            let call = pallet_uniques::Call::<T>::burn {
                class: origin_class_id,
                instance,
                check_owner: check_owner_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn deip_transfer(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            dest: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            // Convert target to source.
            let dest_source = <T::Lookup as StaticLookup>::unlookup(dest.into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = pallet_uniques::Call::<T>::transfer {
                class: origin_class_id,
                instance,
                dest: dest_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::redeposit(instances.len() as u32))]
        pub fn deip_redeposit(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instances: Vec<T::InstanceId>,
        ) -> DispatchResult {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::redeposit(origin, origin_class_id, instances)
        }

        #[pallet::weight(T::WeightInfo::freeze())]
        pub fn deip_freeze(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
        ) -> DispatchResultWithPostInfo {
            // @TODO
            // ensure!(
            //     !InvestmentByAssetId::<T>::contains_key(id),
            //     Error::<T>::ReservedAssetAccountCannotBeFreezed
            // );

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::freeze { class: origin_class_id, instance };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::thaw())]
        pub fn deip_thaw(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
        ) -> DispatchResultWithPostInfo {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::thaw { class: origin_class_id, instance };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::freeze_class())]
        pub fn deip_freeze_class(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            // @TODO
            // ensure!(
            //     !InvestmentByAssetId::<T>::contains_key(id),
            //     Error::<T>::ReservedAssetCannotBeFreezed
            // );

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::freeze_class { class: origin_class_id };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::thaw_class())]
        pub fn deip_thaw_class(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::thaw_class { class: origin_class_id };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::transfer_ownership())]
        pub fn deip_transfer_ownership(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            // Convert target to source.
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::transfer_ownership {
                class: origin_class_id,
                owner: owner_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::set_team())]
        pub fn deip_set_team(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            issuer: T::DeipAccountId,
            admin: T::DeipAccountId,
            freezer: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let issuer_source = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let freezer_source = <T::Lookup as StaticLookup>::unlookup(freezer.into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::set_team {
                class: origin_class_id,
                issuer: issuer_source,
                admin: admin_source,
                freezer: freezer_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        /// Approve an instance to be transferred by a delegated third-party account.
        #[pallet::weight(T::WeightInfo::approve_transfer())]
        pub fn deip_approve_transfer(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            delegate: T::DeipAccountId,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            let delegate = <T::Lookup as StaticLookup>::unlookup(delegate.into());
            // ??? @TODO update inner storages.
            UniquesPallet::<T>::approve_transfer(origin, class, instance, delegate)
        }

        #[pallet::weight(T::WeightInfo::cancel_approval())]
        pub fn deip_cancel_approval(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            maybe_check_delegate: Option<T::DeipAccountId>,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            let maybe_check_delegate =
                maybe_check_delegate.map(|d| <T::Lookup as StaticLookup>::unlookup(d.into()));
            // ??? @TODO update inner storages.
            UniquesPallet::<T>::cancel_approval(origin, class, instance, maybe_check_delegate)
        }

        #[pallet::weight(T::WeightInfo::set_attribute())]
        pub fn deip_set_attribute(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            maybe_instance: Option<T::InstanceId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::set_attribute(origin, class, maybe_instance, key, value)
        }

        #[pallet::weight(T::WeightInfo::clear_attribute())]
        pub fn deip_clear_attribute(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            maybe_instance: Option<T::InstanceId>,
            key: BoundedVec<u8, T::KeyLimit>,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::clear_attribute(origin, class, maybe_instance, key)
        }

        #[pallet::weight(T::WeightInfo::set_metadata())]
        pub fn deip_set_metadata(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            data: BoundedVec<u8, T::StringLimit>,
            is_frozen: bool,
        ) -> DispatchResultWithPostInfo {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = UniquesCall::<T>::set_metadata {
                class: origin_class_id,
                instance,
                data,
                is_frozen,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::clear_metadata())]
        pub fn deip_clear_metadata(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::clear_metadata(origin, class, instance)
        }

        /// Set the metadata for an asset class.
        #[pallet::weight(T::WeightInfo::set_class_metadata())]
        pub fn deip_set_class_metadata(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            data: BoundedVec<u8, T::StringLimit>,
            is_frozen: bool,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::set_class_metadata(origin, class, data, is_frozen)
        }

        /// Clear the metadata for an asset class.
        #[pallet::weight(T::WeightInfo::clear_class_metadata())]
        pub fn deip_clear_class_metadata(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            UniquesPallet::<T>::clear_class_metadata(origin, class)
        }
    }

    impl<T: Config> Pallet<T> {
        /// Convert DeipNftClassId to origin class id.
        fn deip_to_origin_class_id(class: DeipNftClassIdOf<T>) -> Result<T::NftClassId, Error<T>> {
            NftClassIdByDeipNftClassIdV1::<T>::get(class)
                .ok_or(Error::<T>::DeipNftClassIdDoesNotExist)
        }

        fn create_or_force(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            admin: T::DeipAccountId,
            call: impl FnOnce(T::NftClassId, <T::Lookup as StaticLookup>::Source) -> UniquesCall<T>,
        ) -> DispatchResultWithPostInfo {
            // Check if NFT with this deip id exist.
            ensure!(
                !NftClassIdByDeipNftClassIdV1::<T>::contains_key(class),
                Error::<T>::DeipNftClassIdExists
            );

            // Get next origin class id.
            let new_class_id = NextNftClassId::<T>::get();

            // Dispatch call to origin uniques pallet.
            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let call = call(new_class_id, admin_source);
            let post_dispatch_info = call.dispatch_bypass_filter(origin)?;

            // Save next class id.
            let next_class_id =
                new_class_id.checked_add(&One::one()).ok_or(Error::<T>::NftClassIdOverflow)?;
            NextNftClassId::<T>::put(next_class_id);

            // Insert id to map.
            NftClassIdByDeipNftClassIdV1::<T>::insert(class, new_class_id);
            DeipNftClassIdByNftClassIdV1::<T>::insert(new_class_id, class);

            Ok(post_dispatch_info)
        }
    }
}
