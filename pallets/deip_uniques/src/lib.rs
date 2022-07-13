// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{GetStorageVersion, Weight},
        storage::storage_prefix,
        traits::{Get, Hooks, StorageVersion},
    };
    use frame_system::pallet_prelude::BlockNumberFor;
    use sp_io::{storage::clear_prefix, KillStorageResult};

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    pub const V0: StorageVersion = StorageVersion::new(0);
    pub const V1: StorageVersion = StorageVersion::new(1);
    pub const V2: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(V2)]
    pub struct Pallet<T>(_);

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            fn clear_storage(pallet: &[u8], storage: &[u8]) -> u32 {
                let prefix = storage_prefix(pallet, storage);
                let kill_storage_result = clear_prefix(&prefix, None);
                match kill_storage_result {
                    KillStorageResult::AllRemoved(writes) => writes,
                    KillStorageResult::SomeRemaining(writes) => writes,
                }
            }

            let mut writes = 0u64;
            let reads = 0u64;

            if Pallet::<T>::on_chain_storage_version() == V1 &&
                Pallet::<T>::current_storage_version() == V2
            {
                let pallet_name = b"DeipUniques";

                let storages_to_clear = &[
                    "NftClassIdByDeipNftClassId",
                    "NftClassIdByDeipNftClassIdV1",
                    "DeipNftClassIdByNftClassId",
                    "DeipNftClassIdByNftClassIdV1",
                    "CollectionRepo",
                    "NextNftClassId",
                    "ItemRepo",
                    "FractionRepo",
                    "FractionalRepo",
                    "FingerprintByFractionTokenId",
                ];
                for storage in storages_to_clear {
                    writes += clear_storage(pallet_name, storage.as_bytes()) as u64;
                }
            }
            T::DbWeight::get().reads_writes(reads, writes)
        }
    }

    // [deprecated] transferred to pallet_deip_f_nft
    // #[pallet::storage]
    // pub type NextCollectionId<T: Config> = StorageValue<_, T::ClassId, ValueQuery>;
}
