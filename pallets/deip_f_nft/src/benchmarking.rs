#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks_instance_pallet, whitelisted_caller};
use frame_support::{
    assert_ok,
    sp_runtime::traits::{Bounded, StaticLookup},
    traits::{
        tokens::{
            fungibles::Destroy,
            nonfungibles::{Create, Mutate},
        },
        Currency,
    },
};
use frame_system::RawOrigin as SystemOrigin;

use crate::{types::DepositBalanceOf, Call, Config, Event, Pallet};

const SEED: u32 = 0;

fn signed_origin<AccountId, Origin>(caller: AccountId) -> Origin
where
    Origin: From<SystemOrigin<AccountId>>,
{
    SystemOrigin::Signed(caller).into()
}

fn assert_last_event<T: Config<I>, I: 'static>(generic_event: <T as Config<I>>::Event) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn create_default_nft<T: Config<I>, I: 'static>(caller: &T::AccountId) {
    T::Currency::make_free_balance_be(caller, DepositBalanceOf::<T, I>::max_value());
    assert_ok!(T::NonFungible::create_class(&Default::default(), caller, caller));
    assert_ok!(T::NonFungible::mint_into(&Default::default(), &Default::default(), caller));
}

fn create_default_f_nft<T: Config<I>, I: 'static>(caller: T::AccountId) {
    create_default_nft::<T, I>(&caller);
    let origin = signed_origin(caller);
    let id = Default::default();
    let res = Pallet::<T, I>::create(origin, id, Default::default(), Default::default());
    assert_ok!(res);
}

fn create_default_token_asset<T: Config<I>, I: 'static>(caller: T::AccountId) {
    create_default_f_nft::<T, I>(caller.clone());
    let origin = signed_origin(caller);
    let id = Default::default();
    let token = Default::default();
    let res = Pallet::<T, I>::create_token_asset(origin, id, token, true, 1u32.into());
    assert_ok!(res);
}

fn mint_default_token_asset<T: Config<I>, I: 'static>(
    caller: T::AccountId,
    amount: T::FungibleBalance,
) {
    create_default_token_asset::<T, I>(caller.clone());
    assert_ok!(Pallet::<T, I>::mint_token_asset(signed_origin(caller), Default::default(), amount));
}

fn fractionalize_default_f_nft<T: Config<I>, I: 'static>(
    caller: T::AccountId,
    amount: T::FungibleBalance,
) {
    mint_default_token_asset::<T, I>(caller.clone(), amount);
    assert_ok!(Pallet::<T, I>::fractionalize(signed_origin(caller), Default::default()));
}

fn fuse_default_f_nft<T: Config<I>, I: 'static>(caller: T::AccountId, amount: T::FungibleBalance) {
    fractionalize_default_f_nft::<T, I>(caller.clone(), amount);
    assert_ok!(Pallet::<T, I>::fuse(signed_origin(caller), Default::default()));
}

fn burn_default_token_asset<T: Config<I>, I: 'static>(
    caller: T::AccountId,
    amount: T::FungibleBalance,
) {
    fuse_default_f_nft::<T, I>(caller.clone(), amount);
    assert_ok!(Pallet::<T, I>::burn_token_asset(signed_origin(caller), Default::default()));
}

fn destroy_default_token_asset<T: Config<I>, I: 'static>(
    caller: T::AccountId,
    amount: T::FungibleBalance,
) {
    burn_default_token_asset::<T, I>(caller.clone(), amount);
    let witness = T::Fungible::get_destroy_witness(&Default::default()).unwrap();
    let origin = signed_origin(caller);
    let res = Pallet::<T, I>::destroy_token_asset(origin, Default::default(), witness);
    assert_ok!(res);
}

benchmarks_instance_pallet! {
    create {
        let caller: T::AccountId = whitelisted_caller();
        create_default_nft::<T, I>(&caller);
    }: _(SystemOrigin::Signed(caller), Default::default(), Default::default(), Default::default())
    verify {
        let event = Event::Created { id: Default::default(), class: Default::default(), instance: Default::default()}.into();
        assert_last_event::<T, I>(event);
    }

    create_token_asset {
        let caller: T::AccountId = whitelisted_caller();
        create_default_f_nft::<T, I>(caller.clone());
    }: _(SystemOrigin::Signed(caller), Default::default(), Default::default(), true, 1u32.into())
    verify {
        let event = Event::TokenAssetCreated { id: Default::default(), token: Default::default()}.into();
        assert_last_event::<T, I>(event);
    }

    mint_token_asset {
        let amount = T::FungibleBalance::from(100u32);
        let caller: T::AccountId = whitelisted_caller();
        create_default_token_asset::<T, I>(caller.clone());
    }: _(SystemOrigin::Signed(caller), Default::default(), amount)
    verify {
        let event = Event::TokenAssetMinted { id: Default::default(), token: Default::default(), amount }.into();
        assert_last_event::<T, I>(event);
    }

    fractionalize {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        mint_default_token_asset::<T, I>(caller.clone(), amount);
    }: _(SystemOrigin::Signed(caller), Default::default())
    verify {
        let event = Event::Fractionalized { id: Default::default() }.into();
        assert_last_event::<T, I>(event);
    }

    fuse {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        fractionalize_default_f_nft::<T, I>(caller.clone(), amount);
    }: _(SystemOrigin::Signed(caller), Default::default())
    verify {
        let event = Event::Fused { id: Default::default() }.into();
        assert_last_event::<T, I>(event);
    }

    burn_token_asset {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        fuse_default_f_nft::<T, I>(caller.clone(), amount);
    }: _(SystemOrigin::Signed(caller), Default::default())
    verify {
        let event = Event::TokenAssetBurned { id: Default::default(), token: Default::default(), amount }.into();
        assert_last_event::<T, I>(event);
    }

    destroy_token_asset {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        burn_default_token_asset::<T, I>(caller.clone(), amount);
        let witness = T::Fungible::get_destroy_witness(&Default::default()).unwrap();
    }: _(SystemOrigin::Signed(caller), Default::default(), witness)
    verify {
        let event = Event::TokenAssetDestroyed { id: Default::default(), token: Default::default() }.into();
        assert_last_event::<T, I>(event);
    }

    destroy {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        destroy_default_token_asset::<T, I>(caller.clone(), amount);
    }: _(SystemOrigin::Signed(caller), Default::default())
    verify {
        let event = Event::Destroyed { id: Default::default() }.into();
        assert_last_event::<T, I>(event);
    }

    transfer {
        let caller: T::AccountId = whitelisted_caller();
        let amount = T::FungibleBalance::from(100u32);
        let dest: T::AccountId = account("dest", 0, SEED);
        let dest_lookup = T::Lookup::unlookup(dest.clone());
        fractionalize_default_f_nft::<T, I>(caller.clone(), amount);
    }: _(SystemOrigin::Signed(caller.clone()), Default::default(), dest_lookup)
    verify {
        let event = Event::Transferred { id: Default::default(), from: caller, to: dest }.into();
        assert_last_event::<T, I>(event);
    }
}
