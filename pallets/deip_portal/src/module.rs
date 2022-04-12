use crate::{DelegateLookup, Error::*, OwnerLookup, PortalRepository, TenantLookupT, SignedTx, StorageVersionT, V1};
use codec::{Decode, Encode, EncodeLike};
use frame_support::{
    ensure,
    pallet_prelude::{Member, Parameter},
    traits::{ExtrinsicCall, IsSubType},
};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::{Dispatchable, Hash}, DispatchResultWithInfo};
use sp_std::{fmt::Debug, prelude::*};
use crate::portal::*;

impl<T: crate::Config> PortalModuleT<T, crate::PortalCtxOf<T>, crate::Call<T>> for T {}

pub trait PortalModuleT<T: crate::Config, U, LocalCall>
where
    U: crate::PortalCtxT<
        LocalCall,
        PortalId = PortalId<T>,
        BlockNumber = T::BlockNumber,
    >,
{
    #[cfg(not(feature = "runtime-benchmarks"))]
    fn lookup_tenant(key: &PortalOwner<T>) -> Result<PortalId<T>, crate::Error<T>> {
        T::TenantLookup::lookup(key).ok_or(OwnerIsNotATenant)
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn lookup_tenant(key: &PortalOwner<T>) -> Result<PortalId<T>, crate::Error<T>> {
        Ok(<_>::default())
    }

    fn create_portal(
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata,
    ) -> Result<(), crate::Error<T>> {
        let portal_id = T::lookup_tenant(&owner)?;
        T::Portal::not_exist(&portal_id)?;
        let portal = T::Portal::new(portal_id, owner, delegate, metadata);
        T::Portal::insert_owner_lookup(&portal);
        T::Portal::insert_delegate_lookup(&portal);
        T::Portal::insert_portal(portal);
        Ok(())
    }

    fn update_portal(
        owner: PortalOwner<T>,
        update: PortalUpdate<T>,
    ) -> Result<(), crate::Error<T>> {
        let portal_id = T::Portal::lookup_owner(owner)?;
        let mut portal = T::Portal::fetch_portal(&portal_id);
        let PortalUpdate { delegate, metadata } = update;
        let update_delegate = delegate.is_some();
        let update_metadata = metadata.is_some();
        portal.update_delegate(delegate).update_metadata(metadata);
        if update_delegate {
            T::Portal::insert_delegate_lookup(&portal);
        }
        if update_delegate || update_metadata {
            T::Portal::insert_portal(portal);
        }
        Ok(())
    }

    fn sign_tx(xt: T::UncheckedExtrinsic, delegate: PortalDelegate<T>) -> Result<(), crate::Error<T>> {
        if let Some(crate::Call::exec { portal_id, call: _ }) =
            <T as crate::Config>::Call::is_sub_type(ExtrinsicCall::call(&xt))
        {
            ensure!(T::Portal::lookup_delegate(portal_id)? == delegate, DelegateMismatch);
            let xt_hash = V1::<T>::extrinsic_hash(&xt);
            // debug!("{}: {:?}", "Schedule extrinsic", &xt_hash);
            ensure!(!SignedTx::<T>::contains_key(xt_hash), AlreadySigned);
            SignedTx::<T>::insert(xt_hash, portal_id);
            return Ok(())
        } else {
            Err(UnproperCall)?
        }
    }

    fn exec_signed_tx<D: Dispatchable>(
        portal_id: PortalId<T>,
        call: D,
        origin: D::Origin,
    ) -> Result<DispatchResultWithInfo<D::PostInfo>, crate::Error<T>> {
        let ctx = U::current();
        let xt = ctx.extrinsic_data();
        let xt_hash = V1::<T>::extrinsic_hash2(&xt[..]);
        // debug!("{}: {:?}", "Dispatch scheduled", &xt_hash);
        let expected_portal_id = SignedTx::<T>::take(xt_hash);
        ensure!(expected_portal_id.is_some(), NotSigned);
        ensure!(portal_id == expected_portal_id.unwrap(), NotSigned);
        Ok(ctx.dispatch(portal_id, call, origin))
    }

    fn exec_postponed_tx<D: Dispatchable>(
        portal_id: PortalId<T>,
        call: D,
        origin: D::Origin,
    ) -> DispatchResultWithInfo<D::PostInfo> {
        U::current().dispatch(portal_id, call, origin)
    }
}
