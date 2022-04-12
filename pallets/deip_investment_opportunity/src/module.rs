#![allow(type_alias_bounds)]

use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Saturating, Zero},
    SaturatedConversion,
};

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use deip_transaction_ctx::{TransactionCtxT, PortalCtxT, TransactionCtxId};

use codec::{Encode, Decode};
#[cfg(feature = "std")]
use serde::{self, Serialize, Deserialize};
use frame_support::{ensure, RuntimeDebug};
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
use frame_support::log::{debug};
use frame_support::traits::{Get};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use crate::{Config, Error, Event, Call, Pallet};
use deip_asset_system::{DeipAssetSystem, ReserveError, UnreserveError};
pub use deip_asset_system::investment_opportunity::*;
pub use deip_asset_system::asset::*;
use crate::{SimpleCrowdfundingMapV1, InvestmentMapV1};
use crate::weights::WeightInfo;

pub type DeipAssetId<T: Config> =
    <T as DeipAssetSystem<T::AccountId, T::SourceId, InvestmentId>>::AssetId;

pub type DeipAssetBalance<T: Config> =
    <T as DeipAssetSystem<T::AccountId, T::SourceId, InvestmentId>>::Balance;

pub type DeipAsset<T: Config> = Asset<DeipAssetId<T>, DeipAssetBalance<T>>;

pub type FundingModelOf<T: Config> = FundingModel<T::Moment, DeipAsset<T>>;

pub type SimpleCrowdfundingOf<T: Config> = SimpleCrowdfunding<
    T::Moment,
    DeipAssetId<T>,
    DeipAssetBalance<T>,
    TransactionCtxId<<T as Config>::TransactionCtx>,
>;

pub type Investment<T: Config> = Contribution<
    T::AccountId,
    DeipAssetBalance<T>,
    T::Moment
>;

impl<T: Config> Pallet<T> {
    pub(super) fn create_investment_opportunity_impl(
        account: T::AccountId,
        external_id: InvestmentId,
        creator: T::AccountId,
        shares: Vec<DeipAsset<T>>,
        funding_model: FundingModelOf<T>,
    ) -> DispatchResult {
        ensure!(account == creator, Error::<T>::NoPermission);
        ensure!(
            shares.len() <= T::MaxInvestmentShares::get() as usize,
            Error::<T>::TooMuchShares
        );

        match funding_model {
            FundingModel::SimpleCrowdfunding { start_time, end_time, soft_cap, hard_cap } =>
                Self::create_simple_crowdfunding(
                    account,
                    external_id,
                    start_time,
                    end_time,
                    soft_cap,
                    hard_cap,
                    shares,
                ),
        }
    }

    pub(super) fn create_simple_crowdfunding(
        account: T::AccountId,
        external_id: InvestmentId,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: DeipAsset<T>,
        hard_cap: DeipAsset<T>,
        shares: Vec<DeipAsset<T>>,
    ) -> DispatchResult {
        let timestamp = pallet_timestamp::Pallet::<T>::get();
        ensure!(
            start_time >= timestamp,
            Error::<T>::StartTimeMustBeLaterOrEqualCurrentMoment
        );
        ensure!(
            end_time > start_time,
            Error::<T>::EndTimeMustBeLaterStartTime
        );

        let asset_id = soft_cap.id();
        ensure!(asset_id == hard_cap.id(), Error::<T>::CapDifferentAssets);
        ensure!(
            soft_cap.amount() > &Zero::zero(),
            Error::<T>::SoftCapMustBeGreaterOrEqualMinimum
        );
        ensure!(
            hard_cap.amount() >= soft_cap.amount(),
            Error::<T>::HardCapShouldBeGreaterOrEqualSoftCap
        );

        ensure!(!shares.is_empty(), Error::<T>::SecurityTokenNotSpecified);
        let mut shares_to_reserve = Vec::with_capacity(shares.len());
        for token in &shares {
            ensure!(token.id() != asset_id, Error::<T>::WrongAssetId);

            ensure!(
                token.amount() > &Zero::zero(),
                Error::<T>::AssetAmountMustBePositive
            );

            shares_to_reserve.push((*token.id(), *token.amount()));
        }

        ensure!(
            !SimpleCrowdfundingMapV1::<T>::contains_key(external_id),
            Error::<T>::AlreadyExists
        );

        if let Err(e) = T::transactionally_reserve(
            &account,
            external_id,
            &shares_to_reserve,
            *asset_id,
        ) {
            match e {
                ReserveError::<DeipAssetId<T>>::NotEnoughBalance =>
                    return Err(Error::<T>::BalanceIsNotEnough.into()),
                ReserveError::<DeipAssetId<T>>::AssetTransferFailed(_) =>
                    return Err(Error::<T>::FailedToReserveAsset.into()),
                ReserveError::<DeipAssetId<T>>::AlreadyReserved =>
                    return Err(Error::<T>::AlreadyExists.into()),
            };
        }

        let new_token_sale = SimpleCrowdfunding {
            created_ctx: T::TransactionCtx::current().id(),
            external_id,
            start_time,
            end_time,
            asset_id: *asset_id,
            soft_cap: SerializableAtLeast32BitUnsigned(soft_cap.amount().clone()),
            hard_cap: SerializableAtLeast32BitUnsigned(hard_cap.amount().clone()),
            shares,
            ..Default::default()
        };

        SimpleCrowdfundingMapV1::<T>::insert(external_id, new_token_sale);

        Self::deposit_event(Event::<T>::SimpleCrowdfundingCreated(external_id));

        Ok(())
    }

    pub(super) fn collect_funds(sale_id: InvestmentId, amount: DeipAssetBalance<T>) -> Result<(), ()> {
        SimpleCrowdfundingMapV1::<T>::mutate_exists(sale_id, |sale| -> Result<(), ()> {
            match sale.as_mut() {
                Some(s) => s.total_amount.0 = amount.saturating_add(s.total_amount.0),
                None => return Err(()),
            }
            Ok(())
        })
    }

    pub(super) fn finish_crowdfunding_by_id(sale_id: InvestmentId) -> Result<(), ()> {
        match SimpleCrowdfundingMapV1::<T>::try_get(sale_id) {
            Err(_) => Err(()),
            Ok(sale) => {
                Self::update_status(&sale, SimpleCrowdfundingStatus::Finished);
                Self::process_investments(&sale);
                Ok(())
            },
        }
    }

    pub(super) fn activate_crowdfunding_impl(sale_id: InvestmentId) -> DispatchResult {
        SimpleCrowdfundingMapV1::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::NotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                SimpleCrowdfundingStatus::Active => return Ok(()),
                SimpleCrowdfundingStatus::Inactive => ensure!(
                    pallet_timestamp::Pallet::<T>::get() >= sale.start_time,
                    Error::<T>::ShouldBeStarted
                ),
                _ => return Err(Error::<T>::ShouldBeInactive.into()),
            };

            sale.status = SimpleCrowdfundingStatus::Active;
            Self::deposit_event(Event::SimpleCrowdfundingActivated(sale_id));

            Ok(())
        })
    }

    pub(super) fn expire_crowdfunding_impl(sale_id: InvestmentId) -> DispatchResultWithPostInfo {
        SimpleCrowdfundingMapV1::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResultWithPostInfo {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::NotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                SimpleCrowdfundingStatus::Expired => return Ok(Some(
                    T::DeipInvestmentWeightInfo::expire_crowdfunding_already_expired()).into()),
                SimpleCrowdfundingStatus::Active => ensure!(
                    pallet_timestamp::Pallet::<T>::get() >= sale.end_time,
                    Error::<T>::ExpirationWrongState
                ),
                _ => return Err(Error::<T>::ShouldBeActive.into()),
            };

            sale.status = SimpleCrowdfundingStatus::Expired;

            Self::refund(sale);

            Ok(None.into())
        })
    }

    pub(super) fn finish_crowdfunding_impl(sale_id: InvestmentId) -> DispatchResult {
        SimpleCrowdfundingMapV1::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::NotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                SimpleCrowdfundingStatus::Finished => return Ok(()),
                SimpleCrowdfundingStatus::Active => (),
                _ => return Err(Error::<T>::ShouldBeActive.into()),
            };

            sale.status = SimpleCrowdfundingStatus::Finished;

            Self::process_investments(sale);

            Ok(())
        })
    }

    pub(super) fn process_investment_opportunities_offchain() {
        let now = pallet_timestamp::Pallet::<T>::get();
        for (id, sale) in SimpleCrowdfundingMapV1::<T>::iter() {
            if sale.end_time <= now && matches!(sale.status, SimpleCrowdfundingStatus::Active) {
                if sale.total_amount.0 < sale.soft_cap.0 {
                    let call = Call::<T>::expire_crowdfunding { sale_id: id };
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);

                    debug!("submit expire_crowdfunding: {}", submit.is_ok());
                } else if sale.total_amount.0 >= sale.soft_cap.0 {
                    let call = Call::<T>::finish_crowdfunding { sale_id: id };
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    debug!("submit finish_crowdfunding: {}", submit.is_ok());
                }
            } else if sale.end_time > now {
                if now >= sale.start_time && matches!(sale.status, SimpleCrowdfundingStatus::Inactive) {
                    let call = Call::<T>::activate_crowdfunding { sale_id: id };
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    debug!("submit activate_crowdfunding: {}", submit.is_ok());
                }
            }
        }
    }

    fn update_status(sale: &SimpleCrowdfundingOf<T>, new_status: SimpleCrowdfundingStatus) {
        SimpleCrowdfundingMapV1::<T>::mutate_exists(sale.external_id, |maybe_sale| -> () {
            let sale = maybe_sale.as_mut().expect("we keep collections in sync");
            sale.status = new_status;
        });
    }

    fn refund(sale: &SimpleCrowdfundingOf<T>) {
        if let Ok(ref c) = InvestmentMapV1::<T>::try_get(sale.external_id) {
            for (_, ref contribution) in c {
                T::transfer_from_reserved(
                    sale.external_id,
                    &contribution.owner,
                    sale.asset_id,
                    contribution.amount,
                )
                .unwrap_or_else(|_| panic!("user's asset should be reserved earlier"));

                frame_system::Pallet::<T>::dec_consumers(&contribution.owner);
            }
            InvestmentMapV1::<T>::remove(sale.external_id);
        }

        T::transactionally_unreserve(sale.external_id)
            .unwrap_or_else(|_| panic!("assets should be reserved earlier"));

        Self::deposit_event(Event::SimpleCrowdfundingExpired(sale.external_id));
    }

    fn process_investments(sale: &SimpleCrowdfundingOf<T>) {
        let contributions = InvestmentMapV1::<T>::try_get(sale.external_id)
            .expect("about to finish, but there are no contributions?");

        for asset in &sale.shares {
            let mut amount = asset.amount().clone();

            let mut iter = contributions.iter();
            let (_, ref first_contribution) =
                iter.next().expect("about to finish, but there are no contributors?");

            for (_, ref contribution) in iter {
                // similiar to frame_support::traits::Imbalance::ration
                let token_amount = contribution
                    .amount
                    .saturated_into::<u128>()
                    .saturating_mul(asset.amount().clone().saturated_into()) /
                    sale.total_amount.0.saturated_into::<u128>();
                let token_amount: DeipAssetBalance<T> = token_amount.saturated_into();
                if token_amount.is_zero() {
                    continue
                }

                amount -= token_amount;

                T::transfer_from_reserved(
                    sale.external_id,
                    &contribution.owner,
                    *asset.id(),
                    token_amount,
                )
                .unwrap_or_else(|_| panic!("Required token_amount should be reserved"));
            }

            if !amount.is_zero() {
                T::transfer_from_reserved(
                    sale.external_id,
                    &first_contribution.owner,
                    *asset.id(),
                    amount,
                )
                .unwrap_or_else(|_| panic!("Required token_amount should be reserved"));
            }
        }

        T::transactionally_unreserve(sale.external_id)
            .unwrap_or_else(|_| panic!("remaining assets should be reserved earlier"));

        for (_, ref contribution) in contributions {
            frame_system::Pallet::<T>::dec_consumers(&contribution.owner);
        }
        InvestmentMapV1::<T>::remove(sale.external_id);

        Self::deposit_event(Event::SimpleCrowdfundingFinished(sale.external_id));
    }

    pub(super) fn invest_to_crowdfunding_impl(
        account: T::AccountId,
        sale_id: InvestmentId,
        asset: DeipAsset<T>,
    ) -> DispatchResultWithPostInfo {
        let sale = SimpleCrowdfundingMapV1::<T>::try_get(sale_id)
            .map_err(|_| Error::<T>::InvestingNotFound)?;

        ensure!(
            matches!(sale.status, SimpleCrowdfundingStatus::Active),
            Error::<T>::InvestingNotActive
        );

        ensure!(sale.asset_id == *asset.id(), Error::<T>::InvestingWrongAsset);

        let is_hard_cap_reached =
            sale.total_amount.0.saturating_add(*asset.amount()) >= sale.hard_cap.0;
        let amount_to_contribute = if is_hard_cap_reached {
            sale.hard_cap.0.saturating_sub(sale.total_amount.0)
        } else {
            *asset.amount()
        };

        ensure!(
            T::transfer_to_reserved(&account, sale.external_id, amount_to_contribute)
                .is_ok(),
            Error::<T>::InvestingNotEnoughFunds
        );

        InvestmentMapV1::<T>::mutate_exists(sale_id, |contributions| {
            let mut_contributions = match contributions.as_mut() {
                None => {
                    // If the account executes the extrinsic then it exists, so it should have at least one provider
                    // so this cannot fail... but being defensive anyway.
                    let _ = frame_system::Pallet::<T>::inc_consumers(&account);

                    *contributions = Some(vec![(
                        account.clone(),
                        Contribution {
                            sale_id,
                            owner: account.clone(),
                            amount: amount_to_contribute,
                            time: pallet_timestamp::Pallet::<T>::get(),
                        },
                    )]);
                    return
                },
                Some(c) => c,
            };

            match mut_contributions.binary_search_by_key(&&account, |&(ref a, _)| a) {
                Err(i) => {
                    // see comment above
                    let _ = frame_system::Pallet::<T>::inc_consumers(&account);

                    mut_contributions.insert(
                        i,
                        (
                            account.clone(),
                            Contribution {
                                sale_id,
                                owner: account.clone(),
                                amount: amount_to_contribute,
                                time: pallet_timestamp::Pallet::<T>::get(),
                            },
                        ),
                    );
                },
                Ok(i) => {
                    mut_contributions[i].1.amount =
                        amount_to_contribute.saturating_add(mut_contributions[i].1.amount);
                },
            };
        });

        Self::collect_funds(sale_id, amount_to_contribute).expect("collect; already found");

        Self::deposit_event(Event::<T>::Invested(sale_id, account.clone()));

        if is_hard_cap_reached {
            Self::finish_crowdfunding_by_id(sale_id).expect("finish; already found");
            return Ok(Some(T::DeipInvestmentWeightInfo::invest_hard_cap_reached()).into())
        }

        Ok(Some(T::DeipInvestmentWeightInfo::invest()).into())
    }
}
