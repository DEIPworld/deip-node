use crate::{DelegateLookup, Error::*, OwnerLookup, PortalRepository, TenantLookupT};
use codec::{Decode, Encode, EncodeLike};
use frame_support::{
    ensure,
    pallet_prelude::{Member, Parameter},
};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{traits::Dispatchable, DispatchResultWithInfo};
use sp_std::{fmt::Debug, prelude::*};

pub type PortalId<T> = <T as crate::Config>::PortalId;
pub type PortalOwner<T> = <T as frame_system::Config>::AccountId;
pub type PortalDelegate<T> = <T as frame_system::Config>::AccountId;
pub type PortalMetadata = Option<sp_core::H256>;

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
// #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Portal<T: crate::Config> {
    id: PortalId<T>,
    owner: PortalOwner<T>,
    delegate: PortalDelegate<T>,
    metadata: PortalMetadata,
}

#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct PortalUpdate<T: crate::Config + TypeInfo> {
    pub delegate: Option<PortalDelegate<T>>,
    pub metadata: Option<PortalMetadata>,
}

impl<T: crate::Config> PortalT<T> for Portal<T> {
    fn new(
        id: PortalId<T>,
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata,
    ) -> Self {
        Self { id, owner, delegate, metadata }
    }

    fn id(&self) -> &PortalId<T> {
        &self.id
    }
    fn owner(&self) -> &PortalOwner<T> {
        &self.owner
    }
    fn delegate(&self) -> &PortalDelegate<T> {
        &self.delegate
    }
    fn metadata(&self) -> &PortalMetadata {
        &self.metadata
    }

    fn set_delegate(&mut self, delegate: PortalDelegate<T>) -> &mut Self {
        self.delegate = delegate;
        self
    }
    fn set_metadata(&mut self, metadata: PortalMetadata) -> &mut Self {
        self.metadata = metadata;
        self
    }
}

pub trait PortalT<T: crate::Config>: Sized {
    fn new(
        id: PortalId<T>,
        owner: PortalOwner<T>,
        delegate: PortalDelegate<T>,
        metadata: PortalMetadata,
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
