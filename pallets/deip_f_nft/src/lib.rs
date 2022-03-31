// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod traits;
pub mod types;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use crate::{traits::GetToken, types::PayloadDetails};
    use deip_asset_lock::Result as LockResult;
    use frame_support::{
        dispatch::DispatchResult,
        ensure,
        log::error,
        pallet_prelude::{IsType, StorageMap},
        Blake2_128Concat, Parameter,
    };
    use frame_system::{ensure_signed, pallet_prelude::OriginFor};
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + pallet_deip_assets::Config + pallet_deip_uniques::Config
    {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the payload of a future F-NFT
        type PayloadId: Parameter;

        /// Identifier for a payload asset.
        type PayloadAssetId: Parameter
            + GetToken<
                <Self as pallet_deip_assets::Config>::AssetsAssetId,
                Self::ClassId,
                Self::InstanceId,
            >;
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
        /// The given payload ID is uknown.
        UnknownPayload,
        /// Origin should be a creator (owner) of the payload.
        WrongOrigin,
        /// PayloadAsset already is in the payload.
        AlreadyExists,
        /// PayloadAsset is not in the Payload.
        NotInPayload,
        /// Asset lock failed.
        AssetLockFailed,
    }

    #[pallet::storage]
    /// Details of a payload.
    type Payload<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::PayloadId,
        PayloadDetails<T::AccountId, T::PayloadAssetId>,
    >;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1)]
        pub fn create_payload(origin: OriginFor<T>, id: T::PayloadId) -> DispatchResult {
            // Unsigned calls are not permitted.
            let owner = ensure_signed(origin)?;

            ensure!(!Payload::<T>::contains_key(id.clone()), Error::<T>::InUse);

            let details = PayloadDetails { owner: owner.clone(), assets: Vec::new() };
            Payload::<T>::insert(id.clone(), details);

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

            Self::lock(origin.clone(), &asset).map_err(|e| {
                error!("❗️❗️❗️ asset lock failed: {:?}", e);
                Error::<T>::AssetLockFailed
            })?;

            Payload::<T>::mutate(target.clone(), |maybe_details| {
                if let Some(details) = maybe_details {
                    ensure!(origin == details.owner, Error::<T>::WrongOrigin);
                    ensure!(!details.assets.contains(&asset), Error::<T>::AlreadyExists);
                    details.assets.push(asset.clone());
                    Ok(())
                } else {
                    Err(Error::<T>::UnknownPayload)
                }
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
            Payload::<T>::mutate(source.clone(), |maybe_details| {
                if let Some(details) = maybe_details {
                    ensure!(origin == details.owner, Error::<T>::WrongOrigin);
                    let index = details
                        .assets
                        .iter()
                        .position(|v| v == &asset)
                        .ok_or(Error::<T>::NotInPayload)?;
                    details.assets.remove(index);
                    Ok(())
                } else {
                    Err(Error::<T>::UnknownPayload)
                }
            })?;

            let event = Event::AssetRemoved { source, asset };
            Self::deposit_event(event);

            Ok(())
        }
    }

    impl<T: Config> Pallet<T>
    where
        T: pallet_deip_assets::Config + pallet_deip_uniques::Config,
    {
        fn lock(origin: T::AccountId, asset: &T::PayloadAssetId) -> LockResult {
            if let Some(id) = asset.ft_asset_id() {
                pallet_deip_assets::Pallet::<T>::lock_asset(*id)
            } else if let Some((class, instance)) = asset.nft_class_id() {
                pallet_deip_uniques::Pallet::<T>::lock_asset(origin, *class, *instance)
            } else {
                todo!()
            }
        }
    }
}
