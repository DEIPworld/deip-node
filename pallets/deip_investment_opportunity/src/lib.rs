//! # DEIP Investment Opportunity module
//! A module for investment operations
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests;

pub mod benchmarking;
pub mod weights;
pub mod module;

const NON_LOCAL: u8 = 100;

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::{pallet_prelude::*, RawOrigin};
    use frame_system::offchain::SendTransactionTypes;

    use frame_support::{
        pallet_prelude::*,
        weights::{GetDispatchInfo, PostDispatchInfo},
        Hashable,
    };

    use frame_support::traits::{
        Get,
        IsSubType,
        UnfilteredDispatchable,
        StoredMap
    };

    use sp_std::{
        collections::btree_map::BTreeMap,
        iter::FromIterator,
        prelude::*,
        fmt::Debug
    };

    use frame_support::dispatch::DispatchResult;
    use sp_runtime::{
        traits::{Dispatchable, IdentifyAccount},
        MultiSigner,
    };

    use sp_core::H256;
    use crate::module::{InvestmentId, FundingModelOf, DeipAsset, DeipAssetBalance, DeipAssetId};

    use crate::weights::WeightInfo;
    use deip_asset_system::DeipAssetSystem;
    use deip_transaction_ctx::{PortalCtxT, TransactionCtxId};

    /// Configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config +
        pallet_timestamp::Config +
        DeipAssetSystem<Self::AccountId, Self::SourceId, InvestmentId> +
        SendTransactionTypes<Call<Self>>
    {
        type DeipInvestmentWeightInfo: WeightInfo;
        type Event: Parameter
            + Member
            + From<Event<Self>>
            + Debug
            + IsType<<Self as frame_system::Config>::Event>;

        type TransactionCtx: PortalCtxT<Call<Self>>;
        type DeipAccountId: Into<Self::AccountId> + From<Self::AccountId> + Parameter + Member + Default;

        #[pallet::constant]
        type MaxInvestmentShares: Get<u16>;

        type SourceId: Member + Parameter;

    }

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(_n: BlockNumberFor<T>) {
            if !sp_io::offchain::is_validator() {
                return;
            }
            Self::process_investment_opportunities_offchain();
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            // Firstly let's check that we get the local transaction.
            if !matches!(source, TransactionSource::Local | TransactionSource::InBlock) {
                return InvalidTransaction::Custom(crate::NON_LOCAL).into()
            }

            use crate::module::SimpleCrowdfundingStatus;

            match call {
                Call::activate_crowdfunding { sale_id: id } => {
                    let sale = SimpleCrowdfundings::<T>::try_get(id)
                        .map_err(|_| InvalidTransaction::Stale)?;
                    if !matches!(sale.status, SimpleCrowdfundingStatus::Inactive) {
                        return InvalidTransaction::Stale.into()
                    }

                    ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                        .propagate(false)
                        .longevity(5)
                        .and_provides((b"activate_crowdfunding", *id))
                        .build()
                },
                Call::expire_crowdfunding { sale_id: id } => {
                    let sale = SimpleCrowdfundings::<T>::try_get(id)
                        .map_err(|_| InvalidTransaction::Stale)?;
                    if !matches!(sale.status, SimpleCrowdfundingStatus::Active) {
                        return InvalidTransaction::Stale.into()
                    }

                    ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                        .propagate(false)
                        .longevity(5)
                        .and_provides((b"expire_crowdfunding", *id))
                        .build()
                },
                Call::finish_crowdfunding { sale_id: id } => {
                    let sale = SimpleCrowdfundings::<T>::try_get(id)
                        .map_err(|_| InvalidTransaction::Stale)?;
                    if !matches!(sale.status, SimpleCrowdfundingStatus::Active) {
                        return InvalidTransaction::Stale.into()
                    }

                    ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                        .propagate(false)
                        .longevity(5)
                        .and_provides((b"finish_crowdfunding", *id))
                        .build()
                },
                _ => InvalidTransaction::Call.into(),
            }
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        StartTimeMustBeLaterOrEqualCurrentMoment,
        EndTimeMustBeLaterStartTime,
        SoftCapMustBeGreaterOrEqualMinimum,
        HardCapShouldBeGreaterOrEqualSoftCap,
        AlreadyExists,
        BalanceIsNotEnough,
        FailedToReserveAsset,
        AssetAmountMustBePositive,
        SecurityTokenNotSpecified,
        NotFound,
        ShouldBeInactive,
        ShouldBeStarted,
        ShouldBeActive,
        ExpirationWrongState,
        WrongAssetId,
        CapDifferentAssets,
        TooMuchShares,
        // Possible errors when DAO tries to invest to an opportunity
        InvestingNotFound,
        InvestingNotActive,
        InvestingNotEnoughFunds,
        InvestingWrongAsset,
        /// Access Forbdden
        NoPermission,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a simple crowd funding has been created.
        SimpleCrowdfundingCreated(InvestmentId),
        /// Event emitted when a simple crowd funding has been activated.
        SimpleCrowdfundingActivated(InvestmentId),
        /// Event emitted when a simple crowd funding has finished.
        SimpleCrowdfundingFinished(InvestmentId),
        /// Event emitted when a simple crowd funding has expired.
        SimpleCrowdfundingExpired(InvestmentId),
        /// Event emitted when DAO invested to an opportunity
        Invested(InvestmentId, T::AccountId),
    }

    #[doc(hidden)]
    #[pallet::genesis_config]
    pub struct GenesisConfig {}
    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {}
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    {
        /// Allows DAO to create an investment opportunity.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `external_id`: id of the sale. Must be unique.
        /// - `project_id`: id of the project which tokens are intended to sale.
        /// - `investment_type`: specifies type of created investment opportunity. For possible
        /// variants and details see [`FundingModel`].
        #[pallet::weight({
            let s = shares.len() as u32;
            T::DeipInvestmentWeightInfo::create_investment_opportunity(s)
        })]
        pub fn create_investment_opportunity(
            origin: OriginFor<T>,
            external_id: InvestmentId,
            creator: T::DeipAccountId,
            shares: Vec<DeipAsset<T>>,
            funding_model: FundingModelOf<T>,
        ) -> DispatchResult
        {
            let account = ensure_signed(origin)?;
            Self::create_investment_opportunity_impl(account, external_id, creator.into(), shares, funding_model)
        }

        #[pallet::weight(T::DeipInvestmentWeightInfo::activate_crowdfunding())]
        pub fn activate_crowdfunding(
            origin: OriginFor<T>,
            sale_id: InvestmentId
        ) -> DispatchResult
        {
            ensure_none(origin)?;
            Self::activate_crowdfunding_impl(sale_id)
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        pub fn expire_crowdfunding(
            origin: OriginFor<T>,
            sale_id: InvestmentId
        ) -> DispatchResultWithPostInfo
        {
            ensure_none(origin)?;
            Self::expire_crowdfunding_impl(sale_id)
        }

        #[pallet::weight(T::DeipInvestmentWeightInfo::finish_crowdfunding())]
        pub fn finish_crowdfunding(
            origin: OriginFor<T>,
            sale_id: InvestmentId
        ) -> DispatchResult
        {
            ensure_none(origin)?;
            Self::finish_crowdfunding_impl(sale_id)
        }

        /// Allows DAO to invest to an opportunity.
        ///
        /// The origin for this call must be _Signed_.
        ///
        /// - `id`: identifier of the investment opportunity
        /// - `amount`: amount of units to invest. The account should have enough funds on
        ///     the balance. This amount is reserved until the investment finished or expired
        #[pallet::weight(
            T::DeipInvestmentWeightInfo::invest()
                .max(T::DeipInvestmentWeightInfo::invest_hard_cap_reached())
        )]
        pub fn invest(
            origin: OriginFor<T>,
            id: InvestmentId,
            asset: DeipAsset<T>
        ) -> DispatchResultWithPostInfo {
            let account = ensure_signed(origin)?;
            Self::invest_to_crowdfunding_impl(account, id, asset)
        }
    }

    // ==== Storage ====:

    use crate::module::{Investment, SimpleCrowdfundingOf};

    #[pallet::storage]
    pub type Investments<T: Config> = StorageMap<_,
        Blake2_128Concat,
        InvestmentId,
        Vec<(T::AccountId, Investment<T>)>
    >;

    #[pallet::storage]
    pub type SimpleCrowdfundings<T: Config> = StorageMap<_,
        Blake2_128Concat,
        InvestmentId,
        SimpleCrowdfundingOf<T>,
    >;
}
