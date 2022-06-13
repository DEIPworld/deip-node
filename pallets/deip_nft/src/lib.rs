// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_uniques;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{traits::IsType, dispatch::DispatchResult, transactional, sp_runtime::traits::StaticLookup, pallet_prelude::Member, Parameter};
    use frame_system::{pallet_prelude::OriginFor, ensure_signed};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_uniques::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type CollectionId: Member + Parameter;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000_000)]
        #[transactional]
        pub fn create_collection(origin: OriginFor<T>, collection: T::CollectionId) -> DispatchResult {
            let issuer = ensure_signed(origin)?;
            let admin = <T::Lookup as StaticLookup>::unlookup(issuer.into());
            
            pallet_uniques::Pallet::<T>::create(origin, collection, admin)?;
            
            Self::deposit_event(Event::CollectionCreated { issuer, collection_id });
            Ok(())
        }
    }
}