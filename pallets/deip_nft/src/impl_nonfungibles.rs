use frame_support::{
    dispatch::DispatchResult,
    traits::tokens::nonfungibles::{Create, Inspect, Mutate, Transfer},
};

use crate::{Config, Pallet};

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
    type InstanceId = T::ItemId;

    type ClassId = T::CollectionId;

    fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<T::AccountId> {
        todo!()
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {}

impl<T: Config> Create<T::AccountId> for Pallet<T> {
    fn create_class(
        class: &Self::ClassId,
        who: &T::AccountId,
        admin: &T::AccountId,
    ) -> DispatchResult {
        pallet_uniques::Pallet::<T>::create_class(class, who, admin)
    }
}

impl<T: Config> Transfer<T::AccountId> for Pallet<T> {
    fn transfer(
        class: &Self::ClassId,
        instance: &Self::InstanceId,
        destination: &T::AccountId,
    ) -> DispatchResult {
        todo!()
    }
}
