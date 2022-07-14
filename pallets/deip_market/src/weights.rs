use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn buy() -> Weight;
    fn list() -> Weight;
    fn unlist() -> Weight;
    fn make_offer() -> Weight;
    fn withdraw_offer() -> Weight;
    fn accept_offer() -> Weight;
}

pub struct Weights<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for Weights<T> {
    fn list() -> Weight {
        (60_811_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }

    fn unlist() -> Weight {
        (52_578_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }

    fn buy() -> Weight {
        (184_130_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }

    fn make_offer() -> Weight {
        (97_821_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }

    fn withdraw_offer() -> Weight {
        (80_853_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }

    fn accept_offer() -> Weight {
        (211_693_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
}
