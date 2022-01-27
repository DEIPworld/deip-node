// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use deip_projects_info::DeipProjectsInfo;
    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo, UnfilteredDispatchable, Vec},
        ensure,
        pallet_prelude::{OptionQuery, StorageMap, StorageValue, ValueQuery},
        sp_runtime::traits::{CheckedAdd, One, StaticLookup},
        weights::Pays,
        BoundedVec, Identity, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use pallet_uniques::{
        Call as UniquesCall, DestroyWitness, Pallet as UniquesPallet, WeightInfo,
    };
    use sp_std::vec;

    // Helper types.
    type DeipNftClassIdOf<T> = <T as Config>::NftClassId;
    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type DeipProjectIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::ProjectId;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_uniques::Config<ClassId = Self::UniquesNftClassId>
    {
        /// Deip class id.
        type NftClassId: Parameter + Copy;

        /// Deip account id.
        type DeipAccountId: Into<Self::AccountId> + Parameter + Clone;

        /// Deip project id.
        type ProjectId: Parameter;

        /// Type of `pallet_uniques::Config::ClassId`.
        type UniquesNftClassId: Parameter + CheckedAdd + Default + One + Copy;

        /// Additional project info.
        type ProjectsInfo: DeipProjectsInfo<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::genesis_config]
    pub struct GenesisConfig<T> {
        pub _marker: std::marker::PhantomData<T>,
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            NextNftClassId::<T>::put(<T as pallet_uniques::Config>::ClassId::default());
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

    /// Storage for next NFT origin class id.
    #[pallet::storage]
    pub(super) type NextNftClassId<T> =
        StorageValue<_, <T as Config>::UniquesNftClassId, ValueQuery>;

    /// Storage with projects ids.
    #[pallet::storage]
    pub(super) type ProjectIdByNftClassId<T> =
        StorageMap<_, Identity, DeipNftClassIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    /// Storage with assets classes ant accounts which hold corresponding asset.
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Issue a new class of non-fungible assets from a public origin.
        #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::create())]
        pub fn create(
            origin: OriginFor<T>,
            class: <T as pallet_uniques::Config>::ClassId,
            admin: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::create(origin, class, admin)
        }

        #[pallet::weight(<T as pallet_uniques::Config>::WeightInfo::force_create())]
        pub fn force_create(
            origin: OriginFor<T>,
            class: T::ClassId,
            owner: <T::Lookup as StaticLookup>::Source,
            free_holding: bool,
        ) -> DispatchResult {
            UniquesPallet::<T>::force_create(origin, class, owner, free_holding)
        }

        /// Destroy a class of fungible assets.
        #[pallet::weight(pallet_uniques::Call::<T>::destroy{class: *class, witness: *witness}.get_dispatch_info().weight)]
        pub fn destroy(
            origin: OriginFor<T>,
            class: T::ClassId,
            witness: DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            UniquesPallet::<T>::destroy(origin, class, witness)
        }

        /// Mint an asset instance of a particular class.
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            owner: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::mint(origin, class, instance, owner)
        }

        /// Destroy a single asset instance.
        #[pallet::weight(T::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            check_owner: Option<<T::Lookup as StaticLookup>::Source>,
        ) -> DispatchResult {
            UniquesPallet::<T>::burn(origin, class, instance, check_owner)
        }

        /// Move an asset from the sender account to another.
        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::transfer(origin, class, instance, dest)
        }

        /// Reevaluate the deposits on some assets.
        #[pallet::weight(T::WeightInfo::redeposit(instances.len() as u32))]
        pub fn redeposit(
            origin: OriginFor<T>,
            class: T::ClassId,
            instances: Vec<T::InstanceId>,
        ) -> DispatchResult {
            UniquesPallet::<T>::redeposit(origin, class, instances)
        }

        /// Disallow further unprivileged transfer of an asset instance.
        #[pallet::weight(T::WeightInfo::freeze())]
        pub fn freeze(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
        ) -> DispatchResult {
            UniquesPallet::<T>::freeze(origin, class, instance)
        }

        /// Re-allow unprivileged transfer of an asset instance.
        #[pallet::weight(T::WeightInfo::thaw())]
        pub fn thaw(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
        ) -> DispatchResult {
            UniquesPallet::<T>::thaw(origin, class, instance)
        }

        /// Disallow further unprivileged transfers for a whole asset class.
        #[pallet::weight(T::WeightInfo::freeze_class())]
        pub fn freeze_class(origin: OriginFor<T>, class: T::ClassId) -> DispatchResult {
            UniquesPallet::<T>::freeze_class(origin, class)
        }

        /// Re-allow unprivileged transfers for a whole asset class.
        #[pallet::weight(T::WeightInfo::thaw_class())]
        pub fn thaw_class(origin: OriginFor<T>, class: T::ClassId) -> DispatchResult {
            UniquesPallet::<T>::thaw_class(origin, class)
        }

        /// Change the Owner of an asset class.
        #[pallet::weight(T::WeightInfo::transfer_ownership())]
        pub fn transfer_ownership(
            origin: OriginFor<T>,
            class: T::ClassId,
            owner: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::transfer_ownership(origin, class, owner)
        }

        /// Change the Issuer, Admin and Freezer of an asset class.
        #[pallet::weight(T::WeightInfo::set_team())]
        pub fn set_team(
            origin: OriginFor<T>,
            class: T::ClassId,
            issuer: <T::Lookup as StaticLookup>::Source,
            admin: <T::Lookup as StaticLookup>::Source,
            freezer: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::set_team(origin, class, issuer, admin, freezer)
        }

        /// Approve an instance to be transferred by a delegated third-party account.
        #[pallet::weight(T::WeightInfo::approve_transfer())]
        pub fn approve_transfer(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            delegate: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            UniquesPallet::<T>::approve_transfer(origin, class, instance, delegate)
        }

        /// Cancel the prior approval for the transfer of an asset by a delegate.
        #[pallet::weight(T::WeightInfo::cancel_approval())]
        pub fn cancel_approval(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            maybe_check_delegate: Option<<T::Lookup as StaticLookup>::Source>,
        ) -> DispatchResult {
            UniquesPallet::<T>::cancel_approval(origin, class, instance, maybe_check_delegate)
        }

        /// Alter the attributes of a given asset.
        #[pallet::weight(T::WeightInfo::force_asset_status())]
        #[allow(clippy::too_many_arguments)]
        pub fn force_asset_status(
            origin: OriginFor<T>,
            class: T::ClassId,
            owner: <T::Lookup as StaticLookup>::Source,
            issuer: <T::Lookup as StaticLookup>::Source,
            admin: <T::Lookup as StaticLookup>::Source,
            freezer: <T::Lookup as StaticLookup>::Source,
            free_holding: bool,
            is_frozen: bool,
        ) -> DispatchResult {
            UniquesPallet::<T>::force_asset_status(
                origin,
                class,
                owner,
                issuer,
                admin,
                freezer,
                free_holding,
                is_frozen,
            )
        }

        /// Set an attribute for an asset class or instance.
        #[pallet::weight(T::WeightInfo::set_attribute())]
        pub fn set_attribute(
            origin: OriginFor<T>,
            class: T::ClassId,
            maybe_instance: Option<T::InstanceId>,
            key: BoundedVec<u8, T::KeyLimit>,
            value: BoundedVec<u8, T::ValueLimit>,
        ) -> DispatchResult {
            UniquesPallet::<T>::set_attribute(origin, class, maybe_instance, key, value)
        }

        /// Set an attribute for an asset class or instance.
        #[pallet::weight(T::WeightInfo::clear_attribute())]
        pub fn clear_attribute(
            origin: OriginFor<T>,
            class: T::ClassId,
            maybe_instance: Option<T::InstanceId>,
            key: BoundedVec<u8, T::KeyLimit>,
        ) -> DispatchResult {
            UniquesPallet::<T>::clear_attribute(origin, class, maybe_instance, key)
        }

        /// Set the metadata for an asset instance.
        #[pallet::weight(T::WeightInfo::set_metadata())]
        pub fn set_metadata(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
            data: BoundedVec<u8, T::StringLimit>,
            is_frozen: bool,
        ) -> DispatchResult {
            UniquesPallet::<T>::set_metadata(origin, class, instance, data, is_frozen)
        }

        /// Clear the metadata for an asset instance.
        #[pallet::weight(T::WeightInfo::clear_metadata())]
        pub fn clear_metadata(
            origin: OriginFor<T>,
            class: T::ClassId,
            instance: T::InstanceId,
        ) -> DispatchResult {
            UniquesPallet::<T>::clear_metadata(origin, class, instance)
        }

        /// Set the metadata for an asset class.
        #[pallet::weight(T::WeightInfo::set_class_metadata())]
        pub fn set_class_metadata(
            origin: OriginFor<T>,
            class: T::ClassId,
            data: BoundedVec<u8, T::StringLimit>,
            is_frozen: bool,
        ) -> DispatchResult {
            UniquesPallet::<T>::set_class_metadata(origin, class, data, is_frozen)
        }

        /// Clear the metadata for an asset class.
        #[pallet::weight(T::WeightInfo::clear_class_metadata())]
        pub fn clear_class_metadata(origin: OriginFor<T>, class: T::ClassId) -> DispatchResult {
            UniquesPallet::<T>::clear_class_metadata(origin, class)
        }

        #[pallet::weight(T::WeightInfo::create())]
        pub fn deip_create(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            admin: T::DeipAccountId,
            project_id: Option<DeipProjectIdOf<T>>,
        ) -> DispatchResultWithPostInfo {
            let call = |class, admin| UniquesCall::<T>::create { class, admin };
            Self::create_or_force(origin, class, admin, project_id, call)
        }

        #[pallet::weight(T::WeightInfo::force_create())]
        pub fn deip_force_create(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            admin: T::DeipAccountId,
            project_id: Option<DeipProjectIdOf<T>>,
            free_holding: bool,
        ) -> DispatchResultWithPostInfo {
            let call =
                |class, admin| UniquesCall::<T>::force_create { class, owner: admin, free_holding };
            Self::create_or_force(origin, class, admin, project_id, call)
        }

        #[pallet::weight((10_000, Pays::No))] // ??? @TODO benchmark
        pub fn deip_destroy(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            witness: DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            // If id belongs to project, refuse to destroy.
            ensure!(
                !ProjectIdByNftClassId::<T>::contains_key(class),
                Error::<T>::ProjectSecurityTokenCannotBeDestroyed
            );

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch destroy call to origin pallet.
            let call = pallet_uniques::Call::<T>::destroy { class: origin_class_id, witness };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(T::WeightInfo::mint())]
        pub fn deip_mint(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            // Convert target to source.
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.clone().into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch destroy call to origin pallet.
            let call = pallet_uniques::Call::<T>::mint {
                class: origin_class_id,
                instance,
                owner: owner_source,
            };
            let result = call.dispatch_bypass_filter(origin)?;

            // If project id exists for class id.
            if ProjectIdByNftClassId::<T>::contains_key(class) {
                // Check balance map for the class id.
                NftBalanceMap::<T>::mutate_exists(class, |maybe| {
                    let account = owner.into();
                    if let Some(balances) = maybe.as_mut() {
                        // If vec for this class doesn't contain asset, add account to map.
                        if let Err(i) = balances.binary_search(&account) {
                            balances.insert(i, account);
                        }
                    } else {
                        *maybe = Some(vec![account]);
                    }
                });
            }

            Ok(result)
        }

        #[pallet::weight(T::WeightInfo::burn())]
        pub fn deip_burn(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
            check_owner: Option<T::DeipAccountId>,
        ) -> DispatchResultWithPostInfo {
            // If id belongs to project, refuse to burn.
            ensure!(
                !ProjectIdByNftClassId::<T>::contains_key(class),
                Error::<T>::ProjectSecurityTokenCannotBeBurned
            );

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
            let dest_source = <T::Lookup as StaticLookup>::unlookup(dest.clone().into());

            let origin_class_id = Self::deip_to_origin_class_id(class)?;

            // Dispatch call to origin pallet.
            let call = pallet_uniques::Call::<T>::transfer {
                class: origin_class_id,
                instance,
                dest: dest_source,
            };
            let ok = call.dispatch_bypass_filter(origin)?;

            // If project id exists for class id.
            if ProjectIdByNftClassId::<T>::contains_key(class) {
                NftBalanceMap::<T>::mutate_exists(class, |maybe| {
                    let account = dest.into();
                    if let Some(balances) = maybe.as_mut() {
                        if let Err(i) = balances.binary_search(&account) {
                            balances.insert(i, account);
                        }
                        // ??? @TODO remove class id from source account
                    } else {
                        // This shouldn't happen but for any case.
                        // If this happend, it means that asset was minted and NftBalanceMap wasn't updated.
                        *maybe = Some(vec![account]);
                    }
                });
            }

            Ok(ok)
        }

        #[pallet::weight(T::WeightInfo::redeposit(instances.len() as u32))]
        pub fn deip_redeposit(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instances: Vec<T::InstanceId>,
        ) -> DispatchResult {
            let origin_class_id = Self::deip_to_origin_class_id(class)?;
            // ??? Is check for class belonging to the project need.
            // ??? Because from docs: class: The class of the asset to be !frozen!.
            UniquesPallet::<T>::redeposit(origin, origin_class_id, instances)
        }

        #[pallet::weight(T::WeightInfo::freeze())]
        pub fn deip_freeze(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            instance: T::InstanceId,
        ) -> DispatchResultWithPostInfo {
            // If id belongs to project, refuse to freeze.
            ensure!(
                !ProjectIdByNftClassId::<T>::contains_key(class),
                Error::<T>::ProjectSecurityTokenCannotBeFrozen
            );

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
            // If id belongs to project, refuse to freeze.
            ensure!(
                !ProjectIdByNftClassId::<T>::contains_key(class),
                Error::<T>::ProjectSecurityTokenCannotBeFrozen
            );

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

        #[pallet::weight(T::WeightInfo::force_asset_status())]
        #[allow(clippy::too_many_arguments)]
        pub fn deip_force_asset_status(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            owner: T::DeipAccountId,
            issuer: T::DeipAccountId,
            admin: T::DeipAccountId,
            freezer: T::DeipAccountId,
            free_holding: bool,
            is_frozen: bool,
        ) -> DispatchResult {
            let class = Self::deip_to_origin_class_id(class)?;
            let owner = <T::Lookup as StaticLookup>::unlookup(owner.into());
            let issuer = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            let admin = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let freezer = <T::Lookup as StaticLookup>::unlookup(freezer.into());
            UniquesPallet::<T>::force_asset_status(
                origin,
                class,
                owner,
                issuer,
                admin,
                freezer,
                free_holding,
                is_frozen,
            )
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
            let result = call.dispatch_bypass_filter(origin)?;

            // AssetMetadataMap::<T>::insert(
            //     id,
            //     AssetMetadata { name: asset_name, symbol: asset_symbol, decimals },
            // );

            Ok(result)
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
        fn deip_to_origin_class_id(
            class: DeipNftClassIdOf<T>,
        ) -> Result<T::UniquesNftClassId, Error<T>> {
            NftClassIdByDeipNftClassId::<T>::get(class)
                .ok_or(Error::<T>::DeipNftClassIdDoesNotExist)
        }

        fn create_or_force(
            origin: OriginFor<T>,
            class: DeipNftClassIdOf<T>,
            admin: T::DeipAccountId,
            project_id: Option<DeipProjectIdOf<T>>,
            call: impl FnOnce(
                T::UniquesNftClassId,
                <T::Lookup as StaticLookup>::Source,
            ) -> UniquesCall<T>,
        ) -> DispatchResultWithPostInfo {
            // If project id is provided ensure that admin is in team.
            if let Some(project_id) = project_id.as_ref() {
                if let Some(team_id) = T::ProjectsInfo::try_get_project_team(project_id) {
                    let account = ensure_signed(origin.clone())?;
                    ensure!(team_id == account, Error::<T>::ProjectDoesNotBelongToTeam)
                } else {
                    return Err(Error::<T>::ProjectDoesNotExist.into())
                }
            }

            // Check if NFT with this deip id exist.
            ensure!(
                !NftClassIdByDeipNftClassId::<T>::contains_key(class),
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
            NftClassIdByDeipNftClassId::<T>::insert(class, new_class_id);

            // IF project id is provided add id to projects map.
            if let Some(project_id) = project_id {
                ProjectIdByNftClassId::<T>::insert(class, project_id);
                // AssetIdByProjectId::<T>::mutate_exists(project_id, |tokens| {
                //     match tokens.as_mut() {
                //         None => *tokens = Some(vec![id]),
                //         Some(c) => c.push(id),
                //     };
                // });
            }

            Ok(post_dispatch_info)
        }
    }
}
