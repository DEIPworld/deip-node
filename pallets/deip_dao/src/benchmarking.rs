#![cfg(feature = "runtime-benchmarks")]

use super::{dao::*, *};
use core::convert::TryInto;
use frame_benchmarking::{account, benchmarks, whitelist_account, whitelisted_caller};
use frame_support::{ensure, traits::Get};
use frame_system::{Config as Sys, EventRecord, RawOrigin};
use sp_std::prelude::*;

use crate::Pallet;
use frame_support::weights::Weight;

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

fn init_authority<T: Config>(total_members: u16, member_start_idx: u16) -> Authority<T::AccountId> {
    assert!(total_members > 0);
    let mut signatories = (member_start_idx..(total_members + member_start_idx))
        .into_iter()
        .map(|x| init_member::<T>(x as u32))
        .collect::<Vec<_>>();
    let threshold = if signatories.len() == 1 { 0 } else { signatories.len() as u16 };
    InputAuthority::<T::AccountId>::sort_and_dedup(&mut signatories);
    Authority::<T::AccountId> { threshold, signatories }
}

fn init_dao<T: Config>(total_members: u16) -> DaoOf<T> {
    let id = DaoId::from([9; 20]);
    let authority = init_authority::<T>(total_members, 0);
    let authority_key = authority.authority_key();
    whitelist_account!(authority_key);
    let authority = InputAuthority::<T::AccountId>::from(authority);
    let authority = authority.assert::<T>(&authority_key).unwrap();
    let metadata = Some(<_>::default());
    let dao_key = Pallet::<T>::dao_key(&id);
    whitelist_account!(dao_key);
    DaoOf::<T>::new(authority_key, authority, id, dao_key, metadata)
}

fn create_dao<T: Config>(dao: DaoOf<T>) -> DaoOf<T> {
    let DaoOf::<T> { id, authority, authority_key, metadata, .. } = dao;
    Pallet::<T>::create(
        RawOrigin::Signed(authority_key.clone()).into(),
        id,
        authority.into(),
        metadata,
    )
    .unwrap();
    DaoRepository::<T>::get(id).unwrap()
}

fn add_member<T: Config>(dao: &DaoOf<T>, preserve_threshold: bool) -> AlterAuthority<T::AccountId> {
    let member_idx = dao.authority().signatories.len();
    AlterAuthority::<T::AccountId>::AddMember {
        member: init_member::<T>(member_idx as u32),
        preserve_threshold,
    }
}

fn remove_member<T: Config>(
    dao: &DaoOf<T>,
    preserve_threshold: bool,
) -> AlterAuthority<T::AccountId> {
    let member = dao.authority().signatories.last().unwrap().clone();
    AlterAuthority::<T::AccountId>::RemoveMember { member, preserve_threshold }
}

fn replace_authority<T: Config>(dao: &DaoOf<T>) -> AlterAuthority<T::AccountId> {
    let total_members = dao.authority().signatories.len() as u16;
    let authority = init_authority::<T>(total_members, total_members);
    let authority_key = authority.authority_key();
    let authority = authority.into();
    AlterAuthority::<T::AccountId>::ReplaceAuthority { authority_key, authority }
}

benchmarks! {
    create {
        let m in 1 .. T::MaxSignatories::get().try_into().unwrap();
        let dao = init_dao::<T>(m as u16);
    }: _(RawOrigin::Signed(dao.authority_key().clone()),
            dao.id().clone(),
            dao.authority().clone().into(),
            dao.metadata().clone())
    verify {
        assert_last_event::<T>(Event::DaoCreate(dao).into())
    }

    alter_authority_add_member {
        let dao = init_dao::<T>(T::MaxSignatories::get() - 1);
        let dao = create_dao::<T>(dao);
        let preserve_threshold = false;
        let alter_authority = add_member::<T>(&dao, preserve_threshold);
    }: alter_authority(RawOrigin::Signed(dao.dao_key().clone()), alter_authority.clone())
    verify {
        let dao = dao.alter_authoriry::<T>(alter_authority).unwrap();
        assert_last_event::<T>(Event::DaoAlterAuthority(dao).into())
    }

    alter_authority_add_member_preserve_threshold {
        let dao = init_dao::<T>(T::MaxSignatories::get() - 1);
        let dao = create_dao::<T>(dao);
        let preserve_threshold = true;
        let alter_authority = add_member::<T>(&dao, preserve_threshold);
    }: alter_authority(RawOrigin::Signed(dao.dao_key().clone()), alter_authority.clone())
    verify {
        let dao = dao.alter_authoriry::<T>(alter_authority).unwrap();
        assert_last_event::<T>(Event::DaoAlterAuthority(dao).into())
    }

    alter_authority_remove_member {
        let dao = init_dao::<T>(T::MaxSignatories::get());
        let dao = create_dao::<T>(dao);
        let preserve_threshold = false;
        let alter_authority = remove_member::<T>(&dao, preserve_threshold);
    }: alter_authority(RawOrigin::Signed(dao.dao_key().clone()), alter_authority.clone())
    verify {
        let dao = dao.alter_authoriry::<T>(alter_authority).unwrap();
        assert_last_event::<T>(Event::DaoAlterAuthority(dao).into())
    }

    alter_authority_remove_member_preserve_threshold {
        let dao = init_dao::<T>(T::MaxSignatories::get());
        let dao = create_dao::<T>(dao);
        let preserve_threshold = true;
        let alter_authority = remove_member::<T>(&dao, preserve_threshold);
    }: alter_authority(RawOrigin::Signed(dao.dao_key().clone()), alter_authority.clone())
    verify {
        let dao = dao.alter_authoriry::<T>(alter_authority).unwrap();
        assert_last_event::<T>(Event::DaoAlterAuthority(dao).into())
    }

    alter_authority_replace_authority {
        let m in 1 .. T::MaxSignatories::get().try_into().unwrap();
        let dao = init_dao::<T>(m as u16);
        let dao = create_dao::<T>(dao);
        let alter_authority = replace_authority::<T>(&dao);
    }: alter_authority(RawOrigin::Signed(dao.dao_key().clone()), alter_authority.clone())
    verify {
        let dao = dao.alter_authoriry::<T>(alter_authority).unwrap();
        assert_last_event::<T>(Event::DaoAlterAuthority(dao).into())
    }

    update_dao {
        let dao = init_dao::<T>(1);
        let dao = create_dao::<T>(dao);
        let metadata = None;
    }: _(RawOrigin::Signed(dao.dao_key().clone()), metadata.clone())
    verify {
        let dao = dao.update_metadata(metadata);
        assert_last_event::<T>(Event::DaoMetadataUpdated(dao).into())
    }

    on_behalf {
        let dao = init_dao::<T>(1);
        let dao = create_dao::<T>(dao);
        let call = frame_system::Call::<T>::remark{ remark: vec![] }.into();
    }: _(RawOrigin::Signed(dao.authority_key().clone()), dao.id().clone(), Box::new(call))
}
