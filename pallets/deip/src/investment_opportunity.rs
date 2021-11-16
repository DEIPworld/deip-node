use crate::traits::DeipAssetSystem;
use crate::*;
use deip_assets_error::*;

use sp_runtime::{
    traits::{Saturating, Zero, AtLeast32BitUnsigned},
    SaturatedConversion,
};

use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use deip_transaction_ctx::{TransactionCtxT};

/// Unique InvestmentOpportunity ID reference
pub type Id = H160;

/// Type alias to be specialized over Runtime type
#[allow(type_alias_bounds)]
pub type FundingModelOf<T: Config> = FundingModel<MomentOf<T>, DeipAssetOf<T>>;

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum Status {
    Active,
    Finished,
    Expired,
    Inactive,
}

impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum FundingModel<Moment, Asset> {
    SimpleCrowdfunding {
        /// a moment when the crowdfunding starts. Must be later than current moment.
        start_time: Moment,
        /// a moment when the crowdfunding ends. Must be later than `start_time`.
        end_time: Moment,
        /// amount of units to raise.
        soft_cap: Asset,
        /// amount upper limit of units to raise. Must be greater or equal to `soft_cap`.
        hard_cap: Asset,
    },
}

/// The object represents a sale of tokens with various parameters.
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct Info<Moment, AssetId, AssetBalance: Clone + AtLeast32BitUnsigned, CtxId> {
    #[cfg_attr(feature = "std", serde(skip))]
    pub created_ctx: CtxId,
    /// Reference for external world and uniques control
    pub external_id: Id,
    /// When the sale starts
    pub start_time: Moment,
    /// When it supposed to end
    pub end_time: Moment,
    pub status: Status,
    pub asset_id: AssetId,
    /// How many contributions already reserved
    pub total_amount: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub soft_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    pub hard_cap: SerializableAtLeast32BitUnsigned<AssetBalance>,
    /// How many and what tokens supposed to sale
    pub shares: Vec<DeipAsset<AssetId, AssetBalance>>,
}

impl<T: Config> Module<T> {
    pub(super) fn create_investment_opportunity_impl(
        account: AccountIdOf<T>,
        external_id: Id,
        creator: AccountIdOf<T>,
        shares: Vec<DeipAssetOf<T>>,
        funding_model: FundingModelOf<T>,
    ) -> DispatchResult {
        ensure!(account == creator, Error::<T>::NoPermission);

        match funding_model {
            FundingModel::SimpleCrowdfunding {
                start_time,
                end_time,
                soft_cap,
                hard_cap,
            } => Self::create_simple_crowdfunding(
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
        external_id: Id,
        start_time: T::Moment,
        end_time: T::Moment,
        soft_cap: DeipAssetOf<T>,
        hard_cap: DeipAssetOf<T>,
        shares: Vec<DeipAssetOf<T>>,
    ) -> DispatchResult {
        let timestamp = pallet_timestamp::Pallet::<T>::get();
        ensure!(
            start_time >= timestamp,
            Error::<T>::InvestmentOpportunityStartTimeMustBeLaterOrEqualCurrentMoment
        );
        ensure!(
            end_time > start_time,
            Error::<T>::InvestmentOpportunityEndTimeMustBeLaterStartTime
        );

        let asset_id = soft_cap.id();
        ensure!(
            asset_id == hard_cap.id(),
            Error::<T>::InvestmentOpportunityCapDifferentAssets
        );
        ensure!(
            soft_cap.amount() > &Zero::zero(),
            Error::<T>::InvestmentOpportunitySoftCapMustBeGreaterOrEqualMinimum
        );
        ensure!(
            hard_cap.amount() >= soft_cap.amount(),
            Error::<T>::InvestmentOpportunityHardCapShouldBeGreaterOrEqualSoftCap
        );

        ensure!(
            !shares.is_empty(),
            Error::<T>::InvestmentOpportunitySecurityTokenNotSpecified
        );
        let mut shares_to_reserve = Vec::with_capacity(shares.len());
        for token in &shares {
            ensure!(
                token.id() != asset_id,
                Error::<T>::InvestmentOpportunityWrongAssetId
            );

            ensure!(
                token.amount() > &Zero::zero(),
                Error::<T>::InvestmentOpportunityAssetAmountMustBePositive
            );

            shares_to_reserve.push((*token.id(), *token.amount()));
        }

        ensure!(
            !SimpleCrowdfundingMap::<T>::contains_key(external_id),
            Error::<T>::InvestmentOpportunityAlreadyExists
        );

        if let Err(e) =
            T::AssetSystem::transactionally_reserve(&account, external_id, &shares_to_reserve, *asset_id)
        {
            match e {
                ReserveError::<DeipAssetIdOf<T>>::NotEnoughBalance => {
                    return Err(Error::<T>::InvestmentOpportunityBalanceIsNotEnough.into())
                }
                ReserveError::<DeipAssetIdOf<T>>::AssetTransferFailed(_) => {
                    return Err(Error::<T>::InvestmentOpportunityFailedToReserveAsset.into())
                }
                ReserveError::<DeipAssetIdOf<T>>::AlreadyReserved => {
                    return Err(Error::<T>::InvestmentOpportunityAlreadyExists.into())
                }
            };
        }

        let new_token_sale = Info {
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

        SimpleCrowdfundingMap::<T>::insert(external_id, new_token_sale.clone());

        Self::deposit_event(RawEvent::SimpleCrowdfundingCreated(external_id));

        Ok(())
    }

    pub(super) fn collect_funds(sale_id: Id, amount: DeipAssetBalanceOf<T>) -> Result<(), ()> {
        SimpleCrowdfundingMap::<T>::mutate_exists(sale_id, |sale| -> Result<(), ()> {
            match sale.as_mut() {
                Some(s) => s.total_amount.0 = amount.saturating_add(s.total_amount.0),
                None => return Err(()),
            }
            Ok(())
        })
    }

    pub(super) fn finish_crowdfunding_by_id(sale_id: Id) -> Result<(), ()> {
        match SimpleCrowdfundingMap::<T>::try_get(sale_id) {
            Err(_) => Err(()),
            Ok(sale) => {
                Self::update_status(&sale, Status::Finished);
                Self::process_investments(&sale);
                Ok(())
            }
        }
    }

    pub(super) fn activate_crowdfunding_impl(sale_id: Id) -> DispatchResult {
        SimpleCrowdfundingMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::InvestmentOpportunityNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Active => return Ok(()),
                Status::Inactive => ensure!(
                    pallet_timestamp::Pallet::<T>::get() >= sale.start_time,
                    Error::<T>::InvestmentOpportunityShouldBeStarted
                ),
                _ => return Err(Error::<T>::InvestmentOpportunityShouldBeInactive.into()),
            };

            sale.status = Status::Active;
            Self::deposit_event(RawEvent::SimpleCrowdfundingActivated(sale_id));

            Ok(())
        })
    }

    pub(super) fn expire_crowdfunding_impl(sale_id: Id) -> DispatchResult {
        SimpleCrowdfundingMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::InvestmentOpportunityNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Expired => return Ok(()),
                Status::Active => ensure!(
                    pallet_timestamp::Pallet::<T>::get() >= sale.end_time,
                    Error::<T>::InvestmentOpportunityExpirationWrongState
                ),
                _ => return Err(Error::<T>::InvestmentOpportunityShouldBeActive.into()),
            };

            sale.status = Status::Expired;

            Self::refund(sale);

            Ok(())
        })
    }

    pub(super) fn finish_crowdfunding_impl(sale_id: Id) -> DispatchResult {
        SimpleCrowdfundingMap::<T>::mutate_exists(sale_id, |maybe_sale| -> DispatchResult {
            let sale = match maybe_sale.as_mut() {
                None => return Err(Error::<T>::InvestmentOpportunityNotFound.into()),
                Some(s) => s,
            };

            match sale.status {
                Status::Finished => return Ok(()),
                Status::Active => (),
                _ => return Err(Error::<T>::InvestmentOpportunityShouldBeActive.into()),
            };

            sale.status = Status::Finished;

            Self::process_investments(sale);

            Ok(())
        })
    }

    pub(super) fn process_investment_opportunities_offchain() {
        let now = pallet_timestamp::Pallet::<T>::get();
        for (id, sale) in SimpleCrowdfundingMap::<T>::iter() {
            if sale.end_time <= now && matches!(sale.status, Status::Active) {
                if sale.total_amount.0 < sale.soft_cap.0 {
                    let call = Call::expire_crowdfunding(id);
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    
                    debug!("submit expire_crowdfunding: {}", submit.is_ok());
                } else if sale.total_amount.0 >= sale.soft_cap.0 {
                    let call = Call::finish_crowdfunding(id);
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    debug!("submit finish_crowdfunding: {}", submit.is_ok());
                }
            } else if sale.end_time > now {
                if now >= sale.start_time && matches!(sale.status, Status::Inactive) {
                    let call = Call::activate_crowdfunding(id);
                    let submit = T::TransactionCtx::submit_postponed(call.into(), sale.created_ctx);
                    debug!("submit activate_crowdfunding: {}", submit.is_ok());
                }
            }
        }
    }

    fn update_status(sale: &SimpleCrowdfundingOf<T>, new_status: Status) {
        SimpleCrowdfundingMap::<T>::mutate_exists(sale.external_id, |maybe_sale| -> () {
            let sale = maybe_sale.as_mut().expect("we keep collections in sync");
            sale.status = new_status;
        });
    }

    fn refund(sale: &SimpleCrowdfundingOf<T>) {
        if let Ok(ref c) = InvestmentMap::<T>::try_get(sale.external_id) {
            for (_, ref contribution) in c {
                T::AssetSystem::transfer_from_reserved(
                    sale.external_id,
                    &contribution.owner,
                    sale.asset_id,
                    contribution.amount,
                )
                .unwrap_or_else(|_| panic!("user's asset should be reserved earlier"));

                system::pallet::Pallet::<T>::dec_consumers(&contribution.owner);
            }
            InvestmentMap::<T>::remove(sale.external_id);
        }

        T::AssetSystem::transactionally_unreserve(sale.external_id)
            .unwrap_or_else(|_| panic!("assets should be reserved earlier"));

        Self::deposit_event(RawEvent::SimpleCrowdfundingExpired(sale.external_id));
    }

    fn process_investments(sale: &SimpleCrowdfundingOf<T>) {
        let contributions = InvestmentMap::<T>::try_get(sale.external_id)
            .expect("about to finish, but there are no contributions?");

        for asset in &sale.shares {
            let mut amount = asset.amount().clone();

            let mut iter = contributions.iter();
            let (_, ref first_contribution) = iter
                .next()
                .expect("about to finish, but there are no contributors?");

            for (_, ref contribution) in iter {
                // similiar to frame_support::traits::Imbalance::ration
                let token_amount = contribution
                    .amount
                    .saturated_into::<u128>()
                    .saturating_mul(asset.amount().clone().saturated_into())
                    / sale.total_amount.0.saturated_into::<u128>();
                let token_amount: DeipAssetBalanceOf<T> = token_amount.saturated_into();
                if token_amount.is_zero() {
                    continue;
                }

                amount -= token_amount;

                T::AssetSystem::transfer_from_reserved(
                    sale.external_id,
                    &contribution.owner,
                    *asset.id(),
                    token_amount,
                )
                .unwrap_or_else(|_| panic!("Required token_amount should be reserved"));
            }

            if !amount.is_zero() {
                T::AssetSystem::transfer_from_reserved(
                    sale.external_id,
                    &first_contribution.owner,
                    *asset.id(),
                    amount,
                )
                .unwrap_or_else(|_| panic!("Required token_amount should be reserved"));
            }
        }

        T::AssetSystem::transactionally_unreserve(sale.external_id)
            .unwrap_or_else(|_| panic!("remaining assets should be reserved earlier"));

        for (_, ref contribution) in contributions {
            system::pallet::Pallet::<T>::dec_consumers(&contribution.owner);
        }
        InvestmentMap::<T>::remove(sale.external_id);

        Self::deposit_event(RawEvent::SimpleCrowdfundingFinished(sale.external_id));
    }
}
