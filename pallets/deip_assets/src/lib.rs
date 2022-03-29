//! # DEIP Assets Module
//! A module provides functionality of User Issued Assets.
//!
//! - [`Config`](./trait.Config.html)
//!
//! ## Overview
//! The pallet wraps Substrate [`pallet_assets`](../pallet_assets/index.html) and
//! adds additional constraints/features.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * [`create_asset`](./enum.Call.html#variant.create_asset)
//! * [`destroy`](./enum.Call.html#variant.destroy)
//! * [`issue_asset`](./enum.Call.html#variant.issue_asset)
//! * [`burn`](./enum.Call.html#variant.burn)
//! * [`transfer`](./enum.Call.html#variant.transfer)
//! * [`freeze`](./enum.Call.html#variant.freeze)
//! * [`thaw`](./enum.Call.html#variant.thaw)
//! * [`freeze_asset`](./enum.Call.html#variant.freeze_asset)
//! * [`thaw_asset`](./enum.Call.html#variant.thaw_asset)
//! * [`transfer_ownership`](./enum.Call.html#variant.transfer_ownership)
//! * [`set_team`](./enum.Call.html#variant.set_team)
//! * [`set_metadata`](./enum.Call.html#variant.set_metadata)
//!
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export to use implementation details in dependent crates:
pub use pallet_assets;

mod impl_fungibles;

pub use deip_serializable_u128::SerializableAtLeast32BitUnsigned as SerializableAssetBalance;

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::{
        pallet_prelude::{
            ensure, Blake2_128Concat, Decode, DispatchResultWithPostInfo, Encode, Get, Hooks,
            Identity, MaxEncodedLen, Member, OptionQuery, Parameter, Pays, StorageDoubleMap,
            StorageMap, StorageValue, ValueQuery,
        },
        traits::{Currency, ExistenceRequirement, UnfilteredDispatchable, WithdrawReasons},
        transactional, RuntimeDebug,
    };
    use frame_system::{
        pallet_prelude::{BlockNumberFor, OriginFor},
        RawOrigin,
    };
    use scale_info::TypeInfo;
    use sp_runtime::traits::{CheckedAdd, One, StaticLookup, Zero};
    use sp_std::prelude::*;

    use codec::HasCompact;
    use frame_support::dispatch::Weight;

    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use pallet_assets::WeightInfo;

    use deip_asset_system::AssetIdInitT;
    use deip_projects_info::DeipProjectsInfo;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    pub type ProjectsInfoOf<T> = <T as Config>::ProjectsInfo;
    pub type DeipProjectIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::ProjectId;
    type DeipInvestmentIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::InvestmentId;
    pub(crate) type AssetsAssetIdOf<T> = <T as Config>::AssetsAssetId;
    pub(crate) type AssetsBalanceOf<T> = <T as pallet_assets::Config>::Balance;
    pub type DeipAssetIdOf<T> = <T as Config>::AssetId;
    type AssetsWeightInfoOf<T> = <T as pallet_assets::Config>::WeightInfo;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_assets::Config<AssetId = Self::AssetsAssetId>
    {
        type ProjectsInfo: DeipProjectsInfo<Self::AccountId>;
        type DeipAccountId: Into<Self::AccountId> + From<Self::AccountId> + Parameter + Member;

        type AssetsAssetId: Member
            + Parameter
            + Default
            + Copy
            + HasCompact
            + MaxEncodedLen
            + CheckedAdd
            + One;

        #[cfg(not(feature = "std"))]
        type AssetId: Member + Parameter + Default + Copy;
        #[cfg(feature = "std")]
        type AssetId: Member
            + Parameter
            + Default
            + Copy
            + serde::Serialize
            + serde::de::DeserializeOwned
            + TypeInfo;
        type AssetIdInit: AssetIdInitT<<Self as Config>::AssetId>;

        /// Period of check for accounts with zero FTs
        #[pallet::constant]
        type WipePeriod: Get<Self::BlockNumber>;
    }

    use frame_support::traits::{GetStorageVersion, StorageVersion};

    pub const V0: StorageVersion = StorageVersion::new(0);
    pub const V1: StorageVersion = StorageVersion::new(1);

    #[doc(hidden)]
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
                    "AssetIdByDeipAssetId",
                    "DeipAssetIdByAssetId",
                    "NextAssetId",
                    "AssetIdByProjectId",
                    "ProjectIdByAssetId",
                    "InvestmentByAssetId",
                    "InvestmentMap",
                    "FtBalanceMap",
                    "AssetMetadataMap",
                ] {
                    reads += count_items(b"Assets", x.as_bytes());
                    move_storage_from_pallet(
                        x.as_bytes(),
                        "Assets".as_bytes(),
                        "DeipAssets".as_bytes(),
                    );
                }
                AssetIdByDeipAssetId::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, k2, v)| AssetIdByDeipAssetIdV1::<T>::insert(k, k2, v));
                DeipAssetIdByAssetId::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, k2, v)| DeipAssetIdByAssetIdV1::<T>::insert(k, k2, v));
                reads += AssetIdByProjectId::<T>::drain().count();
                reads += ProjectIdByAssetId::<T>::drain().count();
                InvestmentByAssetId::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, v)| InvestmentByAssetIdV1::<T>::insert(k, v));
                InvestmentMap::<T>::drain()
                    .map(|x| {
                        reads += 1;
                        x
                    })
                    .for_each(|(k, v)| InvestmentMapV1::<T>::insert(k, v));
                reads += FtBalanceMap::<T>::drain().count();
                reads += AssetMetadataMap::<T>::drain().count();

                for x in &["Asset", "Account", "Approvals", "Metadata"] {
                    reads += count_items(b"ParityTechAssets", x.as_bytes());
                    move_storage_from_pallet(
                        x.as_bytes(),
                        "ParityTechAssets".as_bytes(),
                        "Assets".as_bytes(),
                    );
                }
                let reads: Weight = reads.try_into().unwrap_or(Weight::MAX);
                return T::DbWeight::get().reads_writes(reads, reads)
            }
            0
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        ProjectDoesNotExist,
        ProjectDoesNotBelongToTeam,
        ReservedAssetCannotBeFreezed,
        ReservedAssetAccountCannotBeFreezed,
        FtNotFound,
        FtBalanceNotFound,
        AssetIdOverflow,
        DeipAssetIdExists,
        /// Asset with DeipAssetId wasn't created.
        DeipAssetIdDoesNotExist,
    }

    #[pallet::storage]
    pub(super) type AssetIdByDeipAssetId<T: Config> = StorageDoubleMap<
        _,
        Identity,
        DeipAssetIdOf<T>,
        Blake2_128Concat,
        AssetsAssetIdOf<T>,
        (),
        OptionQuery,
    >;
    // Migrate key hasher:
    #[pallet::storage]
    pub(super) type AssetIdByDeipAssetIdV1<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        DeipAssetIdOf<T>,
        Blake2_128Concat,
        AssetsAssetIdOf<T>,
        (),
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type DeipAssetIdByAssetId<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        AssetsAssetIdOf<T>,
        Identity,
        DeipAssetIdOf<T>,
        (),
        OptionQuery,
    >;
    // Migrate key hasher:
    #[pallet::storage]
    pub(super) type DeipAssetIdByAssetIdV1<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        AssetsAssetIdOf<T>,
        Blake2_128Concat,
        DeipAssetIdOf<T>,
        (),
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type NextAssetId<T> = StorageValue<_, AssetsAssetIdOf<T>, ValueQuery>;

    /// Deprecated
    #[pallet::storage]
    pub(super) type AssetIdByProjectId<T: Config> =
        StorageMap<_, Identity, DeipProjectIdOf<T>, Vec<DeipAssetIdOf<T>>, OptionQuery>;

    /// Deprecated
    #[pallet::storage]
    pub(super) type ProjectIdByAssetId<T: Config> =
        StorageMap<_, Identity, DeipAssetIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    #[pallet::storage]
    pub(super) type InvestmentByAssetId<T: Config> =
        StorageMap<_, Identity, DeipAssetIdOf<T>, Vec<DeipInvestmentIdOf<T>>, OptionQuery>;
    // Migrate key hasher:
    #[pallet::storage]
    pub(super) type InvestmentByAssetIdV1<T: Config> =
        StorageMap<_, Blake2_128Concat, DeipAssetIdOf<T>, Vec<DeipInvestmentIdOf<T>>, OptionQuery>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
    pub(super) struct Investment<AccountId, AssetId> {
        creator: AccountId,
        assets: Vec<AssetId>,
        asset_id: AssetId,
    }

    #[pallet::storage]
    pub(super) type InvestmentMap<T: Config> = StorageMap<
        _,
        Identity,
        DeipInvestmentIdOf<T>,
        Investment<AccountIdOf<T>, DeipAssetIdOf<T>>,
        OptionQuery,
    >;
    // Migrate key hasher:
    #[pallet::storage]
    pub(super) type InvestmentMapV1<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        DeipInvestmentIdOf<T>,
        Investment<AccountIdOf<T>, DeipAssetIdOf<T>>,
        OptionQuery,
    >;

    /// Deprecated
    #[pallet::storage]
    pub(super) type FtBalanceMap<T: Config> =
        StorageMap<_, Identity, DeipAssetIdOf<T>, Vec<AccountIdOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub(super) type LockedAssets<T: Config> =
        StorageMap<_, Identity, <T as Config>::AssetsAssetId, (), OptionQuery>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
    pub(super) struct AssetMetadata<U8> {
        name: Vec<U8>,
        symbol: Vec<U8>,
        decimals: U8,
    }

    /// Deprecated
    #[pallet::storage]
    pub(super) type AssetMetadataMap<T: Config> =
        StorageMap<_, Identity, DeipAssetIdOf<T>, AssetMetadata<u8>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T> {
        pub _marker: std::marker::PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { _marker: std::marker::PhantomData }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {}
    }

    impl<T: Config> Pallet<T> {
        pub fn project_key(id: &DeipProjectIdOf<T>) -> T::AccountId {
            let entropy =
                (b"deip/projects/", id.as_ref()).using_encoded(sp_io::hashing::blake2_256);
            T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
        }

        pub fn investment_key(id: &DeipInvestmentIdOf<T>) -> T::AccountId {
            let entropy =
                (b"deip/investments/", id.as_ref()).using_encoded(sp_io::hashing::blake2_256);
            T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
        }

        pub fn account_balance(account: &AccountIdOf<T>, asset: &DeipAssetIdOf<T>) -> T::Balance {
            match AssetIdByDeipAssetIdV1::<T>::iter_prefix(*asset).next() {
                None => Default::default(),
                Some(prefix) => pallet_assets::Pallet::<T>::balance(prefix.0, account.clone()),
            }
        }

        pub fn total_supply(asset: &DeipAssetIdOf<T>) -> T::Balance {
            match AssetIdByDeipAssetIdV1::<T>::iter_prefix(*asset).next() {
                None => Zero::zero(),
                Some(prefix) => pallet_assets::Pallet::<T>::total_supply(prefix.0),
            }
        }

        #[transactional]
        pub fn transactionally_transfer(
            from: &AccountIdOf<T>,
            asset: DeipAssetIdOf<T>,
            transfers: &[(AssetsBalanceOf<T>, AccountIdOf<T>)],
        ) -> Result<(), ()> {
            for (amount, to) in transfers {
                let result = Self::deip_transfer_impl(
                    RawOrigin::Signed(from.clone()).into(),
                    asset,
                    to.clone(),
                    *amount,
                );
                if result.is_err() {
                    return Err(())
                }
            }

            Ok(())
        }

        #[transactional]
        pub fn deip_transactionally_reserve(
            account: &T::AccountId,
            id: DeipInvestmentIdOf<T>,
            shares: &[(DeipAssetIdOf<T>, AssetsBalanceOf<T>)],
            asset_to_raise: DeipAssetIdOf<T>,
        ) -> Result<(), deip_assets_error::ReserveError<DeipAssetIdOf<T>>> {
            use deip_assets_error::ReserveError;

            ensure!(!InvestmentMapV1::<T>::contains_key(id.clone()), ReserveError::AlreadyReserved);

            let id_account = Self::investment_key(&id);
            let id_source = <T::Lookup as StaticLookup>::unlookup(id_account.clone());

            let reserved = T::Currency::withdraw(
                account,
                T::Currency::minimum_balance(),
                WithdrawReasons::RESERVE,
                ExistenceRequirement::AllowDeath,
            )
            .map_err(|_| ReserveError::NotEnoughBalance)?;

            T::Currency::resolve_creating(&id_account, reserved);

            let mut assets_to_reserve = Vec::<DeipAssetIdOf<T>>::with_capacity(shares.len());

            for (asset, amount) in shares {
                let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(asset)
                    .next()
                    .ok_or(ReserveError::AssetTransferFailed(*asset))?
                    .0;
                let call = pallet_assets::Call::<T>::transfer {
                    id: asset_id,
                    target: id_source.clone(),
                    amount: *amount,
                };
                let result = call.dispatch_bypass_filter(RawOrigin::Signed(account.clone()).into());
                if result.is_err() {
                    return Err(ReserveError::AssetTransferFailed(*asset))
                }

                assets_to_reserve.push(*asset);

                InvestmentByAssetIdV1::<T>::mutate_exists(*asset, |investments| {
                    match investments.as_mut() {
                        None => *investments = Some(vec![id.clone()]),
                        Some(c) => c.push(id.clone()),
                    };
                });
            }

            InvestmentByAssetIdV1::<T>::mutate_exists(asset_to_raise, |investments| {
                match investments.as_mut() {
                    None => *investments = Some(vec![id.clone()]),
                    Some(c) => c.push(id.clone()),
                };
            });

            InvestmentMapV1::<T>::insert(
                id.clone(),
                Investment {
                    creator: account.clone(),
                    assets: assets_to_reserve,
                    asset_id: asset_to_raise,
                },
            );

            Ok(())
        }

        #[transactional]
        pub fn transactionally_unreserve(
            id: DeipInvestmentIdOf<T>,
        ) -> Result<(), deip_assets_error::UnreserveError<DeipAssetIdOf<T>>> {
            use deip_assets_error::UnreserveError;

            let info = match InvestmentMapV1::<T>::take(id.clone()) {
                Some(i) => i,
                None => return Err(UnreserveError::NoSuchInvestment),
            };

            let deposited =
                T::Currency::deposit_creating(&info.creator, T::Currency::minimum_balance());

            let id_account = Self::investment_key(&id);

            for asset_id in info.assets.iter().chain(&[info.asset_id]) {
                InvestmentByAssetIdV1::<T>::mutate_exists(*asset_id, |maybe_investments| {
                    let investments =
                        maybe_investments.as_mut().expect("checked in transactionally_reserve");
                    let index = investments
                        .iter()
                        .position(|a| *a == id)
                        .expect("checked in transactionally_reserve");
                    investments.remove(index);
                    if investments.is_empty() {
                        *maybe_investments = None;
                    }
                });

                let amount = Self::account_balance(&id_account, asset_id);
                if amount.is_zero() {
                    continue
                }

                let result = Self::deip_transfer_impl(
                    RawOrigin::Signed(id_account.clone()).into(),
                    *asset_id,
                    info.creator.clone(),
                    amount,
                );
                if result.is_err() {
                    return Err(UnreserveError::AssetTransferFailed(*asset_id))
                }
            }

            T::Currency::settle(
                &id_account,
                deposited,
                WithdrawReasons::TRANSFER,
                ExistenceRequirement::AllowDeath,
            )
            .unwrap_or_else(|_| panic!("should be reserved in transactionally_reserve"));

            Ok(())
        }

        pub fn transfer_from_reserved(
            id: DeipInvestmentIdOf<T>,
            who: &T::AccountId,
            asset: DeipAssetIdOf<T>,
            amount: AssetsBalanceOf<T>,
        ) -> Result<(), deip_assets_error::UnreserveError<DeipAssetIdOf<T>>> {
            use deip_assets_error::UnreserveError;

            ensure!(
                InvestmentMapV1::<T>::contains_key(id.clone()),
                UnreserveError::NoSuchInvestment
            );

            let id_account = Self::investment_key(&id);

            let result = Self::deip_transfer_impl(
                RawOrigin::Signed(id_account).into(),
                asset,
                who.clone(),
                amount,
            );
            if result.is_err() {
                return Err(UnreserveError::AssetTransferFailed(asset))
            }

            Ok(())
        }

        pub fn deip_transfer_to_reserved(
            who: &T::AccountId,
            id: DeipInvestmentIdOf<T>,
            amount: AssetsBalanceOf<T>,
        ) -> Result<(), deip_assets_error::UnreserveError<DeipAssetIdOf<T>>> {
            use deip_assets_error::UnreserveError;

            let info = match InvestmentMapV1::<T>::try_get(id.clone()) {
                Ok(i) => i,
                Err(_) => return Err(UnreserveError::NoSuchInvestment),
            };

            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(info.asset_id)
                .next()
                .ok_or(UnreserveError::AssetTransferFailed(info.asset_id))?
                .0;

            let id_account = Self::investment_key(&id);
            let id_source = <T::Lookup as StaticLookup>::unlookup(id_account);

            let call =
                pallet_assets::Call::<T>::transfer { id: asset_id, target: id_source, amount };
            let result = call.dispatch_bypass_filter(RawOrigin::Signed(who.clone()).into());
            if result.is_err() {
                return Err(UnreserveError::AssetTransferFailed(info.asset_id))
            }

            Ok(())
        }

        // stores `to` in the map of FT-balances if the asset tokenizes some active
        fn deip_transfer_impl(
            from: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            to: AccountIdOf<T>,
            amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let target_source = <T::Lookup as StaticLookup>::unlookup(to);
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call =
                pallet_assets::Call::<T>::transfer { id: asset_id, target: target_source, amount };
            call.dispatch_bypass_filter(from)
        }

        fn deip_create_impl(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            admin: T::AccountId,
            min_balance: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                AssetIdByDeipAssetIdV1::<T>::iter_prefix(id).next().is_none(),
                Error::<T>::DeipAssetIdExists
            );

            let asset_id = NextAssetId::<T>::get();
            let next_asset_id =
                asset_id.checked_add(&One::one()).ok_or(Error::<T>::AssetIdOverflow)?;

            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin);
            let call =
                pallet_assets::Call::<T>::create { id: asset_id, admin: admin_source, min_balance };
            let post_dispatch_info = call.dispatch_bypass_filter(origin)?;

            NextAssetId::<T>::put(next_asset_id);
            AssetIdByDeipAssetIdV1::<T>::insert(id, asset_id, ());
            DeipAssetIdByAssetIdV1::<T>::insert(asset_id, id, ());

            Ok(post_dispatch_info)
        }

        fn deip_mint_impl(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            beneficiary: T::AccountId,
            amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let beneficiary_source = <T::Lookup as StaticLookup>::unlookup(beneficiary);

            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call = pallet_assets::Call::<T>::mint {
                id: asset_id,
                beneficiary: beneficiary_source,
                amount,
            };
            call.dispatch_bypass_filter(origin)
        }

        fn deip_set_metadata_impl(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call =
                pallet_assets::Call::<T>::set_metadata { id: asset_id, name, symbol, decimals };
            call.dispatch_bypass_filter(origin)
        }

        pub fn lock_asset(id: <T as Config>::AssetsAssetId) -> LockResult {
            LockedAssets::<T>::mutate_exists(id, |maybe_asset| {
                if maybe_asset.is_some() {
                    Err(LockError::AlreadyLocked)
                } else {
                    *maybe_asset = Some(());
                    Ok(())
                }
            })
        }

        pub fn unlock_asset(id: <T as Config>::AssetsAssetId) -> LockResult {
            LockedAssets::<T>::mutate_exists(id, |maybe_asset| {
                if maybe_asset.is_none() {
                    Err(LockError::NotLocked)
                } else {
                    *maybe_asset = None;
                    Ok(())
                }
            })
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(AssetsWeightInfoOf::<T>::create())]
        pub fn deip_create(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            admin: T::DeipAccountId,
            min_balance: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            Self::deip_create_impl(origin, id, admin.into(), min_balance)
        }

        #[pallet::weight((10_000, Pays::No))]
        pub fn deip_destroy(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            witness: pallet_assets::DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;

            let call = pallet_assets::Call::<T>::destroy { id: asset_id, witness };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::mint())]
        pub fn deip_mint(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            beneficiary: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            Self::deip_mint_impl(origin, id, beneficiary.into(), amount)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::burn())]
        pub fn deip_burn(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            who: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;

            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::burn { id: asset_id, who: who_source, amount };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer())]
        pub fn deip_transfer(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            target: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            Self::deip_transfer_impl(origin, id, target.into(), amount)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze())]
        pub fn deip_freeze(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !InvestmentByAssetIdV1::<T>::contains_key(id),
                Error::<T>::ReservedAssetAccountCannotBeFreezed
            );

            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::freeze { id: asset_id, who: who_source };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw())]
        pub fn deip_thaw(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call = pallet_assets::Call::<T>::thaw { id: asset_id, who: who_source };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze_asset())]
        pub fn deip_freeze_asset(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !InvestmentByAssetIdV1::<T>::contains_key(id),
                Error::<T>::ReservedAssetCannotBeFreezed
            );

            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call = pallet_assets::Call::<T>::freeze_asset { id: asset_id };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw_asset())]
        pub fn deip_thaw_asset(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call = pallet_assets::Call::<T>::thaw_asset { id: asset_id };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer_ownership())]
        pub fn deip_transfer_ownership(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call =
                pallet_assets::Call::<T>::transfer_ownership { id: asset_id, owner: owner_source };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_team())]
        pub fn deip_set_team(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            issuer: T::DeipAccountId,
            admin: T::DeipAccountId,
            freezer: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let issuer_source = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let freezer_source = <T::Lookup as StaticLookup>::unlookup(freezer.into());
            let asset_id = AssetIdByDeipAssetIdV1::<T>::iter_prefix(id)
                .next()
                .ok_or(Error::<T>::DeipAssetIdDoesNotExist)?
                .0;
            let call = pallet_assets::Call::<T>::set_team {
                id: asset_id,
                issuer: issuer_source,
                admin: admin_source,
                freezer: freezer_source,
            };
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_metadata(name.len() as u32, symbol.len() as u32))]
        pub fn deip_set_metadata(
            origin: OriginFor<T>,
            id: DeipAssetIdOf<T>,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            Self::deip_set_metadata_impl(origin, id, name, symbol, decimals)
        }
    }
}

#[cfg(feature = "std")]
impl<T: Config> GenesisConfig<T> {
    /// Direct implementation of `GenesisBuild::build_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
        <Self as frame_support::traits::GenesisBuild<T>>::build_storage(self)
    }

    /// Direct implementation of `GenesisBuild::assimilate_storage`.
    ///
    /// Kept in order not to break dependency.
    pub fn assimilate_storage(&self, storage: &mut sp_runtime::Storage) -> Result<(), String> {
        <Self as frame_support::traits::GenesisBuild<T>>::assimilate_storage(self, storage)
    }
}
