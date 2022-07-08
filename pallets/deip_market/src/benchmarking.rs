#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks, whitelist_account};
use frame_system::{Call as SystemCall, RawOrigin};
use sp_runtime::traits::Bounded;

use crate::Pallet as Market;
use core::ops::Range;
use deip_asset_system::*;
use frame_system::Config as SystemConfig;
use pallet_deip_nft::Config as DeipNftConfig;
use pallet_deip_nft::Pallet as DeipNft;
use sp_runtime::traits::Hash;
use sp_io::hashing::twox_256;
use sp_core::H160;

const SEED: u32 = 1;

fn new_account<T: Config>(i: u32) -> Result<T::AccountId, &'static str> {
    let account = account::<T::AccountId>("user", i, SEED);
    whitelist_account!(account);
    // Give them some balance for a possible deposit
    let balance = BalanceOf::<T>::max_value();
    T::Currency::make_free_balance_be(&account, balance);
    Ok(account)
}

fn prepare_token<T>(owner: &T::AccountId) -> <T as Config>::Token
where
    T: Config<AssetId = <T as SystemConfig>::Hash> + DeipNftConfig,
{
    let caller: T::AccountId = admin.clone();
    let balance = BalanceOf::<T>::max_value();
    <T as Config>::Currency::make_free_balance_be(&caller, balance);
    let collection = H160::from([0u8; 20]).into();
    create_collection::<DeipNft<T>>(&caller, collection, 1u32.into()).unwrap();
    let item = <T as SystemConfig>::Hashing::hash_of(&1u32);
    (item, caller)
}

fn mint_token<T>(
    owner: T::AccountId,
    token: <T as Config>::Token,
) where
    T: Config<AssetId = <T as SystemConfig>::Hash, AssetBalance = T::Balance> + DeipNftConfig,
{
    DeipNft::<T>::mint_item(RawOrigin::Signed(caller.clone()).into(), collection, item).unwrap();
}

fn now<T: Config>() -> TimeOf<T> {
    Market::<T>::timepoint()
}


benchmarks! {
    where_clause {
        where T: Config<AssetId=<T as SystemConfig>::Hash,
                AssetBalance=<T as AssetsConfig>::Balance>
            + DeipNftConfig
    }
    list {
        let owner = new_account::<T>(1);
        let token = prepare_token::<T>(owner);
        mint_token::<T>(owner, token);
        let price = 10000u32.into();
    }: list(RawOrigin::Signed(buyer.clone()), token, price, None)
    verify {
        let listing = ListingOf::<T> { owner, price, expires: None };
        assert_eq!(Listing::<T>::get(&token), Some(listing));
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    unlist {
        let owner = new_account::<T>(1);
        let token = prepare_token::<T>(owner);
        mint_token::<T>(owner, token);
        let price = 10000u32.into();
        Market::<T>::list(RawOrigin::Signed(owner.clone()).into(), token, value, None).unwrap();
    }: unlist(RawOrigin::Signed(owner.clone()), token)
    verify {
        assert_eq!(Listing::<T>::get(&token), None);
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    buy {
        let owner = new_account::<T>(1);
        let token = prepare_token::<T>(owner);
        mint_token::<T>(owner, token);
        let value = 10000u32.into();
        Market::<T>::list(RawOrigin::Signed(owner.clone()).into(), token, value, None).unwrap();
        let buyer = new_account::<T>(2);
        assert!(owner != buyer);
    }: buy(RawOrigin::Signed(buyer.clone()), token, value)
    verify {
        assert_eq(Offers::<T>::get(&token), None);
        assert_eq!(Listing::<T>::get(&token), None);
        assert_eq!(Market::<T>::owner(&token), Some(buyer));
    }
    make_offer {
        let owner = new_account::<T>(1);
        let token = prepare_token::<T>(owner);
        mint_token::<T>(owner, token);
        let price = 10000u32.into();
        let buyer = new_account::<T>(2);
    }: make_offer(RawOrigin::Signed(buyer.clone()), token, price, None)
    verify {
        let offer = OfferOf<T> { maker: buyer.clone(), price, expires: None };
        assert_eq!(Offers::<T>::get(&token, &buyer), Some(offer));
        assert_eq!(Market::<T>::owner(&token), Some(owner));
    }
    withdraw_offer {
        let owner = new_account::<T>(1);
        let token = prepare_token::<T>(owner);
        mint_token::<T>(owner, token);
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
