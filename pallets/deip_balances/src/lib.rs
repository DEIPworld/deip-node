// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use pallet_balances;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::dispatch::{Weight};
    use frame_support::traits::Hooks;
    use frame_system::pallet_prelude::{BlockNumberFor};

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_balances::Config {}

    use frame_support::traits::{StorageVersion, GetStorageVersion};

    pub const V0: StorageVersion = StorageVersion::new(0);
    pub const V1: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(V1)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            use frame_support::storage::migration::move_storage_from_pallet;
            if Self::current_storage_version() == V1
                && Self::on_chain_storage_version() == V0
            {
                for x in &[
                    "TotalIssuance",
                    "Account",
                    "Locks",
                    "Reserves",
                    "StorageVersion",
                ] {
                    move_storage_from_pallet(
                        x.as_bytes(),
                        "ParityTechBalances".as_bytes(),
                        "Balances".as_bytes()
                    );
                }
                return 0;
            }
            0
        }
    }
}
