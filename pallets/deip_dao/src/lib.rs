//! # DEIP DAO Module
//! A module for manage DAO and perform actions on behalf of it
//! 
//! - [`Config`](./trait.Config.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//! A module for manage DAO and perform actions on behalf of it
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `create` - Create a DAO.
//! * `alter_authority` - Alter DAO's authority.
//! * `on_behalf` - Perform action on behalf of a DAO.
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(test)]
mod tests;

pub mod api;

#[doc(inline)]
pub use pallet::*;

#[frame_support::pallet]
#[doc(hidden)]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    
    use frame_support::pallet_prelude::*;
    use frame_support::{Hashable};
    use frame_support::weights::{PostDispatchInfo, GetDispatchInfo};
    
    use frame_support::traits::{UnfilteredDispatchable, IsSubType};
    
    use sp_std::prelude::*;
    use sp_std::collections::{btree_map::BTreeMap};
    use sp_std::iter::FromIterator;
    
    use sp_runtime::{MultiSigner, traits::{Dispatchable, IdentifyAccount}};
    use frame_support::dispatch::DispatchResult;
    
    use sp_core::H256;
    
    use deip_storage_ops::StorageOpsTransaction;

    /// Configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Type represents events
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Type represents particular call from batch-transaction 
        type Call: Parameter +
             Dispatchable<Origin = Self::Origin, PostInfo = PostDispatchInfo> +
             GetDispatchInfo +
             From<frame_system::pallet::Call<Self>> +
             UnfilteredDispatchable<Origin = Self::Origin> +
             frame_support::dispatch::Codec + 
             IsSubType<Call<Self>>;
    }
    
    #[doc(hidden)]
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[doc(hidden)]
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    #[pallet::error]
    pub enum Error<T> {
        /// Already exists (unique by `name`)
        Exists,
        /// Not found
        NotFound,
        /// Access denied
        Forbidden,
        ///
        AuthorityMismatch
    }
    
    #[pallet::event]
    #[pallet::metadata(u32 = "SpecialU32")]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        /// Emits when DAO created
        DaoCreate(DaoOf<T>),
        /// Emits when authority alteration
        DaoAlterAuthority(DaoOf<T>),
        DaoMetadataUpdated(DaoOf<T>),
    }
    
    #[doc(hidden)]
    #[pallet::genesis_config]
    #[derive(Default)]
    pub struct GenesisConfig {}
    
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }
    
    use dao::*;
    pub mod dao {
        use sp_std::prelude::*;
        use frame_support::pallet_prelude::*;
        use super::{Config, DaoRepository, Error, DaoLookup};
        
        #[cfg(feature = "std")]
        use serde::{Serialize, Deserialize};
        use frame_system::Key;
        use codec::Codec;
        use sp_core::H256;

        #[allow(type_alias_bounds)]
        pub type DaoOf<T: Config> = Dao<T::AccountId, DaoId>;
        pub type DaoId = sp_core::H160;
        
        pub enum KeyType<'a, K> {
            Members(&'a K),
            Own(&'a K)
        }
        impl<'a, K> KeyType<'a, K> {
            pub fn members(k: &'a K) -> Self { Self::Members(k) }
            pub fn own(k: &'a K) -> Self { Self::Own(k) }
        }
        pub trait MatchKey<T: Config> {
            fn match_key(&self, dao: &DaoOf<T>) -> bool;
        }
        impl<T: Config> MatchKey<T> for KeyType<'_, T::AccountId> {
            fn match_key(&self, dao: &DaoOf<T>) -> bool {
                match self {
                    Self::Members(k) => {
                        *k == dao.authority_key()
                    },
                    Self::Own(k) => {
                        *k == dao.dao_key()
                    },
                }
            }
        }
        
        pub enum LoadBy<'a, AccountId> {
            DaoId { id: &'a DaoId, who: KeyType<'a, AccountId> },
            DaoKey { dao_key: &'a AccountId }
        }
        
        pub fn load_dao<T: Config>(
            q: LoadBy<'_, T::AccountId>,
        )
            -> Result<DaoOf<T>, Error<T>>
        {
            let (dao, who) = match q {
                LoadBy::DaoId { id: name, who } => {
                    let dao = DaoRepository::<T>::get(name)
                        .ok_or(Error::<T>::NotFound)?;
                    (dao, who)
                },
                LoadBy::DaoKey { dao_key } => {
                    let dao_id = DaoLookup::<T>::get(dao_key)
                        .ok_or(Error::<T>::NotFound)?;
                    let dao = DaoRepository::<T>::get(&dao_id)
                        .ok_or(Error::<T>::NotFound)?;
                    (dao, KeyType::Own(dao_key))
                }
            };
            ensure!(MatchKey::<T>::match_key(&who, &dao), Error::<T>::Forbidden);
            Ok(dao)
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct Authority<AccountId> {
            signatories: Vec<AccountId>,
            threshold: u16 
        }
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct InputAuthority<AccountId> {
            pub signatories: Vec<AccountId>,
            pub threshold: u16 
        }
        pub enum AuthorityAssert {
            EmptySignatories,
            /// We expect signatures list with exactly one element for plain account
            PlainAccountExpect,
            ThresholdMismatch,
            KeyMismatch,
        }
        impl<T: Config> From<AuthorityAssert> for Error<T> {
            fn from(source: AuthorityAssert) -> Self {
                Error::<T>::AuthorityMismatch
            }
        }
        pub trait AssertAuthority<T: Config> {
            // fn assert(self, origin: &T::AccountId) -> Result<Authority<T::AccountId>, AuthorityAssert>;
        }
        fn multi_account_id<T: Codec + Default>(who: &[T], threshold: u16) -> T {
            let entropy = (b"modlpy/utilisuba", who, threshold).using_encoded(sp_io::hashing::blake2_256);
            T::decode(&mut &entropy[..]).unwrap_or_default()
        }
        impl<T: Config> AssertAuthority<T> for InputAuthority<T::AccountId> {
            
        }
        impl<AccountId: Codec + Default + Clone> Authority<AccountId> {
            pub fn authority_key(&self) -> AccountId {
                if self.threshold == 0 { return unsafe { self.signatories.get_unchecked(0).clone() } }
                multi_account_id::<AccountId>(&self.signatories[..], self.threshold)
            }
        }
        impl<AccountId: Ord + Eq + PartialEq> Authority<AccountId> {
            pub fn add_member(&mut self, member: AccountId) {
                if let Err(pos) = self.signatories.binary_search(&member) {
                    self.signatories.insert(pos, member);
                    self.threshold += 1;
                }
            }
            pub fn remove_member(&mut self, member: AccountId) {
                if self.signatories.len() == 1 { return }
                if let Ok(pos) = self.signatories.binary_search(&member) {
                    self.signatories.remove(pos);
                    if self.signatories.len() == 1 {
                        self.threshold = 0;
                        return
                    }
                    if self.signatories.len() > 1 && (self.threshold - 1) > 0 {
                        self.threshold -= 1;
                    }
                }
            }
        }
        impl<AccountId: Codec + Default + Clone + Ord + Eq + PartialEq> InputAuthority<AccountId> {
            pub(crate) fn assert(self, authority_key: &AccountId) -> Result<Authority<AccountId>, AuthorityAssert>
            {
                let Self { mut signatories, threshold } = self;
                ensure!(!signatories.is_empty(), AuthorityAssert::EmptySignatories);
                signatories.sort();
                signatories.dedup_by(|x, y| x == y);
                
                // zero threshold adjusts plain non-multisig account
                if threshold == 0 {
                    ensure!(signatories.len() == 1, AuthorityAssert::PlainAccountExpect);
                } else {
                    ensure!(threshold as usize <= signatories.len(), AuthorityAssert::ThresholdMismatch);
                };
                
                let authority = Authority { signatories, threshold };
                ensure!(authority_key == &authority.authority_key(), AuthorityAssert::KeyMismatch);
                
                Ok(authority)
            }
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        pub struct Dao<AccountId, Id> {
            /// Authority aka "control" key. Not fixed, may changes in future
            authority_key: AccountId,
            /// Details of control key: multi-sig or plain account
            authority: Authority<AccountId>,
            /// Unique DAO ID
            id: Id,
            /// Own key of DAO for keeping assets,
            /// to signing of an extrinsics calls dispatched on behalf of DAO etc ..
            /// Must be generated internally when DAO will be created,
            /// nobody knows private half of this key
            dao_key: AccountId,
            metadata: Option<H256>
        }
        impl<AccountId, Id> Dao<AccountId, Id> {
            pub fn new(
                authority_key: AccountId,
                authority: Authority<AccountId>,
                id: Id,
                dao_key: AccountId,
                metadata: Option<H256>,
            )
                -> Self
            {
                Self { authority_key, authority, id, dao_key, metadata }
            }
            
            pub fn authority_key(&self) -> &AccountId { &self.authority_key }
            pub fn authority(&self) -> &Authority<AccountId>{ &self.authority }
            pub fn id(&self) -> &Id { &self.id }
            pub fn dao_key(&self) -> &AccountId { &self.dao_key }
            
            pub fn alter_authoriry(self, op: AlterAuthority<AccountId>) -> Result<Self, AuthorityAssert>
                where
                    AccountId: Codec + Default + Clone + Ord + Eq + PartialEq            {
                let Self {
                    authority_key: _,
                    mut authority,
                    id,
                    dao_key,
                    metadata
                } = self;
                match op {
                    AlterAuthority::AddMember { member } => {
                        authority.add_member(member);
                    },
                    AlterAuthority::RemoveMember { member } => {
                        authority.remove_member(member);
                    },
                    AlterAuthority::ReplaceAuthority { authority_key: new_authority_key, authority: new_authority } => {
                        authority = new_authority.assert(&new_authority_key)?;
                    },
                }
                Ok(Self::new(authority.authority_key(), authority, id, dao_key, metadata))
            }

            pub fn update_metadata(mut self, new_metadata: Option<H256>,) -> Self {
                self.metadata = new_metadata;
                self
            }
        }
        
        #[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
        #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "std", serde(tag = "operation", content = "data"))]
        pub enum AlterAuthority<AccountId> {
            AddMember { member: AccountId },
            RemoveMember { member: AccountId},
            ReplaceAuthority { authority_key: AccountId, authority: InputAuthority<AccountId> }
        }
    }
    
    impl<T: Config> Pallet<T> {
        pub fn dao_key(dao_id: &DaoId) -> T::AccountId {
            dao_key::<T::AccountId>(dao_id)
        }
    }
    pub fn dao_key<T: Decode + Default>(dao_id: &DaoId) -> T {
        let entropy = (b"deip/DAOs/", dao_id.as_bytes()).using_encoded(sp_io::hashing::blake2_256);
        T::decode(&mut &entropy[..]).unwrap_or_default()
    }
    pub fn dao_key2<T: frame_system::Config>(dao_id: &DaoId) -> T::AccountId {
        dao_key::<T::AccountId>(dao_id)
    }
    
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        #[pallet::weight(10_000)]
        pub fn create(
            origin: OriginFor<T>,
            name: DaoId,
            authority: InputAuthority<T::AccountId>,
            metadata: Option<H256>,
        )
            -> DispatchResultWithPostInfo
        {
            let authority_key = ensure_signed(origin)?;
            let authority = authority.assert(&authority_key).map_err::<Error<T>, _>(Into::into)?;
            ensure!(!DaoRepository::<T>::contains_key(&name), Error::<T>::Exists);
            let dao_key = Self::dao_key(&name);
            let dao = DaoOf::<T>::new(
                authority_key,
                authority,
                name,
                dao_key,
                metadata,
            );
            StorageOpsTransaction::<StorageOps<T>>::new()
                .commit(move |ops| {
                    ops.push_op(StorageOps::CreateDao(dao.clone()));
                    ops.push_op(StorageOps::DepositEvent(Event::<T>::DaoCreate(dao)));
                });
            Ok(Some(0).into())
        }
        
        #[pallet::weight(10_000)]
        pub fn alter_authority(
            origin: OriginFor<T>,
            alter_authority: AlterAuthority<T::AccountId>,
        )
            -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            let mut dao = load_dao::<T>(LoadBy::DaoKey { dao_key: &who })?;
            dao = dao.alter_authoriry(alter_authority).map_err::<Error<T>, _>(Into::into)?;
            StorageOpsTransaction::<StorageOps<T>>::new()
                .commit(move |ops| {
                    ops.push_op(StorageOps::UpdateDao(dao.clone()));
                    ops.push_op(StorageOps::DepositEvent(Event::<T>::DaoAlterAuthority(dao)));
                });
            Ok(Some(0).into())
        }

        #[pallet::weight(10_000)]
        pub fn update_dao(
            origin: OriginFor<T>,
            new_metadata: Option<H256>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let mut dao = load_dao::<T>(LoadBy::DaoKey { dao_key: &who })?;
            dao = dao.update_metadata(new_metadata);
            StorageOpsTransaction::<StorageOps<T>>::new()
                .commit(move |ops| {
                    ops.push_op(StorageOps::UpdateDao(dao.clone()));
                    ops.push_op(StorageOps::DepositEvent(Event::<T>::DaoMetadataUpdated(dao)));
                });

            Ok(None.into())
        }

        #[pallet::weight(10_000)]
        pub fn on_behalf(
            origin: OriginFor<T>,
            name: DaoId,
            call: Box<<T as Config>::Call>,
        ) -> DispatchResultWithPostInfo
        {
            let who = ensure_signed(origin)?;
            let dao = load_dao::<T>(LoadBy::DaoId {
                id: &name,
                who: KeyType::Members(&who)
            })?;
            call.dispatch(RawOrigin::Signed(dao.dao_key().clone()).into())
        }
    }
    
    // ==== Storage ====:
    
    #[pallet::storage]
    #[pallet::getter(fn get_dao)]
    pub(super) type DaoRepository<T: Config> = StorageMap<_,
        Blake2_128Concat,
        DaoId,
        DaoOf<T>,
        OptionQuery
    >;
    
    #[pallet::storage]
    #[pallet::getter(fn lookup_dao)]
    pub(super) type DaoLookup<T: Config> = StorageMap<_,
        Blake2_128Concat,
        T::AccountId,
        DaoId,
        OptionQuery
    >;
    
    use storage_ops::*;
    /// Module contains abstractions over pallet storage operations
    pub mod storage_ops {
        use sp_std::prelude::*;
        use deip_storage_ops::StorageOp;
        use super::{Config, Event, Pallet};
        use super::{DaoOf, DaoRepository, DaoLookup};

        /// Storage operations
        pub enum StorageOps<T: Config> {
            /// Deposit event
            DepositEvent(Event<T>),
            /// Create DAO
            CreateDao(DaoOf<T>),
            /// Update DAO
            UpdateDao(DaoOf<T>),
            
        }
        impl<T: Config> StorageOp for StorageOps<T> {
            fn exec(self) {
                match self {
                    Self::DepositEvent(e) => {
                        Pallet::<T>::deposit_event(e)
                    },
                    Self::CreateDao(dao) => {
                        DaoLookup::<T>::insert(dao.dao_key().clone(), dao.id().clone());
                        DaoRepository::<T>::insert(*dao.id(), dao);
                    }
                    Self::UpdateDao(dao) => {
                        DaoRepository::<T>::insert(*dao.id(), dao);
                    }
                }
            }
        }
    }
}
