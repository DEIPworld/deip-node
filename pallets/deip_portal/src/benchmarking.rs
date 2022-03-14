#![cfg(feature = "runtime-benchmarks")]

use super::{*, module::*};
use frame_system::{RawOrigin, EventRecord};
use frame_system::Config as Sys;
use frame_support::{ensure, traits::Get, Hashable};
use frame_benchmarking::{benchmarks, account, whitelisted_caller, whitelist_account};
use sp_std::prelude::*;
use core::convert::TryInto;

use crate::Pallet;
use frame_support::weights::Weight;
use sp_runtime::traits::Extrinsic;
use frame_system::offchain::CreateSignedTransaction;
use sp_application_crypto::RuntimeAppPublic;
use codec::{Codec, Encode, Decode};

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

use frame_system::{
    self as system,
    offchain::{
        AppCrypto, SendSignedTransaction, SendUnsignedTransaction,
        SignedPayload, Signer, SigningTypes, SubmitTransaction,
    },
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::generic::UncheckedExtrinsic;

const KEY_TYPE: KeyTypeId = KeyTypeId(*b"deip");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
mod crypto {
    use super::KEY_TYPE;
    use sp_runtime::app_crypto::{app_crypto, sr25519};
    use sp_runtime::{MultiSignature, MultiSigner};
    app_crypto!(sr25519, KEY_TYPE);

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for Public {
        type RuntimeAppPublic = Public;
        type GenericPublic = sp_core::sr25519::Public;
        type GenericSignature = sp_core::sr25519::Signature;
    }
}

/// Identity of an pallet authority.
type AuthorityId = crypto::Public;

benchmarks! {
    where_clause {
        where
            T: CreateSignedTransaction<crate::Call<T>>,
            AuthorityId: AppCrypto<T::Public, T::Signature>,
            T::UncheckedExtrinsic: codec::Codec
    }

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

    sign {
        let s in 0 .. 50_000;

        let portal = init_portal::<T>();
        let portal = create_portal::<T>(portal);

        let call = frame_system::Call::<T>::remark { remark: vec![0; s as usize] };

        let who = init_member::<T>(99);

        let xt = to_sign::<T>(
            who.clone(),
            *portal.id(),
            call.into()
        );
    }: _(RawOrigin::Signed(portal.delegate().clone()), Box::new(xt))
    verify {}

    exec {
        let portal = init_portal::<T>();
        let portal = create_portal::<T>(portal);

        let who = init_member::<T>(99);

        let call = frame_system::Call::<T>::remark { remark: vec![0; 10_000] };

        let xt = to_sign::<T>(
            who.clone(),
            *portal.id(),
            call.clone().into()
        );

        _sign::<T>(&portal, xt.clone());
        set_extrinsic_data::<T>(xt);

    }: _(RawOrigin::Signed(who), *portal.id(), Box::new(call.into()))
    verify {}

    exec_postponed {
        let portal = init_portal::<T>();
        let portal = create_portal::<T>(portal);

        let call = frame_system::Call::<T>::remark { remark: vec![0; 10_000] };

        let who = init_member::<T>(99);

    }: _(RawOrigin::Signed(who), *portal.id(), Box::new(call.into()))
    verify {}
}

fn to_sign<T: Config>(
    account: T::AccountId,
    portal_id: PortalId<T>,
    call: <T as Config>::Call,
) -> T::UncheckedExtrinsic
where
    T: CreateSignedTransaction<crate::Call<T>>,
    AuthorityId: AppCrypto<T::Public, T::Signature>
{
    let call = crate::Call::<T>::exec { portal_id, call: Box::new(call) };

    let pair = <AuthorityId as AppCrypto<T::Public, T::Signature>>::RuntimeAppPublic::generate_pair(None);
    let generic_pub = <AuthorityId as AppCrypto<T::Public, T::Signature>>::GenericPublic::from(pair);

    let (_, sig) = T::create_transaction::<AuthorityId>(
        call.clone().into(),
        generic_pub.into(),
        account,
        <_>::default(),
    ).unwrap();
    T::UncheckedExtrinsic::new(call.into(), Some(sig)).unwrap()
}

fn _sign<T: Config>(portal: &T::Portal, xt: T::UncheckedExtrinsic)
{
    Pallet::<T>::sign(
        RawOrigin::Signed(portal.delegate().clone()).into(),
        Box::new(xt)
    ).unwrap();
}

fn set_extrinsic_data<T: Config>(xt: T::UncheckedExtrinsic) {
    let ctx = TransactionCtx::<T>::current();
    let mut key = Vec::with_capacity(40);
    use sp_io::hashing::twox_128;
    key.extend(twox_128(b"System"));
    key.extend(twox_128(b"ExtrinsicData"));
    key.extend(ctx.extrinsic_id().twox_64_concat());
    let xt_data = xt.encode();
    sp_io::storage::set(&key[..], &xt_data.encode()[..]);
}
