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
pub mod crowdfunding;

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
        transactional
    };

    use frame_support::traits::{
        Get,
        IsSubType,
        UnfilteredDispatchable,
        StoredMap,
        fungibles,
        tokens::nonfungibles,
        ReservableCurrency,
    };

    use sp_std::{
        iter::FromIterator,
        prelude::*,
        fmt::Debug
    };

    use frame_support::dispatch::DispatchResult;
    use sp_runtime::{
        traits::{Dispatchable, IdentifyAccount, AtLeast32BitUnsigned},
        MultiSigner,
        FixedPointOperand
    };

    use crate::module::{*};

    use crate::weights::WeightInfo;
    use deip_asset_system::{NFTImplT, NFTokenItemIdT, FTImplT, NFTokenFractionT};
    use deip_transaction_ctx::{PortalCtxT, TransactionCtxId};

    /// Configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config +
        pallet_timestamp::Config +
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
        type MaxShares: Get<u16>;

        type Currency: ReservableCurrency<Self::AccountId>;

        type Crowdfunding: CrowdfundingT<Self> + Parameter + Member;

        type AssetAmount:
            Default +
            AtLeast32BitUnsigned +
            FixedPointOperand +
            Clone + Parameter + Member + Copy;

        type AssetId:
            NFTokenItemIdT<Self::AssetImpl> +
            Default + Parameter + Member + Clone + Copy;

        type AssetImpl: NFTImplT<
            Account=Self::AccountId,
            FTokenAmount=Self::AssetAmount,
            NFTokenItemId=Self::AssetId
        >;

        type Asset: NFTokenFractionT<Self::AssetImpl>;
    }

    use frame_support::traits::StorageVersion;
    use frame_support::dispatch::GetStorageVersion;

    pub const V1: StorageVersion = StorageVersion::new(1);

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(V1)]
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

            use crate::module::CrowdfundingStatus;

            match call {
                Call::activate { id } => {
                    let sale = SimpleCrowdfundingMapV1::<T>::try_get(id)
                        .map_err(|_| InvalidTransaction::Stale)?;
                    if !matches!(sale.status, CrowdfundingStatus::Ready) {
                        return InvalidTransaction::Stale.into()
                    }

                    ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                        .propagate(false)
                        .longevity(5)
                        .and_provides((b"activate_crowdfunding", *id))
                        .build()
                },
                Call::expire { id } => {
                    let sale = SimpleCrowdfundingMapV1::<T>::try_get(id)
                        .map_err(|_| InvalidTransaction::Stale)?;
                    if !matches!(sale.status, CrowdfundingStatus::Active) {
                        return InvalidTransaction::Stale.into()
                    }

                    ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                        .propagate(false)
                        .longevity(5)
                        .and_provides((b"expire_crowdfunding", *id))
                        .build()
                },
                // Call::finish { id } => {
                //     let sale = SimpleCrowdfundingMapV1::<T>::try_get(id)
                //         .map_err(|_| InvalidTransaction::Stale)?;
                //     if !matches!(sale.status, CrowdfundingStatus::Active) {
                //         return InvalidTransaction::Stale.into()
                //     }
                //
                //     ValidTransaction::with_tag_prefix("DeipInvestmentOpportunityOCW")
                //         .propagate(false)
                //         .longevity(5)
                //         .and_provides((b"finish_crowdfunding", *id))
                //         .build()
                // },
                _ => InvalidTransaction::Call.into(),
            }
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        StartTimeMiscondition,
        EndTimeMiscondition,
        SoftCapMiscondition,
        HardCapMiscondition,
        AlreadyExists,
        BalanceIsNotEnough,
        SecurityTokenNotSpecified,
        NotFound,
        ImpossibleSituation,
        WrongAssetId,
        NoShares,
        TooMuchShares,
        WrongAsset,
        NoPermission,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a simple crowd funding has been created.
        Created(T::Crowdfunding),
        /// Event emitted when a simple crowd funding has been activated.
        Activated(CrowdfundingId),
        /// Event emitted when a simple crowd funding has finished.
        SimpleCrowdfundingFinished(CrowdfundingId),
        /// Event emitted when a simple crowd funding has expired.
        Expired(CrowdfundingId, CrowdfundingStatus),
        /// Event emitted when DAO invested to an opportunity
        Invested(CrowdfundingId, T::AccountId),
        HardCapReached(CrowdfundingId, T::AccountId),
        CommitShares {
            id: CrowdfundingId,
            shares: (T::AssetId, T::AssetAmount)
        },
        RollbackShares{
            id: CrowdfundingId,
            shares: (T::AssetId, T::AssetAmount)
        },
        Refund(CrowdfundingId, T::AccountId),
        Refunded(CrowdfundingId),
        StatusUpdated(CrowdfundingId, CrowdfundingStatus),
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

    use crate::module::{*};

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
            let s = 0;//shares.len() as u32;
            T::DeipInvestmentWeightInfo::create_investment_opportunity(s)
        })]
        #[transactional]
        pub fn create(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            creator: T::DeipAccountId,
            shares: (T::AssetId, T::AssetAmount),
            fund: T::AssetId,
        ) -> DispatchResult
        {
            T::create::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                shares,
                fund
            )
        }

        #[pallet::weight(T::DeipInvestmentWeightInfo::activate_crowdfunding())]
        #[transactional]
        pub fn commit_shares(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            shares: (T::AssetId, T::AssetAmount)
        ) -> DispatchResult
        {
            T::commit_shares::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                shares
            )
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        #[transactional]
        pub fn rollback_shares(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            shares: T::AssetId
        ) -> DispatchResult
        {
            T::rollback_shares::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                shares,
            )
        }

        #[pallet::weight(T::DeipInvestmentWeightInfo::activate_crowdfunding())]
        pub fn ready(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            start_time: Option<T::Moment>,
            end_time: T::Moment,
            soft_cap: T::AssetAmount,
            hard_cap: T::AssetAmount,
        ) -> DispatchResult
        {
            T::ready::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                start_time,
                end_time,
                soft_cap,
                hard_cap
            )
        }

        #[pallet::weight(T::DeipInvestmentWeightInfo::activate_crowdfunding())]
        pub fn activate(
            origin: OriginFor<T>,
            id: CrowdfundingId
        ) -> DispatchResult
        {
            ensure_none(origin)?;
            T::activate::<CrowdfundingStatus>(id)
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
        #[transactional]
        pub fn invest(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            amount: T::AssetAmount
        ) -> DispatchResultWithPostInfo
        {
            T::invest::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                amount
            )
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        #[transactional]
        pub fn payout(
            origin: OriginFor<T>,
            investor: Option<T::AccountId>,
            id: CrowdfundingId,
            shares: T::AssetId
        ) -> DispatchResult
        {
            T::payout::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                investor,
                id,
                shares
            )
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        #[transactional]
        pub fn raise(
            origin: OriginFor<T>,
            id: CrowdfundingId
        ) -> DispatchResult
        {
            T::raise::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id
            )
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        pub fn expire(
            origin: OriginFor<T>,
            id: CrowdfundingId
        ) -> DispatchResultWithPostInfo
        {
            ensure_none(origin)?;
            T::expire::<CrowdfundingStatus>(id)
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        #[transactional]
        pub fn refund(
            origin: OriginFor<T>,
            investor: Option<T::AccountId>,
            id: CrowdfundingId
        ) -> DispatchResult
        {
            T::refund::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                investor,
                id
            )
        }

        #[pallet::weight(
            T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()
                .max(T::DeipInvestmentWeightInfo::expire_crowdfunding())
        )]
        #[transactional]
        pub fn release_shares(
            origin: OriginFor<T>,
            id: CrowdfundingId,
            shares: T::AssetId
        ) -> DispatchResult
        {
            T::release_shares::<CrowdfundingStatus>(
                ensure_signed(origin)?,
                id,
                shares,
            )
        }
    }

    // ==== Storage ====:

    use crate::module::{Investment, SimpleCrowdfundingOf};

    #[pallet::storage]
    pub type InvestmentMapV1<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        Vec<(T::AccountId, Investment<T>)>
    >;

    #[pallet::storage]
    pub type InvestmentMapV2<T: Config> = StorageDoubleMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        Blake2_128Concat,
        T::AccountId,
        Investment<T>
    >;

    #[pallet::storage]
    pub type PayoutMapV2<T: Config> = StorageNMap<_,
        (
            NMapKey<Blake2_128Concat, CrowdfundingId>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Blake2_128Concat, T::AssetId>,
        ),
        ()
    >;

    #[pallet::storage]
    pub type SimpleCrowdfundingMapV1<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        SimpleCrowdfundingOf<T>,
    >;

    #[pallet::storage]
    pub(crate) type CrowdfundingStatusV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        CrowdfundingStatus
    >;

    #[pallet::storage]
    pub(crate) type SharesMapV2<T: Config> = StorageDoubleMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        Blake2_128Concat,
        T::AssetId,
        T::AssetAmount
    >;

    /// <INCOMPLeTE> is NOT! ComplEte to activAtion!
    pub(crate) struct IncompleteRepo<T>(PhantomData<T>);
    /// <READY> to ActivatiOn;
    pub(crate) struct ReadyRepo<T>(PhantomData<T>);
    pub(crate) struct ActiveRepo<T>(PhantomData<T>);

    pub(crate) struct PayoutRepo<T>(PhantomData<T>);
    pub(crate) struct RaiseRepo<T>(PhantomData<T>);

    pub(crate) struct RefundRepo<T>(PhantomData<T>);
    pub(crate) struct ReleaseSharesRepo<T>(PhantomData<T>);

    impl<T: Config> RepositoryT<T> for IncompleteRepo<T> {
        type S = IncompleteCrowdfundingMapV2<T>;
    }
    impl<T: Config> RepositoryT<T> for ReadyRepo<T> {
        type S = ReadyCrowdfundingMapV2<T>;
    }
    impl<T: Config> RepositoryT<T> for ActiveRepo<T> {
        type S = ActiveCrowdfundingMapV2<T>;
    }

    impl<T: Config> RepositoryT<T> for PayoutRepo<T> {
        type S = PayoutCrowdfundingMapV2<T>;
    }
    impl<T: Config> RepositoryT<T> for RaiseRepo<T> {
        type S = RaiseCrowdfundingMapV2<T>;
    }

    impl<T: Config> RepositoryT<T> for RefundRepo<T> {
        type S = RefundCrowdfundingMapV2<T>;
    }
    impl<T: Config> RepositoryT<T> for ReleaseSharesRepo<T> {
        type S = ReleaseSharesCrowdfundingMapV2<T>;
    }

    #[pallet::storage]
    pub(crate) type IncompleteCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;
    #[pallet::storage]
    pub(crate) type ReadyCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;
    #[pallet::storage]
    pub(crate) type ActiveCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;

    #[pallet::storage]
    pub(crate) type PayoutCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;
    #[pallet::storage]
    pub(crate) type RaiseCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;

    #[pallet::storage]
    pub(crate) type RefundCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;
    #[pallet::storage]
    pub(crate) type ReleaseSharesCrowdfundingMapV2<T: Config> = StorageMap<_,
        Blake2_128Concat,
        CrowdfundingId,
        T::Crowdfunding
    >;
}
