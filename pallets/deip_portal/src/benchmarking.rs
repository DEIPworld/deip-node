#![cfg(feature = "runtime-benchmarks")]

use super::{*, portal::*};
use frame_system::{RawOrigin, EventRecord};
use frame_system::Config as Sys;
use frame_support::{ensure, traits::Get};
use frame_benchmarking::{benchmarks, account, whitelisted_caller, whitelist_account};
use sp_std::prelude::*;
use core::convert::TryInto;

use crate::Pallet;
use frame_support::weights::Weight;

const SEED: u32 = 0;

// fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
// 	let events = frame_system::Pallet::<T>::events();
// 	let system_event: <T as frame_system::Config>::Event = generic_event.into();
// 	// compare to the last event record
// 	let EventRecord { event, .. } = &events[events.len() - 1];
// 	assert_eq!(event, &system_event);
// }

fn init_member<T: Config>(index: u32) -> T::AccountId {
    let member = account::<T::AccountId>("member", index, SEED);
    whitelist_account!(member);
    member
}

fn init_portal<T: Config>() -> T::Portal {
    let owner = whitelisted_caller::<T::AccountId>();
    let id = T::lookup_tenant(&owner).unwrap();
    let delegate = init_member::<T>(0);
    let metadata = Some(<_>::default());
    T::Portal::new(id, owner, delegate, metadata)
}

fn create_portal<T: Config>(portal: T::Portal) -> T::Portal {
    T::create_portal(
        portal.owner().clone(),
        portal.delegate().clone(),
        portal.metadata().clone()
    ).unwrap();
    PortalRepository::<T>::get(*portal.id()).unwrap()
}

benchmarks! {
    create {
        let portal = init_portal::<T>();
    }: _(RawOrigin::Signed(portal.owner().clone()),
            portal.delegate().clone(),
            portal.metadata().clone())
    verify {
        let id = *portal.id();
        assert_eq!(PortalRepository::<T>::get(id).unwrap(), portal)
    }
    
    update {
        let portal = init_portal::<T>();
        let mut portal = create_portal::<T>(portal);
        let delegate = Some(init_member::<T>(99));
        let metadata = Some(None);
        let up = PortalUpdate::<T> {
            delegate,
            metadata
        };
    }: _(RawOrigin::Signed(portal.owner().clone()), up.clone())
    verify {
        let id = *portal.id();
        let PortalUpdate { delegate, metadata } = up;
        portal.update_delegate(delegate).update_metadata(metadata);
        assert_eq!(PortalRepository::<T>::get(id).unwrap(), portal)
    }
    
    // schedule {
    //     let portal = init_portal::<T>();
    //     let mut portal = create_portal::<T>(portal);
    //     let call = frame_system::Call<T>::remark(vec![]);
    //     T::UncheckedExtrinsic::new(call.into(), None)
    // }: _(RawOrigin::Signed(portal.delegate().clone()), xt)
    // verify {
    //     let id = *portal.id();
    //     let PortalUpdate { delegate, metadata } = up;
    //     portal.update_delegate(delegate).update_metadata(metadata);
    //     assert_eq!(PortalRepository::<T>::get(id).unwrap(), portal)
    // }
}
