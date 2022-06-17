#![allow(type_alias_bounds)]

use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{self, Serialize, Deserialize};
use sp_runtime::traits::{AtLeast32BitUnsigned, Saturating, Zero, One, Member};
use sp_runtime::{FixedPointOperand, FixedU128, FixedPointNumber};
use frame_support::{RuntimeDebug, ensure, Parameter};
use frame_support::traits::Get;
use scale_info::TypeInfo;
use sp_runtime::SaturatedConversion;
use sp_std::prelude::*;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::default::Default;
use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use deip_asset_system::asset::{Asset};
use deip_transaction_ctx::{TransactionCtxId, TransactionCtxT};

use crate::module::{*};
use crate::{*};

impl<T: crate::Config> CrowdfundingT<T> for SimpleCrowdfundingV2<T>
{
    type Investment = InvestmentV2<
        T::AccountId,
        T::AssetAmount,
        T::Moment,
    >;

    fn new(
        ctx: T::TransactionCtx,
        creator: T::AccountId,
        account: T::AccountId,
        external_id: CrowdfundingId,
        fund: T::AssetId,
    ) -> Self
    {
        SimpleCrowdfundingV2 {
            v1: SimpleCrowdfunding {
                created_ctx: ctx.id(),
                external_id,
                start_time: <_>::default(),
                end_time: <_>::default(),
                status: CrowdfundingStatus::Incomplete,
                asset_id: fund,
                total_amount: Default::default(),
                soft_cap: SerializableAtLeast32BitUnsigned(<_>::default()),
                hard_cap: SerializableAtLeast32BitUnsigned(<_>::default()),
                shares: vec![],
            },
            creator,
            account,
            shares: <_>::default(),
            payouts: 0,
        }
    }

    fn ctx(&self) -> &TransactionCtxId<T::TransactionCtx> {
        &self.v1.created_ctx
    }

    fn id(&self) -> &CrowdfundingId {
        &self.v1.external_id
    }

    fn creator(&self) -> &T::AccountId {
        &self.creator
    }

    fn account(&self) -> &T::AccountId {
        &self.account
    }

    fn fund(
        &self,
        account: &T::AccountId,
        amount: T::AssetAmount
    ) -> Result<T::Asset, crate::Error<T>>
    {
        T::Asset::pick_fraction(self.v1.asset_id, account)
            .ok_or_else(|| crate::Error::BalanceIsNotEnough)
    }

    fn fund_balance(&self) -> Result<T::Asset, crate::Error<T>> {
        let balance
            = *T::Asset::pick_fraction(self.v1.asset_id, self.account())
            .ok_or_else(|| crate::Error::BalanceIsNotEnough)?
            .amount();
        self.fund(self.account(), balance)
    }

    fn start_time(&self) -> &T::Moment {
        &self.v1.start_time
    }

    fn end_time(&self) -> &T::Moment {
        &self.v1.end_time
    }

    fn soft_cap(&self) -> &T::AssetAmount {
        &self.v1.soft_cap.0
    }

    fn hard_cap(&self) -> &T::AssetAmount {
        &self.v1.hard_cap.0
    }

    fn shares(&self) -> u16 {
        self.shares
    }

    fn status(&self) -> CrowdfundingStatus {
        self.v1.status
    }

    fn fund_amount(&self) -> &T::AssetAmount {
        &self.v1.total_amount.0
    }

    fn payouts(&self) -> u16 {
        self.payouts
    }

    fn set_status(&mut self, status: CrowdfundingStatus) {
        self.v1.status = status;
    }

    fn commit_shares(
        &mut self,
        shares: (T::AssetId, T::AssetAmount)
    ) -> Result<SharesTransfer<'_, T>, crate::Error<T>>
    {
        ensure!(self.shares < T::MaxShares::get(), crate::Error::TooMuchShares);
        self.shares.saturating_inc();
        let (id, amount) = shares;
        Ok(SharesTransfer::<T>::new(
            T::Asset::pick_fraction(id, self.creator())
                .ok_or_else(|| crate::Error::BalanceIsNotEnough)?,
            self.account(),
            amount
        ))
    }

    fn rollback_shares(
        &mut self,
        shares: (T::AssetId, T::AssetAmount)
    ) -> Result<SharesTransfer<'_, T>, crate::Error<T>>
    {
        ensure!(self.shares > 0, crate::Error::NoShares);
        self.shares.saturating_dec();
        let (id, amount) = shares;
        Ok(SharesTransfer::<T>::new(
            T::Asset::pick_fraction(id, self.account())
                .ok_or_else(|| crate::Error::ImpossibleSituation)?,
            self.creator(),
            amount
        ))
    }

    fn ready(
        &mut self,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: T::AssetAmount,
        hard_cap: T::AssetAmount
    ) -> Result<(), crate::Error<T>>
    {
        ensure!(self.shares > 0, crate::Error::NoShares);

        ensure!(start_time < end_time, crate::Error::EndTimeMiscondition);
        ensure!(T::AssetAmount::zero() < soft_cap, crate::Error::SoftCapMiscondition);
        ensure!(hard_cap >= soft_cap, crate::Error::HardCapMiscondition);

        self.v1.start_time = start_time;
        self.v1.end_time = end_time;
        self.v1.soft_cap.0 = soft_cap;
        self.v1.hard_cap.0 = hard_cap;

        Ok(())
    }

    fn invest(
        &mut self,
        amount: T::AssetAmount,
        investor: &T::AccountId
    ) -> Result<Purchase<'_, T, Self>, crate::Error<T>>
    {
        ensure!(!self.hard_cap_reached(), crate::Error::ImpossibleSituation);

        let maybe_total = self.fund_amount().saturating_add(amount);

        let accepted_amount = if &maybe_total > self.hard_cap()
        {
            self.v1.total_amount.0 = *self.hard_cap();
            self.hard_cap().saturating_sub(*self.fund_amount())
        }
        else {
            self.v1.total_amount.0.saturating_accrue(amount);
            amount
        };

        self.payouts.saturating_accrue(self.shares());

        Ok(Purchase::<T, Self> {
            asset: self.fund(investor, accepted_amount)?,
            cf: self,
            amount: accepted_amount
        })
    }

    fn increase_investment(
        &mut self,
        amount: T::AssetAmount,
        investor: &T::AccountId
    ) -> Result<Purchase<'_, T, Self>, crate::Error<T>>
    {
        ensure!(!self.no_payouts(), crate::Error::ImpossibleSituation);
        self.payouts.saturating_reduce(self.shares());
        self.invest(amount, investor)
    }

    fn payout<'a>(
        &mut self,
        investment: &'a mut Self::Investment,
        shares: (T::AssetId, T::AssetAmount),
    ) -> Result<Payout<'a, T, Self::Investment>, crate::Error<T>>
    {
        ensure!(!self.no_payouts(), crate::Error::ImpossibleSituation);

        let amount = Payout::<T, Self::Investment>::amount(
            self,
            investment,
            &shares
        );

        let (id, _) = shares;
        let asset = T::Asset::pick_fraction(
            id,
            self.account(),
        ).ok_or_else(|| crate::Error::ImpossibleSituation)?;

        self.payouts.saturating_dec();

        Ok(Payout { asset, investment, amount: amount.calc() })
    }

    fn no_payouts(&self) -> bool {
        self.payouts() == 0
    }
}

pub struct SharesTransfer<'a, T: Config> {
    asset: T::Asset,
    to: &'a T::AccountId,
    amount: T::AssetAmount
}

impl<'a, T: Config> SharesTransfer<'a, T> {
    fn new(asset: T::Asset, to: &'a T::AccountId, amount: T::AssetAmount) -> Self {
        Self { asset, to, amount }
    }
    pub fn transfer(self) {
        self.asset.transfer_amount(self.to, self.amount);
    }
}

pub struct Payout<'a, T: Config, I: InvestmentT<T>> {
    asset: T::Asset,
    investment: &'a mut I,
    amount: T::AssetAmount
}

impl<'a, T: Config, I: InvestmentT<T>> Payout<'a, T, I>
{
    pub fn amount(
        cf: &impl CrowdfundingT<T>,
        investment: &'a I,
        shares: &(T::AssetId, T::AssetAmount)
    ) -> PayoutAmount<T::AssetAmount>
    {
        PayoutAmount {
            investment: *investment.amount(),
            shares: shares.1,
            fund: *cf.fund_amount()
        }
    }

    pub fn payout(self) {
        let Self { asset: unit, investment, amount } = self;
        unit.transfer_amount(investment.investor(), amount);
        investment.payout();
        if investment.no_payouts() {
            frame_system::Pallet::<T>::dec_consumers(investment.investor());
        }
    }
}

pub struct PayoutAmount<Amount> {
    pub investment: Amount,
    pub shares: Amount,
    pub fund: Amount
}

impl<Amount: FixedPointOperand + One> PayoutAmount<Amount>
{
    pub fn calc(self) -> Amount
    {
        // [ investment_amount / x = fund_amount / share_amount ]
        // [ x = investment_amount * share_amount / fund_amount ]
        let Self { investment, shares, fund } = self;
        FixedU128::checked_from_rational(
            investment.saturating_mul(shares),
            fund
        ).unwrap_or_default().saturating_mul_int(Amount::one())
    }
}

pub struct Purchase<'a, T: Config, CF: CrowdfundingT<T>> {
    asset: T::Asset,
    amount: T::AssetAmount,
    cf: &'a CF
}

impl<T: Config, CF: CrowdfundingT<T>> Purchase<'_, T, CF> {

    pub fn accepted_amount(&self) -> &T::AssetAmount {
        &self.amount
    }

    pub fn investor(&self) -> &T::AccountId {
        self.asset.account()
    }

    pub fn invest(self, time: T::Moment) -> CF::Investment {
        let investment = CF::Investment::new(
            self.cf,
            self.investor().clone(),
            self.amount,
            time
        );
        // If the account executes the extrinsic then it exists, so it should have at least one provider
        // so this cannot fail... but being defensive anyway.
        let _ = frame_system::Pallet::<T>::inc_consumers(self.investor());
        self.asset.transfer_amount(self.cf.account(), self.amount);
        investment
    }

    pub fn increase_investment(self, investment: &mut CF::Investment) {
        investment.increase_amount(*self.accepted_amount());
        self.asset.transfer_amount(self.cf.account(), self.amount);
    }
}

pub type Investment<T: crate::Config> = <T::Crowdfunding as CrowdfundingT<T>>::Investment;

pub trait CrowdfundingT<T: crate::Config>: Sized
{
    type Investment: InvestmentT<T> + Parameter;

    fn new(
        ctx: T::TransactionCtx,
        creator: T::AccountId,
        account: T::AccountId,
        external_id: CrowdfundingId,
        fund: T::AssetId,
    ) -> Self;

    // Getters:

    fn ctx(&self) -> &TransactionCtxId<T::TransactionCtx>;

    fn id(&self) -> &CrowdfundingId;

    fn creator(&self) -> &T::AccountId;

    fn account(&self) -> &T::AccountId;

    fn fund(
        &self,
        account: &T::AccountId,
        amount: T::AssetAmount
    ) -> Result<T::Asset, crate::Error<T>>;

    fn fund_balance(&self) -> Result<T::Asset, crate::Error<T>>;

    fn start_time(&self) -> &T::Moment;

    fn end_time(&self) -> &T::Moment;

    fn soft_cap(&self) -> &T::AssetAmount;

    fn hard_cap(&self) -> &T::AssetAmount;

    fn shares(&self) -> u16;

    fn status(&self) -> CrowdfundingStatus;

    fn fund_amount(&self) -> &T::AssetAmount;

    fn payouts(&self) -> u16;

    // Setters:

    fn set_status(&mut self, status: CrowdfundingStatus);

    // Mutation invariants:

    fn commit_shares(
        &mut self,
        shares: (T::AssetId, T::AssetAmount)
    ) -> Result<SharesTransfer<T>, crate::Error<T>>;

    fn rollback_shares(
        &mut self,
        shares: (T::AssetId, T::AssetAmount)
    ) -> Result<SharesTransfer<T>, crate::Error<T>>;

    fn ready(
        &mut self,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: T::AssetAmount,
        hard_cap: T::AssetAmount
    ) -> Result<(), crate::Error<T>>;

    fn invest(
        &mut self,
        amount: T::AssetAmount,
        investor: &T::AccountId
    ) -> Result<Purchase<'_, T, Self>, crate::Error<T>>;

    fn increase_investment(
        &mut self,
        amount: T::AssetAmount,
        investor: &T::AccountId
    ) -> Result<Purchase<'_, T, Self>, crate::Error<T>>;

    fn payout<'a>(
        &mut self,
        investment: &'a mut Self::Investment,
        shares: (T::AssetId, T::AssetAmount),
    ) -> Result<Payout<'a, T, Self::Investment>, crate::Error<T>>;

    // Invariants:

    fn is_creator(&self, x: &T::AccountId) -> Result<(), crate::Error<T>> {
        Ok(ensure!(self.creator() == x, crate::Error::NoPermission))
    }

    fn expired(&self, now: T::Moment) -> Result<(), crate::Error<T>> {
        Ok(ensure!(
            *self.end_time() <= now,
            crate::Error::ImpossibleSituation
        ))
    }

    fn started(&self, now: T::Moment) -> Result<(), crate::Error<T>> {
        Ok(ensure!(
            *self.start_time() <= now,
            crate::Error::ImpossibleSituation
        ))
    }

    fn hard_cap_reached(&self) -> bool {
        self.fund_amount() == self.hard_cap()
    }

    fn soft_cap_reached(&self) -> bool {
        self.fund_amount() >= self.soft_cap()
    }

    fn no_shares(&self) -> bool {
        self.shares() == 0
    }

    fn no_payouts(&self) -> bool;
}

use frame_support::storage::{IterableStorageMap, StorageMap};
use deip_asset_system::{NFTokenFractionT};

impl<T: Config> StateTransitionT<T> for CrowdfundingStatus {
    type IncompleteR = IncompleteRepo<T>;
    type ReadyR = ReadyRepo<T>;
    type ActiveR = ActiveRepo<T>;
    type PayoutR = PayoutRepo<T>;
    type RaiseR = RaiseRepo<T>;
    type RefundR = RefundRepo<T>;
    type ReleaseSharesR = ReleaseSharesRepo<T>;

    fn transit(cf: T::Crowdfunding)
    {
        match cf.status() {
            // Incomplete -> Ready -> Active
            Self::Incomplete => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::IncompleteR::insert(cf);
            },
            Self::Ready => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::IncompleteR::remove(&cf);
                Self::ReadyR::insert(cf);
            },
            Self::Active => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::ReadyR::remove(&cf);
                Self::ActiveR::insert(cf);
            },
            // Active -> Payout -> Raise -> RaiseDone
            Self::Payout => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::ActiveR::remove(&cf);
                Self::PayoutR::insert(cf);
            },
            Self::Raise => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::PayoutR::remove(&cf);
                Self::RaiseR::insert(cf);
            },
            Self::RaiseDone => {
                CrowdfundingStatusV2::<T>::remove(*cf.id());
                Self::RaiseR::remove(&cf);
            },
            // Active -> Refund -> ReleaseShares -> RefundDone
            Self::Refund => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::ActiveR::remove(&cf);
                Self::RefundR::insert(cf);
            },
            Self::ReleaseShares => {
                CrowdfundingStatusV2::<T>::insert(*cf.id(), cf.status());
                Self::RefundR::remove(&cf);
                Self::ReleaseSharesR::insert(cf);
            },
            Self::RefundDone => {
                CrowdfundingStatusV2::<T>::remove(*cf.id());
                Self::ReleaseSharesR::remove(&cf);
            },
        }
    }
}

pub(crate) trait StateTransitionT<T: Config> {
    // Incomplete -> Ready -> Active
    type IncompleteR:    RepositoryT<T>;
    type ReadyR:         RepositoryT<T>;
    type ActiveR:        RepositoryT<T>;
    // Active -> Payout -> Raise
    type PayoutR:        RepositoryT<T>;
    type RaiseR:         RepositoryT<T>;
    // Active -> Refund -> ReleaseShares
    type RefundR:        RepositoryT<T>;
    type ReleaseSharesR: RepositoryT<T>;

    fn transit(cf: T::Crowdfunding);

    fn status(id: CrowdfundingId) -> Result<CrowdfundingStatus, crate::Error<T>> {
        CrowdfundingStatusV2::<T>::try_get(id)
            .map_err(|_| crate::Error::NotFound)
    }

    fn not_exist(id: CrowdfundingId) -> Result<(), crate::Error<T>> {
        Ok(ensure!(
            Self::status(id).is_err(),
            crate::Error::AlreadyExists
        ))
    }
}

 #[allow(type_alias_bounds)]
type I<T: Config, S> = <S as IterableStorageMap<CrowdfundingId, T::Crowdfunding>>::Iterator;

pub trait RepositoryT<T: crate::Config>
{
    type S: IterableStorageMap<CrowdfundingId, T::Crowdfunding>;

    fn insert(cf: T::Crowdfunding) {
        Self::S::insert(*cf.id(), cf);
    }

    fn find(id: CrowdfundingId) -> Result<T::Crowdfunding, crate::Error<T>> {
        Self::S::try_get(id)
            .map_err(|_| crate::Error::NotFound)
    }

    fn remove(cf: &T::Crowdfunding) {
        Self::S::remove(*cf.id());
    }

    fn filter<F: Fn(&<I<T, Self::S> as Iterator>::Item) -> bool>(
        f: F
    ) -> core::iter::Filter<I<T, Self::S>, F>
    {
        Self::S::iter().filter(f)
    }

    fn insert_shares(
        cf: &T::Crowdfunding,
        shares: (T::AssetId, T::AssetAmount)
    ) {
        let (id, amount) = shares;
        SharesMapV2::<T>::insert(*cf.id(), id, amount);
    }

    fn find_shares(
        cf: &T::Crowdfunding,
        shares: T::AssetId
    ) -> Result<(T::AssetId, T::AssetAmount), crate::Error<T>>
    {
        let amount = SharesMapV2::<T>::try_get(*cf.id(), shares)
            .map_err(|_| crate::Error::NotFound)?;
        Ok((shares, amount))
    }

    fn take_shares(
        cf: &T::Crowdfunding,
        shares: T::AssetId
    ) -> Result<(T::AssetId, T::AssetAmount), crate::Error<T>>
    {
        let amount = SharesMapV2::<T>::take(*cf.id(), shares)
            .ok_or_else(|| crate::Error::NotFound)?;
        Ok((shares, amount))
    }

    fn insert_investment(
        cf: &T::Crowdfunding,
        investment: Investment<T>
    )
    {
        InvestmentMapV2::<T>::insert(
            *cf.id(),
            investment.investor().clone(),
            investment
        );
    }

    fn find_investment(
        cf: &T::Crowdfunding,
        investor: T::AccountId
    ) -> Result<Investment<T>, crate::Error<T>>
    {
        InvestmentMapV2::<T>::try_get(*cf.id(), investor)
            .map_err(|_| crate::Error::NotFound)
    }

    fn pop_investment(
        cf: &T::Crowdfunding,
    ) -> Result<Investment<T>, crate::Error<T>>
    {
        InvestmentMapV2::<T>::drain_prefix(*cf.id())
            .next()
            .map(|(_, x)| x)
            .ok_or_else(|| crate::Error::NotFound)
    }

    fn remove_investment(
        cf: &T::Crowdfunding,
        investor: T::AccountId
    )
    {
        InvestmentMapV2::<T>::remove(*cf.id(), investor);
    }

    fn list_investments(
        cf: &T::Crowdfunding,
    ) -> Vec<Investment<T>>
    {
        InvestmentMapV2::<T>::iter_prefix(*cf.id())
            .map(|(_, x)| x)
            .collect()
    }

    fn clear_investments(
        cf: &T::Crowdfunding,
    ) -> Vec<Investment<T>>
    {
        InvestmentMapV2::<T>::drain_prefix(*cf.id())
            .map(|(_, x)| x)
            .collect()
    }

    fn has_investments(
        cf: &T::Crowdfunding,
    ) -> bool
    {
        InvestmentMapV2::<T>::iter_prefix(*cf.id())
            .take(1)
            .next()
            .is_some()
    }

    fn insert_payout(
        cf: &T::Crowdfunding,
        investment: &Investment<T>,
        shares: T::AssetId
    )
    {
        PayoutMapV2::<T>::insert(
            (
                *cf.id(),
                investment.investor().clone(),
                shares
            ),
            ()
        );
    }

    fn not_exist_payout(
        cf: &T::Crowdfunding,
        investment: &Investment<T>,
        shares: T::AssetId
    ) -> Result<(), crate::Error<T>>
    {
        let exist = PayoutMapV2::<T>::contains_key(
            (
                *cf.id(),
                investment.investor().clone(),
                shares
            ),
        );
        if exist {
            return Err(crate::Error::AlreadyExists)
        }
        Ok(())
    }

    fn clear_payouts(
        cf: &T::Crowdfunding,
        investment: &Investment<T>
    )
    {
        PayoutMapV2::<T>::remove_prefix(
            (
                *cf.id(),
                investment.investor().clone(),
            ),
            None
        );
    }
}

impl<T: crate::Config> InvestmentT<T>
    for InvestmentV2<T::AccountId, T::AssetAmount, T::Moment>
{
    fn new(
        cf: &impl CrowdfundingT<T>,
        investor: T::AccountId,
        amount: T::AssetAmount,
        time: T::Moment
    ) -> Self
    {
        Self {
            v1: Contribution {
                sale_id: *cf.id(),
                owner: investor,
                amount,
                time
            },
            payouts: cf.shares()
        }
    }

    fn investor(&self) -> &T::AccountId {
        &self.v1.owner
    }

    fn amount(&self) -> &T::AssetAmount {
        &self.v1.amount
    }

    fn time(&self) -> &T::Moment {
        &self.v1.time
    }

    fn no_payouts(&self) -> bool {
        self.payouts == 0
    }

    fn increase_amount(&mut self, by: T::AssetAmount) {
        self.v1.amount.saturating_accrue(by);
    }

    fn payout(&mut self) {
        self.payouts.saturating_dec();
    }
}

pub trait InvestmentT<T: crate::Config>: Sized
{
    // Constructors:

    fn new(
        cf: &impl CrowdfundingT<T>,
        investor: T::AccountId,
        amount: T::AssetAmount,
        time: T::Moment
    ) -> Self;

    // Getters:

    fn investor(&self) -> &T::AccountId;

    fn amount(&self) -> &T::AssetAmount;

    fn time(&self) -> &T::Moment;

    fn no_payouts(&self) -> bool;

    // Mutators:

    fn increase_amount(&mut self, by: T::AssetAmount);

    fn payout(&mut self);
}

/// Unique Crowdfunding ID reference
pub type CrowdfundingId = sp_core::H160;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum CrowdfundingStatus {
    // Incomplete -> Ready -> Active
    Incomplete,
    Ready,
    Active,
    // Active -> Payout -> Raise -> RaiseDone
    Payout,
    Raise,
    RaiseDone,
    // Active -> Refund -> ReleaseShares -> RefundDone
    Refund,
    ReleaseShares,
    RefundDone,
}

impl Default for CrowdfundingStatus {
    fn default() -> Self {
        CrowdfundingStatus::Incomplete
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct SimpleFundingModel<Moment, AssetId, Balance> {
    /// a moment when the crowdfunding starts. Must be later than current moment.
    pub start_time: Moment,
    /// a moment when the crowdfunding ends. Must be later than `start_time`.
    pub end_time: Moment,
    /// An identifier of a fund currency.
    pub fund: AssetId,
    /// amount of units to raise.
    pub soft_cap: Balance,
    /// amount upper limit of units to raise. Must be greater or equal to `soft_cap`.
    pub hard_cap: Balance,
}

pub fn fund_account<T: Config>(id: &[u8]) -> T::AccountId {
    let entropy =
        (b"deip/investments/", id).using_encoded(sp_io::hashing::blake2_256);
    T::AccountId::decode(&mut &entropy[..]).unwrap_or_default()
}

/// The object represents a sale of tokens with various parameters.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct SimpleCrowdfundingV2<T: crate::Config> {
    v1: SimpleCrowdfunding<
        T::Moment,
        T::AssetId,
        T::AssetAmount,
        TransactionCtxId<T::TransactionCtx>
    >,
    creator: T::AccountId,
    account: T::AccountId,
    shares: u16,
    payouts: u16,
}

/// The object represents a sale of tokens with various parameters.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct SimpleCrowdfunding<Moment, AssetId, AssetBalance: Clone + AtLeast32BitUnsigned, CtxId> {
    pub created_ctx: CtxId,
    /// Reference for external world and uniques control
    pub external_id: CrowdfundingId,
    /// When the sale starts
    pub start_time: Moment,
    /// When it supposed to end
    pub end_time: Moment,
    pub status: CrowdfundingStatus,
    pub asset_id: AssetId,
    /// How many contributions already reserved
    pub total_amount: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub soft_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub hard_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    /// How many and what tokens supposed to sale
    pub shares: Vec<Asset<AssetId, AssetBalance>>,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct InvestmentV2<Account, Amount, Time> {
    v1: Contribution<Account, Amount, Time>,
    payouts: u16,
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Contribution<AccountId, Balance, Moment> {
    pub sale_id: CrowdfundingId,
    pub owner: AccountId,
    pub amount: Balance,
    pub time: Moment,
}
