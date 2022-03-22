// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod traits;
pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::types::PayloadDetails;
    use frame_support::{
        dispatch::DispatchResult,
        ensure,
        pallet_prelude::{IsType, StorageMap},
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use sp_std::{vec, vec::Vec};

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the payload of a future F-NFT
        type PayloadId: Parameter;

        /// Identifier for a payload asset.
        type PayloadAssetId: Parameter;
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        /// A F-NFT payload was created.
        Created { id: T::PayloadId, creator: T::AccountId },
        /// Asset was added to payload.
        AssetAdded { target: T::PayloadId, asset: T::PayloadAssetId },
        /// Asset was removed from payload.
        AssetRemoved { source: T::PayloadId, asset: T::PayloadAssetId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The ID is already taken.
        InUse,
        /// The given ID is uknown.
        Unknown,
        /// Origin should be a creator (owner) of the payload.
        WrongOrigin,
        /// PayloadAsset already is in the payload.
        AlreadyExists,
        /// PayloadAsset is not in the Payload.
        NotInPayload,
    }

    #[pallet::storage]
    /// Details of a payload.
    type Payload<T: Config> =
        StorageMap<_, Blake2_128Concat, T::PayloadId, PayloadDetails<T::AccountId>>;

    #[pallet::storage]
    /// Assets in payloads.
    type PayloadContainers<T: Config> =
        StorageMap<_, Blake2_128Concat, T::PayloadId, Vec<T::PayloadAssetId>>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub fn create_payload(origin: OriginFor<T>, id: T::PayloadId) -> DispatchResult {
            // Unsigned calls are not permitted.
            let owner = ensure_signed(origin)?;

            ensure!(!Payload::<T>::contains_key(id.clone()), Error::<T>::InUse);

            Payload::<T>::insert(id.clone(), PayloadDetails { owner: owner.clone() });

            let event = Event::Created { id, creator: owner };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
        pub fn add_asset(
            origin: OriginFor<T>,
            target: T::PayloadId,
            asset: T::PayloadAssetId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;

            // asset.lock()?;

            let details = Payload::<T>::try_get(target.clone()).map_err(|_| Error::<T>::Unknown)?;
            ensure!(origin == details.owner, Error::<T>::WrongOrigin);
            PayloadContainers::<T>::mutate(target.clone(), |assets| -> DispatchResult {
                if let Some(assets) = assets {
                    ensure!(!assets.contains(&asset), Error::<T>::AlreadyExists);
                    assets.push(asset.clone());
                } else {
                    *assets = Some(vec![asset.clone()]);
                }
                Ok(())
            })?;

            let event = Event::AssetAdded { target, asset };
            Self::deposit_event(event);
            Ok(())
        }

        #[pallet::weight(1)]
        pub fn remove_asset(
            origin: OriginFor<T>,
            source: T::PayloadId,
            asset: T::PayloadAssetId,
        ) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            let details = Payload::<T>::try_get(source.clone()).map_err(|_| Error::<T>::Unknown)?;
            ensure!(origin == details.owner, Error::<T>::WrongOrigin);
            PayloadContainers::<T>::mutate(source.clone(), |assets| {
                if let Some(assets) = assets {
                    let index =
                        assets.iter().position(|v| v == &asset).ok_or(Error::<T>::NotInPayload)?;
                    assets.remove(index);
                    Ok(())
                } else {
                    Err(Error::<T>::NotInPayload)
                }
            })?;

            let event = Event::AssetRemoved { source, asset };
            Self::deposit_event(event);

            Ok(())
        }
    }
}
