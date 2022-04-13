#![allow(type_alias_bounds)]

use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Saturating, Zero, One},
    SaturatedConversion,
    DispatchError,
    FixedPointNumber, FixedPointOperand, FixedU128
};

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use deip_transaction_ctx::{TransactionCtxT, PortalCtxT, TransactionCtxId};

use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{self, Serialize, Deserialize};
use frame_support::{ensure, IterableStorageMap, RuntimeDebug};
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
use frame_support::log::{debug};
use frame_support::traits::{Get, fungibles::Inspect};
use frame_support::traits::{Currency, ReservableCurrency, WithdrawReasons, ExistenceRequirement};
use frame_support::traits::tokens::Balance;
use scale_info::TypeInfo;
use sp_core::H160;
use sp_std::prelude::*;
use crate::{Config, Error, Event, Call, Pallet};
use deip_asset_system::{ReserveError, UnreserveError};
pub use crate::crowdfunding::*;
pub use deip_asset_system::asset::*;
use crate::{
    SimpleCrowdfundingMapV1, InvestmentMapV1,
    ActiveRepo, IncompleteRepo, ReadyRepo, PayoutRepo, RefundRepo
};
use crate::weights::WeightInfo;


pub type SimpleCrowdfundingOf<T: Config> = SimpleCrowdfunding<
    T::Moment,
    T::FundAssetId,
    T::AssetAmount,
    TransactionCtxId<<T as Config>::TransactionCtx>,
>;

fn deposit_event<T: Config>(event: Event<T>) {
    Pallet::<T>::deposit_event(event);
}

impl<T: Config> CrowdfundingAccount<T> for T {}

pub(crate) trait CrowdfundingAccount<T: Config> {

    fn _create_account(cf: &T::Crowdfunding) -> Result<(), DispatchError>
    {
        let reserved = T::Currency::withdraw(
            cf.creator(),
            T::Currency::minimum_balance(),
            WithdrawReasons::RESERVE,
            ExistenceRequirement::AllowDeath,
        )?;
        T::Currency::resolve_creating(
            cf.account(),
            reserved
        );
        Ok(())
    }

    fn _destroy_account(cf: &T::Crowdfunding) {
        let deposited =
            T::Currency::deposit_creating(
                cf.creator(),
                T::Currency::minimum_balance()
            );
        T::Currency::settle(
            cf.account(),
            deposited,
            WithdrawReasons::TRANSFER,
            ExistenceRequirement::AllowDeath,
        ).unwrap_or_else(|_| panic!("should be reserved in transactionally_reserve"));
    }
}

impl<T: Config + CrowdfundingAccount<T>> ModuleT<T> for T {}

pub(crate) trait ModuleT<T: Config>: CrowdfundingAccount<T>
{
    fn create<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId,
        shares: (T::SharesAssetId, T::AssetAmount),
        fund: T::FundAssetId,
    ) -> DispatchResult
    {
        S::not_exist(id)?;

        let mut cf = T::Crowdfunding::new(
            T::TransactionCtx::current(),
            creator,
            fund_account::<T>(id.as_bytes()),
            id,
            fund,
        );
        Self::_create_account(&cf)
            .map_err(|_| Error::<T>::BalanceIsNotEnough)?;

        cf.commit_shares(shares)?.transfer();

        S::transit(cf.clone());

        deposit_event::<T>(Event::<T>::Created(cf));

        Ok(())
    }

    fn commit_shares<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId,
        shares: (T::SharesAssetId, T::AssetAmount)
    ) -> DispatchResult
    {
        let mut cf = S::IncompleteR::find(id)?;

        cf.is_creator(&creator)?;

        cf.commit_shares(shares)?.transfer();

        if let Ok(mut exist) = S::IncompleteR::find_shares(&cf, shares.0) {
            exist.1.saturating_accrue(shares.1);
            S::IncompleteR::insert_shares(&cf, exist);
        } else {
            S::IncompleteR::insert_shares(&cf, shares)
        }

        deposit_event::<T>(Event::<T>::CommitShares { id, shares });

        Ok(())
    }

    fn rollback_shares<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId,
        shares: T::SharesAssetId
    ) -> DispatchResult
    {
        let mut cf = S::IncompleteR::find(id)?;

        cf.is_creator(&creator)?;

        let shares = S::IncompleteR::take_shares(&cf, shares)?;

        cf.rollback_shares(shares)?.transfer();

        deposit_event::<T>(Event::RollbackShares { id, shares });

        Ok(())
    }

    fn ready<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId,
        start_time: Option<T::Moment>,
        end_time: T::Moment,
        soft_cap: T::AssetAmount,
        hard_cap: T::AssetAmount
    ) -> DispatchResult
    {
        let mut cf = S::IncompleteR::find(id)?;

        cf.is_creator(&creator)?;

        let now = pallet_timestamp::Pallet::<T>::get();
        let mut start_time = start_time.unwrap_or(now);
        if start_time < now {
            start_time = now;
        }

        cf.ready(
            start_time,
            end_time,
            soft_cap,
            hard_cap
        )?;

        cf.set_status(CrowdfundingStatus::Ready);
        S::transit(cf);

        Ok(())
    }

    fn activate<S: StateTransitionT<T>>(id: CrowdfundingId) -> DispatchResult
    {
        let mut cf = S::ReadyR::find(id)?;

        cf.started(pallet_timestamp::Pallet::<T>::get())?;

        cf.set_status(CrowdfundingStatus::Active);
        S::transit(cf);

        deposit_event::<T>(Event::Activated(id));

        Ok(())
    }

    fn invest<S: StateTransitionT<T>>(
        investor: T::AccountId,
        id: CrowdfundingId,
        amount: T::AssetAmount,
    ) -> DispatchResultWithPostInfo
    {
        let mut cf = S::ActiveR::find(id)?;

        if let Ok(mut investment) = S::ActiveR::find_investment(&cf, investor.clone())
        {
            let purchase = cf.increase_investment(amount, &investor)?;
            purchase.increase_investment(&mut investment);
            S::ActiveR::insert_investment(&cf, investment);
        }
        else {
            let purchase = cf.invest(amount, &investor)?;
            let investment = purchase.invest(pallet_timestamp::Pallet::<T>::get());
            S::ActiveR::insert_investment(&cf, investment);
        }

        deposit_event::<T>(Event::Invested(id, investor));

        let weight = if cf.hard_cap_reached() {
            cf.set_status(CrowdfundingStatus::Payout);
            S::transit(cf);
            T::DeipInvestmentWeightInfo::invest_hard_cap_reached()
        }
        else {
            S::ActiveR::insert(cf);
            T::DeipInvestmentWeightInfo::invest()
        };

        Ok(Some(weight).into())
    }

    fn payout<S: StateTransitionT<T>>(
        who: T::AccountId,
        investor: Option<T::AccountId>,
        id: CrowdfundingId,
        shares: T::SharesAssetId
    ) -> DispatchResult
    {
        let mut cf = S::PayoutR::find(id)?;

        let investor = if investor.is_some() {
            cf.is_creator(&who)?;
            investor.unwrap()
        } else {
            who
        };

        let mut investment = S::PayoutR::find_investment(&cf, investor.clone())?;

        S::PayoutR::not_exist_payout(&cf, &investment, shares)?;

        let shares = S::PayoutR::find_shares(&cf, shares)?;

        cf.payout(&mut investment, shares)?.payout();

        if investment.no_payouts() {
            S::PayoutR::remove_investment(&cf, investor.clone());
            S::PayoutR::clear_payouts(&cf, &investment);
        } else {
            S::PayoutR::insert_payout(&cf, &investment, shares.0);
            S::PayoutR::insert_investment(&cf, investment);
        }

        // deposit_event::<T>(Event::Payout(*cf.id(), investor));

        if cf.no_payouts() {
            cf.set_status(CrowdfundingStatus::Raise);
            deposit_event::<T>(Event::StatusUpdated(*cf.id(), cf.status()));
            S::transit(cf);
        } else {
            S::PayoutR::insert(cf);
        }

        Ok(())
    }

    fn raise<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId
    ) -> DispatchResult
    {
        let mut cf = S::RaiseR::find(id)?;

        cf.is_creator(&creator)?;

        cf.fund_balance()?.transfer(
            cf.creator(),
        );
        T::_destroy_account(&cf);

        cf.set_status(CrowdfundingStatus::RaiseDone);

        deposit_event::<T>(Event::StatusUpdated(*cf.id(), cf.status()));

        S::transit(cf);

        Ok(())
    }

    fn expire<S: StateTransitionT<T>>(id: CrowdfundingId) -> DispatchResultWithPostInfo
    {
        let mut cf = S::ActiveR::find(id)?;

        cf.expired(pallet_timestamp::Pallet::<T>::get())?;

        if cf.soft_cap_reached() {
            cf.set_status(CrowdfundingStatus::Payout);
            deposit_event::<T>(Event::Expired(id, cf.status()));
            S::transit(cf);
        }
        else {
            cf.set_status(CrowdfundingStatus::Refund);
            deposit_event::<T>(Event::Expired(id, cf.status()));
            S::transit(cf);
        }

        Ok(None.into())
    }

    fn refund<S: StateTransitionT<T>>(
        who: T::AccountId,
        investor: Option<T::AccountId>,
        id: CrowdfundingId,
    ) -> DispatchResult
    {
        let mut cf = S::RefundR::find(id)?;

        let investor = if investor.is_some() {
            cf.is_creator(&who)?;
            investor.unwrap()
        } else {
            who
        };

        let inv = S::RefundR::find_investment(&cf, investor.clone())?;

        cf.fund(cf.account(), *inv.amount())?.transfer(
            &investor,
        );
        deposit_event::<T>(Event::Refund(id, investor.clone()));

        frame_system::Pallet::<T>::dec_consumers(&investor);

        S::RefundR::remove_investment(&cf, investor);

        if !S::RefundR::has_investments(&cf) {
            cf.set_status(CrowdfundingStatus::ReleaseShares);
            deposit_event::<T>(Event::StatusUpdated(id, cf.status()));
            S::transit(cf);
        }

        Ok(())
    }

    fn release_shares<S: StateTransitionT<T>>(
        creator: T::AccountId,
        id: CrowdfundingId,
        shares: T::SharesAssetId
    ) -> DispatchResult
    {
        let mut cf = S::ReleaseSharesR::find(id)?;

        cf.is_creator(&creator)?;

        let shares = S::IncompleteR::find_shares(&cf, shares)?;

        cf.rollback_shares(shares)?.transfer();

        deposit_event::<T>(Event::RollbackShares { id, shares });

        if cf.no_shares() {
            cf.set_status(CrowdfundingStatus::RefundDone);
            deposit_event::<T>(Event::StatusUpdated(id, cf.status()));
            S::transit(cf);
        } else {
            S::ReleaseSharesR::insert(cf);
        }

        Ok(())
    }
}

impl<T: Config> Pallet<T> {

    pub(super) fn process_investment_opportunities_offchain() {

        for (_, ready) in ReadyRepo::<T>::filter(|_| true) {
            let submit = T::TransactionCtx::submit_postponed(
                Call::<T>::activate { id: *ready.id() }.into(),
                ready.ctx().clone()
            );
            debug!("submit expire_crowdfunding: {}", submit.is_ok());
        }

        let now = pallet_timestamp::Pallet::<T>::get();

        for (_, expired) in ActiveRepo::<T>::filter(|(_, cf)| { cf.expired(now).is_ok() }) {
            let submit = T::TransactionCtx::submit_postponed(
                Call::<T>::expire { id: *expired.id() }.into(),
                expired.ctx().clone()
            );
            debug!("submit expire_crowdfunding: {}", submit.is_ok());
        }

        for (id, sale) in SimpleCrowdfundingMapV1::<T>::iter() {
            if sale.end_time <= now && matches!(sale.status, CrowdfundingStatus::Active) {
                if sale.total_amount.0 < sale.soft_cap.0 {
                    let call = Call::<T>::expire { id };
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);

                    debug!("submit expire_crowdfunding: {}", submit.is_ok());
                } else if sale.total_amount.0 >= sale.soft_cap.0 {
                    // let call = Call::<T>::finish { id };
                    // let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    // debug!("submit finish_crowdfunding: {}", submit.is_ok());
                }
            } else if sale.end_time > now {
                if now >= sale.start_time && matches!(sale.status, CrowdfundingStatus::Ready) {
                    let call = Call::<T>::activate { id };
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    debug!("submit activate_crowdfunding: {}", submit.is_ok());
                }
            }
        }
    }
}
