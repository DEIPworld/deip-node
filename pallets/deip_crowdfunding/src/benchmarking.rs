#![cfg(feature = "runtime-benchmarks")]

#![allow(dead_code)]
#![allow(unused_imports)]

use super::{*};
use frame_system::{RawOrigin, EventRecord};
use frame_support::{traits::Get};
use frame_benchmarking::{benchmarks, account, whitelisted_caller, whitelist_account};
use sp_std::prelude::*;
use core::convert::TryInto;
use frame_support::weights::PostDispatchInfo;
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo, DispatchError};
use sp_core::H160;

use crate::Pallet;
use sp_runtime::traits::{Hash, Saturating, Scale, StaticLookup};

use pallet_deip_assets::{
    Pallet as DeipAssets,
    ProjectsInfoOf,
    Config as DeipAssetsConfig,
    DeipAssetIdOf
};
use pallet_assets::Config as AssetsConfig;
use deip_projects_info::DeipProjectsInfo;
use pallet_balances::Config as BalancesConfig;
use deip_serializable_u128::SerializableAtLeast32BitUnsigned;
use crate::module::*;
use deip_transaction_ctx::{TransactionCtx, TransactionCtxId};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

fn init_member<T: Config>(index: u32) -> T::AccountId {
    let member = account::<T::AccountId>("member", index, SEED);
    whitelist_account!(member);
    member
}

fn now<T: Config>() -> T::Moment {
    pallet_timestamp::Pallet::<T>::get()
}

benchmarks! {
    where_clause { where T: pallet_deip_assets::Config + pallet_balances::Config }

    create_investment_opportunity {
        let s in 1 .. 10;
        let crowdfunding = init_simple_crowdfunding::<T>(1, s as u8);
        let PreSimpleCrowdfunding::<T> {
            investment,
            funding_model,
            shares
        } = pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());

        let external_id = investment.sale_id.clone();

    }: _(RawOrigin::Signed(investment.owner.clone()),
            external_id,
            investment.owner.clone().into(),
            shares,
            funding_model)
    verify {
        assert_last_event::<T>(Event::<T>::SimpleCrowdfundingCreated(
            external_id
        ).into());
    }

    activate_crowdfunding {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);

    }: _(RawOrigin::None, crowdfunding.external_id)
    verify {
        assert_last_event::<T>(Event::<T>::SimpleCrowdfundingActivated(
            crowdfunding.external_id
        ).into());
    }

    expire_crowdfunding_already_expired {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);
        let crowdfunding = _activate_crowdfunding::<T>(crowdfunding);
        let crowdfunding = set_crowdfunding_end_time::<T>(crowdfunding, now::<T>());
        let crowdfunding = _expire_crowdfunding::<T>(crowdfunding);

    }: expire_crowdfunding(RawOrigin::None, crowdfunding.external_id)
    verify {}

    expire_crowdfunding {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);
        let crowdfunding = _activate_crowdfunding::<T>(crowdfunding);
        let crowdfunding = set_crowdfunding_end_time::<T>(crowdfunding, now::<T>());

    }: _(RawOrigin::None, crowdfunding.external_id)
    verify {
        assert_last_event::<T>(Event::<T>::SimpleCrowdfundingExpired(
            crowdfunding.external_id
        ).into());
    }

    finish_crowdfunding {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);
        let crowdfunding = _activate_crowdfunding::<T>(crowdfunding);
        _invest::<T>(&crowdfunding, whitelisted_caller());

    }: _(RawOrigin::None, crowdfunding.external_id)
    verify {
        assert_last_event::<T>(Event::<T>::SimpleCrowdfundingFinished(
            crowdfunding.external_id
        ).into());
    }

    invest {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);
        let crowdfunding = _activate_crowdfunding::<T>(crowdfunding);
        let investor: T::AccountId = whitelisted_caller();

    }: _(RawOrigin::Signed(investor.clone()),
            crowdfunding.external_id,
            DeipAsset::<T>::new(crowdfunding.asset_id, crowdfunding.soft_cap.0)
            )
    verify {
        assert_last_event::<T>(Event::<T>::Invested(
            crowdfunding.external_id,
            investor,
        ).into());
    }

    invest_hard_cap_reached {
        let crowdfunding = init_simple_crowdfunding::<T>(1, 10);
        let pre_crowdfunding =
            pre_simple_crowdfunding::<T>(crowdfunding, whitelisted_caller());
        let crowdfunding =
            _create_investment_opportunity::<T>(pre_crowdfunding);
        let crowdfunding = _activate_crowdfunding::<T>(crowdfunding);
        let investor: T::AccountId = whitelisted_caller();

    }: invest(RawOrigin::Signed(investor.clone()),
            crowdfunding.external_id,
            DeipAsset::<T>::new(crowdfunding.asset_id, crowdfunding.hard_cap.0)
            )
    verify {
        assert_last_event::<T>(Event::<T>::SimpleCrowdfundingFinished(
            crowdfunding.external_id,
        ).into());
    }
}

use sp_runtime::traits::Bounded;

fn _add_balance<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    party: T::AccountId,
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
) -> DispatchResultWithPostInfo
{
    // pallet_balances::Pallet::<T>::set_balance(
    //     RawOrigin::Root.into(),
    //     T::Lookup::unlookup(party.clone()),
    //     <T as BalancesConfig>::Balance::max_value(),
    //     <T as BalancesConfig>::Balance::from(0u16),
    // ).unwrap();

    let asset_admin: <T as DeipAssetsConfig>::DeipAccountId = party.clone().into();
    let min_balance = <T as AssetsConfig>::Balance::from(200u16);

    DeipAssets::<T>::deip_create(
        RawOrigin::Signed(party.clone()).into(),
        asset_id.clone(),
        asset_admin.clone(),
        min_balance.clone(),
    ).unwrap();

    DeipAssets::<T>::deip_mint(
        RawOrigin::Signed(party.clone()).into(),
        asset_id,
        asset_admin,
        min_balance
    )?;

    Ok(None.into())
}

fn _create_asset<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
    admin: T::AccountId,
    min_balance: <T as AssetsConfig>::Balance,
) -> DispatchResultWithPostInfo
{
    // pallet_balances::Pallet::<T>::set_balance(
    //     RawOrigin::Root.into(),
    //     T::Lookup::unlookup(admin.clone()),
    //     <T as BalancesConfig>::Balance::max_value(),
    //     <T as BalancesConfig>::Balance::min_value(),
    // )?;

    let asset_admin: <T as DeipAssetsConfig>::DeipAccountId = admin.clone().into();

    DeipAssets::<T>::deip_create(
        RawOrigin::Signed(admin).into(),
        asset_id,
        asset_admin,
        min_balance,
    )
}

fn _mint<T: Config + AssetsConfig + DeipAssetsConfig + BalancesConfig>(
    asset_id: pallet_deip_assets::DeipAssetIdOf<T>,
    admin: T::AccountId,
    beneficiary: T::AccountId,
    amount: <T as AssetsConfig>::Balance,
) -> DispatchResultWithPostInfo
{
    DeipAssets::<T>::deip_mint(
        RawOrigin::Signed(admin).into(),
        asset_id,
        beneficiary.into(),
        amount
    )
}

fn init_investment_opportunity<T: Config>(idx: u8) -> Investment<T> {
    let sale_id: CrowdfundingId = CrowdfundingId::from([idx; 20]);
    let owner: T::AccountId = whitelisted_caller();
    let amount = FTokenAmount::<T>::from(200u16);
    let time = T::Moment::from(1u16).mul(T::BlockNumber::from(10u16));
    Investment::<T> {
        sale_id,
        owner,
        amount,
        time,
    }
}

fn init_funding_model<T: Config>(investment: &Investment<T>) -> FundingModelOf<T> {
    let start_time: T::Moment = now::<T>();
    let end_time: T::Moment = start_time + investment.time;

    let asset_id = T::asset_id([1u8; 20].as_slice());

    let soft_cap = FToken::<T>::new(asset_id, FTokenAmount::<T>::from(100u16));
    let hard_cap = FToken::<T>::new(asset_id, FTokenAmount::<T>::from(200u16));
    FundingModelOf::<T>::SimpleCrowdfunding {
        start_time,
        end_time,
        soft_cap,
        hard_cap
    }
}

fn _create_simple_crowdfunding<T: Config>(
    investment: Investment<T>,
    funding_model: FundingModelOf<T>,
    shares: Vec<FToken<T>>,
) -> Result<SimpleCrowdfundingOf<T>, DispatchError>
{
    let Investment::<T> {
        sale_id,
        owner,
        amount: _,
        time: _
    } = investment;
    Pallet::<T>::create(
        RawOrigin::Signed(owner.clone()).into(),
        sale_id,
        owner.into(),
        shares,
        funding_model
    )?;
    Ok(SimpleCrowdfundingMapV1::<T>::get(sale_id).unwrap())
}

type CrowdfundingBalance<T> = SerializableAtLeast32BitUnsigned<FTokenAmount<T>>;

fn init_simple_crowdfunding<T: Config + BalancesConfig + DeipAssetsConfig>(
    idx: u8,
    shares: u8,
) -> SimpleCrowdfundingOf<T>
{
    let created_ctx: TransactionCtxId<<T as Config>::TransactionCtx> =
        Default::default();

    let external_id: CrowdfundingId =
        CrowdfundingId::from([idx; 20]);

    let start_time: T::Moment
        = now::<T>();

    use sp_runtime::traits::{One, Zero};
    let end_time: T::Moment =
        start_time + T::Moment::one().mul(T::BlockNumber::from(10u16));

    let status: CrowdfundingStatus =
        CrowdfundingStatus::Active;

    let asset_id: FTokenId<T> =
        T::asset_id(external_id.as_bytes());

    let share_ratio = 5u16;
    let shares: Vec<FToken<T>> =
        (1..shares+1).map(|i| {
            FToken::<T>::new(
                T::asset_id([idx+i; 20].as_slice()),
                FTokenAmount::<T>::from(i as u16 * share_ratio * 2)
            )
        }).collect();

    let total_amount = shares.iter()
        .map(|x| FTokenAmount::<T>::from(*x.amount()))
        .fold(FTokenAmount::<T>::zero(), |acc, x| acc + x);
    let soft_cap = total_amount - FTokenAmount::<T>::from(share_ratio);
    let hard_cap = total_amount;

    let total_amount: CrowdfundingBalance<T> =
        SerializableAtLeast32BitUnsigned(total_amount);

    let soft_cap: CrowdfundingBalance<T> =
        SerializableAtLeast32BitUnsigned(soft_cap);

    let hard_cap: CrowdfundingBalance<T> =
        SerializableAtLeast32BitUnsigned(hard_cap);

    SimpleCrowdfundingOf::<T> {
        created_ctx,
        external_id,
        start_time,
        end_time,
        status,
        asset_id,
        total_amount,
        soft_cap,
        hard_cap,
        shares,
    }
}

struct PreSimpleCrowdfunding<T: Config> {
    investment: Investment<T>,
    funding_model: FundingModelOf<T>,
    shares: Vec<FToken<T>>,
}

fn pre_simple_crowdfunding<T: Config + DeipAssetsConfig + BalancesConfig>(
    crowdfunding: SimpleCrowdfundingOf<T>,
    investment_owner: T::AccountId,
) -> PreSimpleCrowdfunding<T>
{
    let SimpleCrowdfundingOf::<T> {
        created_ctx: _,
        external_id,
        start_time,
        end_time,
        status: _,
        asset_id,
        total_amount,
        soft_cap,
        hard_cap,
        shares,
    } = crowdfunding;

    use sp_runtime::traits::{Zero, One};
    _create_asset::<T>(
        T::AssetIdInit::asset_id(external_id.as_bytes()),
        investment_owner.clone(),
        <_>::one()
    ).unwrap();
    _mint::<T>(
        T::AssetIdInit::asset_id(external_id.as_bytes()),
        investment_owner.clone(),
        investment_owner.clone(),
        <T as AssetsConfig>::Balance::from(unsafe { TryInto::<u16>::try_into(total_amount.0).unwrap_unchecked() }),
    ).unwrap();

    shares.iter().for_each(|x| {
        _create_asset::<T>(
            T::AssetIdInit::asset_id(x.id().as_ref()),
            investment_owner.clone(),
            <_>::one()
        ).unwrap();
        _mint::<T>(
            T::AssetIdInit::asset_id(x.id().as_ref()),
            investment_owner.clone(),
            investment_owner.clone(),
            <T as AssetsConfig>::Balance::from(unsafe { TryInto::<u16>::try_into(*x.amount()).unwrap_unchecked() }),
        ).unwrap();
    });

    let investment = Investment::<T> {
        sale_id: external_id,
        owner: investment_owner,
        amount: total_amount.0,
        time: end_time - start_time,
    };
    let funding_model = FundingModelOf::<T>::SimpleCrowdfunding {
        start_time,
        end_time,
        soft_cap: FToken::<T>::new(asset_id, soft_cap.0),
        hard_cap: FToken::<T>::new(asset_id, hard_cap.0),
    };
    PreSimpleCrowdfunding::<T> {
        investment,
        funding_model,
        shares
    }
}

fn _create_investment_opportunity<T: Config>(
    crowdfunding: PreSimpleCrowdfunding<T>
) -> SimpleCrowdfundingOf<T>
{
    let PreSimpleCrowdfunding::<T> {
        investment,
        funding_model,
        shares,
    } = crowdfunding;
    let external_id = investment.sale_id.clone();
    Pallet::<T>::create(
        RawOrigin::Signed(investment.owner.clone()).into(),
        external_id,
        investment.owner.clone().into(),
        shares,
        funding_model
    ).unwrap();
    SimpleCrowdfundingMapV1::<T>::get(external_id).unwrap()
}

fn _activate_crowdfunding<T: Config>(
    crowdfunding: SimpleCrowdfundingOf<T>
) -> SimpleCrowdfundingOf<T>
{
    Pallet::<T>::activate(
        RawOrigin::None.into(),
        crowdfunding.external_id
    ).unwrap();
    SimpleCrowdfundingMapV1::<T>::get(crowdfunding.external_id).unwrap()
}

fn _expire_crowdfunding<T: Config>(
    crowdfunding: SimpleCrowdfundingOf<T>,
) -> SimpleCrowdfundingOf<T>
{
    Pallet::<T>::expire(
        RawOrigin::None.into(),
        crowdfunding.external_id,
    ).unwrap();
    SimpleCrowdfundingMapV1::<T>::get(crowdfunding.external_id).unwrap()
}

fn set_crowdfunding_end_time<T: Config>(
    mut crowdfunding: SimpleCrowdfundingOf<T>,
    end_time: T::Moment,
) -> SimpleCrowdfundingOf<T>
{
    let external_id = crowdfunding.external_id;
    crowdfunding.end_time = end_time;
    SimpleCrowdfundingMapV1::<T>::insert(external_id, crowdfunding);
    SimpleCrowdfundingMapV1::<T>::get(external_id).unwrap()
}

fn _invest<T: Config>(
    crowdfunding: &SimpleCrowdfundingOf<T>,
    owner: T::AccountId,
)
{
    Pallet::<T>::invest(
        RawOrigin::Signed(owner).into(),
        crowdfunding.external_id,
        FToken::<T>::new(crowdfunding.asset_id, crowdfunding.soft_cap.0)
    ).unwrap();
}
