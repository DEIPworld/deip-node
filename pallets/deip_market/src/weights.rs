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
    fn buy() -> Weight {
        100000
    }

    fn list() -> Weight {
        100000
    }

    fn unlist() -> Weight {
        100000
    }

    fn make_offer() -> Weight {
        100000
    }

    fn withdraw_offer() -> Weight {
        100000
    }

    fn accept_offer() -> Weight {
        100000
    }
}