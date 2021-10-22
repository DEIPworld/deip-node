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

pub mod traits;

pub mod serializable;
pub use serializable::{AssetBalance as SerializableAssetBalance, AssetId as SerializableAssetId};

#[doc(inline)]
pub use pallet::*;

const NON_LOCAL: u8 = 101;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::{
        traits::{Currency, ExistenceRequirement, UnfilteredDispatchable, WithdrawReasons},
        transactional,
    };
    use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};
    use frame_system::{pallet_prelude::*, RawOrigin};
    use sp_runtime::traits::{One, StaticLookup, Zero};
    use sp_std::{prelude::*, vec};

    #[cfg(feature = "std")]
    use frame_support::traits::GenesisBuild;

    use pallet_assets::WeightInfo;

    use super::traits::DeipProjectsInfo;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type DeipProjectIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::ProjectId;
    type DeipInvestmentIdOf<T> =
        <<T as Config>::ProjectsInfo as DeipProjectsInfo<AccountIdOf<T>>>::InvestmentId;
    pub(crate) type AssetsAssetIdOf<T> = <T as pallet_assets::Config>::AssetId;
    pub(crate) type AssetsBalanceOf<T> = <T as pallet_assets::Config>::Balance;
    type AssetsWeightInfoOf<T> = <T as pallet_assets::Config>::WeightInfo;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_assets::Config + SendTransactionTypes<Call<Self>>
    {
        type ProjectsInfo: DeipProjectsInfo<Self::AccountId>;
        type DeipAccountId: Into<Self::AccountId> + Parameter + Member;

        /// Period of check for accounts with zero NFTs
        #[pallet::constant]
        type WipePeriod: Get<Self::BlockNumber>;
    }

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(n: T::BlockNumber) {
            if !sp_io::offchain::is_validator() {
                return;
            }

            if n % T::WipePeriod::get() != Zero::zero() {
                return;
            }

            for (asset, balances) in NftBalanceMap::<T>::iter() {
                for balance in balances {
                    if !Self::account_balance(&balance, &asset).is_zero() {
                        continue;
                    }

                    let call = Call::wipe_zero_balance(asset, balance);
                    let _submit =
                        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
                }
            }
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if !matches!(
                source,
                TransactionSource::Local | TransactionSource::InBlock
            ) {
                return InvalidTransaction::Custom(super::NON_LOCAL).into();
            }

            if let Call::wipe_zero_balance(ref asset, ref account) = call {
                if !Self::account_balance(account, asset).is_zero() {
                    return InvalidTransaction::Stale.into();
                }

                let balances = match NftBalanceMap::<T>::try_get(*asset) {
                    Err(_) => return InvalidTransaction::Stale.into(),
                    Ok(b) => b,
                };

                if let Err(_) = balances.binary_search_by_key(&account, |a| a) {
                    return InvalidTransaction::Stale.into();
                }

                ValidTransaction::with_tag_prefix("DeipAssetsOffchainWorker")
                    .propagate(false)
                    .longevity(5)
                    .and_provides((*asset, account.clone()))
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        ProjectDoesNotExist,
        ProjectDoesNotBelongToTeam,
        ProjectSecurityTokenCannotBeDestroyed,
        ProjectSecurityTokenCannotBeBurned,
        ProjectSecurityTokenCannotBeFreezed,
        ProjectSecurityTokenAccountCannotBeFreezed,
        ReservedAssetCannotBeFreezed,
        ReservedAssetAccountCannotBeFreezed,
        NFTNotFound,
        NFTBalanceNotFound,
    }

    #[pallet::storage]
    pub(super) type AssetIdByProjectId<T: Config> =
        StorageMap<_, Identity, DeipProjectIdOf<T>, Vec<AssetsAssetIdOf<T>>, OptionQuery>;

    #[pallet::storage]
    pub(super) type ProjectIdByAssetId<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, DeipProjectIdOf<T>, OptionQuery>;

    #[pallet::storage]
    pub(super) type CoreAssetId<T> = StorageValue<_, AssetsAssetIdOf<T>, ValueQuery>;

    #[pallet::storage]
    pub(super) type InvestmentByAssetId<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, Vec<DeipInvestmentIdOf<T>>, OptionQuery>;

    #[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
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
        Investment<AccountIdOf<T>, AssetsAssetIdOf<T>>,
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type NftBalanceMap<T: Config> =
        StorageMap<_, Identity, AssetsAssetIdOf<T>, Vec<AccountIdOf<T>>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub core_asset_admin: AccountIdOf<T>,
        pub core_asset_id: super::serializable::AssetId<T>,
        pub balances: Vec<(AccountIdOf<T>, super::serializable::AssetBalance<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                core_asset_admin: Default::default(),
                core_asset_id: Default::default(),
                balances: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            CoreAssetId::<T>::put(self.core_asset_id.0);

            let admin_source = T::Lookup::unlookup(self.core_asset_admin.clone());
            let call = pallet_assets::Call::<T>::create(
                self.core_asset_id.0,
                admin_source,
                One::one(),
            );
            let result = call
                .dispatch_bypass_filter(RawOrigin::Signed(self.core_asset_admin.clone()).into());
            assert!(result.is_ok());

            // ensure no duplicates exist.
            let endowed_accounts = self
                .balances
                .iter()
                .map(|(x, _)| x)
                .cloned()
                .collect::<std::collections::BTreeSet<_>>();

            assert!(
                endowed_accounts.len() == self.balances.len(),
                "duplicate balances in genesis."
            );

            for (ref who, amount) in &self.balances {
                let who_source = <T::Lookup as StaticLookup>::unlookup(who.clone());
                let call =
                    pallet_assets::Call::<T>::mint(self.core_asset_id.0, who_source, amount.0);
                let result = call.dispatch_bypass_filter(
                    RawOrigin::Signed(self.core_asset_admin.clone()).into(),
                );
                assert!(result.is_ok());
            }
        }
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

        pub fn try_get_tokenized_project(id: &T::AssetId) -> Option<DeipProjectIdOf<T>> {
            match ProjectIdByAssetId::<T>::try_get(*id) {
                Ok(project_id) => Some(project_id),
                Err(_) => None,
            }
        }

        pub fn account_balance(account: &AccountIdOf<T>, asset: &T::AssetId) -> T::Balance {
            pallet_assets::Pallet::<T>::balance(*asset, account.clone())
        }

        pub fn total_supply(asset: &T::AssetId) -> T::Balance {
            pallet_assets::Pallet::<T>::total_supply(*asset)
        }

        pub fn get_project_nfts(id: &DeipProjectIdOf<T>) -> Vec<T::AssetId> {
            AssetIdByProjectId::<T>::try_get(id.clone()).unwrap_or_default()
        }

        pub fn get_nft_balances(id: &T::AssetId) -> Option<Vec<AccountIdOf<T>>> {
            NftBalanceMap::<T>::try_get(*id).ok()
        }

        #[transactional]
        pub fn transactionally_transfer(
            from: &AccountIdOf<T>,
            asset: T::AssetId,
            transfers: &[(T::Balance, AccountIdOf<T>)],
        ) -> Result<(), ()> {
            for (amount, to) in transfers {
                let result = Self::transfer_impl(
                    RawOrigin::Signed(from.clone()).into(),
                    asset,
                    to.clone(),
                    *amount,
                );
                if result.is_err() {
                    return Err(());
                }
            }

            Ok(())
        }

        #[transactional]
        pub fn transactionally_reserve(
            account: &T::AccountId,
            id: DeipInvestmentIdOf<T>,
            shares: &[(T::AssetId, T::Balance)],
            asset_to_raise: T::AssetId,
        ) -> Result<(), deip_assets_error::ReserveError<T::AssetId>> {
            use deip_assets_error::ReserveError;

            ensure!(
                !InvestmentMap::<T>::contains_key(id.clone()),
                ReserveError::AlreadyReserved
            );

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

            let mut assets_to_reserve =
                Vec::<T::AssetId>::with_capacity(shares.len());

            for (asset, amount) in shares {
                let call = pallet_assets::Call::<T>::transfer(*asset, id_source.clone(), *amount);
                let result = call.dispatch_bypass_filter(RawOrigin::Signed(account.clone()).into());
                if result.is_err() {
                    return Err(ReserveError::AssetTransferFailed(*asset));
                }

                assets_to_reserve.push(*asset);

                InvestmentByAssetId::<T>::mutate_exists(*asset, |investments| {
                    match investments.as_mut() {
                        None => *investments = Some(vec![id.clone()]),
                        Some(c) => c.push(id.clone()),
                    };
                });
            }

            InvestmentByAssetId::<T>::mutate_exists(asset_to_raise, |investments| {
                match investments.as_mut() {
                    None => *investments = Some(vec![id.clone()]),
                    Some(c) => c.push(id.clone()),
                };
            });

            InvestmentMap::<T>::insert(
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
        ) -> Result<(), deip_assets_error::UnreserveError<T::AssetId>> {
            use deip_assets_error::UnreserveError;

            let info = match InvestmentMap::<T>::take(id.clone()) {
                Some(i) => i,
                None => return Err(UnreserveError::NoSuchInvestment),
            };

            let deposited =
                T::Currency::deposit_creating(&info.creator, T::Currency::minimum_balance());

            let id_account = Self::investment_key(&id);

            for asset_id in info.assets.iter().chain(&[info.asset_id]) {
                InvestmentByAssetId::<T>::mutate_exists(*asset_id, |maybe_investments| {
                    let investments = maybe_investments
                        .as_mut()
                        .expect("checked in transactionally_reserve");
                    let index = investments
                        .iter()
                        .position(|a| *a == id)
                        .expect("checked in transactionally_reserve");
                    investments.remove(index);
                    if investments.is_empty() {
                        *maybe_investments = None;
                    }
                });

                let amount = pallet_assets::Pallet::<T>::balance(*asset_id, id_account.clone());
                if amount.is_zero() {
                    continue;
                }

                let result = Self::transfer_impl(
                    RawOrigin::Signed(id_account.clone()).into(),
                    *asset_id,
                    info.creator.clone(),
                    amount,
                );
                if result.is_err() {
                    return Err(UnreserveError::AssetTransferFailed(*asset_id));
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
            asset: T::AssetId,
            amount: T::Balance,
        ) -> Result<(), deip_assets_error::UnreserveError<T::AssetId>> {
            use deip_assets_error::UnreserveError;

            ensure!(
                InvestmentMap::<T>::contains_key(id.clone()),
                UnreserveError::NoSuchInvestment
            );

            let id_account = Self::investment_key(&id);

            let result = Self::transfer_impl(
                RawOrigin::Signed(id_account.clone()).into(),
                asset,
                who.clone(),
                amount,
            );
            if result.is_err() {
                return Err(UnreserveError::AssetTransferFailed(asset));
            }

            Ok(())
        }

        pub fn transfer_to_reserved(
            who: &T::AccountId,
            id: DeipInvestmentIdOf<T>,
            amount: T::Balance,
        ) -> Result<(), deip_assets_error::UnreserveError<T::AssetId>> {
            use deip_assets_error::UnreserveError;

            let info = match InvestmentMap::<T>::try_get(id.clone()) {
                Ok(i) => i,
                Err(_) => return Err(UnreserveError::NoSuchInvestment),
            };

            let id_account = Self::investment_key(&id);
            let id_source = <T::Lookup as StaticLookup>::unlookup(id_account);

            let call = pallet_assets::Call::<T>::transfer(info.asset_id, id_source, amount);
            let result = call.dispatch_bypass_filter(RawOrigin::Signed(who.clone()).into());
            if result.is_err() {
                return Err(UnreserveError::AssetTransferFailed(info.asset_id));
            }

            Ok(())
        }

        // stores `to` in the map of NFT-balances if the asset tokenizes some active
        fn transfer_impl(
            from: OriginFor<T>,
            id: T::AssetId,
            to: AccountIdOf<T>,
            amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let target_source = <T::Lookup as StaticLookup>::unlookup(to.clone());
            let call = pallet_assets::Call::<T>::transfer(id, target_source, amount);
            let ok = call.dispatch_bypass_filter(from)?;

            if let Some(_) = Self::try_get_tokenized_project(&id) {
                NftBalanceMap::<T>::mutate_exists(id, |maybe| match maybe.as_mut() {
                    None => {
                        // this cannot happen but for any case
                        *maybe = Some(vec![to]);
                        return;
                    }
                    Some(b) => match b.binary_search_by_key(&&to, |a| a) {
                        Ok(_) => (),
                        Err(i) => b.insert(i, to),
                    },
                });
            }

            Ok(ok)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(AssetsWeightInfoOf::<T>::create())]
        pub fn create_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            admin: T::DeipAccountId,
            min_balance: AssetsBalanceOf<T>,
            project_id: Option<DeipProjectIdOf<T>>,
        ) -> DispatchResultWithPostInfo {
            if let Some(ref id) = project_id {
                match T::ProjectsInfo::try_get_project_team(id) {
                    None => return Err(Error::<T>::ProjectDoesNotExist.into()),
                    Some(team_id) => {
                        let account = ensure_signed(origin.clone())?;
                        ensure!(team_id == account, Error::<T>::ProjectDoesNotBelongToTeam)
                    }
                };
            }

            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let call = pallet_assets::Call::<T>::create(id, admin_source, min_balance);
            let result = call.dispatch_bypass_filter(origin);
            if result.is_err() {
                return result;
            }

            if let Some(project_id) = project_id {
                ProjectIdByAssetId::<T>::insert(id, project_id.clone());
                AssetIdByProjectId::<T>::mutate_exists(project_id, |tokens| {
                    match tokens.as_mut() {
                        None => *tokens = Some(vec![id]),
                        Some(c) => c.push(id),
                    };
                });
            }

            result
        }

        #[pallet::weight((10_000, Pays::No))]
        pub fn destroy(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            witness: pallet_assets::DestroyWitness,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeDestroyed
            );

            let call = pallet_assets::Call::<T>::destroy(id, witness);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::mint())]
        pub fn issue_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            beneficiary: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let beneficiary_source =
                <T::Lookup as StaticLookup>::unlookup(beneficiary.clone().into());
            let call = pallet_assets::Call::<T>::mint(id, beneficiary_source, amount);
            let result = call.dispatch_bypass_filter(origin)?;

            if let Some(_) = Self::try_get_tokenized_project(&id) {
                NftBalanceMap::<T>::mutate_exists(id, |maybe| {
                    let balances = match maybe.as_mut() {
                        None => {
                            *maybe = Some(vec![beneficiary.into()]);
                            return;
                        }
                        Some(b) => b,
                    };

                    let account = beneficiary.into();
                    match balances.binary_search_by_key(&&account, |a| a) {
                        Ok(_) => (),
                        Err(i) => balances.insert(i, account),
                    };
                });
            }

            Ok(result)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeBurned
            );

            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::burn(id, who_source, amount);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            target: T::DeipAccountId,
            #[pallet::compact] amount: AssetsBalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            Self::transfer_impl(origin, id, target.into(), amount)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze())]
        pub fn freeze(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenAccountCannotBeFreezed
            );

            ensure!(
                !InvestmentByAssetId::<T>::contains_key(id),
                Error::<T>::ReservedAssetAccountCannotBeFreezed
            );

            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::freeze(id, who_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw())]
        pub fn thaw(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            who: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let who_source = <T::Lookup as StaticLookup>::unlookup(who.into());
            let call = pallet_assets::Call::<T>::thaw(id, who_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::freeze_asset())]
        pub fn freeze_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                !ProjectIdByAssetId::<T>::contains_key(id),
                Error::<T>::ProjectSecurityTokenCannotBeFreezed
            );

            ensure!(
                !InvestmentByAssetId::<T>::contains_key(id),
                Error::<T>::ReservedAssetCannotBeFreezed
            );

            let call = pallet_assets::Call::<T>::freeze_asset(id);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::thaw_asset())]
        pub fn thaw_asset(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::thaw_asset(id);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::transfer_ownership())]
        pub fn transfer_ownership(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            owner: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let owner_source = <T::Lookup as StaticLookup>::unlookup(owner.into());
            let call = pallet_assets::Call::<T>::transfer_ownership(id, owner_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_team())]
        pub fn set_team(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            issuer: T::DeipAccountId,
            admin: T::DeipAccountId,
            freezer: T::DeipAccountId,
        ) -> DispatchResultWithPostInfo {
            let issuer_source = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            let admin_source = <T::Lookup as StaticLookup>::unlookup(admin.into());
            let freezer_source = <T::Lookup as StaticLookup>::unlookup(freezer.into());
            let call =
                pallet_assets::Call::<T>::set_team(id, issuer_source, admin_source, freezer_source);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(AssetsWeightInfoOf::<T>::set_metadata(name.len() as u32, symbol.len() as u32))]
        pub fn set_metadata(
            origin: OriginFor<T>,
            #[pallet::compact] id: T::AssetId,
            name: Vec<u8>,
            symbol: Vec<u8>,
            decimals: u8,
        ) -> DispatchResultWithPostInfo {
            let call = pallet_assets::Call::<T>::set_metadata(id, name, symbol, decimals);
            call.dispatch_bypass_filter(origin)
        }

        #[pallet::weight(10_000)]
        pub fn wipe_zero_balance(
            origin: OriginFor<T>,
            asset: AssetsAssetIdOf<T>,
            account: AccountIdOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            NftBalanceMap::<T>::mutate_exists(asset, |maybe| match maybe.as_mut() {
                None => Err(Error::<T>::NFTNotFound.into()),
                Some(b) => match b.binary_search_by_key(&&account, |a| a) {
                    Err(_) => Err(Error::<T>::NFTBalanceNotFound.into()),
                    Ok(i) => {
                        b.remove(i);
                        if b.is_empty() {
                            *maybe = None;
                        }
                        Ok(Some(0).into())
                    }
                },
            })
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
