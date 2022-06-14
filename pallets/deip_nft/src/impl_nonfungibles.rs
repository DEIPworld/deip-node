use frame_support::traits::tokens::nonfungibles::{Inspect, Mutate};

use crate::{Config, Pallet};

impl<T: Config> Inspect<T::AccountId> for Pallet<T> {
    type InstanceId;

    type ClassId;

    fn owner(class: &Self::ClassId, instance: &Self::InstanceId) -> Option<T::AccountId> {
        todo!()
    }
}

impl<T: Config> Mutate<T::AccountId> for Pallet<T> {}
