//! # DEIP Portal Module
//! A module to make transactions with a Portal signature
//!
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! A module to make transactions with a Portal signature
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create a Portal.
//! * `update` - Update a Portal.
//! * `schedule` - Schedule an extrinsic to be executed within Portal context.
//! * `exec` - Call-wrapper that may be scheduled.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod module;
#[cfg(test)]
mod tests;
mod transaction_ctx;

pub mod benchmarking;
pub mod weights;
mod portal;

#[doc(inline)]
pub use pallet::*;
#[doc(inline)]
pub use portal::*;
#[doc(inline)]
pub use module::*;
#[doc(inline)]
pub use transaction_ctx::*;

#[doc(hidden)]
pub use deip_transaction_ctx::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::{pallet_prelude::*, RawOrigin};

    use frame_support::{
        pallet_prelude::*,
        weights::{GetDispatchInfo, PostDispatchInfo},
        Hashable,
    };

    use frame_support::{
        log::debug,
        traits::{ExtrinsicCall, IsSubType, UnfilteredDispatchable},
    };

    use sp_std::{collections::btree_map::BTreeMap, iter::FromIterator, prelude::*};

    use codec::EncodeLike;
    use frame_support::dispatch::DispatchResult;
    use sp_runtime::{
        traits::{Dispatchable, IdentifyAccount},
        DispatchResultWithInfo, MultiSigner,
    };

    use super::*;
    use crate::weights::WeightInfo;

    use frame_system::offchain::SendTransactionTypes;
    use sp_runtime::traits::Extrinsic;
    use sp_std::fmt::Debug;

    /// Configuration trait
    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + SendTransactionTypes<Call<Self>>
        + PortalModuleT<Self, crate::PortalCtxOf<Self>, crate::Call<Self>>
        + Debug
        + TypeInfo
    {
        type TenantLookup: TenantLookupT<Self::AccountId, TenantId = Self::PortalId> + Debug;

        type PortalId: Member + Parameter + Copy + Default;
        type Portal: PortalT<Self> + Member + Parameter + TypeInfo;

        type Call: Parameter
            + Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo>
            + GetDispatchInfo
            + From<frame_system::pallet::Call<Self>>
            + From<crate::Call<Self>>
            + UnfilteredDispatchable<Origin = Self::Origin>
            + frame_support::dispatch::Codec
            + IsSubType<Call<Self>>
            + TypeInfo;

        type UnsignedValidator: ValidateUnsigned<Call = <Self as Config>::Call>;

        type UncheckedExtrinsic: Parameter
            + PartialEq
            + frame_support::traits::ExtrinsicCall
            + Extrinsic<Call = <Self as Config>::Call>
            + TypeInfo;

        type DeipPortalWeightInfo: WeightInfo;
    }

    use frame_support::traits::StorageVersion;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    use frame_support::storage::{StoragePrefixedMap};
    use sp_runtime::traits::{Hash};

    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_runtime_upgrade() -> Weight {
            if Pallet::<T>::current_storage_version() == V1::<T>::version()
                && Pallet::<T>::on_chain_storage_version() == V0::<T>::version()
            {
                let mut pending: usize = 0;
                PendingTx::<T>::drain().for_each(|(_, _, xt)| {
                    let portal_id = ScheduledTx::<T>::take(V0::<T>::extrinsic_hash(&xt)).unwrap();
                    SignedTx::<T>::insert(V1::<T>::extrinsic_hash(&xt), portal_id);
                    pending += 1;
                });
                let _ = SignedTx::<T>::remove_all(None);
                return T::DbWeight::get().reads_writes(pending as Weight * 2, pending as Weight + 1);
            }
            return 0;
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        DelegateMismatch,
        PortalMismatch,
        AlreadyScheduled,
        UnproperCall,
        NotSigned,
        OwnerIsNotATenant,
        PortalAlreadyExist,
        PortalNotFound,
    }

    // #[pallet::event]
    // #[pallet::metadata(u32 = "SpecialU32")]
    // #[pallet::generate_deposit(fn deposit_event)]
    // pub enum Event<T: Config> {}

    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::exec_postponed { portal_id: _, call: target_call } = call {
                T::UnsignedValidator::validate_unsigned(source, target_call)
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    use deip_transaction_ctx::{PortalCtxT, TransactionCtxT};

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((
            T::DeipPortalWeightInfo::create()
                // 1 DB read for the tenant_lookup that noop while benchmarking
                + T::DbWeight::get().reads(1 as Weight),
            DispatchClass::Normal,
            Pays::Yes
        ))]
        pub fn create(
            origin: OriginFor<T>,
            delegate: PortalDelegate<T>,
            metadata: PortalMetadata,
        ) -> DispatchResultWithPostInfo
        {
            let owner = ensure_signed(origin)?;
            T::create_portal(owner, delegate, metadata)?;
            Ok(Some(0).into())
        }

        #[pallet::weight((
            T::DeipPortalWeightInfo::update(),
            DispatchClass::Normal,
            Pays::Yes
        ))]
        pub fn update(origin: OriginFor<T>, update: PortalUpdate<T>) -> DispatchResultWithPostInfo
        {
            let owner = ensure_signed(origin)?;
            T::update_portal(owner, update)?;
            Ok(Some(0).into())
        }

        #[pallet::weight((
            10_000,
            DispatchClass::Normal,
            Pays::Yes
        ))]
        pub fn sign(
            origin: OriginFor<T>,
            xt: Box<T::UncheckedExtrinsic>,
        ) -> DispatchResultWithPostInfo
        {
            let delegate = ensure_signed(origin)?;
            T::sign_tx(*xt, delegate)?;
            Ok(Some(0).into())
        }

        #[pallet::weight((
            10_000,
            DispatchClass::Normal,
            Pays::Yes
        ))]
        pub fn exec(
            origin: OriginFor<T>,
            portal_id: PortalId<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo
        {
            T::exec_signed_tx(portal_id, *call, origin)?
        }

        #[pallet::weight((
            10_000,
            DispatchClass::Normal,
            Pays::Yes
        ))]
        pub fn exec_postponed(
            origin: OriginFor<T>,
            portal_id: PortalId<T>,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo
        {
            ensure_none(origin)?;
            T::exec_postponed_tx(portal_id, *call, RawOrigin::None.into())
        }
    }

    // ==== Storage ====:

    pub type ExtrinsicId = u32;
    pub type ExtrinsicIdList = Vec<ExtrinsicId>;
    pub type PortalInfo<PortalId> = Vec<(PortalId, ExtrinsicIdList)>;
    pub type ExtrinsicHash<T, V> = <V as StorageVersionT<T>>::ExtrinsicHash;

    pub fn transpose<'a, T, S, PortalId>(source: S) -> T
    where
        T: FromIterator<(&'a ExtrinsicId, &'a PortalId)> + 'a,
        S: Iterator<Item = &'a (PortalId, ExtrinsicIdList)> + 'a,
        PortalId: 'a,
    {
        T::from_iter(source.map(|(x, y)| y.iter().map(move |z| (z, x))).flatten())
    }

    pub trait StorageVersionT<T: Config>: Sized {
        type ExtrinsicHash: Parameter + Member;

        fn version() -> StorageVersion;
        fn extrinsic_hash(xt: &T::UncheckedExtrinsic) -> ExtrinsicHash<T, Self> {
            xt.using_encoded(|b| Self::extrinsic_hash2(b))
        }
        fn extrinsic_hash2(xt: &[u8]) -> Self::ExtrinsicHash;
    }

    pub struct V0<T: Config>(PhantomData<T>) where Self: StorageVersionT<T>;

    impl<T: Config> StorageVersionT<T> for V0<T> {
        type ExtrinsicHash = [u8; 32];

        fn version() -> StorageVersion {
            StorageVersion::new(0)
        }
        fn extrinsic_hash2(xt: &[u8]) -> ExtrinsicHash<T, Self> {
            sp_io::hashing::twox_256(&mut &xt)
        }
    }

    pub struct V1<T: Config>(PhantomData<T>) where Self: StorageVersionT<T>;

    impl<T: Config> StorageVersionT<T> for V1<T> {
        type ExtrinsicHash = T::Hash;

        fn version() -> StorageVersion {
            STORAGE_VERSION
        }
        fn extrinsic_hash2(xt: &[u8]) -> ExtrinsicHash<T, Self> {
            T::Hashing::hash(xt)
        }
    }

    // Migration to V2:
    // Removed
    #[pallet::storage]
    pub(super) type PendingTx<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        T::BlockNumber,
        Blake2_128Concat,
        ExtrinsicHash<T, V0<T>>,
        T::UncheckedExtrinsic,
    >;

    // Migration to V2:
    // Moved to SignedTx
    #[pallet::storage]
    pub(super) type ScheduledTx<T: Config> = StorageMap<_,
        Blake2_128Concat,
        ExtrinsicHash<T, V0<T>>,
        PortalId<T>
    >;

    // Migration to V2:
    // Moved from ScheduledTx
    #[pallet::storage]
    pub(super) type SignedTx<T: Config> = StorageMap<_,
        Identity,
        ExtrinsicHash<T, V1<T>>,
        PortalId<T>
    >;

    #[pallet::storage]
    pub(super) type PortalTagOfTransaction<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        BlockNumberFor<T>,
        Blake2_128Concat,
        PortalId<T>,
        ExtrinsicIdList,
        OptionQuery,
    >;

    #[pallet::storage]
    pub(super) type PortalRepository<T: Config> =
        StorageMap<_, Blake2_128Concat, PortalId<T>, T::Portal>;

    #[pallet::storage]
    pub(super) type DelegateLookup<T: Config> =
        StorageMap<_, Blake2_128Concat, PortalId<T>, PortalDelegate<T>>;

    #[pallet::storage]
    pub(super) type OwnerLookup<T: Config> =
        StorageMap<_, Blake2_128Concat, PortalOwner<T>, PortalId<T>>;
}
