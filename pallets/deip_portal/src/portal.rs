use frame_support::pallet_prelude::{Member, Parameter};
use codec::{EncodeLike, Encode, Decode};
use sp_std::fmt::Debug;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use crate::TenantLookupT;
use frame_support::ensure;
use crate::{PortalRepository, DelegateLookup, Error::*, OwnerLookup};
use sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};

pub type PortalId<T> = <T as crate::Config>::PortalId;
pub type PortalOwner<T> = <T as frame_system::Config>::AccountId;
pub type PortalDelegate<T> = <T as frame_system::Config>::AccountId;
pub type PortalMetadata = Option<sp_core::H256>;

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Portal<T: crate::Config>
{
    id: PortalId<T>,
    owner: PortalOwner<T>,
    delegate: PortalDelegate<T>,
    metadata: PortalMetadata,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct PortalUpdate<T: crate::Config> {
    pub delegate: Option<PortalDelegate<T>>,
    pub metadata: Option<PortalMetadata>
}

impl<T: crate::Config> PortalModuleT<T, crate::PortalCtxOf<T>, crate::Call<T>> for T {}

pub trait PortalModuleT<T: crate::Config, U, LocalCall>
    where U: crate::PortalCtxT<
                LocalCall,
                PortalId=PortalId<T>,
                Extrinsic=T::UncheckedExtrinsic,
                Error=crate::Error<T>,
                Delegate=PortalDelegate<T>,
                BlockNumber=T::BlockNumber>,
{
    fn create_portal(
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata
    ) -> Result<(), crate::Error<T>>
    {
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
    ) -> Result<(), crate::Error<T>>
    {
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

    fn schedule_tx(xt: U::Extrinsic, delegate: U::Delegate)
        -> Result<(), U::Error>
    {
        U::current().schedule_extrinsic(xt, delegate)
    }
    
    fn submit_scheduled_tx(at: T::BlockNumber) -> Result<Vec<()>, ()> {
        U::submit_scheduled(at)
    }
    
    fn dispatch_scheduled_tx<D: Dispatchable>(
        portal_id: PortalId<T>,
        call: D,
        origin: D::Origin
    ) -> Result<DispatchResultWithInfo<D::PostInfo>, U::Error>
    {
        U::current().dispatch_scheduled(portal_id, call, origin)
    }
    
    fn exec_postponed_tx<D: Dispatchable>(
        portal_id: PortalId<T>,
        call: D,
        origin: D::Origin
    ) -> DispatchResultWithInfo<D::PostInfo>
    {
        U::current().dispatch(portal_id, call, origin)
    }
    
    #[cfg(not(feature = "runtime-benchmarks"))]
    fn lookup_tenant(key: &PortalOwner<T>) -> Result<PortalId<T>, crate::Error<T>> {
        T::TenantLookup::lookup(key).ok_or(OwnerIsNotATenant)
    }
    #[cfg(feature = "runtime-benchmarks")]
    fn lookup_tenant(key: &PortalOwner<T>) -> Result<PortalId<T>, crate::Error<T>> {
        Ok(<_>::default())
    }
}

pub trait PortalT<T: crate::Config>: Sized {
    fn new(
        id: PortalId<T>,
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata
    ) -> Self;
    
    fn id(&self) -> &PortalId<T>;
    fn owner(&self) -> &PortalOwner<T>;
    fn delegate(&self) -> &PortalDelegate<T>;
    fn metadata(&self) -> &PortalMetadata;
    
    fn set_delegate(&mut self, delegate: PortalDelegate<T>) -> &mut Self;
    fn set_metadata(&mut self, metadata: PortalMetadata) -> &mut Self;
    
    fn update_delegate(&mut self, delegate: Option<PortalDelegate<T>>) -> &mut Self {
        if let Some(delegate) = delegate {
            return self.set_delegate(delegate)
        }
        self
    }
    fn update_metadata(&mut self, metadata: Option<PortalMetadata>) -> &mut Self {
        if let Some(metadata) = metadata {
            return self.set_metadata(metadata)
        }
        self
    }
    
    fn not_exist(id: &PortalId<T>) -> Result<(), crate::Error<T>> {
        Ok(ensure!(!PortalRepository::<T>::contains_key(id), PortalAlreadyExist))
    }
    
    fn insert_delegate_lookup(portal: &T::Portal) {
        DelegateLookup::<T>::insert(*portal.id(), portal.delegate().clone());
    }
    fn insert_owner_lookup(portal: &T::Portal) {
        OwnerLookup::<T>::insert(portal.owner(), portal.id().clone());
    }
    fn insert_portal(portal: T::Portal) {
        PortalRepository::<T>::insert(portal.id().clone(), portal);
    }
    
    fn lookup_owner(owner: PortalOwner<T>) -> Result<PortalId<T>, crate::Error<T>> {
        OwnerLookup::<T>::get(owner).ok_or(PortalNotFound)
    }
    
    fn lookup_delegate(portal_id: &PortalId<T>) -> Result<PortalDelegate<T>, crate::Error<T>> {
        DelegateLookup::<T>::get(portal_id).ok_or(DelegateMismatch)
    }
    
    fn fetch_portal(id: &PortalId<T>) -> T::Portal {
        PortalRepository::<T>::get(id).expect("Portal must exist")
    }
}

impl<T: crate::Config> PortalT<T> for Portal<T>
{
    fn new(
        id: PortalId<T>,
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata
    ) -> Self
    {
        Self { id, owner, delegate, metadata }
    }

    fn id(&self) -> &PortalId<T> { &self.id }
    fn owner(&self) -> &PortalOwner<T> { &self.owner }
    fn delegate(&self) -> &PortalDelegate<T> { &self.delegate }
    fn metadata(&self) -> &PortalMetadata { &self.metadata }

    fn set_delegate(&mut self, delegate: PortalDelegate<T>) -> &mut Self {
        self.delegate = delegate;
        self
    }
    fn set_metadata(&mut self, metadata: PortalMetadata) -> &mut Self {
        self.metadata = metadata;
        self
    }
}
