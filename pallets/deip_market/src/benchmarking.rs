#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, whitelist_account};
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;

use crate::Pallet as Market;
use deip_asset_system::*;
use frame_system::Config as SystemConfig;
use pallet_deip_nft::Config as DeipNftConfig;
use pallet_deip_nft::Pallet as DeipNft;
use sp_runtime::traits::Hash;
use sp_core::H160;

const SEED: u32 = 1;

fn new_account<T: Config>(i: u32) -> T::AccountId {
    let account = account::<T::AccountId>("user", i, SEED);
    whitelist_account!(account);
    // Give them some balance for a possible deposit
    let balance = BalanceOf::<T>::max_value();
    T::Currency::make_free_balance_be(&account, balance);
    account
}

fn prepare_token<T>(owner: &T::AccountId) -> (T::NFTCollectionId, T::NFTItemId)
where
    T: DeipNftConfig + Config,
{
    let balance = BalanceOf::<T>::max_value();
    <T as Config>::Currency::make_free_balance_be(&owner, balance);
    let collection = H160::from([0u8; 20]).into();
    create_collection::<DeipNft<T>>(&owner, collection, 1u32.into()).unwrap();
    let item = <T as SystemConfig>::Hashing::hash_of(&1u32);
    (collection, item)
}

fn mint_token<T>(
    owner: &T::AccountId,
    collection: T::NFTCollectionId,
    item: T::NFTItemId,
) where
    T: DeipNftConfig,
{
    mint_item::<DeipNft<T>>(collection, owner, OpaqueUnique::<DeipNft<T>>(item)).unwrap()
}

fn now<T: Config>() -> <T as SystemConfig>::BlockNumber {
    Market::<T>::current_time()
}


benchmarks! {
    where_clause {
        where T: Config<Token=<T as SystemConfig>::Hash>
            + DeipNftConfig
    }
    list {
        let owner = new_account::<T>(1);
        let (collection, token) = prepare_token::<T>(&owner);
        mint_token::<T>(&owner, collection, token);
        let price = 10000u32.into();
    }: list(RawOrigin::Signed(owner.clone()), token, price, None)
    verify {
        let listing = ListingOf::<T> { owner: owner.clone(), price, until: None };
        assert_eq!(Listed::<T>::get(&token), Some(listing));
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    unlist {
        let owner = new_account::<T>(1);
        let (collection, token) = prepare_token::<T>(&owner);
        mint_token::<T>(&owner, collection, token);
        let price = 10000u32.into();
        Market::<T>::list(RawOrigin::Signed(owner.clone()).into(), token, price, None).unwrap();
    }: unlist(RawOrigin::Signed(owner.clone()), token)
    verify {
        assert_eq!(Listed::<T>::get(&token), None);
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    buy {
        let owner = new_account::<T>(1);
        let (collection, token) = prepare_token::<T>(&owner);
        mint_token::<T>(&owner, collection, token);
        let value = 10000u32.into();
        Market::<T>::list(RawOrigin::Signed(owner.clone()).into(), token, value, None).unwrap();
        let buyer = new_account::<T>(2);
        assert!(owner != buyer);
    }: buy(RawOrigin::Signed(buyer.clone()), token, value)
    verify {
        assert_eq!(Offers::<T>::get(&token, &buyer), None);
        assert_eq!(Listed::<T>::get(&token), None);
        assert_eq!(Market::<T>::owner(&token), Some(buyer));
    }
    make_offer {
        let owner = new_account::<T>(1);
        let (collection, token) = prepare_token::<T>(&owner);
        mint_token::<T>(&owner, collection, token);
        let price = 10000u32.into();
        let buyer = new_account::<T>(2);
    }: make_offer(RawOrigin::Signed(buyer.clone()), token, price, None)
    verify {
        let offer = OfferOf::<T> { maker: buyer.clone(), price, until: None };
        assert_eq!(Offers::<T>::get(&token, &buyer), Some(offer));
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    withdraw_offer {
        let owner = new_account::<T>(1);
        let (collection, token) = prepare_token::<T>(&owner);
        mint_token::<T>(&owner, collection, token);
        let price = 10000u32.into();
        let buyer = new_account::<T>(2);
        Market::<T>::make_offer(RawOrigin::Signed(buyer.clone()).into(), token, price, None).unwrap();
    }: withdraw_offer(RawOrigin::Signed(buyer.clone()), token)
    verify {
        assert_eq!(Offers::<T>::get(&token, &buyer), None);
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }

    impl_benchmark_test_suite!(Market, crate::tests::new_test_ext(), crate::tests::Test);
}
