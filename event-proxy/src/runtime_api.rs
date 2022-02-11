#[allow(clippy::all)]
#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(
        :: subxt :: codec :: Encode,
        :: subxt :: codec :: Decode,
        Debug,
        Clone,
        Eq,
        PartialEq,
        scale_info :: TypeInfo,
    )]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 6)]
        OctopusAppchain(octopus_appchain::Event),
        #[codec(index = 7)]
        OctopusLpos(octopus_lpos::Event),
        #[codec(index = 8)]
        OctopusUpwardMessages(octopus_upward_messages::Event),
        #[codec(index = 9)]
        Session(session::Event),
        #[codec(index = 10)]
        Grandpa(grandpa::Event),
        #[codec(index = 11)]
        Sudo(sudo::Event),
        #[codec(index = 12)]
        ImOnline(im_online::Event),
        #[codec(index = 15)]
        ParityTechAssets(parity_tech_assets::Event),
        #[codec(index = 16)]
        ParityTechUniques(parity_tech_uniques::Event),
        #[codec(index = 19)]
        Multisig(multisig::Event),
        #[codec(index = 20)]
        Utility(utility::Event),
        #[codec(index = 21)]
        Deip(deip::Event),
        #[codec(index = 24)]
        DeipProposal(deip_proposal::Event),
        #[codec(index = 25)]
        DeipDao(deip_dao::Event),
        #[codec(index = 27)]
        DeipVesting(deip_vesting::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct fill_block {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for fill_block {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_heap_pages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_heap_pages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_code {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_code_without_checks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code_without_checks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_storage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for set_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct kill_storage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for kill_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct kill_prefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_prefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct remark_with_event {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark_with_event {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, fill_block, DispatchError>
                {
                    let call = fill_block { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark, DispatchError>
                {
                    let call = remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_heap_pages, DispatchError>
                {
                    let call = set_heap_pages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_code, DispatchError>
                {
                    let call = set_code { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code_without_checks,
                    DispatchError,
                > {
                    let call = set_code_without_checks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_storage, DispatchError>
                {
                    let call = set_storage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_storage, DispatchError>
                {
                    let call = kill_storage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_prefix, DispatchError>
                {
                    let call = kill_prefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark_with_event, DispatchError>
                {
                    let call = remark_with_event { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::appchain_deip_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::appchain_deip_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                            192u8, 110u8, 150u8, 166u8, 46u8, 1u8, 0u8, 0u8, 1u8, 0u8, 152u8,
                            247u8, 62u8, 93u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8,
                            246u8, 232u8, 16u8, 163u8, 1u8, 0u8, 0u8, 1u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 1u8, 0u8, 136u8, 82u8, 106u8, 116u8, 0u8, 0u8,
                            0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8, 80u8, 0u8][..],
                    )?)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[96u8, 9u8, 0u8, 0u8][..])?)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 120u8, 125u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 225u8, 245u8, 5u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            32u8, 97u8, 112u8, 112u8, 99u8, 104u8, 97u8, 105u8, 110u8, 52u8, 97u8,
                            112u8, 112u8, 99u8, 104u8, 97u8, 105u8, 110u8, 45u8, 100u8, 101u8,
                            105u8, 112u8, 1u8, 0u8, 0u8, 0u8, 101u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8,
                            0u8, 56u8, 223u8, 106u8, 203u8, 104u8, 153u8, 7u8, 96u8, 155u8, 3u8,
                            0u8, 0u8, 0u8, 55u8, 227u8, 151u8, 252u8, 124u8, 145u8, 245u8, 228u8,
                            1u8, 0u8, 0u8, 0u8, 64u8, 254u8, 58u8, 212u8, 1u8, 248u8, 149u8, 154u8,
                            5u8, 0u8, 0u8, 0u8, 210u8, 188u8, 152u8, 151u8, 238u8, 208u8, 143u8,
                            21u8, 3u8, 0u8, 0u8, 0u8, 247u8, 139u8, 39u8, 139u8, 229u8, 63u8, 69u8,
                            76u8, 2u8, 0u8, 0u8, 0u8, 171u8, 60u8, 5u8, 114u8, 41u8, 31u8, 235u8,
                            139u8, 1u8, 0u8, 0u8, 0u8, 237u8, 153u8, 197u8, 172u8, 178u8, 94u8,
                            237u8, 245u8, 3u8, 0u8, 0u8, 0u8, 203u8, 202u8, 37u8, 227u8, 159u8,
                            20u8, 35u8, 135u8, 2u8, 0u8, 0u8, 0u8, 188u8, 157u8, 137u8, 144u8,
                            79u8, 91u8, 146u8, 63u8, 1u8, 0u8, 0u8, 0u8, 55u8, 200u8, 187u8, 19u8,
                            80u8, 169u8, 162u8, 168u8, 1u8, 0u8, 0u8, 0u8, 145u8, 213u8, 223u8,
                            24u8, 176u8, 210u8, 207u8, 88u8, 1u8, 0u8, 0u8, 0u8, 73u8, 234u8,
                            175u8, 27u8, 84u8, 138u8, 12u8, 176u8, 1u8, 0u8, 0u8, 0u8, 242u8, 5u8,
                            131u8, 199u8, 176u8, 112u8, 220u8, 153u8, 1u8, 0u8, 0u8, 0u8, 86u8,
                            149u8, 236u8, 225u8, 13u8, 62u8, 16u8, 95u8, 1u8, 0u8, 0u8, 0u8, 1u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[42u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct plan_config_change {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for plan_config_change {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, plan_config_change, DispatchError>
                {
                    let call = plan_config_change { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn epoch_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[200u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn expected_block_time(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[184u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set, DispatchError>
                {
                    let call = set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[220u8, 5u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_uncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for set_uncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_uncles, DispatchError>
                {
                    let call = set_uncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn uncle_generations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_balance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_balance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_transfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_keep_alive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_all {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for transfer_all {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_unreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_unreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_balance, DispatchError>
                {
                    let call = set_balance { who, new_free, new_reserved };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer { source, dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_keep_alive, DispatchError>
                {
                    let call = transfer_keep_alive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_all, DispatchError>
                {
                    let call = transfer_all { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unreserve, DispatchError>
                {
                    let call = force_unreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 122u8, 16u8, 243u8, 90u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 228u8, 11u8, 84u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8][..])?)
                }
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod octopus_appchain {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct submit_observations {
                pub payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                    runtime_types::sp_runtime::MultiSigner,
                    ::core::primitive::u32,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub signature: runtime_types::sp_runtime::MultiSignature,
            }
            impl ::subxt::Call for submit_observations {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "submit_observations";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_set_is_activated {
                pub is_activated: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_set_is_activated {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_is_activated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct force_set_next_set_id {
                pub next_set_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_set_next_set_id {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_next_set_id";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_set_planned_validators {
                pub validators: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Call for force_set_planned_validators {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "force_set_planned_validators";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct lock {
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for lock {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "lock";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct mint_asset {
                pub asset_id: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for mint_asset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "mint_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct burn_asset {
                pub asset_id: ::core::primitive::u32,
                pub receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for burn_asset {
                const PALLET: &'static str = "OctopusAppchain";
                const FUNCTION: &'static str = "burn_asset";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn submit_observations(
                    &self,
                    payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    signature: runtime_types::sp_runtime::MultiSignature,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit_observations, DispatchError>
                {
                    let call = submit_observations { payload, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_is_activated(
                    &self,
                    is_activated: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_is_activated, DispatchError>
                {
                    let call = force_set_is_activated { is_activated };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_next_set_id(
                    &self,
                    next_set_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_next_set_id, DispatchError>
                {
                    let call = force_set_next_set_id { next_set_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_planned_validators(
                    &self,
                    validators: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_set_planned_validators,
                    DispatchError,
                > {
                    let call = force_set_planned_validators { validators };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn lock(
                    &self,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, lock, DispatchError>
                {
                    let call = lock { receiver_id, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    sender_id: ::std::vec::Vec<::core::primitive::u8>,
                    receiver: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, mint_asset, DispatchError>
                {
                    let call = mint_asset { asset_id, sender_id, receiver, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn_asset(
                    &self,
                    asset_id: ::core::primitive::u32,
                    receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, burn_asset, DispatchError>
                {
                    let call = burn_asset { asset_id, receiver_id, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_appchain::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NewPlannedValidators(
                pub ::core::primitive::u32,
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            );
            impl ::subxt::Event for NewPlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "NewPlannedValidators";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Locked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for Locked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Unlocked(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unlocked {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct UnlockFailed(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UnlockFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "UnlockFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AssetMinted(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetMinted {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMinted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AssetBurned(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetBurned {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetBurned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AssetMintFailed(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetMintFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetMintFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AssetIdGetFailed(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for AssetIdGetFailed {
                const PALLET: &'static str = "OctopusAppchain";
                const EVENT: &'static str = "AssetIdGetFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AnchorContract;
            impl ::subxt::StorageEntry for AnchorContract {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AnchorContract";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssetIdByName(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for AssetIdByName {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "AssetIdByName";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct IsActivated;
            impl ::subxt::StorageEntry for IsActivated {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "IsActivated";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextSetId;
            impl ::subxt::StorageEntry for NextSetId {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextSetId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PlannedValidators;
            impl ::subxt::StorageEntry for PlannedValidators {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PlannedValidators";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextNotificationId;
            impl ::subxt::StorageEntry for NextNotificationId {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NextNotificationId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Observations(
                pub runtime_types::pallet_octopus_appchain::ObservationType,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Observations {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observations";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct Observing(
                pub  runtime_types::pallet_octopus_appchain::Observation<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::StorageEntry for Observing {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "Observing";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct PalletAccount;
            impl ::subxt::StorageEntry for PalletAccount {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "PalletAccount";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NotificationHistory(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for NotificationHistory {
                const PALLET: &'static str = "OctopusAppchain";
                const STORAGE: &'static str = "NotificationHistory";
                type Value = runtime_types::pallet_octopus_appchain::NotificationResult;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn anchor_contract(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = AnchorContract;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssetIdByName(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_name_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByName>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn is_activated(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsActivated;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn planned_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = PlannedValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_notification_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextNotificationId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations(
                    &self,
                    _0: runtime_types::pallet_octopus_appchain::ObservationType,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_octopus_appchain::Observation<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Observations(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observations_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Observations>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn observing(
                    &self,
                    _0: runtime_types::pallet_octopus_appchain::Observation<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Observing(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn observing_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Observing>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn pallet_account(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = PalletAccount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn notification_history(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_appchain::NotificationResult,
                    ::subxt::BasicError,
                > {
                    let entry = NotificationHistory(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn notification_history_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NotificationHistory>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn grace_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8, 0u8, 0u8, 0u8][..])?)
                }
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn request_event_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[10u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod octopus_lpos {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_history_depth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_history_depth {
                const PALLET: &'static str = "OctopusLpos";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct force_set_era_payout {
                pub era_payout: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_set_era_payout {
                const PALLET: &'static str = "OctopusLpos";
                const FUNCTION: &'static str = "force_set_era_payout";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_history_depth, DispatchError>
                {
                    let call = set_history_depth { new_history_depth, era_items_deleted };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_era_payout(
                    &self,
                    era_payout: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_era_payout, DispatchError>
                {
                    let call = force_set_era_payout { era_payout };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_lpos::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct PlanNewEra(pub ::core::primitive::u32);
            impl ::subxt::Event for PlanNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEra";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct PlanNewEraFailed;
            impl ::subxt::Event for PlanNewEraFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "PlanNewEraFailed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct TriggerNewEra;
            impl ::subxt::Event for TriggerNewEra {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "TriggerNewEra";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct EraPayout(
                pub ::core::primitive::u32,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            );
            impl ::subxt::Event for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayout";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct EraPayoutFailed(pub ::core::primitive::u32);
            impl ::subxt::Event for EraPayoutFailed {
                const PALLET: &'static str = "OctopusLpos";
                const EVENT: &'static str = "EraPayoutFailed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::pallet_octopus_lpos::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasStakers";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::pallet_octopus_lpos::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EraPayout;
            impl ::subxt::StorageEntry for EraPayout {
                const PALLET: &'static str = "OctopusLpos";
                const STORAGE: &'static str = "EraPayout";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::pallet_octopus_lpos::ActiveEraInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasStakers>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_octopus_lpos::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn era_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = EraPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[6u8, 0u8, 0u8, 0u8][..])?)
                }
                pub fn blocks_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[144u8, 1u8, 0u8, 0u8][..])?)
                }
                pub fn bonding_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[160u8, 2u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod octopus_upward_messages {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
            }
        }
        pub type Event = runtime_types::pallet_octopus_upward_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
        }
        pub mod storage {
            use super::runtime_types;
            pub struct MessageQueue;
            impl ::subxt::StorageEntry for MessageQueue {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "MessageQueue";
                type Value =
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nonce;
            impl ::subxt::StorageEntry for Nonce {
                const PALLET: &'static str = "OctopusUpwardMessages";
                const STORAGE: &'static str = "Nonce";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn message_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::pallet_octopus_upward_messages::Message>,
                    ::subxt::BasicError,
                > {
                    let entry = MessageQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Nonce;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn upward_messages_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[10u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_keys {
                pub keys: runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct purge_keys;
            impl ::subxt::Call for purge_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_keys, DispatchError>
                {
                    let call = set_keys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, purge_keys, DispatchError>
                {
                    let call = purge_keys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::appchain_deip_runtime::opaque::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                pub runtime_types::sp_core::crypto::KeyTypeId,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct note_stalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for note_stalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, note_stalled, DispatchError>
                {
                    let call = note_stalled { delay, best_finalized_block_number };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct sudo {
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct sudo_unchecked_weight {
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for sudo_unchecked_weight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_key {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_key {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct sudo_as {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for sudo_as {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo, DispatchError>
                {
                    let call = sudo { call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::appchain_deip_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_unchecked_weight, DispatchError>
                {
                    let call = sudo_unchecked_weight { call: ::std::boxed::Box::new(call), weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_key, DispatchError>
                {
                    let call = set_key { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_as, DispatchError>
                {
                    let call = sudo_as { who, call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct KeyChanged {
                pub new_sudoer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, heartbeat, DispatchError>
                {
                    let call = heartbeat { heartbeat, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct HeartbeatReceived {
                pub authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            }
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AllGood;
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SomeOffline {
                pub offline: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                )>,
            }
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8][..],
                    )?)
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod randomness_collective_flip {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn random_material(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = RandomMaterial;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod parity_tech_assets {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Created {
                pub asset_id: ::core::primitive::u32,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Issued {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub total_supply: ::core::primitive::u128,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Transferred {
                pub asset_id: ::core::primitive::u32,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Burned {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct TeamChanged {
                pub asset_id: ::core::primitive::u32,
                pub issuer: ::subxt::sp_core::crypto::AccountId32,
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct OwnerChanged {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Frozen {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Thawed {
                pub asset_id: ::core::primitive::u32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
                serde::Serialize,
            )]
            pub struct AssetFrozen {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetFrozen {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
                serde::Serialize,
            )]
            pub struct AssetThawed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetThawed {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
                serde::Serialize,
            )]
            pub struct Destroyed {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct ForceCreated {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct MetadataSet {
                pub asset_id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
                serde::Serialize,
            )]
            pub struct MetadataCleared {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct ApprovedTransfer {
                pub asset_id: ::core::primitive::u32,
                pub source: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct ApprovalCancelled {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct TransferredApproved {
                pub asset_id: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub destination: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for TransferredApproved {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
                serde::Serialize,
            )]
            pub struct AssetStatusChanged {
                pub asset_id: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "ParityTechAssets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Asset(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "ParityTechAssets";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_assets::types::AssetDetails<
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "ParityTechAssets";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Approvals(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "ParityTechAssets";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::pallet_assets::types::Approval<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Metadata(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Metadata {
                const PALLET: &'static str = "ParityTechAssets";
                const STORAGE: &'static str = "Metadata";
                type Value = runtime_types::pallet_assets::types::AssetMetadata<
                    ::core::primitive::u128,
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::AssetDetails<
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Asset(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetBalance<::core::primitive::u128, ()>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    _2: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_assets::types::Approval<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Approvals(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn approvals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Approvals>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn metadata(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Metadata(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn metadata_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Metadata>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn asset_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 193u8, 111u8, 242u8, 134u8, 35u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 122u8, 16u8, 243u8, 90u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn approval_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 122u8, 16u8, 243u8, 90u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod parity_tech_uniques {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_uniques::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Created {
                pub class: ::core::primitive::u32,
                pub creator: ::subxt::sp_core::crypto::AccountId32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ForceCreated {
                pub class: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ForceCreated {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Destroyed {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for Destroyed {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Issued {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Issued {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Transferred {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Transferred {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Burned {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for Burned {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Frozen {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for Frozen {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Thawed {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for Thawed {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassFrozen {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassFrozen {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ClassFrozen";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassThawed {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassThawed {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ClassThawed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct OwnerChanged {
                pub class: ::core::primitive::u32,
                pub new_owner: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for OwnerChanged {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct TeamChanged {
                pub class: ::core::primitive::u32,
                pub issuer: ::subxt::sp_core::crypto::AccountId32,
                pub admin: ::subxt::sp_core::crypto::AccountId32,
                pub freezer: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for TeamChanged {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ApprovedTransfer {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovedTransfer {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ApprovalCancelled {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner: ::subxt::sp_core::crypto::AccountId32,
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for ApprovalCancelled {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct AssetStatusChanged {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for AssetStatusChanged {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ClassMetadataSet {
                pub class: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for ClassMetadataSet {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ClassMetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ClassMetadataCleared {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Event for ClassMetadataCleared {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "ClassMetadataCleared";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MetadataSet {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Event for MetadataSet {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MetadataCleared {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Event for MetadataCleared {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Redeposited {
                pub class: ::core::primitive::u32,
                pub successful_instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Event for Redeposited {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "Redeposited";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AttributeSet {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeSet {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "AttributeSet";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AttributeCleared {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Event for AttributeCleared {
                const PALLET: &'static str = "ParityTechUniques";
                const EVENT: &'static str = "AttributeCleared";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Class(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Class {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "Class";
                type Value = runtime_types::pallet_uniques::types::ClassDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Account(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "Account";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Asset(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Asset {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "Asset";
                type Value = runtime_types::pallet_uniques::types::InstanceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ClassMetadataOf(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ClassMetadataOf {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "ClassMetadataOf";
                type Value =
                    runtime_types::pallet_uniques::types::ClassMetadata<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct InstanceMetadataOf(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for InstanceMetadataOf {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "InstanceMetadataOf";
                type Value =
                    runtime_types::pallet_uniques::types::InstanceMetadata<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Attribute(
                pub ::core::primitive::u32,
                pub ::core::option::Option<::core::primitive::u32>,
                pub  runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            );
            impl ::subxt::StorageEntry for Attribute {
                const PALLET: &'static str = "ParityTechUniques";
                const STORAGE: &'static str = "Attribute";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.2,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn class(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Class(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Class>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    _2: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = Account(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Asset(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Asset>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn class_metadata_of(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::ClassMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ClassMetadataOf(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn class_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ClassMetadataOf>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn instance_metadata_of(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_uniques::types::InstanceMetadata<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InstanceMetadataOf(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn instance_metadata_of_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InstanceMetadataOf>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn attribute(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::option::Option<::core::primitive::u32>,
                    _2: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Attribute(_0, _1, _2);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn attribute_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Attribute>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn class_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn instance_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn attribute_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn deposit_per_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 198u8, 164u8, 126u8, 141u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn string_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
                pub fn key_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
                pub fn value_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod mmr {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RootHash;
            impl ::subxt::StorageEntry for RootHash {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "RootHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NumberOfLeaves;
            impl ::subxt::StorageEntry for NumberOfLeaves {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "NumberOfLeaves";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nodes(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Nodes {
                const PALLET: &'static str = "Mmr";
                const STORAGE: &'static str = "Nodes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn root_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = RootHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn number_of_leaves(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = NumberOfLeaves;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nodes(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Nodes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nodes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nodes>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod beefy {
        use super::runtime_types;
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct as_multi_threshold_1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for as_multi_threshold_1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold_1";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call: ::subxt::WrapperKeepOpaque<runtime_types::appchain_deip_runtime::Call>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct approve_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for approve_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct cancel_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for cancel_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi_threshold_1, DispatchError>
                {
                    let call = as_multi_threshold_1 {
                        other_signatories,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::subxt::WrapperKeepOpaque<runtime_types::appchain_deip_runtime::Call>,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi, DispatchError>
                {
                    let call = as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_as_multi, DispatchError>
                {
                    let call = approve_as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_as_multi, DispatchError>
                {
                    let call =
                        cancel_as_multi { threshold, other_signatories, timepoint, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NewMultisig {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MultisigApproval {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MultisigExecuted {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<runtime_types::appchain_deip_runtime::Call>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<runtime_types::appchain_deip_runtime::Call>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 240u8, 28u8, 10u8, 219u8, 237u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 204u8, 123u8, 159u8, 174u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_signatories(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct batch {
                pub calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct as_derivative {
                pub index: ::core::primitive::u16,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for as_derivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct batch_all {
                pub calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for batch_all {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct dispatch_as {
                pub as_origin:
                    ::std::boxed::Box<runtime_types::appchain_deip_runtime::OriginCaller>,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for dispatch_as {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "dispatch_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch, DispatchError>
                {
                    let call = batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_derivative, DispatchError>
                {
                    let call = as_derivative { index, call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch_all, DispatchError>
                {
                    let call = batch_all { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::appchain_deip_runtime::OriginCaller,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, dispatch_as, DispatchError>
                {
                    let call = dispatch_as {
                        as_origin: ::std::boxed::Box::new(as_origin),
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BatchCompleted;
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ItemCompleted;
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DispatchedAs(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn batched_calls_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[170u8, 42u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod deip {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create_project {
                pub is_private: ::core::primitive::bool,
                pub external_id: runtime_types::primitive_types::H160,
                pub team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub description: ::subxt::sp_core::H256,
                pub domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
            }
            impl ::subxt::Call for create_project {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "create_project";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create_investment_opportunity {
                pub external_id: runtime_types::primitive_types::H160,
                pub creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub shares: ::std::vec::Vec<
                    runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                >,
                pub funding_model: runtime_types::pallet_deip::investment_opportunity::FundingModel<
                    ::core::primitive::u64,
                    runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                >,
            }
            impl ::subxt::Call for create_investment_opportunity {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "create_investment_opportunity";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct activate_crowdfunding {
                pub sale_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for activate_crowdfunding {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "activate_crowdfunding";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct expire_crowdfunding {
                pub sale_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for expire_crowdfunding {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "expire_crowdfunding";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct finish_crowdfunding {
                pub sale_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for finish_crowdfunding {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "finish_crowdfunding";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct invest {
                pub id: runtime_types::primitive_types::H160,
                pub asset: runtime_types::pallet_deip::asset::Asset<
                    runtime_types::primitive_types::H160,
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for invest {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "invest";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct update_project {
                pub project_id: runtime_types::primitive_types::H160,
                pub description: ::core::option::Option<::subxt::sp_core::H256>,
                pub is_private: ::core::option::Option<::core::primitive::bool>,
            }
            impl ::subxt::Call for update_project {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "update_project";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create_project_content {
                pub external_id: runtime_types::primitive_types::H160,
                pub project_external_id: runtime_types::primitive_types::H160,
                pub team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub content_type: runtime_types::pallet_deip::ProjectContentType,
                pub description: ::subxt::sp_core::H256,
                pub content: ::subxt::sp_core::H256,
                pub authors: ::std::vec::Vec<
                    runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                >,
                pub references:
                    ::core::option::Option<::std::vec::Vec<runtime_types::primitive_types::H160>>,
            }
            impl ::subxt::Call for create_project_content {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "create_project_content";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create_review {
                pub external_id: runtime_types::primitive_types::H160,
                pub author: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub content: ::subxt::sp_core::H256,
                pub domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                pub assessment_model: ::core::primitive::u32,
                pub weight: ::std::vec::Vec<::core::primitive::u8>,
                pub project_content_external_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for create_review {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "create_review";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct upvote_review {
                pub review_id: runtime_types::primitive_types::H160,
                pub domain_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for upvote_review {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "upvote_review";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct add_domain {
                pub domain: runtime_types::pallet_deip::Domain,
            }
            impl ::subxt::Call for add_domain {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "add_domain";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create_contract_agreement {
                pub id: runtime_types::primitive_types::H160,
                pub creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub parties: ::std::vec::Vec<
                    runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                >,
                pub hash: ::subxt::sp_core::H256,
                pub activation_time: ::core::option::Option<::core::primitive::u64>,
                pub expiration_time: ::core::option::Option<::core::primitive::u64>,
                pub terms: runtime_types::pallet_deip::contract::Terms<
                    runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                >,
            }
            impl ::subxt::Call for create_contract_agreement {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "create_contract_agreement";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct accept_contract_agreement {
                pub id: runtime_types::primitive_types::H160,
                pub party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for accept_contract_agreement {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "accept_contract_agreement";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct reject_contract_agreement {
                pub id: runtime_types::primitive_types::H160,
                pub party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for reject_contract_agreement {
                const PALLET: &'static str = "Deip";
                const FUNCTION: &'static str = "reject_contract_agreement";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn create_project(
                    &self,
                    is_private: ::core::primitive::bool,
                    external_id: runtime_types::primitive_types::H160,
                    team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    description: ::subxt::sp_core::H256,
                    domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create_project, DispatchError>
                {
                    let call =
                        create_project { is_private, external_id, team_id, description, domains };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_investment_opportunity(
                    &self,
                    external_id: runtime_types::primitive_types::H160,
                    creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    shares: ::std::vec::Vec<
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                    funding_model: runtime_types::pallet_deip::investment_opportunity::FundingModel<
                        ::core::primitive::u64,
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    create_investment_opportunity,
                    DispatchError,
                > {
                    let call = create_investment_opportunity {
                        external_id,
                        creator,
                        shares,
                        funding_model,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn activate_crowdfunding(
                    &self,
                    sale_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, activate_crowdfunding, DispatchError>
                {
                    let call = activate_crowdfunding { sale_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn expire_crowdfunding(
                    &self,
                    sale_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, expire_crowdfunding, DispatchError>
                {
                    let call = expire_crowdfunding { sale_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn finish_crowdfunding(
                    &self,
                    sale_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, finish_crowdfunding, DispatchError>
                {
                    let call = finish_crowdfunding { sale_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn invest(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    asset: runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, invest, DispatchError>
                {
                    let call = invest { id, asset };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_project(
                    &self,
                    project_id: runtime_types::primitive_types::H160,
                    description: ::core::option::Option<::subxt::sp_core::H256>,
                    is_private: ::core::option::Option<::core::primitive::bool>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, update_project, DispatchError>
                {
                    let call = update_project { project_id, description, is_private };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_project_content(
                    &self,
                    external_id: runtime_types::primitive_types::H160,
                    project_external_id: runtime_types::primitive_types::H160,
                    team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    content_type: runtime_types::pallet_deip::ProjectContentType,
                    description: ::subxt::sp_core::H256,
                    content: ::subxt::sp_core::H256,
                    authors: ::std::vec::Vec<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    references: ::core::option::Option<
                        ::std::vec::Vec<runtime_types::primitive_types::H160>,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create_project_content, DispatchError>
                {
                    let call = create_project_content {
                        external_id,
                        project_external_id,
                        team_id,
                        content_type,
                        description,
                        content,
                        authors,
                        references,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_review(
                    &self,
                    external_id: runtime_types::primitive_types::H160,
                    author: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    content: ::subxt::sp_core::H256,
                    domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                    assessment_model: ::core::primitive::u32,
                    weight: ::std::vec::Vec<::core::primitive::u8>,
                    project_content_external_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create_review, DispatchError>
                {
                    let call = create_review {
                        external_id,
                        author,
                        content,
                        domains,
                        assessment_model,
                        weight,
                        project_content_external_id,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn upvote_review(
                    &self,
                    review_id: runtime_types::primitive_types::H160,
                    domain_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, upvote_review, DispatchError>
                {
                    let call = upvote_review { review_id, domain_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_domain(
                    &self,
                    domain: runtime_types::pallet_deip::Domain,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_domain, DispatchError>
                {
                    let call = add_domain { domain };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create_contract_agreement(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    parties: ::std::vec::Vec<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    hash: ::subxt::sp_core::H256,
                    activation_time: ::core::option::Option<::core::primitive::u64>,
                    expiration_time: ::core::option::Option<::core::primitive::u64>,
                    terms: runtime_types::pallet_deip::contract::Terms<
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    create_contract_agreement,
                    DispatchError,
                > {
                    let call = create_contract_agreement {
                        id,
                        creator,
                        parties,
                        hash,
                        activation_time,
                        expiration_time,
                        terms,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn accept_contract_agreement(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    accept_contract_agreement,
                    DispatchError,
                > {
                    let call = accept_contract_agreement { id, party };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_contract_agreement(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    reject_contract_agreement,
                    DispatchError,
                > {
                    let call = reject_contract_agreement { id, party };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_deip::RawEvent<
            ::subxt::sp_core::crypto::AccountId32,
            runtime_types::pallet_deip::Project<
                ::subxt::sp_core::H256,
                ::subxt::sp_core::crypto::AccountId32,
            >,
            runtime_types::pallet_deip::review::Review<
                ::subxt::sp_core::H256,
                ::subxt::sp_core::crypto::AccountId32,
            >,
        >;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ProjectCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_deip::Project<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::Event for ProjectCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ProjectCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ProjectRemoved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_deip::Project<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::Event for ProjectRemoved {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ProjectRemoved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ProjectUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for ProjectUpdated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ProjectUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ProjectContnetCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for ProjectContnetCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ProjectContnetCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NdaCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for NdaCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "NdaCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NdaAccessRequestCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for NdaAccessRequestCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "NdaAccessRequestCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NdaAccessRequestFulfilled(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for NdaAccessRequestFulfilled {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "NdaAccessRequestFulfilled";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NdaAccessRequestRejected(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for NdaAccessRequestRejected {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "NdaAccessRequestRejected";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DomainAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for DomainAdded {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "DomainAdded";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ReviewCreated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  runtime_types::pallet_deip::review::Review<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            );
            impl ::subxt::Event for ReviewCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ReviewCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ReviewUpvoted(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::Event for ReviewUpvoted {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ReviewUpvoted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SimpleCrowdfundingCreated(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for SimpleCrowdfundingCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "SimpleCrowdfundingCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SimpleCrowdfundingActivated(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for SimpleCrowdfundingActivated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "SimpleCrowdfundingActivated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SimpleCrowdfundingFinished(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for SimpleCrowdfundingFinished {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "SimpleCrowdfundingFinished";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SimpleCrowdfundingExpired(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for SimpleCrowdfundingExpired {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "SimpleCrowdfundingExpired";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Invested(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Invested {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "Invested";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ContractAgreementCreated(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for ContractAgreementCreated {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ContractAgreementCreated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ContractAgreementAccepted(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ContractAgreementAccepted {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ContractAgreementAccepted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ContractAgreementFinalized(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for ContractAgreementFinalized {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ContractAgreementFinalized";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ContractAgreementRejected(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for ContractAgreementRejected {
                const PALLET: &'static str = "Deip";
                const EVENT: &'static str = "ContractAgreementRejected";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ProjectMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ProjectMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ProjectMap";
                type Value = runtime_types::pallet_deip::Project<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ProjectIdByTeamId(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ProjectIdByTeamId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ProjectIdByTeamId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct SimpleCrowdfundingMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for SimpleCrowdfundingMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "SimpleCrowdfundingMap";
                type Value = runtime_types::pallet_deip::investment_opportunity::Info<
                    ::core::primitive::u64,
                    runtime_types::primitive_types::H160,
                    ::core::primitive::u128,
                    runtime_types::deip_transaction_ctx::transaction_ctx::TransactionCtxId<
                        runtime_types::pallet_deip_portal::transaction_ctx::PortalCtx<
                            runtime_types::deip_transaction_ctx::transaction_ctx::TransactionCtx<
                                runtime_types::appchain_deip_runtime::Runtime,
                            >,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct InvestmentMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for InvestmentMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "InvestmentMap";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pallet_deip::contribution::Contribution<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u64,
                    >,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ProjectContentMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ProjectContentMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ProjectContentMap";
                type Value = runtime_types::pallet_deip::ProjectContent<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ContentIdByProjectId(
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ContentIdByProjectId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ContentIdByProjectId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Identity),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct Ndas;
            impl ::subxt::StorageEntry for Ndas {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "Ndas";
                type Value = ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    ::subxt::sp_core::crypto::AccountId32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NdaMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for NdaMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "NdaMap";
                type Value = runtime_types::pallet_deip::Nda<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct NdaAccessRequests;
            impl ::subxt::StorageEntry for NdaAccessRequests {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "NdaAccessRequests";
                type Value = ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    runtime_types::primitive_types::H160,
                    ::subxt::sp_core::crypto::AccountId32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NdaAccessRequestMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for NdaAccessRequestMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "NdaAccessRequestMap";
                type Value = runtime_types::pallet_deip::NdaAccessRequest<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ReviewMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ReviewMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ReviewMap";
                type Value = runtime_types::pallet_deip::review::Review<
                    ::subxt::sp_core::H256,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ReviewIdByProjectId(
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ReviewIdByProjectId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ReviewIdByProjectId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Identity),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct ReviewIdByContentId(
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ReviewIdByContentId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ReviewIdByContentId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Identity),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct ReviewIdByAccountId(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ReviewIdByAccountId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ReviewIdByAccountId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct ReviewVoteMap(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ReviewVoteMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ReviewVoteMap";
                type Value = runtime_types::pallet_deip::review::Vote<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u64,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct VoteIdByReviewId(
                pub runtime_types::primitive_types::H160,
                pub  (
                    runtime_types::primitive_types::H160,
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                ),
            );
            impl ::subxt::StorageEntry for VoteIdByReviewId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "VoteIdByReviewId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Identity),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VoteIdByAccountId(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub  (
                    runtime_types::primitive_types::H160,
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                ),
            );
            impl ::subxt::StorageEntry for VoteIdByAccountId {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "VoteIdByAccountId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Domains(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for Domains {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "Domains";
                type Value = runtime_types::pallet_deip::Domain;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct DomainCount;
            impl ::subxt::StorageEntry for DomainCount {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "DomainCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ContractAgreementMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ContractAgreementMap {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ContractAgreementMap";
                type Value = runtime_types::pallet_deip::contract::Agreement<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::sp_core::H256,
                    ::core::primitive::u64,
                    runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ContractAgreementIdByType(
                pub runtime_types::pallet_deip::contract::IndexTerms,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for ContractAgreementIdByType {
                const PALLET: &'static str = "Deip";
                const STORAGE: &'static str = "ContractAgreementIdByType";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn project_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::Project<
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ProjectMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn project_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ProjectMap>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn project_id_by_team_id(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ProjectIdByTeamId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn project_id_by_team_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProjectIdByTeamId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }                pub async fn simple_crowdfunding_map (& self , _0 : runtime_types :: primitive_types :: H160 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_deip :: investment_opportunity :: Info < :: core :: primitive :: u64 , runtime_types :: primitive_types :: H160 , :: core :: primitive :: u128 , runtime_types :: deip_transaction_ctx :: transaction_ctx :: TransactionCtxId < runtime_types :: pallet_deip_portal :: transaction_ctx :: PortalCtx < runtime_types :: deip_transaction_ctx :: transaction_ctx :: TransactionCtx < runtime_types :: appchain_deip_runtime :: Runtime > > > > , :: subxt :: BasicError >{
                    let entry = SimpleCrowdfundingMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn simple_crowdfunding_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SimpleCrowdfundingMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn investment_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_deip::contribution::Contribution<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u64,
                        >,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = InvestmentMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn investment_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InvestmentMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn project_content_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::ProjectContent<
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ProjectContentMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn project_content_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProjectContentMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn content_id_by_project_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ContentIdByProjectId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn content_id_by_project_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ContentIdByProjectId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn ndas(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Ndas;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nda_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::Nda<
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NdaMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nda_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NdaMap>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn nda_access_requests(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NdaAccessRequests;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nda_access_request_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::NdaAccessRequest<
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NdaAccessRequestMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nda_access_request_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NdaAccessRequestMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn review_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::review::Review<
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReviewMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn review_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ReviewMap>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn review_id_by_project_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ReviewIdByProjectId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn review_id_by_project_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReviewIdByProjectId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn review_id_by_content_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ReviewIdByContentId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn review_id_by_content_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReviewIdByContentId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn review_id_by_account_id(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ReviewIdByAccountId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn review_id_by_account_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReviewIdByAccountId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn review_vote_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    _2: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::review::Vote<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReviewVoteMap(_0, _1, _2);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn review_vote_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReviewVoteMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn vote_id_by_review_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: (
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    ),
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = VoteIdByReviewId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn vote_id_by_review_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VoteIdByReviewId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn vote_id_by_account_id(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: (
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    ),
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = VoteIdByAccountId(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn vote_id_by_account_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VoteIdByAccountId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn domains(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::pallet_deip::Domain, ::subxt::BasicError>
                {
                    let entry = Domains(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn domains_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Domains>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn domain_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = DomainCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn contract_agreement_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_deip::contract::Agreement<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                        ::core::primitive::u64,
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ContractAgreementMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn contract_agreement_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ContractAgreementMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn contract_agreement_id_by_type(
                    &self,
                    _0: runtime_types::pallet_deip::contract::IndexTerms,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<(), ::subxt::BasicError> {
                    let entry = ContractAgreementIdByType(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn contract_agreement_id_by_type_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ContractAgreementIdByType>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod assets {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create {
                pub id: ::core::primitive::u32,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_create {
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub is_sufficient: ::core::primitive::bool,
                pub min_balance: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_create {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct destroy {
                pub id: ::core::primitive::u32,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for destroy {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct mint {
                pub id: ::core::primitive::u32,
                pub beneficiary:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for mint {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct burn {
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for burn {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer {
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_keep_alive {
                pub id: ::core::primitive::u32,
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_transfer {
                pub id: ::core::primitive::u32,
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct freeze {
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for freeze {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct thaw {
                pub id: ::core::primitive::u32,
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for thaw {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct freeze_asset {
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for freeze_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "freeze_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct thaw_asset {
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for thaw_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "thaw_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_ownership {
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for transfer_ownership {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_team {
                pub id: ::core::primitive::u32,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_team {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_metadata {
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for set_metadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct clear_metadata {
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for clear_metadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_set_metadata {
                pub id: ::core::primitive::u32,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_set_metadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct force_clear_metadata {
                pub id: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_clear_metadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_asset_status {
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub min_balance: ::core::primitive::u128,
                pub is_sufficient: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_asset_status {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct approve_transfer {
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for approve_transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct cancel_approval {
                pub id: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for cancel_approval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_cancel_approval {
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for force_cancel_approval {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "force_cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_approved {
                pub id: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub destination:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_approved {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "transfer_approved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_create_asset {
                pub id: runtime_types::primitive_types::H160,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub min_balance: ::core::primitive::u128,
                pub project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
            }
            impl ::subxt::Call for deip_create_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_create_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_destroy {
                pub id: runtime_types::primitive_types::H160,
                pub witness: runtime_types::pallet_assets::types::DestroyWitness,
            }
            impl ::subxt::Call for deip_destroy {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_destroy";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_issue_asset {
                pub id: runtime_types::primitive_types::H160,
                pub beneficiary: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for deip_issue_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_issue_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_burn {
                pub id: runtime_types::primitive_types::H160,
                pub who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for deip_burn {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_burn";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_transfer {
                pub id: runtime_types::primitive_types::H160,
                pub target: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                #[codec(compact)]
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for deip_transfer {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_freeze {
                pub id: runtime_types::primitive_types::H160,
                pub who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_freeze {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_freeze";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_thaw {
                pub id: runtime_types::primitive_types::H160,
                pub who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_thaw {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_thaw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_freeze_asset {
                pub id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for deip_freeze_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_freeze_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_thaw_asset {
                pub id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for deip_thaw_asset {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_thaw_asset";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_transfer_ownership {
                pub id: runtime_types::primitive_types::H160,
                pub owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_transfer_ownership {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_transfer_ownership";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_team {
                pub id: runtime_types::primitive_types::H160,
                pub issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_set_team {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_set_team";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_metadata {
                pub id: runtime_types::primitive_types::H160,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimals: ::core::primitive::u8,
            }
            impl ::subxt::Call for deip_set_metadata {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_wipe_zero_balance {
                pub asset: runtime_types::primitive_types::H160,
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for deip_wipe_zero_balance {
                const PALLET: &'static str = "Assets";
                const FUNCTION: &'static str = "deip_wipe_zero_balance";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn create(
                    &self,
                    id: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create { id, admin, min_balance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    is_sufficient: ::core::primitive::bool,
                    min_balance: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_create, DispatchError>
                {
                    let call = force_create { id, owner, is_sufficient, min_balance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    id: ::core::primitive::u32,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, destroy, DispatchError>
                {
                    let call = destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    id: ::core::primitive::u32,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, mint, DispatchError>
                {
                    let call = mint { id, beneficiary, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, burn, DispatchError>
                {
                    let call = burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: ::core::primitive::u32,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_keep_alive, DispatchError>
                {
                    let call = transfer_keep_alive { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer { id, source, dest, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, freeze, DispatchError>
                {
                    let call = freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    id: ::core::primitive::u32,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, thaw, DispatchError>
                {
                    let call = thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, freeze_asset, DispatchError>
                {
                    let call = freeze_asset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_asset(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, thaw_asset, DispatchError>
                {
                    let call = thaw_asset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_ownership, DispatchError>
                {
                    let call = transfer_ownership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    id: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_team, DispatchError>
                {
                    let call = set_team { id, issuer, admin, freezer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_metadata, DispatchError>
                {
                    let call = set_metadata { id, name, symbol, decimals };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_metadata, DispatchError>
                {
                    let call = clear_metadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_set_metadata(
                    &self,
                    id: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_set_metadata, DispatchError>
                {
                    let call = force_set_metadata { id, name, symbol, decimals, is_frozen };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_clear_metadata(
                    &self,
                    id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_clear_metadata, DispatchError>
                {
                    let call = force_clear_metadata { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    min_balance: ::core::primitive::u128,
                    is_sufficient: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_asset_status, DispatchError>
                {
                    let call = force_asset_status {
                        id,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        min_balance,
                        is_sufficient,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_transfer, DispatchError>
                {
                    let call = approve_transfer { id, delegate, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_approval, DispatchError>
                {
                    let call = cancel_approval { id, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_cancel_approval(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_cancel_approval, DispatchError>
                {
                    let call = force_cancel_approval { id, owner, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_approved(
                    &self,
                    id: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    destination: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_approved, DispatchError>
                {
                    let call = transfer_approved { id, owner, destination, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_create_asset(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    min_balance: ::core::primitive::u128,
                    project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_create_asset, DispatchError>
                {
                    let call = deip_create_asset { id, admin, min_balance, project_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_destroy(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    witness: runtime_types::pallet_assets::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_destroy, DispatchError>
                {
                    let call = deip_destroy { id, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_issue_asset(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    beneficiary: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_issue_asset, DispatchError>
                {
                    let call = deip_issue_asset { id, beneficiary, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_burn(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_burn, DispatchError>
                {
                    let call = deip_burn { id, who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_transfer(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    target: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_transfer, DispatchError>
                {
                    let call = deip_transfer { id, target, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_freeze(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_freeze, DispatchError>
                {
                    let call = deip_freeze { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_thaw(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_thaw, DispatchError>
                {
                    let call = deip_thaw { id, who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_freeze_asset(
                    &self,
                    id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_freeze_asset, DispatchError>
                {
                    let call = deip_freeze_asset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_thaw_asset(
                    &self,
                    id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_thaw_asset, DispatchError>
                {
                    let call = deip_thaw_asset { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_transfer_ownership(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    deip_transfer_ownership,
                    DispatchError,
                > {
                    let call = deip_transfer_ownership { id, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_team(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_set_team, DispatchError>
                {
                    let call = deip_set_team { id, issuer, admin, freezer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_metadata(
                    &self,
                    id: runtime_types::primitive_types::H160,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    symbol: ::std::vec::Vec<::core::primitive::u8>,
                    decimals: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_set_metadata, DispatchError>
                {
                    let call = deip_set_metadata { id, name, symbol, decimals };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_wipe_zero_balance(
                    &self,
                    asset: runtime_types::primitive_types::H160,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_wipe_zero_balance, DispatchError>
                {
                    let call = deip_wipe_zero_balance { asset, account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AssetIdByDeipAssetId(
                pub runtime_types::primitive_types::H160,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for AssetIdByDeipAssetId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "AssetIdByDeipAssetId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Identity),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct DeipAssetIdByAssetId(
                pub ::core::primitive::u32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for DeipAssetIdByAssetId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "DeipAssetIdByAssetId";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Identity),
                    ])
                }
            }
            pub struct NextAssetId;
            impl ::subxt::StorageEntry for NextAssetId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "NextAssetId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssetIdByProjectId(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for AssetIdByProjectId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "AssetIdByProjectId";
                type Value = ::std::vec::Vec<runtime_types::primitive_types::H160>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ProjectIdByAssetId(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ProjectIdByAssetId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "ProjectIdByAssetId";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct InvestmentByAssetId(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for InvestmentByAssetId {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "InvestmentByAssetId";
                type Value = ::std::vec::Vec<runtime_types::primitive_types::H160>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct InvestmentMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for InvestmentMap {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "InvestmentMap";
                type Value = runtime_types::pallet_deip_assets::pallet::Investment<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct FtBalanceMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for FtBalanceMap {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "FtBalanceMap";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct AssetMetadataMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for AssetMetadataMap {
                const PALLET: &'static str = "Assets";
                const STORAGE: &'static str = "AssetMetadataMap";
                type Value =
                    runtime_types::pallet_deip_assets::pallet::AssetMetadata<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn asset_id_by_deip_asset_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = AssetIdByDeipAssetId(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_id_by_deip_asset_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByDeipAssetId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn deip_asset_id_by_asset_id(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = DeipAssetIdByAssetId(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn deip_asset_id_by_asset_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DeipAssetIdByAssetId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_asset_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextAssetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_id_by_project_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<runtime_types::primitive_types::H160>>,
                    ::subxt::BasicError,
                > {
                    let entry = AssetIdByProjectId(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_id_by_project_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetIdByProjectId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn project_id_by_asset_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::primitive_types::H160>,
                    ::subxt::BasicError,
                > {
                    let entry = ProjectIdByAssetId(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn project_id_by_asset_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProjectIdByAssetId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn investment_by_asset_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<runtime_types::primitive_types::H160>>,
                    ::subxt::BasicError,
                > {
                    let entry = InvestmentByAssetId(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn investment_by_asset_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InvestmentByAssetId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn investment_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_assets::pallet::Investment<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InvestmentMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn investment_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InvestmentMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn ft_balance_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>>,
                    ::subxt::BasicError,
                > {
                    let entry = FtBalanceMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ft_balance_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, FtBalanceMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_metadata_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_assets::pallet::AssetMetadata<
                            ::core::primitive::u8,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = AssetMetadataMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn asset_metadata_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AssetMetadataMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn wipe_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[128u8, 112u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod uniques {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create {
                pub class: ::core::primitive::u32,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_create {
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub free_holding: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct destroy {
                pub class: ::core::primitive::u32,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            impl ::subxt::Call for destroy {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "destroy";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct mint {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for mint {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "mint";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct burn {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub check_owner: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for burn {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "burn";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct redeposit {
                pub class: ::core::primitive::u32,
                pub instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for redeposit {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "redeposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct freeze {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for freeze {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct thaw {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for thaw {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct freeze_class {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for freeze_class {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "freeze_class";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct thaw_class {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for thaw_class {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "thaw_class";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct transfer_ownership {
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for transfer_ownership {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "transfer_ownership";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_team {
                pub class: ::core::primitive::u32,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_team {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_team";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct approve_transfer {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub delegate:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for approve_transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "approve_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct cancel_approval {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub maybe_check_delegate: ::core::option::Option<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for cancel_approval {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct force_asset_status {
                pub class: ::core::primitive::u32,
                pub owner:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub issuer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub admin:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub freezer:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub free_holding: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for force_asset_status {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "force_asset_status";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_attribute {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for set_attribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_attribute";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct clear_attribute {
                pub class: ::core::primitive::u32,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for clear_attribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_attribute";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_metadata {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct clear_metadata {
                pub class: ::core::primitive::u32,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for clear_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct set_class_metadata {
                pub class: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_class_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "set_class_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct clear_class_metadata {
                pub class: ::core::primitive::u32,
            }
            impl ::subxt::Call for clear_class_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "clear_class_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_create {
                pub class: runtime_types::primitive_types::H160,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
            }
            impl ::subxt::Call for deip_create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_force_create {
                pub class: runtime_types::primitive_types::H160,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                pub free_holding: ::core::primitive::bool,
            }
            impl ::subxt::Call for deip_force_create {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_force_create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_destroy {
                pub class: runtime_types::primitive_types::H160,
                pub witness: runtime_types::pallet_uniques::types::DestroyWitness,
            }
            impl ::subxt::Call for deip_destroy {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_destroy";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_mint {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_mint {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_mint";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_burn {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub check_owner: ::core::option::Option<
                    runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                >,
            }
            impl ::subxt::Call for deip_burn {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_burn";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_transfer {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub dest: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_redeposit {
                pub class: runtime_types::primitive_types::H160,
                pub instances: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for deip_redeposit {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_redeposit";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_freeze {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for deip_freeze {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_freeze";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_thaw {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for deip_thaw {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_thaw";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_freeze_class {
                pub class: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for deip_freeze_class {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_freeze_class";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_thaw_class {
                pub class: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for deip_thaw_class {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_thaw_class";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_transfer_ownership {
                pub class: runtime_types::primitive_types::H160,
                pub owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_transfer_ownership {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_transfer_ownership";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_team {
                pub class: runtime_types::primitive_types::H160,
                pub issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_set_team {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_set_team";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_approve_transfer {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub delegate: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            }
            impl ::subxt::Call for deip_approve_transfer {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_approve_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_cancel_approval {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub maybe_check_delegate: ::core::option::Option<
                    runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                >,
            }
            impl ::subxt::Call for deip_cancel_approval {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_cancel_approval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_force_asset_status {
                pub class: runtime_types::primitive_types::H160,
                pub owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
                pub free_holding: ::core::primitive::bool,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for deip_force_asset_status {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_force_asset_status";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_attribute {
                pub class: runtime_types::primitive_types::H160,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for deip_set_attribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_set_attribute";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_clear_attribute {
                pub class: runtime_types::primitive_types::H160,
                pub maybe_instance: ::core::option::Option<::core::primitive::u32>,
                pub key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
            }
            impl ::subxt::Call for deip_clear_attribute {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_clear_attribute";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_metadata {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for deip_set_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_set_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_clear_metadata {
                pub class: runtime_types::primitive_types::H160,
                pub instance: ::core::primitive::u32,
            }
            impl ::subxt::Call for deip_clear_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_clear_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_set_class_metadata {
                pub class: runtime_types::primitive_types::H160,
                pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub is_frozen: ::core::primitive::bool,
            }
            impl ::subxt::Call for deip_set_class_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_set_class_metadata";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct deip_clear_class_metadata {
                pub class: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for deip_clear_class_metadata {
                const PALLET: &'static str = "Uniques";
                const FUNCTION: &'static str = "deip_clear_class_metadata";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn create(
                    &self,
                    class: ::core::primitive::u32,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create { class, admin };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_create(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_create, DispatchError>
                {
                    let call = force_create { class, owner, free_holding };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn destroy(
                    &self,
                    class: ::core::primitive::u32,
                    witness: runtime_types::pallet_uniques::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, destroy, DispatchError>
                {
                    let call = destroy { class, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn mint(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, mint, DispatchError>
                {
                    let call = mint { class, instance, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn burn(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    check_owner: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, burn, DispatchError>
                {
                    let call = burn { class, instance, check_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { class, instance, dest };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn redeposit(
                    &self,
                    class: ::core::primitive::u32,
                    instances: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, redeposit, DispatchError>
                {
                    let call = redeposit { class, instances };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, freeze, DispatchError>
                {
                    let call = freeze { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, thaw, DispatchError>
                {
                    let call = thaw { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn freeze_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, freeze_class, DispatchError>
                {
                    let call = freeze_class { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn thaw_class(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, thaw_class, DispatchError>
                {
                    let call = thaw_class { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_ownership(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_ownership, DispatchError>
                {
                    let call = transfer_ownership { class, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_team(
                    &self,
                    class: ::core::primitive::u32,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_team, DispatchError>
                {
                    let call = set_team { class, issuer, admin, freezer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_transfer(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    delegate: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_transfer, DispatchError>
                {
                    let call = approve_transfer { class, instance, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_approval(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    maybe_check_delegate: ::core::option::Option<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_approval, DispatchError>
                {
                    let call = cancel_approval { class, instance, maybe_check_delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_asset_status(
                    &self,
                    class: ::core::primitive::u32,
                    owner: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    issuer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    admin: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    freezer: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    free_holding: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_asset_status, DispatchError>
                {
                    let call = force_asset_status {
                        class,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        free_holding,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_attribute, DispatchError>
                {
                    let call = set_attribute { class, maybe_instance, key, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_attribute(
                    &self,
                    class: ::core::primitive::u32,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_attribute, DispatchError>
                {
                    let call = clear_attribute { class, maybe_instance, key };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_metadata, DispatchError>
                {
                    let call = set_metadata { class, instance, data, is_frozen };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_metadata, DispatchError>
                {
                    let call = clear_metadata { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_class_metadata, DispatchError>
                {
                    let call = set_class_metadata { class, data, is_frozen };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn clear_class_metadata(
                    &self,
                    class: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, clear_class_metadata, DispatchError>
                {
                    let call = clear_class_metadata { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_create(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_create, DispatchError>
                {
                    let call = deip_create { class, admin, project_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_force_create(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                    free_holding: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_force_create, DispatchError>
                {
                    let call = deip_force_create { class, admin, project_id, free_holding };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_destroy(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    witness: runtime_types::pallet_uniques::types::DestroyWitness,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_destroy, DispatchError>
                {
                    let call = deip_destroy { class, witness };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_mint(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_mint, DispatchError>
                {
                    let call = deip_mint { class, instance, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_burn(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    check_owner: ::core::option::Option<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_burn, DispatchError>
                {
                    let call = deip_burn { class, instance, check_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_transfer(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    dest: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_transfer, DispatchError>
                {
                    let call = deip_transfer { class, instance, dest };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_redeposit(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instances: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_redeposit, DispatchError>
                {
                    let call = deip_redeposit { class, instances };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_freeze(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_freeze, DispatchError>
                {
                    let call = deip_freeze { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_thaw(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_thaw, DispatchError>
                {
                    let call = deip_thaw { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_freeze_class(
                    &self,
                    class: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_freeze_class, DispatchError>
                {
                    let call = deip_freeze_class { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_thaw_class(
                    &self,
                    class: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_thaw_class, DispatchError>
                {
                    let call = deip_thaw_class { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_transfer_ownership(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    deip_transfer_ownership,
                    DispatchError,
                > {
                    let call = deip_transfer_ownership { class, owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_team(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_set_team, DispatchError>
                {
                    let call = deip_set_team { class, issuer, admin, freezer };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_approve_transfer(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    delegate: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_approve_transfer, DispatchError>
                {
                    let call = deip_approve_transfer { class, instance, delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_cancel_approval(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    maybe_check_delegate: ::core::option::Option<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_cancel_approval, DispatchError>
                {
                    let call = deip_cancel_approval { class, instance, maybe_check_delegate };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_force_asset_status(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    free_holding: ::core::primitive::bool,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    deip_force_asset_status,
                    DispatchError,
                > {
                    let call = deip_force_asset_status {
                        class,
                        owner,
                        issuer,
                        admin,
                        freezer,
                        free_holding,
                        is_frozen,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_attribute(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_set_attribute, DispatchError>
                {
                    let call = deip_set_attribute { class, maybe_instance, key, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_clear_attribute(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    maybe_instance: ::core::option::Option<::core::primitive::u32>,
                    key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_clear_attribute, DispatchError>
                {
                    let call = deip_clear_attribute { class, maybe_instance, key };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_metadata(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_set_metadata, DispatchError>
                {
                    let call = deip_set_metadata { class, instance, data, is_frozen };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_clear_metadata(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    instance: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deip_clear_metadata, DispatchError>
                {
                    let call = deip_clear_metadata { class, instance };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_set_class_metadata(
                    &self,
                    class: runtime_types::primitive_types::H160,
                    data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    is_frozen: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    deip_set_class_metadata,
                    DispatchError,
                > {
                    let call = deip_set_class_metadata { class, data, is_frozen };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deip_clear_class_metadata(
                    &self,
                    class: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    deip_clear_class_metadata,
                    DispatchError,
                > {
                    let call = deip_clear_class_metadata { class };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct NftClassIdByDeipNftClassId(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for NftClassIdByDeipNftClassId {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "NftClassIdByDeipNftClassId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct NextNftClassId;
            impl ::subxt::StorageEntry for NextNftClassId {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "NextNftClassId";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ProjectIdByNftClassId(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ProjectIdByNftClassId {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "ProjectIdByNftClassId";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct NftBalanceMap(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for NftBalanceMap {
                const PALLET: &'static str = "Uniques";
                const STORAGE: &'static str = "NftBalanceMap";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn nft_class_id_by_deip_nft_class_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NftClassIdByDeipNftClassId(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nft_class_id_by_deip_nft_class_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NftClassIdByDeipNftClassId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn next_nft_class_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = NextNftClassId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn project_id_by_nft_class_id(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::primitive_types::H160>,
                    ::subxt::BasicError,
                > {
                    let entry = ProjectIdByNftClassId(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn project_id_by_nft_class_id_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProjectIdByNftClassId>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nft_balance_map(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>>,
                    ::subxt::BasicError,
                > {
                    let entry = NftBalanceMap(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nft_balance_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NftBalanceMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod deip_proposal {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct propose {
                pub batch: ::std::vec::Vec<
                    runtime_types::pallet_deip_proposal::proposal::BatchItem<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        runtime_types::appchain_deip_runtime::Call,
                    >,
                >,
                pub external_id: ::core::option::Option<runtime_types::primitive_types::H160>,
            }
            impl ::subxt::Call for propose {
                const PALLET: &'static str = "DeipProposal";
                const FUNCTION: &'static str = "propose";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct decide {
                pub proposal_id: runtime_types::primitive_types::H160,
                pub decision: runtime_types::pallet_deip_proposal::proposal::ProposalMemberDecision,
                pub batch_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for decide {
                const PALLET: &'static str = "DeipProposal";
                const FUNCTION: &'static str = "decide";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct expire {
                pub proposal_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for expire {
                const PALLET: &'static str = "DeipProposal";
                const FUNCTION: &'static str = "expire";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn propose(
                    &self,
                    batch: ::std::vec::Vec<
                        runtime_types::pallet_deip_proposal::proposal::BatchItem<
                            runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::primitive_types::H160,
                            >,
                            runtime_types::appchain_deip_runtime::Call,
                        >,
                    >,
                    external_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, propose, DispatchError>
                {
                    let call = propose { batch, external_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn decide(
                    &self,
                    proposal_id: runtime_types::primitive_types::H160,
                    decision: runtime_types::pallet_deip_proposal::proposal::ProposalMemberDecision,
                    batch_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, decide, DispatchError>
                {
                    let call = decide { proposal_id, decision, batch_weight };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn expire(
                    &self,
                    proposal_id: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, expire, DispatchError>
                {
                    let call = expire { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_deip_proposal::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Proposed {
                pub author: ::subxt::sp_core::crypto::AccountId32,
                pub batch: ::std::vec::Vec<
                    runtime_types::pallet_deip_proposal::proposal::BatchItem<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::appchain_deip_runtime::Call,
                    >,
                >,
                pub proposal_id: runtime_types::primitive_types::H160,
                pub batch_weight: ::core::primitive::u64,
            }
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "DeipProposal";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Approved {
                pub member: ::subxt::sp_core::crypto::AccountId32,
                pub proposal_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Event for Approved {
                const PALLET: &'static str = "DeipProposal";
                const EVENT: &'static str = "Approved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct RevokedApproval {
                pub member: ::subxt::sp_core::crypto::AccountId32,
                pub proposal_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Event for RevokedApproval {
                const PALLET: &'static str = "DeipProposal";
                const EVENT: &'static str = "RevokedApproval";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Resolved {
                pub member: ::subxt::sp_core::crypto::AccountId32,
                pub proposal_id: runtime_types::primitive_types::H160,
                pub state: runtime_types::pallet_deip_proposal::proposal::ProposalState,
            }
            impl ::subxt::Event for Resolved {
                const PALLET: &'static str = "DeipProposal";
                const EVENT: &'static str = "Resolved";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Expired {
                pub proposal_id: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Event for Expired {
                const PALLET: &'static str = "DeipProposal";
                const EVENT: &'static str = "Expired";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ProposalRepository(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for ProposalRepository {
                const PALLET: &'static str = "DeipProposal";
                const STORAGE: &'static str = "ProposalRepository";
                type Value = runtime_types::pallet_deip_proposal::proposal::DeipProposal<
                    runtime_types::appchain_deip_runtime::Runtime,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposal_repository(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_proposal::proposal::DeipProposal<
                            runtime_types::appchain_deip_runtime::Runtime,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ProposalRepository(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposal_repository_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ProposalRepository>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn ttl(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 132u8, 12u8, 36u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn expire_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[176u8, 4u8, 0u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod deip_dao {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create {
                pub name: runtime_types::primitive_types::H160,
                pub authority: runtime_types::pallet_deip_dao::pallet::dao::InputAuthority<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub metadata: ::core::option::Option<::subxt::sp_core::H256>,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "DeipDao";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct alter_authority {
                pub authority: runtime_types::pallet_deip_dao::pallet::dao::AlterAuthority<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for alter_authority {
                const PALLET: &'static str = "DeipDao";
                const FUNCTION: &'static str = "alter_authority";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct update_dao {
                pub new_metadata: ::core::option::Option<::subxt::sp_core::H256>,
            }
            impl ::subxt::Call for update_dao {
                const PALLET: &'static str = "DeipDao";
                const FUNCTION: &'static str = "update_dao";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct on_behalf {
                pub name: runtime_types::primitive_types::H160,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for on_behalf {
                const PALLET: &'static str = "DeipDao";
                const FUNCTION: &'static str = "on_behalf";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn create(
                    &self,
                    name: runtime_types::primitive_types::H160,
                    authority: runtime_types::pallet_deip_dao::pallet::dao::InputAuthority<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    metadata: ::core::option::Option<::subxt::sp_core::H256>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create { name, authority, metadata };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn alter_authority(
                    &self,
                    authority: runtime_types::pallet_deip_dao::pallet::dao::AlterAuthority<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, alter_authority, DispatchError>
                {
                    let call = alter_authority { authority };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_dao(
                    &self,
                    new_metadata: ::core::option::Option<::subxt::sp_core::H256>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, update_dao, DispatchError>
                {
                    let call = update_dao { new_metadata };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn on_behalf(
                    &self,
                    name: runtime_types::primitive_types::H160,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, on_behalf, DispatchError>
                {
                    let call = on_behalf { name, call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_deip_dao::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DaoCreate(
                pub  runtime_types::pallet_deip_dao::pallet::dao::Dao<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            );
            impl ::subxt::Event for DaoCreate {
                const PALLET: &'static str = "DeipDao";
                const EVENT: &'static str = "DaoCreate";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DaoAlterAuthority(
                pub  runtime_types::pallet_deip_dao::pallet::dao::Dao<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            );
            impl ::subxt::Event for DaoAlterAuthority {
                const PALLET: &'static str = "DeipDao";
                const EVENT: &'static str = "DaoAlterAuthority";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct DaoMetadataUpdated(
                pub  runtime_types::pallet_deip_dao::pallet::dao::Dao<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >,
            );
            impl ::subxt::Event for DaoMetadataUpdated {
                const PALLET: &'static str = "DeipDao";
                const EVENT: &'static str = "DaoMetadataUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct DaoRepository(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for DaoRepository {
                const PALLET: &'static str = "DeipDao";
                const STORAGE: &'static str = "DaoRepository";
                type Value = runtime_types::pallet_deip_dao::pallet::dao::Dao<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::primitive_types::H160,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct DaoLookup(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for DaoLookup {
                const PALLET: &'static str = "DeipDao";
                const STORAGE: &'static str = "DaoLookup";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn dao_repository(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_dao::pallet::dao::Dao<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = DaoRepository(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn dao_repository_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DaoRepository>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn dao_lookup(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::primitive_types::H160>,
                    ::subxt::BasicError,
                > {
                    let entry = DaoLookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn dao_lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, DaoLookup>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_signatories(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[50u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod deip_portal {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct create {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub metadata: ::core::option::Option<::subxt::sp_core::H256>,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "DeipPortal";
                const FUNCTION: &'static str = "create";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct update {
                pub update: runtime_types::pallet_deip_portal::portal::PortalUpdate<
                    runtime_types::appchain_deip_runtime::Runtime,
                >,
            }
            impl ::subxt::Call for update {
                const PALLET: &'static str = "DeipPortal";
                const FUNCTION: &'static str = "update";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct schedule { pub xt : :: std :: boxed :: Box < runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , () > , runtime_types :: appchain_deip_runtime :: Call , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > , }
            impl ::subxt::Call for schedule {
                const PALLET: &'static str = "DeipPortal";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct exec {
                pub portal_id: runtime_types::primitive_types::H160,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for exec {
                const PALLET: &'static str = "DeipPortal";
                const FUNCTION: &'static str = "exec";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct exec_postponed {
                pub portal_id: runtime_types::primitive_types::H160,
                pub call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
            }
            impl ::subxt::Call for exec_postponed {
                const PALLET: &'static str = "DeipPortal";
                const FUNCTION: &'static str = "exec_postponed";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn create(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    metadata: ::core::option::Option<::subxt::sp_core::H256>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create { delegate, metadata };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update(
                    &self,
                    update: runtime_types::pallet_deip_portal::portal::PortalUpdate<
                        runtime_types::appchain_deip_runtime::Runtime,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, update, DispatchError>
                {
                    let call = update { update };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule(
                    &self,
                    xt : runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , () > , runtime_types :: appchain_deip_runtime :: Call , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, schedule, DispatchError>
                {
                    let call = schedule { xt: ::std::boxed::Box::new(xt) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn exec(
                    &self,
                    portal_id: runtime_types::primitive_types::H160,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, exec, DispatchError>
                {
                    let call = exec { portal_id, call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn exec_postponed(
                    &self,
                    portal_id: runtime_types::primitive_types::H160,
                    call: runtime_types::appchain_deip_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, exec_postponed, DispatchError>
                {
                    let call = exec_postponed { portal_id, call: ::std::boxed::Box::new(call) };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingTx(pub ::core::primitive::u32, pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for PendingTx {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "PendingTx";
                type Value = runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , () > , runtime_types :: appchain_deip_runtime :: Call , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct ScheduledTx(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for ScheduledTx {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "ScheduledTx";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct PortalTagOfTransaction(
                pub ::core::primitive::u32,
                pub runtime_types::primitive_types::H160,
            );
            impl ::subxt::StorageEntry for PortalTagOfTransaction {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "PortalTagOfTransaction";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct PortalRepository(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for PortalRepository {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "PortalRepository";
                type Value = runtime_types::pallet_deip_portal::portal::Portal<
                    runtime_types::appchain_deip_runtime::Runtime,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct DelegateLookup(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for DelegateLookup {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "DelegateLookup";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OwnerLookup(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for OwnerLookup {
                const PALLET: &'static str = "DeipPortal";
                const STORAGE: &'static str = "OwnerLookup";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }                pub async fn pending_tx (& self , _0 : :: core :: primitive :: u32 , _1 : [:: core :: primitive :: u8 ; 32usize] , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , () > , runtime_types :: appchain_deip_runtime :: Call , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > , :: subxt :: BasicError >{
                    let entry = PendingTx(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pending_tx_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, PendingTx>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn scheduled_tx(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::primitive_types::H160>,
                    ::subxt::BasicError,
                > {
                    let entry = ScheduledTx(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn scheduled_tx_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ScheduledTx>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn portal_tag_of_transaction(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u32>>,
                    ::subxt::BasicError,
                > {
                    let entry = PortalTagOfTransaction(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn portal_tag_of_transaction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PortalTagOfTransaction>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn portal_repository(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_portal::portal::Portal<
                            runtime_types::appchain_deip_runtime::Runtime,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PortalRepository(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn portal_repository_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PortalRepository>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn delegate_lookup(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = DelegateLookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn delegate_lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, DelegateLookup>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn owner_lookup(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::primitive_types::H160>,
                    ::subxt::BasicError,
                > {
                    let entry = OwnerLookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn owner_lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, OwnerLookup>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod deip_vesting {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct vested_transfer {
                pub target:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub plan: runtime_types::pallet_deip_vesting::pallet::VestingPlan<
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for vested_transfer {
                const PALLET: &'static str = "DeipVesting";
                const FUNCTION: &'static str = "vested_transfer";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct unlock;
            impl ::subxt::Call for unlock {
                const PALLET: &'static str = "DeipVesting";
                const FUNCTION: &'static str = "unlock";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client, marker: ::core::marker::PhantomData }
                }
                pub fn vested_transfer(
                    &self,
                    target: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    plan: runtime_types::pallet_deip_vesting::pallet::VestingPlan<
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, vested_transfer, DispatchError>
                {
                    let call = vested_transfer { target, plan };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unlock(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, unlock, DispatchError>
                {
                    let call = unlock {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_deip_vesting::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct VestingUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for VestingUpdated {
                const PALLET: &'static str = "DeipVesting";
                const EVENT: &'static str = "VestingUpdated";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct VestingCompleted(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for VestingCompleted {
                const PALLET: &'static str = "DeipVesting";
                const EVENT: &'static str = "VestingCompleted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct VestingPlans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for VestingPlans {
                const PALLET: &'static str = "DeipVesting";
                const STORAGE: &'static str = "VestingPlans";
                type Value = runtime_types::pallet_deip_vesting::pallet::VestingPlan<
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn vesting_plans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_deip_vesting::pallet::VestingPlan<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = VestingPlans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn vesting_plans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VestingPlans>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn min_vested_transfer(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod deip_ecosystem_fund {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct FeeRecipient;
            impl ::subxt::StorageEntry for FeeRecipient {
                const PALLET: &'static str = "DeipEcosystemFund";
                const STORAGE: &'static str = "FeeRecipient";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn fee_recipient(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = FeeRecipient;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod appchain_deip_runtime {
            use super::runtime_types;
            pub mod deip_account {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum DeipAccountId<_0, _1> {
                    #[codec(index = 0)]
                    Native(_0),
                    #[codec(index = 1)]
                    Dao(_1),
                }
            }
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    pub beefy: runtime_types::beefy_primitives::crypto::Public,
                    pub octopus: runtime_types::pallet_octopus_appchain::crypto::Public,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 3)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Call),
                #[codec(index = 7)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Call),
                #[codec(index = 8)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Call),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 11)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 19)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 20)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 21)]
                Deip(runtime_types::pallet_deip::Call),
                #[codec(index = 22)]
                Assets(runtime_types::pallet_deip_assets::pallet::Call),
                #[codec(index = 23)]
                Uniques(runtime_types::pallet_deip_uniques::pallet::Call),
                #[codec(index = 24)]
                DeipProposal(runtime_types::pallet_deip_proposal::pallet::Call),
                #[codec(index = 25)]
                DeipDao(runtime_types::pallet_deip_dao::pallet::Call),
                #[codec(index = 26)]
                DeipPortal(runtime_types::pallet_deip_portal::pallet::Call),
                #[codec(index = 27)]
                DeipVesting(runtime_types::pallet_deip_vesting::pallet::Call),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 6)]
                OctopusAppchain(runtime_types::pallet_octopus_appchain::pallet::Event),
                #[codec(index = 7)]
                OctopusLpos(runtime_types::pallet_octopus_lpos::pallet::Event),
                #[codec(index = 8)]
                OctopusUpwardMessages(runtime_types::pallet_octopus_upward_messages::pallet::Event),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 10)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 11)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 12)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 15)]
                ParityTechAssets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 16)]
                ParityTechUniques(runtime_types::pallet_uniques::pallet::Event),
                #[codec(index = 19)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 20)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 21)]
                Deip(
                    runtime_types::pallet_deip::RawEvent<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pallet_deip::Project<
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        runtime_types::pallet_deip::review::Review<
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                ),
                #[codec(index = 24)]
                DeipProposal(runtime_types::pallet_deip_proposal::pallet::Event),
                #[codec(index = 25)]
                DeipDao(runtime_types::pallet_deip_dao::pallet::Event),
                #[codec(index = 27)]
                DeipVesting(runtime_types::pallet_deip_vesting::pallet::Event),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 1)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Runtime;
        }
        pub mod beefy_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
        }
        pub mod deip_serializable_u128 {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct SerializableAtLeast32BitUnsigned<_0>(pub _0);
        }
        pub mod deip_transaction_ctx {
            use super::runtime_types;
            pub mod transaction_ctx {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct TransactionCtx<_0>(::core::marker::PhantomData<_0>);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct TransactionCtxId<_0> {
                    pub block_number: ::core::primitive::u32,
                    pub extrinsic_id: ::core::primitive::u32,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct WrapperKeepOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode,
                            :: subxt :: codec :: Decode,
                            Debug,
                            Clone,
                            Eq,
                            PartialEq,
                            scale_info :: TypeInfo,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block { ratio: runtime_types::sp_arithmetic::per_things::Perbill },
                    #[codec(index = 1)]
                    remark { remark: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code { code: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 4)]
                    set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
                    #[codec(index = 5)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage { keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(::subxt::sp_core::crypto::AccountId32, ::subxt::sp_core::H256),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    BalanceZero,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    NoProvider,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        total_supply: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 7)]
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 8)]
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 13)]
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 15)]
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 16)]
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        destination: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 17)]
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct AssetBalance<_0, _1> {
                    pub balance: _0,
                    pub is_frozen: ::core::primitive::bool,
                    pub sufficient: ::core::primitive::bool,
                    pub extra: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _0,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub accounts: ::core::primitive::u32,
                    #[codec(compact)]
                    pub sufficients: ::core::primitive::u32,
                    #[codec(compact)]
                    pub approvals: ::core::primitive::u32,
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_deip {
            use super::runtime_types;
            pub mod asset {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Asset<_0, _1> {
                    pub id: _0,
                    pub amount:
                        runtime_types::deip_serializable_u128::SerializableAtLeast32BitUnsigned<_1>,
                }
            }
            pub mod contract {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Agreement<_0, _1, _2, _3> {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    License(runtime_types::pallet_deip::contract::LicenseStatus<_0, _1, _2, _3>),
                    #[codec(index = 2)]
                    GeneralContract(
                        runtime_types::pallet_deip::contract::GeneralContractStatus<_0, _1, _2>,
                    ),
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct GeneralContract<_0, _1, _2> {
                    pub id: runtime_types::primitive_types::H160,
                    pub creator: _0,
                    pub parties: ::std::vec::Vec<_0>,
                    pub hash: _1,
                    pub activation_time: ::core::option::Option<_2>,
                    pub expiration_time: ::core::option::Option<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum GeneralContractStatus<_0, _1, _2> {
                    #[codec(index = 0)]
                    PartiallyAccepted {
                        contract: runtime_types::pallet_deip::contract::GeneralContract<_0, _1, _2>,
                        accepted_by: ::std::vec::Vec<_0>,
                    },
                    #[codec(index = 1)]
                    Accepted(runtime_types::pallet_deip::contract::GeneralContract<_0, _1, _2>),
                    #[codec(index = 2)]
                    Rejected(runtime_types::pallet_deip::contract::GeneralContract<_0, _1, _2>),
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum IndexTerms {
                    #[codec(index = 0)]
                    LicenseAgreement,
                    #[codec(index = 1)]
                    GeneralContractAgreement,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct License<_0, _1, _2, _3> {
                    pub id: runtime_types::primitive_types::H160,
                    pub creator: _0,
                    pub licenser: _0,
                    pub licensee: _0,
                    pub hash: _1,
                    pub activation_time: ::core::option::Option<_2>,
                    pub expiration_time: ::core::option::Option<_2>,
                    pub project_id: runtime_types::primitive_types::H160,
                    pub price: _3,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum LicenseStatus<_0, _1, _2, _3> {
                    #[codec(index = 0)]
                    Unsigned(runtime_types::pallet_deip::contract::License<_0, _1, _2, _3>),
                    #[codec(index = 1)]
                    SignedByLicenser(runtime_types::pallet_deip::contract::License<_0, _1, _2, _3>),
                    #[codec(index = 2)]
                    Signed(runtime_types::pallet_deip::contract::License<_0, _1, _2, _3>),
                    #[codec(index = 3)]
                    Rejected(runtime_types::pallet_deip::contract::License<_0, _1, _2, _3>),
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Terms<_0> {
                    #[codec(index = 0)]
                    LicenseAgreement { source: runtime_types::primitive_types::H160, price: _0 },
                    #[codec(index = 1)]
                    GeneralContractAgreement,
                }
            }
            pub mod contribution {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Contribution<_0, _1, _2> {
                    pub sale_id: runtime_types::primitive_types::H160,
                    pub owner: _0,
                    pub amount: _1,
                    pub time: _2,
                }
            }
            pub mod investment_opportunity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum FundingModel<_0, _1> {
                    #[codec(index = 0)]
                    SimpleCrowdfunding { start_time: _0, end_time: _0, soft_cap: _1, hard_cap: _1 },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Info<_0, _1, _2, _3> {
                    pub created_ctx: _3,
                    pub external_id: _1,
                    pub start_time: _0,
                    pub end_time: _0,
                    pub status: runtime_types::pallet_deip::investment_opportunity::Status,
                    pub asset_id: _1,
                    pub total_amount:
                        runtime_types::deip_serializable_u128::SerializableAtLeast32BitUnsigned<_2>,
                    pub soft_cap:
                        runtime_types::deip_serializable_u128::SerializableAtLeast32BitUnsigned<_2>,
                    pub hard_cap:
                        runtime_types::deip_serializable_u128::SerializableAtLeast32BitUnsigned<_2>,
                    pub shares: ::std::vec::Vec<runtime_types::pallet_deip::asset::Asset<_1, _2>>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Status {
                    #[codec(index = 0)]
                    Active,
                    #[codec(index = 1)]
                    Finished,
                    #[codec(index = 2)]
                    Expired,
                    #[codec(index = 3)]
                    Inactive,
                }
            }
            pub mod review {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                    serde::Serialize,
                )]
                pub struct Review<_0, _1> {
                    pub external_id: runtime_types::primitive_types::H160,
                    pub author: _1,
                    pub content: _0,
                    pub domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                    pub assessment_model: ::core::primitive::u32,
                    pub weight: ::std::vec::Vec<::core::primitive::u8>,
                    pub project_content_external_id: runtime_types::primitive_types::H160,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Vote<_0, _1> {
                    pub dao: _0,
                    pub review_id: runtime_types::primitive_types::H160,
                    pub domain_id: runtime_types::primitive_types::H160,
                    pub voting_time: _1,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Call {
                #[codec(index = 0)]
                create_project {
                    is_private: ::core::primitive::bool,
                    external_id: runtime_types::primitive_types::H160,
                    team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    description: ::subxt::sp_core::H256,
                    domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                },
                #[codec(index = 1)]
                create_investment_opportunity {
                    external_id: runtime_types::primitive_types::H160,
                    creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    shares: ::std::vec::Vec<
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                    funding_model: runtime_types::pallet_deip::investment_opportunity::FundingModel<
                        ::core::primitive::u64,
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                },
                #[codec(index = 2)]
                activate_crowdfunding { sale_id: runtime_types::primitive_types::H160 },
                #[codec(index = 3)]
                expire_crowdfunding { sale_id: runtime_types::primitive_types::H160 },
                #[codec(index = 4)]
                finish_crowdfunding { sale_id: runtime_types::primitive_types::H160 },
                #[codec(index = 5)]
                invest {
                    id: runtime_types::primitive_types::H160,
                    asset: runtime_types::pallet_deip::asset::Asset<
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 6)]
                update_project {
                    project_id: runtime_types::primitive_types::H160,
                    description: ::core::option::Option<::subxt::sp_core::H256>,
                    is_private: ::core::option::Option<::core::primitive::bool>,
                },
                #[codec(index = 7)]
                create_project_content {
                    external_id: runtime_types::primitive_types::H160,
                    project_external_id: runtime_types::primitive_types::H160,
                    team_id: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    content_type: runtime_types::pallet_deip::ProjectContentType,
                    description: ::subxt::sp_core::H256,
                    content: ::subxt::sp_core::H256,
                    authors: ::std::vec::Vec<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    references: ::core::option::Option<
                        ::std::vec::Vec<runtime_types::primitive_types::H160>,
                    >,
                },
                #[codec(index = 8)]
                create_review {
                    external_id: runtime_types::primitive_types::H160,
                    author: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    content: ::subxt::sp_core::H256,
                    domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
                    assessment_model: ::core::primitive::u32,
                    weight: ::std::vec::Vec<::core::primitive::u8>,
                    project_content_external_id: runtime_types::primitive_types::H160,
                },
                #[codec(index = 9)]
                upvote_review {
                    review_id: runtime_types::primitive_types::H160,
                    domain_id: runtime_types::primitive_types::H160,
                },
                #[codec(index = 10)]
                add_domain { domain: runtime_types::pallet_deip::Domain },
                #[codec(index = 11)]
                create_contract_agreement {
                    id: runtime_types::primitive_types::H160,
                    creator: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                    parties: ::std::vec::Vec<
                        runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    >,
                    hash: ::subxt::sp_core::H256,
                    activation_time: ::core::option::Option<::core::primitive::u64>,
                    expiration_time: ::core::option::Option<::core::primitive::u64>,
                    terms: runtime_types::pallet_deip::contract::Terms<
                        runtime_types::pallet_deip::asset::Asset<
                            runtime_types::primitive_types::H160,
                            ::core::primitive::u128,
                        >,
                    >,
                },
                #[codec(index = 12)]
                accept_contract_agreement {
                    id: runtime_types::primitive_types::H160,
                    party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                },
                #[codec(index = 13)]
                reject_contract_agreement {
                    id: runtime_types::primitive_types::H160,
                    party: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                    >,
                },
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Domain {
                pub external_id: runtime_types::primitive_types::H160,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Error {
                #[codec(index = 0)]
                NoSuchProject,
                #[codec(index = 1)]
                NotProjectOwner,
                #[codec(index = 2)]
                DomainNotExists,
                #[codec(index = 3)]
                ProjectAlreadyExists,
                #[codec(index = 4)]
                ProjectContentAlreadyExists,
                #[codec(index = 5)]
                ProjectNotBelongToTeam,
                #[codec(index = 6)]
                NoSuchProjectContent,
                #[codec(index = 7)]
                NoSuchReference,
                #[codec(index = 8)]
                ProjectAlreadyFinished,
                #[codec(index = 9)]
                DomainLimitReached,
                #[codec(index = 10)]
                DomainAlreadyExists,
                #[codec(index = 11)]
                NdaAlreadyExists,
                #[codec(index = 12)]
                NdaAccessRequestAlreadyExists,
                #[codec(index = 13)]
                NoSuchNda,
                #[codec(index = 14)]
                NoSuchNdaAccessRequest,
                #[codec(index = 15)]
                NdaContractIsNotActiveYet,
                #[codec(index = 16)]
                NdaStartDateMustBeLaterOrEqualCurrentMoment,
                #[codec(index = 17)]
                NdaEndDateMustBeLaterCurrentMoment,
                #[codec(index = 18)]
                NdaStartDateMustBeLessThanEndDate,
                #[codec(index = 19)]
                TeamOfAllProjectsMustSpecifiedAsParty,
                #[codec(index = 20)]
                NdaAccessRequestAlreadyFinalized,
                #[codec(index = 21)]
                TooMuchNdaParties,
                #[codec(index = 22)]
                ReviewAlreadyExists,
                #[codec(index = 23)]
                ReviewNoDomainSpecified,
                #[codec(index = 24)]
                ReviewVoteAlreadyExists,
                #[codec(index = 25)]
                ReviewVoteNoSuchDomain,
                #[codec(index = 26)]
                ReviewVoteNoSuchReview,
                #[codec(index = 27)]
                ReviewVoteUnrelatedDomain,
                #[codec(index = 28)]
                ReviewAlreadyVotedWithDomain,
                #[codec(index = 29)]
                NoPermission,
                #[codec(index = 30)]
                InvestmentOpportunityStartTimeMustBeLaterOrEqualCurrentMoment,
                #[codec(index = 31)]
                InvestmentOpportunityEndTimeMustBeLaterStartTime,
                #[codec(index = 32)]
                InvestmentOpportunitySoftCapMustBeGreaterOrEqualMinimum,
                #[codec(index = 33)]
                InvestmentOpportunityHardCapShouldBeGreaterOrEqualSoftCap,
                #[codec(index = 34)]
                InvestmentOpportunityAlreadyExists,
                #[codec(index = 35)]
                InvestmentOpportunityBalanceIsNotEnough,
                #[codec(index = 36)]
                InvestmentOpportunityFailedToReserveAsset,
                #[codec(index = 37)]
                InvestmentOpportunityAssetAmountMustBePositive,
                #[codec(index = 38)]
                InvestmentOpportunitySecurityTokenNotSpecified,
                #[codec(index = 39)]
                InvestmentOpportunityNotFound,
                #[codec(index = 40)]
                InvestmentOpportunityShouldBeInactive,
                #[codec(index = 41)]
                InvestmentOpportunityShouldBeStarted,
                #[codec(index = 42)]
                InvestmentOpportunityShouldBeActive,
                #[codec(index = 43)]
                InvestmentOpportunityExpirationWrongState,
                #[codec(index = 44)]
                InvestmentOpportunityWrongAssetId,
                #[codec(index = 45)]
                InvestmentOpportunityCapDifferentAssets,
                #[codec(index = 46)]
                InvestingNotFound,
                #[codec(index = 47)]
                InvestingNotActive,
                #[codec(index = 48)]
                InvestingNotEnoughFunds,
                #[codec(index = 49)]
                InvestingWrongAsset,
                #[codec(index = 50)]
                ContractAgreementNoParties,
                #[codec(index = 51)]
                ContractAgreementStartTimeMustBeLaterOrEqualCurrentMoment,
                #[codec(index = 52)]
                ContractAgreementEndTimeMustBeLaterStartTime,
                #[codec(index = 53)]
                ContractAgreementAlreadyExists,
                #[codec(index = 54)]
                ContractAgreementFeeMustBePositive,
                #[codec(index = 55)]
                ContractAgreementLicenseTwoPartiesRequired,
                #[codec(index = 56)]
                ContractAgreementLicenseProjectTeamIsNotListedInParties,
                #[codec(index = 57)]
                ContractAgreementNotFound,
                #[codec(index = 58)]
                ContractAgreementWrongAgreement,
                #[codec(index = 59)]
                ContractAgreementAlreadyAccepted,
                #[codec(index = 60)]
                ContractAgreementLicensePartyIsNotLicenser,
                #[codec(index = 61)]
                ContractAgreementLicensePartyIsNotLicensee,
                #[codec(index = 62)]
                ContractAgreementLicenseExpired,
                #[codec(index = 63)]
                ContractAgreementLicenseNotEnoughBalance,
                #[codec(index = 64)]
                ContractAgreementLicenseFailedToChargeFee,
                #[codec(index = 65)]
                ContractAgreementLicenseIsNotActive,
                #[codec(index = 66)]
                ContractAgreementPartyIsNotListed,
                #[codec(index = 67)]
                ContractAgreementAlreadyAcceptedByParty,
                #[codec(index = 68)]
                ContractAgreementRejected,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Nda<_0, _1, _2> {
                pub contract_creator: _1,
                pub external_id: runtime_types::primitive_types::H160,
                pub end_date: _2,
                pub start_date: ::core::option::Option<_2>,
                pub contract_hash: _0,
                pub parties: ::std::vec::Vec<_1>,
                pub projects: ::std::vec::Vec<runtime_types::primitive_types::H160>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct NdaAccessRequest<_0, _1> {
                pub external_id: runtime_types::primitive_types::H160,
                pub nda_external_id: runtime_types::primitive_types::H160,
                pub requester: _1,
                pub encrypted_payload_hash: _0,
                pub encrypted_payload_iv: ::std::vec::Vec<::core::primitive::u8>,
                pub status: runtime_types::pallet_deip::NdaAccessRequestStatus,
                pub grantor: ::core::option::Option<_1>,
                pub encrypted_payload_encryption_key:
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub proof_of_encrypted_payload_encryption_key:
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum NdaAccessRequestStatus {
                #[codec(index = 0)]
                Pending,
                #[codec(index = 1)]
                Fulfilled,
                #[codec(index = 2)]
                Rejected,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct Project<_0, _1> {
                pub is_private: ::core::primitive::bool,
                pub external_id: runtime_types::primitive_types::H160,
                pub team_id: _1,
                pub description: _0,
                pub domains: ::std::vec::Vec<runtime_types::primitive_types::H160>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ProjectContent<_0, _1> {
                pub external_id: runtime_types::primitive_types::H160,
                pub project_external_id: runtime_types::primitive_types::H160,
                pub team_id: _1,
                pub content_type: runtime_types::pallet_deip::ProjectContentType,
                pub description: _0,
                pub content: _0,
                pub authors: ::std::vec::Vec<_1>,
                pub references:
                    ::core::option::Option<::std::vec::Vec<runtime_types::primitive_types::H160>>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum ProjectContentType {
                #[codec(index = 0)]
                Announcement,
                #[codec(index = 1)]
                FinalResult,
                #[codec(index = 2)]
                MilestoneArticle,
                #[codec(index = 3)]
                MilestoneBook,
                #[codec(index = 4)]
                MilestoneChapter,
                #[codec(index = 5)]
                MilestoneCode,
                #[codec(index = 6)]
                MilestoneConferencePaper,
                #[codec(index = 7)]
                MilestoneCoverPage,
                #[codec(index = 8)]
                MilestoneData,
                #[codec(index = 9)]
                MilestoneExperimentFindings,
                #[codec(index = 10)]
                MilestoneMethod,
                #[codec(index = 11)]
                MilestoneNegativeResults,
                #[codec(index = 12)]
                MilestonePatent,
                #[codec(index = 13)]
                MilestonePoster,
                #[codec(index = 14)]
                MilestonePreprint,
                #[codec(index = 15)]
                MilestonePresentation,
                #[codec(index = 16)]
                MilestoneRawData,
                #[codec(index = 17)]
                MilestoneResearchProposal,
                #[codec(index = 18)]
                MilestoneTechnicalReport,
                #[codec(index = 19)]
                MilestoneThesis,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum RawEvent<_0, _1, _2> {
                #[codec(index = 0)]
                ProjectCreated(_0, _1),
                #[codec(index = 1)]
                ProjectRemoved(_0, _1),
                #[codec(index = 2)]
                ProjectUpdated(_0, runtime_types::primitive_types::H160),
                #[codec(index = 3)]
                ProjectContnetCreated(_0, runtime_types::primitive_types::H160),
                #[codec(index = 4)]
                NdaCreated(_0, runtime_types::primitive_types::H160),
                #[codec(index = 5)]
                NdaAccessRequestCreated(_0, runtime_types::primitive_types::H160),
                #[codec(index = 6)]
                NdaAccessRequestFulfilled(_0, runtime_types::primitive_types::H160),
                #[codec(index = 7)]
                NdaAccessRequestRejected(_0, runtime_types::primitive_types::H160),
                #[codec(index = 8)]
                DomainAdded(_0, runtime_types::primitive_types::H160),
                #[codec(index = 9)]
                ReviewCreated(_0, _2),
                #[codec(index = 10)]
                ReviewUpvoted(
                    runtime_types::primitive_types::H160,
                    _0,
                    runtime_types::primitive_types::H160,
                ),
                #[codec(index = 11)]
                SimpleCrowdfundingCreated(runtime_types::primitive_types::H160),
                #[codec(index = 12)]
                SimpleCrowdfundingActivated(runtime_types::primitive_types::H160),
                #[codec(index = 13)]
                SimpleCrowdfundingFinished(runtime_types::primitive_types::H160),
                #[codec(index = 14)]
                SimpleCrowdfundingExpired(runtime_types::primitive_types::H160),
                #[codec(index = 15)]
                Invested(runtime_types::primitive_types::H160, _0),
                #[codec(index = 16)]
                ContractAgreementCreated(runtime_types::primitive_types::H160),
                #[codec(index = 17)]
                ContractAgreementAccepted(runtime_types::primitive_types::H160, _0),
                #[codec(index = 18)]
                ContractAgreementFinalized(runtime_types::primitive_types::H160),
                #[codec(index = 19)]
                ContractAgreementRejected(runtime_types::primitive_types::H160, _0),
            }
        }
        pub mod pallet_deip_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct AssetMetadata<_0> {
                    pub name: ::std::vec::Vec<_0>,
                    pub symbol: ::std::vec::Vec<_0>,
                    pub decimals: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        id: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        is_sufficient: ::core::primitive::bool,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    destroy {
                        id: ::core::primitive::u32,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    burn {
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    transfer {
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    transfer_keep_alive {
                        id: ::core::primitive::u32,
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    force_transfer {
                        id: ::core::primitive::u32,
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    freeze {
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 9)]
                    thaw {
                        id: ::core::primitive::u32,
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 10)]
                    freeze_asset { id: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    thaw_asset { id: ::core::primitive::u32 },
                    #[codec(index = 12)]
                    transfer_ownership {
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    set_team {
                        id: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    set_metadata {
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 15)]
                    clear_metadata { id: ::core::primitive::u32 },
                    #[codec(index = 16)]
                    force_set_metadata {
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 17)]
                    force_clear_metadata { id: ::core::primitive::u32 },
                    #[codec(index = 18)]
                    force_asset_status {
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    approve_transfer {
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    cancel_approval {
                        id: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 21)]
                    force_cancel_approval {
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 22)]
                    transfer_approved {
                        id: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        destination: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    deip_create_asset {
                        id: runtime_types::primitive_types::H160,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        min_balance: ::core::primitive::u128,
                        project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                    },
                    #[codec(index = 24)]
                    deip_destroy {
                        id: runtime_types::primitive_types::H160,
                        witness: runtime_types::pallet_assets::types::DestroyWitness,
                    },
                    #[codec(index = 25)]
                    deip_issue_asset {
                        id: runtime_types::primitive_types::H160,
                        beneficiary:
                            runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::primitive_types::H160,
                            >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    deip_burn {
                        id: runtime_types::primitive_types::H160,
                        who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 27)]
                    deip_transfer {
                        id: runtime_types::primitive_types::H160,
                        target: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 28)]
                    deip_freeze {
                        id: runtime_types::primitive_types::H160,
                        who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 29)]
                    deip_thaw {
                        id: runtime_types::primitive_types::H160,
                        who: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 30)]
                    deip_freeze_asset { id: runtime_types::primitive_types::H160 },
                    #[codec(index = 31)]
                    deip_thaw_asset { id: runtime_types::primitive_types::H160 },
                    #[codec(index = 32)]
                    deip_transfer_ownership {
                        id: runtime_types::primitive_types::H160,
                        owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 33)]
                    deip_set_team {
                        id: runtime_types::primitive_types::H160,
                        issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 34)]
                    deip_set_metadata {
                        id: runtime_types::primitive_types::H160,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 35)]
                    deip_wipe_zero_balance {
                        asset: runtime_types::primitive_types::H160,
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    ProjectDoesNotExist,
                    #[codec(index = 1)]
                    ProjectDoesNotBelongToTeam,
                    #[codec(index = 2)]
                    ProjectSecurityTokenCannotBeDestroyed,
                    #[codec(index = 3)]
                    ProjectSecurityTokenCannotBeBurned,
                    #[codec(index = 4)]
                    ProjectSecurityTokenCannotBeFreezed,
                    #[codec(index = 5)]
                    ProjectSecurityTokenAccountCannotBeFreezed,
                    #[codec(index = 6)]
                    ReservedAssetCannotBeFreezed,
                    #[codec(index = 7)]
                    ReservedAssetAccountCannotBeFreezed,
                    #[codec(index = 8)]
                    FtNotFound,
                    #[codec(index = 9)]
                    FtBalanceNotFound,
                    #[codec(index = 10)]
                    AssetIdOverflow,
                    #[codec(index = 11)]
                    DeipAssetIdExists,
                    #[codec(index = 12)]
                    DeipAssetIdDoesNotExist,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Investment<_0, _1> {
                    pub creator: _0,
                    pub assets: ::std::vec::Vec<_1>,
                    pub asset_id: _1,
                }
            }
        }
        pub mod pallet_deip_dao {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                pub mod dao {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub enum AlterAuthority<_0> {
                        #[codec(index = 0)]
                        AddMember { member: _0, preserve_threshold: ::core::primitive::bool },
                        #[codec(index = 1)]
                        RemoveMember { member: _0, preserve_threshold: ::core::primitive::bool },
                        #[codec(index = 2)]
                        ReplaceAuthority {
                            authority_key: _0,
                            authority:
                                runtime_types::pallet_deip_dao::pallet::dao::InputAuthority<_0>,
                        },
                    }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                        serde::Serialize,
                    )]
                    pub struct Authority<_0> {
                        pub signatories: ::std::vec::Vec<_0>,
                        pub threshold: ::core::primitive::u16,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                        serde::Serialize,
                    )]
                    pub struct Dao<_0, _1> {
                        pub authority_key: _0,
                        pub authority: runtime_types::pallet_deip_dao::pallet::dao::Authority<_0>,
                        pub id: _1,
                        pub dao_key: _0,
                        pub metadata: ::core::option::Option<::subxt::sp_core::H256>,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct InputAuthority<_0> {
                        pub signatories: ::std::vec::Vec<_0>,
                        pub threshold: ::core::primitive::u16,
                    }
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        name: runtime_types::primitive_types::H160,
                        authority: runtime_types::pallet_deip_dao::pallet::dao::InputAuthority<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        metadata: ::core::option::Option<::subxt::sp_core::H256>,
                    },
                    #[codec(index = 1)]
                    alter_authority {
                        authority: runtime_types::pallet_deip_dao::pallet::dao::AlterAuthority<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 2)]
                    update_dao { new_metadata: ::core::option::Option<::subxt::sp_core::H256> },
                    #[codec(index = 3)]
                    on_behalf {
                        name: runtime_types::primitive_types::H160,
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    Exists,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    Forbidden,
                    #[codec(index = 3)]
                    AuthorityMismatch,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    DaoCreate(
                        runtime_types::pallet_deip_dao::pallet::dao::Dao<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    ),
                    #[codec(index = 1)]
                    DaoAlterAuthority(
                        runtime_types::pallet_deip_dao::pallet::dao::Dao<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    ),
                    #[codec(index = 2)]
                    DaoMetadataUpdated(
                        runtime_types::pallet_deip_dao::pallet::dao::Dao<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    ),
                }
            }
        }
        pub mod pallet_deip_ecosystem_fund {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {}
            }
        }
        pub mod pallet_deip_portal {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    # [codec (index = 0)] create { delegate : :: subxt :: sp_core :: crypto :: AccountId32 , metadata : :: core :: option :: Option < :: subxt :: sp_core :: H256 > , } , # [codec (index = 1)] update { update : runtime_types :: pallet_deip_portal :: portal :: PortalUpdate < runtime_types :: appchain_deip_runtime :: Runtime > , } , # [codec (index = 2)] schedule { xt : :: std :: boxed :: Box < runtime_types :: sp_runtime :: generic :: unchecked_extrinsic :: UncheckedExtrinsic < :: subxt :: sp_runtime :: MultiAddress < :: subxt :: sp_core :: crypto :: AccountId32 , () > , runtime_types :: appchain_deip_runtime :: Call , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > , } , # [codec (index = 3)] exec { portal_id : runtime_types :: primitive_types :: H160 , call : :: std :: boxed :: Box < runtime_types :: appchain_deip_runtime :: Call > , } , # [codec (index = 4)] exec_postponed { portal_id : runtime_types :: primitive_types :: H160 , call : :: std :: boxed :: Box < runtime_types :: appchain_deip_runtime :: Call > , } , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    DelegateMismatch,
                    #[codec(index = 1)]
                    PortalMismatch,
                    #[codec(index = 2)]
                    AlreadyScheduled,
                    #[codec(index = 3)]
                    UnproperCall,
                    #[codec(index = 4)]
                    NotScheduled,
                    #[codec(index = 5)]
                    OwnerIsNotATenant,
                    #[codec(index = 6)]
                    PortalAlreadyExist,
                    #[codec(index = 7)]
                    PortalNotFound,
                }
            }
            pub mod portal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Portal<_0> {
                    pub id: runtime_types::primitive_types::H160,
                    pub owner: ::subxt::sp_core::crypto::AccountId32,
                    pub delegate: ::subxt::sp_core::crypto::AccountId32,
                    pub metadata: ::core::option::Option<::subxt::sp_core::H256>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct PortalUpdate<_0> {
                    pub delegate: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    pub metadata:
                        ::core::option::Option<::core::option::Option<::subxt::sp_core::H256>>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_0>,
                }
            }
            pub mod transaction_ctx {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct PortalCtx<_0>(pub _0);
            }
        }
        pub mod pallet_deip_proposal {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    propose {
                        batch: ::std::vec::Vec<
                            runtime_types::pallet_deip_proposal::proposal::BatchItem<
                                runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    runtime_types::primitive_types::H160,
                                >,
                                runtime_types::appchain_deip_runtime::Call,
                            >,
                        >,
                        external_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                    },
                    #[codec(index = 1)]
                    decide {
                        proposal_id: runtime_types::primitive_types::H160,
                        decision:
                            runtime_types::pallet_deip_proposal::proposal::ProposalMemberDecision,
                        batch_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    expire { proposal_id: runtime_types::primitive_types::H160 },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NotFound,
                    #[codec(index = 1)]
                    AlreadyExist,
                    #[codec(index = 2)]
                    NotAMember,
                    #[codec(index = 3)]
                    AlreadyResolved,
                    #[codec(index = 4)]
                    ImpossibleDecision,
                    #[codec(index = 5)]
                    ReachDepthLimit,
                    #[codec(index = 6)]
                    ReachSizeLimit,
                    #[codec(index = 7)]
                    SelfReferential,
                    #[codec(index = 8)]
                    NotExpired,
                    #[codec(index = 9)]
                    BatchWeightTooLow,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Proposed {
                        author: ::subxt::sp_core::crypto::AccountId32,
                        batch: ::std::vec::Vec<
                            runtime_types::pallet_deip_proposal::proposal::BatchItem<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::appchain_deip_runtime::Call,
                            >,
                        >,
                        proposal_id: runtime_types::primitive_types::H160,
                        batch_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 1)]
                    Approved {
                        member: ::subxt::sp_core::crypto::AccountId32,
                        proposal_id: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 2)]
                    RevokedApproval {
                        member: ::subxt::sp_core::crypto::AccountId32,
                        proposal_id: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 3)]
                    Resolved {
                        member: ::subxt::sp_core::crypto::AccountId32,
                        proposal_id: runtime_types::primitive_types::H160,
                        state: runtime_types::pallet_deip_proposal::proposal::ProposalState,
                    },
                    #[codec(index = 4)]
                    Expired { proposal_id: runtime_types::primitive_types::H160 },
                }
            }
            pub mod proposal {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                    serde::Serialize,
                )]
                pub struct BatchItem<_0, _1> {
                    pub account: _0,
                    pub call: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct DeipProposal < _0 > { pub id : runtime_types :: primitive_types :: H160 , pub batch : :: std :: vec :: Vec < runtime_types :: pallet_deip_proposal :: proposal :: BatchItem < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: appchain_deip_runtime :: Call > > , pub batch_weight : :: core :: primitive :: u64 , pub decisions : :: std :: collections :: BTreeMap < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: pallet_deip_proposal :: proposal :: ProposalMemberDecision > , pub state : runtime_types :: pallet_deip_proposal :: proposal :: ProposalState , pub author : :: subxt :: sp_core :: crypto :: AccountId32 , pub created_at : :: core :: primitive :: u64 , pub created_ctx : runtime_types :: deip_transaction_ctx :: transaction_ctx :: TransactionCtxId < runtime_types :: pallet_deip_portal :: transaction_ctx :: PortalCtx < runtime_types :: deip_transaction_ctx :: transaction_ctx :: TransactionCtx < _0 > > > , }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum ProposalMemberDecision {
                    #[codec(index = 0)]
                    Pending,
                    #[codec(index = 1)]
                    Approve,
                    #[codec(index = 2)]
                    Reject,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                    serde::Serialize,
                )]
                pub enum ProposalState {
                    #[codec(index = 0)]
                    Pending,
                    #[codec(index = 1)]
                    Rejected,
                    #[codec(index = 2)]
                    Done,
                    #[codec(index = 3)]
                    Failed(runtime_types::sp_runtime::DispatchError),
                }
            }
        }
        pub mod pallet_deip_uniques {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        class: ::core::primitive::u32,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    force_create {
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        free_holding: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    destroy {
                        class: ::core::primitive::u32,
                        witness: runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    #[codec(index = 3)]
                    mint {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 4)]
                    burn {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        check_owner: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 5)]
                    transfer {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 6)]
                    redeposit {
                        class: ::core::primitive::u32,
                        instances: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 7)]
                    freeze { class: ::core::primitive::u32, instance: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    thaw { class: ::core::primitive::u32, instance: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    freeze_class { class: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    thaw_class { class: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    transfer_ownership {
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    set_team {
                        class: ::core::primitive::u32,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 13)]
                    approve_transfer {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        delegate: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 14)]
                    cancel_approval {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        maybe_check_delegate: ::core::option::Option<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 15)]
                    force_asset_status {
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        issuer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        admin: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        freezer: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        free_holding: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    set_attribute {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 17)]
                    clear_attribute {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 18)]
                    set_metadata {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 19)]
                    clear_metadata {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 20)]
                    set_class_metadata {
                        class: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 21)]
                    clear_class_metadata { class: ::core::primitive::u32 },
                    #[codec(index = 22)]
                    deip_create {
                        class: runtime_types::primitive_types::H160,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                    },
                    #[codec(index = 23)]
                    deip_force_create {
                        class: runtime_types::primitive_types::H160,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        project_id: ::core::option::Option<runtime_types::primitive_types::H160>,
                        free_holding: ::core::primitive::bool,
                    },
                    #[codec(index = 24)]
                    deip_destroy {
                        class: runtime_types::primitive_types::H160,
                        witness: runtime_types::pallet_uniques::types::DestroyWitness,
                    },
                    #[codec(index = 25)]
                    deip_mint {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 26)]
                    deip_burn {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        check_owner: ::core::option::Option<
                            runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::primitive_types::H160,
                            >,
                        >,
                    },
                    #[codec(index = 27)]
                    deip_transfer {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        dest: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 28)]
                    deip_redeposit {
                        class: runtime_types::primitive_types::H160,
                        instances: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 29)]
                    deip_freeze {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 30)]
                    deip_thaw {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 31)]
                    deip_freeze_class { class: runtime_types::primitive_types::H160 },
                    #[codec(index = 32)]
                    deip_thaw_class { class: runtime_types::primitive_types::H160 },
                    #[codec(index = 33)]
                    deip_transfer_ownership {
                        class: runtime_types::primitive_types::H160,
                        owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 34)]
                    deip_set_team {
                        class: runtime_types::primitive_types::H160,
                        issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 35)]
                    deip_approve_transfer {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        delegate: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                    },
                    #[codec(index = 36)]
                    deip_cancel_approval {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        maybe_check_delegate: ::core::option::Option<
                            runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::primitive_types::H160,
                            >,
                        >,
                    },
                    #[codec(index = 37)]
                    deip_force_asset_status {
                        class: runtime_types::primitive_types::H160,
                        owner: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        issuer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        admin: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        freezer: runtime_types::appchain_deip_runtime::deip_account::DeipAccountId<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::primitive_types::H160,
                        >,
                        free_holding: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 38)]
                    deip_set_attribute {
                        class: runtime_types::primitive_types::H160,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 39)]
                    deip_clear_attribute {
                        class: runtime_types::primitive_types::H160,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 40)]
                    deip_set_metadata {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 41)]
                    deip_clear_metadata {
                        class: runtime_types::primitive_types::H160,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 42)]
                    deip_set_class_metadata {
                        class: runtime_types::primitive_types::H160,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 43)]
                    deip_clear_class_metadata { class: runtime_types::primitive_types::H160 },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    DeipNftClassIdExists,
                    #[codec(index = 1)]
                    DeipNftClassIdDoesNotExist,
                    #[codec(index = 2)]
                    NftClassIdOverflow,
                    #[codec(index = 3)]
                    ProjectDoesNotExist,
                    #[codec(index = 4)]
                    ProjectDoesNotBelongToTeam,
                    #[codec(index = 5)]
                    ProjectSecurityTokenCannotBeDestroyed,
                    #[codec(index = 6)]
                    ProjectSecurityTokenCannotBeBurned,
                    #[codec(index = 7)]
                    ProjectSecurityTokenCannotBeFrozen,
                }
            }
        }
        pub mod pallet_deip_vesting {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    vested_transfer {
                        target: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        plan: runtime_types::pallet_deip_vesting::pallet::VestingPlan<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 1)]
                    unlock,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    ExistingVestingPlan,
                    #[codec(index = 1)]
                    AmountLow,
                    #[codec(index = 2)]
                    InvalidVestingPlan,
                    #[codec(index = 3)]
                    NoVestingPlan,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    VestingUpdated(::subxt::sp_core::crypto::AccountId32, ::core::primitive::u128),
                    #[codec(index = 1)]
                    VestingCompleted(::subxt::sp_core::crypto::AccountId32),
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct VestingPlan<_0> {
                    pub start_time: ::core::primitive::u64,
                    pub cliff_duration: ::core::primitive::u64,
                    pub total_duration: ::core::primitive::u64,
                    pub interval: ::core::primitive::u64,
                    pub initial_amount: _0,
                    pub total_amount: _0,
                    pub vesting_during_cliff: ::core::primitive::bool,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline {
                        offline: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call:
                            ::subxt::WrapperKeepOpaque<runtime_types::appchain_deip_runtime::Call>,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    MultisigApproval {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    MultisigExecuted {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    MultisigCancelled {
                        cancelling: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_octopus_appchain {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_observations {
                        payload: runtime_types::pallet_octopus_appchain::ObservationsPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 1)]
                    force_set_is_activated { is_activated: ::core::primitive::bool },
                    #[codec(index = 2)]
                    force_set_next_set_id { next_set_id: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    force_set_planned_validators {
                        validators: ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 4)]
                    lock {
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    mint_asset {
                        asset_id: ::core::primitive::u32,
                        sender_id: ::std::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    burn_asset {
                        asset_id: ::core::primitive::u32,
                        receiver_id: ::std::vec::Vec<::core::primitive::u8>,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    WrongSetId,
                    #[codec(index = 1)]
                    InvalidNotificationId,
                    #[codec(index = 2)]
                    NotValidator,
                    #[codec(index = 3)]
                    AmountOverflow,
                    #[codec(index = 4)]
                    NextNotificationIdOverflow,
                    #[codec(index = 5)]
                    WrongAssetId,
                    #[codec(index = 6)]
                    InvalidActiveTotalStake,
                    #[codec(index = 7)]
                    NotActivated,
                    #[codec(index = 8)]
                    InvalidReceiverId,
                    #[codec(index = 9)]
                    InvalidTokenId,
                    #[codec(index = 10)]
                    NextSetIdOverflow,
                    #[codec(index = 11)]
                    ObservationsExceededLimit,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewPlannedValidators(
                        ::core::primitive::u32,
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Locked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 2)]
                    Unlocked(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    UnlockFailed(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    AssetMinted(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    AssetBurned(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    AssetMintFailed(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    AssetIdGetFailed(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BurnEvent<_0> {
                pub index: ::core::primitive::u32,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct LockAssetEvent<_0> {
                pub index: ::core::primitive::u32,
                pub token_id: ::std::vec::Vec<::core::primitive::u8>,
                pub sender_id: ::std::vec::Vec<::core::primitive::u8>,
                pub receiver: _0,
                pub amount: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum NotificationResult {
                #[codec(index = 0)]
                Success,
                #[codec(index = 1)]
                UnlockFailed,
                #[codec(index = 2)]
                AssetMintFailed,
                #[codec(index = 3)]
                AssetGetFailed,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Observation<_0> {
                #[codec(index = 0)]
                UpdateValidatorSet(runtime_types::pallet_octopus_appchain::ValidatorSet<_0>),
                #[codec(index = 1)]
                LockAsset(runtime_types::pallet_octopus_appchain::LockAssetEvent<_0>),
                #[codec(index = 2)]
                Burn(runtime_types::pallet_octopus_appchain::BurnEvent<_0>),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum ObservationType {
                #[codec(index = 0)]
                UpdateValidatorSet,
                #[codec(index = 1)]
                Burn,
                #[codec(index = 2)]
                LockAsset,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ObservationsPayload<_0, _1, _2> {
                pub public: _0,
                pub block_number: _1,
                pub observations:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Observation<_2>>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Validator<_0> {
                pub validator_id_in_appchain: _0,
                pub total_stake: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ValidatorSet<_0> {
                pub set_id: ::core::primitive::u32,
                pub validators:
                    ::std::vec::Vec<runtime_types::pallet_octopus_appchain::Validator<_0>>,
            }
        }
        pub mod pallet_octopus_lpos {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_history_depth {
                        #[codec(compact)]
                        new_history_depth: ::core::primitive::u32,
                        #[codec(compact)]
                        era_items_deleted: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    force_set_era_payout { era_payout: ::core::primitive::u128 },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NotController,
                    #[codec(index = 1)]
                    NotStash,
                    #[codec(index = 2)]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    EmptyTargets,
                    #[codec(index = 5)]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    InsufficientBond,
                    #[codec(index = 8)]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    FundedTarget,
                    #[codec(index = 11)]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    BadState,
                    #[codec(index = 18)]
                    TooManyTargets,
                    #[codec(index = 19)]
                    BadTarget,
                    #[codec(index = 20)]
                    CannotChillOther,
                    #[codec(index = 21)]
                    TooManyNominators,
                    #[codec(index = 22)]
                    TooManyValidators,
                    #[codec(index = 23)]
                    NoClaimedRewards,
                    #[codec(index = 24)]
                    AmountOverflow,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    PlanNewEra(::core::primitive::u32),
                    #[codec(index = 1)]
                    PlanNewEraFailed,
                    #[codec(index = 2)]
                    TriggerNewEra,
                    #[codec(index = 3)]
                    EraPayout(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ),
                    #[codec(index = 4)]
                    EraPayoutFailed(::core::primitive::u32),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ActiveEraInfo {
                pub index: ::core::primitive::u32,
                pub set_id: ::core::primitive::u32,
                pub start: ::core::option::Option<::core::primitive::u64>,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u32,
                pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
            }
        }
        pub mod pallet_octopus_support {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum PayloadType {
                    #[codec(index = 0)]
                    Lock,
                    #[codec(index = 1)]
                    BurnAsset,
                    #[codec(index = 2)]
                    PlanNewEra,
                    #[codec(index = 3)]
                    EraPayout,
                }
            }
        }
        pub mod pallet_octopus_upward_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {}
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NonceOverflow,
                    #[codec(index = 1)]
                    QueueSizeLimitReached,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {}
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct Message {
                pub nonce: ::core::primitive::u64,
                pub payload_type: runtime_types::pallet_octopus_support::types::PayloadType,
                pub payload: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::appchain_deip_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession { session_index: ::core::primitive::u32 },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo { call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call> },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged { new_sudoer: ::subxt::sp_core::crypto::AccountId32 },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_uniques {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NoPermission,
                    #[codec(index = 1)]
                    Unknown,
                    #[codec(index = 2)]
                    AlreadyExists,
                    #[codec(index = 3)]
                    WrongOwner,
                    #[codec(index = 4)]
                    BadWitness,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    Frozen,
                    #[codec(index = 7)]
                    WrongDelegate,
                    #[codec(index = 8)]
                    NoDelegate,
                    #[codec(index = 9)]
                    Unapproved,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        class: ::core::primitive::u32,
                        creator: ::subxt::sp_core::crypto::AccountId32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 1)]
                    ForceCreated {
                        class: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 2)]
                    Destroyed { class: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    Issued {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    Transferred {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    Burned {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen { class: ::core::primitive::u32, instance: ::core::primitive::u32 },
                    #[codec(index = 7)]
                    Thawed { class: ::core::primitive::u32, instance: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    ClassFrozen { class: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    ClassThawed { class: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    OwnerChanged {
                        class: ::core::primitive::u32,
                        new_owner: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 11)]
                    TeamChanged {
                        class: ::core::primitive::u32,
                        issuer: ::subxt::sp_core::crypto::AccountId32,
                        admin: ::subxt::sp_core::crypto::AccountId32,
                        freezer: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 12)]
                    ApprovedTransfer {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 13)]
                    ApprovalCancelled {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        owner: ::subxt::sp_core::crypto::AccountId32,
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 14)]
                    AssetStatusChanged { class: ::core::primitive::u32 },
                    #[codec(index = 15)]
                    ClassMetadataSet {
                        class: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    ClassMetadataCleared { class: ::core::primitive::u32 },
                    #[codec(index = 17)]
                    MetadataSet {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                        data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 18)]
                    MetadataCleared {
                        class: ::core::primitive::u32,
                        instance: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    Redeposited {
                        class: ::core::primitive::u32,
                        successful_instances: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 20)]
                    AttributeSet {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                        value: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                    #[codec(index = 21)]
                    AttributeCleared {
                        class: ::core::primitive::u32,
                        maybe_instance: ::core::option::Option<::core::primitive::u32>,
                        key: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct ClassDetails<_0, _1> {
                    pub owner: _0,
                    pub issuer: _0,
                    pub admin: _0,
                    pub freezer: _0,
                    pub total_deposit: _1,
                    pub free_holding: ::core::primitive::bool,
                    pub instances: ::core::primitive::u32,
                    pub instance_metadatas: ::core::primitive::u32,
                    pub attributes: ::core::primitive::u32,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct ClassMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct DestroyWitness {
                    #[codec(compact)]
                    pub instances: ::core::primitive::u32,
                    #[codec(compact)]
                    pub instance_metadatas: ::core::primitive::u32,
                    #[codec(compact)]
                    pub attributes: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct InstanceDetails<_0, _1> {
                    pub owner: _0,
                    pub approved: ::core::option::Option<_0>,
                    pub is_frozen: ::core::primitive::bool,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct InstanceMetadata<_0> {
                    pub deposit: _0,
                    pub data: runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub is_frozen: ::core::primitive::bool,
                }
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    batch { calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call> },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    batch_all { calls: ::std::vec::Vec<runtime_types::appchain_deip_runtime::Call> },
                    #[codec(index = 3)]
                    dispatch_as {
                        as_origin:
                            ::std::boxed::Box<runtime_types::appchain_deip_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::appchain_deip_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                    #[codec(index = 3)]
                    DispatchedAs(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub struct H160(pub [::core::primitive::u8; 20usize]);
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode,
                        :: subxt :: codec :: Decode,
                        Debug,
                        Clone,
                        Eq,
                        PartialEq,
                        scale_info :: TypeInfo,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    Eq,
                    PartialEq,
                    scale_info :: TypeInfo,
                )]
                pub struct BlakeTwo256;
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module { index: ::core::primitive::u8, error: ::core::primitive::u8 },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
                serde::Serialize,
            )]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                Eq,
                PartialEq,
                scale_info :: TypeInfo,
            )]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
    }
    /// The default error type returned when there is a runtime issue.
    pub type DispatchError = self::runtime_types::sp_runtime::DispatchError;
    pub struct ErrorDetails {
        pub pallet: &'static str,
        pub error: &'static str,
        pub docs: &'static str,
    }
    impl DispatchError {
        pub fn details(&self) -> Option<ErrorDetails> {
            if let Self::Module { index, error } = self {
                match (index , error) { (0u8 , 0u8) => Some (ErrorDetails { pallet : "System" , error : "InvalidSpecName" , docs : "The name of specification does not match between the current runtime\nand the new runtime." }) , (0u8 , 1u8) => Some (ErrorDetails { pallet : "System" , error : "SpecVersionNeedsToIncrease" , docs : "The specification version is not allowed to decrease between the current runtime\nand the new runtime." }) , (0u8 , 2u8) => Some (ErrorDetails { pallet : "System" , error : "FailedToExtractRuntimeVersion" , docs : "Failed to extract the runtime version from the new runtime.\n\nEither calling `Core_version` or decoding `RuntimeVersion` failed." }) , (0u8 , 3u8) => Some (ErrorDetails { pallet : "System" , error : "NonDefaultComposite" , docs : "Suicide called when the account has non-default composite data." }) , (0u8 , 4u8) => Some (ErrorDetails { pallet : "System" , error : "NonZeroRefCount" , docs : "There is a non-zero reference count preventing the account from being purged." }) , (0u8 , 5u8) => Some (ErrorDetails { pallet : "System" , error : "CallFiltered" , docs : "The origin filter prevent the call to be dispatched." }) , (1u8 , 0u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (1u8 , 1u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (1u8 , 2u8) => Some (ErrorDetails { pallet : "Babe" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (3u8 , 0u8) => Some (ErrorDetails { pallet : "Authorship" , error : "InvalidUncleParent" , docs : "The uncle parent not in the chain." }) , (3u8 , 1u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UnclesAlreadySet" , docs : "Uncles already set in the block." }) , (3u8 , 2u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooManyUncles" , docs : "Too many uncles." }) , (3u8 , 3u8) => Some (ErrorDetails { pallet : "Authorship" , error : "GenesisUncle" , docs : "The uncle is genesis." }) , (3u8 , 4u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooHighUncle" , docs : "The uncle is too high in chain." }) , (3u8 , 5u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UncleAlreadyIncluded" , docs : "The uncle is already included." }) , (3u8 , 6u8) => Some (ErrorDetails { pallet : "Authorship" , error : "OldUncle" , docs : "The uncle isn't recent enough to be included." }) , (4u8 , 0u8) => Some (ErrorDetails { pallet : "Balances" , error : "VestingBalance" , docs : "Vesting balance too high to send value" }) , (4u8 , 1u8) => Some (ErrorDetails { pallet : "Balances" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal" }) , (4u8 , 2u8) => Some (ErrorDetails { pallet : "Balances" , error : "InsufficientBalance" , docs : "Balance too low to send value" }) , (4u8 , 3u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit" }) , (4u8 , 4u8) => Some (ErrorDetails { pallet : "Balances" , error : "KeepAlive" , docs : "Transfer/payment would kill account" }) , (4u8 , 5u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account" }) , (4u8 , 6u8) => Some (ErrorDetails { pallet : "Balances" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist" }) , (4u8 , 7u8) => Some (ErrorDetails { pallet : "Balances" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (6u8 , 0u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "WrongSetId" , docs : "The set id of new validator set was wrong." }) , (6u8 , 1u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "InvalidNotificationId" , docs : "Invalid notification id of observation." }) , (6u8 , 2u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "NotValidator" , docs : "Must be a validator." }) , (6u8 , 3u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "AmountOverflow" , docs : "Amount overflow." }) , (6u8 , 4u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "NextNotificationIdOverflow" , docs : "Next notification Id overflow." }) , (6u8 , 5u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "WrongAssetId" , docs : "Wrong Asset Id." }) , (6u8 , 6u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "InvalidActiveTotalStake" , docs : "Invalid active total stake." }) , (6u8 , 7u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "NotActivated" , docs : "Appchain is not activated." }) , (6u8 , 8u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "InvalidReceiverId" , docs : "ReceiverId is not a valid utf8 string." }) , (6u8 , 9u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "InvalidTokenId" , docs : "Token is not a valid utf8 string." }) , (6u8 , 10u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "NextSetIdOverflow" , docs : "Next set Id overflow." }) , (6u8 , 11u8) => Some (ErrorDetails { pallet : "OctopusAppchain" , error : "ObservationsExceededLimit" , docs : "Observations exceeded limit." }) , (7u8 , 0u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NotController" , docs : "Not a controller account." }) , (7u8 , 1u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NotStash" , docs : "Not a stash account." }) , (7u8 , 2u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "AlreadyBonded" , docs : "Stash is already bonded." }) , (7u8 , 3u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "AlreadyPaired" , docs : "Controller is already paired." }) , (7u8 , 4u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "EmptyTargets" , docs : "Targets cannot be empty." }) , (7u8 , 5u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "DuplicateIndex" , docs : "Duplicate index." }) , (7u8 , 6u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "InvalidSlashIndex" , docs : "Slash record index out of bounds." }) , (7u8 , 7u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "InsufficientBond" , docs : "Can not bond with value less than minimum required." }) , (7u8 , 8u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NoMoreChunks" , docs : "Can not schedule more unlock chunks." }) , (7u8 , 9u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NoUnlockChunk" , docs : "Can not rebond without unlocking chunks." }) , (7u8 , 10u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "FundedTarget" , docs : "Attempting to target a stash that still has funds." }) , (7u8 , 11u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "InvalidEraToReward" , docs : "Invalid era to reward." }) , (7u8 , 12u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "InvalidNumberOfNominations" , docs : "Invalid number of nominations." }) , (7u8 , 13u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NotSortedAndUnique" , docs : "Items are not sorted and unique." }) , (7u8 , 14u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "AlreadyClaimed" , docs : "Rewards for this era have already been claimed for this validator." }) , (7u8 , 15u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "IncorrectHistoryDepth" , docs : "Incorrect previous history depth input provided." }) , (7u8 , 16u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "IncorrectSlashingSpans" , docs : "Incorrect number of slashing spans provided." }) , (7u8 , 17u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "BadState" , docs : "Internal state has become somehow corrupted and the operation cannot continue." }) , (7u8 , 18u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "TooManyTargets" , docs : "Too many nomination targets supplied." }) , (7u8 , 19u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "BadTarget" , docs : "A nomination target was supplied that was blocked or otherwise not a validator." }) , (7u8 , 20u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "CannotChillOther" , docs : "The user has enough bond and thus cannot be chilled forcefully by an external person." }) , (7u8 , 21u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "TooManyNominators" , docs : "There are too many nominators in the system. Governance needs to adjust the staking settings\nto keep things safe for the runtime." }) , (7u8 , 22u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "TooManyValidators" , docs : "There are too many validators in the system. Governance needs to adjust the staking settings\nto keep things safe for the runtime." }) , (7u8 , 23u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "NoClaimedRewards" , docs : "There are not claimed rewards for this validator." }) , (7u8 , 24u8) => Some (ErrorDetails { pallet : "OctopusLpos" , error : "AmountOverflow" , docs : "Amount overflow." }) , (8u8 , 0u8) => Some (ErrorDetails { pallet : "OctopusUpwardMessages" , error : "NonceOverflow" , docs : "Nonce overflow." }) , (8u8 , 1u8) => Some (ErrorDetails { pallet : "OctopusUpwardMessages" , error : "QueueSizeLimitReached" , docs : "Queue size limit reached." }) , (9u8 , 0u8) => Some (ErrorDetails { pallet : "Session" , error : "InvalidProof" , docs : "Invalid ownership proof." }) , (9u8 , 1u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAssociatedValidatorId" , docs : "No associated validator ID for account." }) , (9u8 , 2u8) => Some (ErrorDetails { pallet : "Session" , error : "DuplicatedKey" , docs : "Registered duplicate key." }) , (9u8 , 3u8) => Some (ErrorDetails { pallet : "Session" , error : "NoKeys" , docs : "No keys are associated with this account." }) , (9u8 , 4u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAccount" , docs : "Key setting account is not live, so it's impossible to associate keys." }) , (10u8 , 0u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "PauseFailed" , docs : "Attempt to signal GRANDPA pause when the authority set isn't live\n(either paused or already pending pause)." }) , (10u8 , 1u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ResumeFailed" , docs : "Attempt to signal GRANDPA resume when the authority set isn't paused\n(either live or already pending resume)." }) , (10u8 , 2u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ChangePending" , docs : "Attempt to signal GRANDPA change with one already pending." }) , (10u8 , 3u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "TooSoon" , docs : "Cannot signal forced change so soon after last." }) , (10u8 , 4u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (10u8 , 5u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (10u8 , 6u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (11u8 , 0u8) => Some (ErrorDetails { pallet : "Sudo" , error : "RequireSudo" , docs : "Sender must be the Sudo account" }) , (12u8 , 0u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "InvalidKey" , docs : "Non existent public key." }) , (12u8 , 1u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "DuplicatedHeartbeat" , docs : "Duplicated heartbeat." }) , (15u8 , 0u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "BalanceLow" , docs : "Account balance must be greater than or equal to the transfer amount." }) , (15u8 , 1u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "BalanceZero" , docs : "Balance should be non-zero." }) , (15u8 , 2u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "NoPermission" , docs : "The signing account has no permission to do the operation." }) , (15u8 , 3u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "Unknown" , docs : "The given asset ID is unknown." }) , (15u8 , 4u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "Frozen" , docs : "The origin account is frozen." }) , (15u8 , 5u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "InUse" , docs : "The asset ID is already taken." }) , (15u8 , 6u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "BadWitness" , docs : "Invalid witness data given." }) , (15u8 , 7u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "MinBalanceZero" , docs : "Minimum balance should be non-zero." }) , (15u8 , 8u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "NoProvider" , docs : "No provider reference exists to allow a non-zero balance of a non-self-sufficient\nasset." }) , (15u8 , 9u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "BadMetadata" , docs : "Invalid metadata given." }) , (15u8 , 10u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "Unapproved" , docs : "No approval exists that would allow the transfer." }) , (15u8 , 11u8) => Some (ErrorDetails { pallet : "ParityTechAssets" , error : "WouldDie" , docs : "The source account would not survive the transfer and it needs to stay alive." }) , (16u8 , 0u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "NoPermission" , docs : "The signing account has no permission to do the operation." }) , (16u8 , 1u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "Unknown" , docs : "The given asset ID is unknown." }) , (16u8 , 2u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "AlreadyExists" , docs : "The asset instance ID has already been used for an asset." }) , (16u8 , 3u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "WrongOwner" , docs : "The owner turned out to be different to what was expected." }) , (16u8 , 4u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "BadWitness" , docs : "Invalid witness data given." }) , (16u8 , 5u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "InUse" , docs : "The asset ID is already taken." }) , (16u8 , 6u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "Frozen" , docs : "The asset instance or class is frozen." }) , (16u8 , 7u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "WrongDelegate" , docs : "The delegate turned out to be different to what was expected." }) , (16u8 , 8u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "NoDelegate" , docs : "There is no delegate approved." }) , (16u8 , 9u8) => Some (ErrorDetails { pallet : "ParityTechUniques" , error : "Unapproved" , docs : "No approval exists that would allow the transfer." }) , (19u8 , 0u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MinimumThreshold" , docs : "Threshold must be 2 or greater." }) , (19u8 , 1u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyApproved" , docs : "Call is already approved by this signatory." }) , (19u8 , 2u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoApprovalsNeeded" , docs : "Call doesn't need any (more) approvals." }) , (19u8 , 3u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooFewSignatories" , docs : "There are too few signatories in the list." }) , (19u8 , 4u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooManySignatories" , docs : "There are too many signatories in the list." }) , (19u8 , 5u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SignatoriesOutOfOrder" , docs : "The signatories were provided out of order; they should be ordered." }) , (19u8 , 6u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SenderInSignatories" , docs : "The sender was contained in the other signatories; it shouldn't be." }) , (19u8 , 7u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotFound" , docs : "Multisig operation not found when attempting to cancel." }) , (19u8 , 8u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotOwner" , docs : "Only the account that originally created the multisig is able to cancel it." }) , (19u8 , 9u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoTimepoint" , docs : "No timepoint was given, yet the multisig operation is already underway." }) , (19u8 , 10u8) => Some (ErrorDetails { pallet : "Multisig" , error : "WrongTimepoint" , docs : "A different timepoint was given to the multisig operation that is underway." }) , (19u8 , 11u8) => Some (ErrorDetails { pallet : "Multisig" , error : "UnexpectedTimepoint" , docs : "A timepoint was given, yet no multisig operation is underway." }) , (19u8 , 12u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MaxWeightTooLow" , docs : "The maximum weight information provided was too low." }) , (19u8 , 13u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyStored" , docs : "The data to be stored is already stored." }) , (20u8 , 0u8) => Some (ErrorDetails { pallet : "Utility" , error : "TooManyCalls" , docs : "Too many calls batched." }) , (21u8 , 0u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoSuchProject" , docs : "The project does not exist." }) , (21u8 , 1u8) => Some (ErrorDetails { pallet : "Deip" , error : "NotProjectOwner" , docs : "The project is created by another account, so caller can't remove it." }) , (21u8 , 2u8) => Some (ErrorDetails { pallet : "Deip" , error : "DomainNotExists" , docs : "Cannot add domain into the porject because this domain not exists" }) , (21u8 , 3u8) => Some (ErrorDetails { pallet : "Deip" , error : "ProjectAlreadyExists" , docs : "Cannot add a project because a project with this ID is already a exists" }) , (21u8 , 4u8) => Some (ErrorDetails { pallet : "Deip" , error : "ProjectContentAlreadyExists" , docs : "Cannot add a project content because a project content with this ID is already a exists." }) , (21u8 , 5u8) => Some (ErrorDetails { pallet : "Deip" , error : "ProjectNotBelongToTeam" , docs : "Project does not belong to the team." }) , (21u8 , 6u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoSuchProjectContent" , docs : "The project content does not exist." }) , (21u8 , 7u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoSuchReference" , docs : "The Reference does not exist." }) , (21u8 , 8u8) => Some (ErrorDetails { pallet : "Deip" , error : "ProjectAlreadyFinished" , docs : "Cannot add a project content because a project with this ID is already a finished" }) , (21u8 , 9u8) => Some (ErrorDetails { pallet : "Deip" , error : "DomainLimitReached" , docs : "Cannot add another domain because the limit is already reached" }) , (21u8 , 10u8) => Some (ErrorDetails { pallet : "Deip" , error : "DomainAlreadyExists" , docs : "Cannot add domain because this domain is already a exists" }) , (21u8 , 11u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaAlreadyExists" , docs : "Cannot add a NDA because a NDA with this ID is already a exists." }) , (21u8 , 12u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaAccessRequestAlreadyExists" , docs : "Nda Access Request with this ID is  already a exists." }) , (21u8 , 13u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoSuchNda" , docs : "The NDA with this ID does not exist." }) , (21u8 , 14u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoSuchNdaAccessRequest" , docs : "The NDA Access Request with this ID does not exist." }) , (21u8 , 15u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaContractIsNotActiveYet" , docs : "The start of the contract has not yet arrived, so contract can't be fulfilled or rejected" }) , (21u8 , 16u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaStartDateMustBeLaterOrEqualCurrentMoment" , docs : "NDA start date must be later or equal current moment" }) , (21u8 , 17u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaEndDateMustBeLaterCurrentMoment" , docs : "NDA end date must be later current moment" }) , (21u8 , 18u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaStartDateMustBeLessThanEndDate" , docs : "NDA start date must be less than end date" }) , (21u8 , 19u8) => Some (ErrorDetails { pallet : "Deip" , error : "TeamOfAllProjectsMustSpecifiedAsParty" , docs : "Team of all projects must specified as party" }) , (21u8 , 20u8) => Some (ErrorDetails { pallet : "Deip" , error : "NdaAccessRequestAlreadyFinalized" , docs : "Nda access request already finalized" }) , (21u8 , 21u8) => Some (ErrorDetails { pallet : "Deip" , error : "TooMuchNdaParties" , docs : "" }) , (21u8 , 22u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewAlreadyExists" , docs : "Cannot add a review because a review with this ID already exists" }) , (21u8 , 23u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewNoDomainSpecified" , docs : "" }) , (21u8 , 24u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewVoteAlreadyExists" , docs : "" }) , (21u8 , 25u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewVoteNoSuchDomain" , docs : "" }) , (21u8 , 26u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewVoteNoSuchReview" , docs : "" }) , (21u8 , 27u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewVoteUnrelatedDomain" , docs : "" }) , (21u8 , 28u8) => Some (ErrorDetails { pallet : "Deip" , error : "ReviewAlreadyVotedWithDomain" , docs : "" }) , (21u8 , 29u8) => Some (ErrorDetails { pallet : "Deip" , error : "NoPermission" , docs : "Access Forbiten" }) , (21u8 , 30u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityStartTimeMustBeLaterOrEqualCurrentMoment" , docs : "" }) , (21u8 , 31u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityEndTimeMustBeLaterStartTime" , docs : "" }) , (21u8 , 32u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunitySoftCapMustBeGreaterOrEqualMinimum" , docs : "" }) , (21u8 , 33u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityHardCapShouldBeGreaterOrEqualSoftCap" , docs : "" }) , (21u8 , 34u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityAlreadyExists" , docs : "" }) , (21u8 , 35u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityBalanceIsNotEnough" , docs : "" }) , (21u8 , 36u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityFailedToReserveAsset" , docs : "" }) , (21u8 , 37u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityAssetAmountMustBePositive" , docs : "" }) , (21u8 , 38u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunitySecurityTokenNotSpecified" , docs : "" }) , (21u8 , 39u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityNotFound" , docs : "" }) , (21u8 , 40u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityShouldBeInactive" , docs : "" }) , (21u8 , 41u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityShouldBeStarted" , docs : "" }) , (21u8 , 42u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityShouldBeActive" , docs : "" }) , (21u8 , 43u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityExpirationWrongState" , docs : "" }) , (21u8 , 44u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityWrongAssetId" , docs : "" }) , (21u8 , 45u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestmentOpportunityCapDifferentAssets" , docs : "" }) , (21u8 , 46u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestingNotFound" , docs : "" }) , (21u8 , 47u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestingNotActive" , docs : "" }) , (21u8 , 48u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestingNotEnoughFunds" , docs : "" }) , (21u8 , 49u8) => Some (ErrorDetails { pallet : "Deip" , error : "InvestingWrongAsset" , docs : "" }) , (21u8 , 50u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementNoParties" , docs : "" }) , (21u8 , 51u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementStartTimeMustBeLaterOrEqualCurrentMoment" , docs : "" }) , (21u8 , 52u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementEndTimeMustBeLaterStartTime" , docs : "" }) , (21u8 , 53u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementAlreadyExists" , docs : "" }) , (21u8 , 54u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementFeeMustBePositive" , docs : "" }) , (21u8 , 55u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseTwoPartiesRequired" , docs : "" }) , (21u8 , 56u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseProjectTeamIsNotListedInParties" , docs : "" }) , (21u8 , 57u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementNotFound" , docs : "" }) , (21u8 , 58u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementWrongAgreement" , docs : "" }) , (21u8 , 59u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementAlreadyAccepted" , docs : "" }) , (21u8 , 60u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicensePartyIsNotLicenser" , docs : "" }) , (21u8 , 61u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicensePartyIsNotLicensee" , docs : "" }) , (21u8 , 62u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseExpired" , docs : "" }) , (21u8 , 63u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseNotEnoughBalance" , docs : "" }) , (21u8 , 64u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseFailedToChargeFee" , docs : "" }) , (21u8 , 65u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementLicenseIsNotActive" , docs : "" }) , (21u8 , 66u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementPartyIsNotListed" , docs : "" }) , (21u8 , 67u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementAlreadyAcceptedByParty" , docs : "" }) , (21u8 , 68u8) => Some (ErrorDetails { pallet : "Deip" , error : "ContractAgreementRejected" , docs : "" }) , (22u8 , 0u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectDoesNotExist" , docs : "" }) , (22u8 , 1u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectDoesNotBelongToTeam" , docs : "" }) , (22u8 , 2u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectSecurityTokenCannotBeDestroyed" , docs : "" }) , (22u8 , 3u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectSecurityTokenCannotBeBurned" , docs : "" }) , (22u8 , 4u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectSecurityTokenCannotBeFreezed" , docs : "" }) , (22u8 , 5u8) => Some (ErrorDetails { pallet : "Assets" , error : "ProjectSecurityTokenAccountCannotBeFreezed" , docs : "" }) , (22u8 , 6u8) => Some (ErrorDetails { pallet : "Assets" , error : "ReservedAssetCannotBeFreezed" , docs : "" }) , (22u8 , 7u8) => Some (ErrorDetails { pallet : "Assets" , error : "ReservedAssetAccountCannotBeFreezed" , docs : "" }) , (22u8 , 8u8) => Some (ErrorDetails { pallet : "Assets" , error : "FtNotFound" , docs : "" }) , (22u8 , 9u8) => Some (ErrorDetails { pallet : "Assets" , error : "FtBalanceNotFound" , docs : "" }) , (22u8 , 10u8) => Some (ErrorDetails { pallet : "Assets" , error : "AssetIdOverflow" , docs : "" }) , (22u8 , 11u8) => Some (ErrorDetails { pallet : "Assets" , error : "DeipAssetIdExists" , docs : "" }) , (22u8 , 12u8) => Some (ErrorDetails { pallet : "Assets" , error : "DeipAssetIdDoesNotExist" , docs : "Asset with DeipAssetId wasn't created." }) , (23u8 , 0u8) => Some (ErrorDetails { pallet : "Uniques" , error : "DeipNftClassIdExists" , docs : "" }) , (23u8 , 1u8) => Some (ErrorDetails { pallet : "Uniques" , error : "DeipNftClassIdDoesNotExist" , docs : "" }) , (23u8 , 2u8) => Some (ErrorDetails { pallet : "Uniques" , error : "NftClassIdOverflow" , docs : "" }) , (23u8 , 3u8) => Some (ErrorDetails { pallet : "Uniques" , error : "ProjectDoesNotExist" , docs : "" }) , (23u8 , 4u8) => Some (ErrorDetails { pallet : "Uniques" , error : "ProjectDoesNotBelongToTeam" , docs : "" }) , (23u8 , 5u8) => Some (ErrorDetails { pallet : "Uniques" , error : "ProjectSecurityTokenCannotBeDestroyed" , docs : "" }) , (23u8 , 6u8) => Some (ErrorDetails { pallet : "Uniques" , error : "ProjectSecurityTokenCannotBeBurned" , docs : "" }) , (23u8 , 7u8) => Some (ErrorDetails { pallet : "Uniques" , error : "ProjectSecurityTokenCannotBeFrozen" , docs : "" }) , (24u8 , 0u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "NotFound" , docs : "Proposal not found" }) , (24u8 , 1u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "AlreadyExist" , docs : "Proposal already exist" }) , (24u8 , 2u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "NotAMember" , docs : "Current origin is not a member of Proposal" }) , (24u8 , 3u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "AlreadyResolved" , docs : "Proposal already resolved (done, failed or rejected)" }) , (24u8 , 4u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "ImpossibleDecision" , docs : "Decision in not possible in the current state" }) , (24u8 , 5u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "ReachDepthLimit" , docs : "Reach depth limit of nested proposals" }) , (24u8 , 6u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "ReachSizeLimit" , docs : "Reach size limit of proposal's batch" }) , (24u8 , 7u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "SelfReferential" , docs : "Self-referential proposal" }) , (24u8 , 8u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "NotExpired" , docs : "Not expired yet" }) , (24u8 , 9u8) => Some (ErrorDetails { pallet : "DeipProposal" , error : "BatchWeightTooLow" , docs : "Provided batch weight is lower than expected" }) , (25u8 , 0u8) => Some (ErrorDetails { pallet : "DeipDao" , error : "Exists" , docs : "Already exists (unique by `name`)" }) , (25u8 , 1u8) => Some (ErrorDetails { pallet : "DeipDao" , error : "NotFound" , docs : "Not found" }) , (25u8 , 2u8) => Some (ErrorDetails { pallet : "DeipDao" , error : "Forbidden" , docs : "Access denied" }) , (25u8 , 3u8) => Some (ErrorDetails { pallet : "DeipDao" , error : "AuthorityMismatch" , docs : "" }) , (26u8 , 0u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "DelegateMismatch" , docs : "" }) , (26u8 , 1u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "PortalMismatch" , docs : "" }) , (26u8 , 2u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "AlreadyScheduled" , docs : "" }) , (26u8 , 3u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "UnproperCall" , docs : "" }) , (26u8 , 4u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "NotScheduled" , docs : "" }) , (26u8 , 5u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "OwnerIsNotATenant" , docs : "" }) , (26u8 , 6u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "PortalAlreadyExist" , docs : "" }) , (26u8 , 7u8) => Some (ErrorDetails { pallet : "DeipPortal" , error : "PortalNotFound" , docs : "" }) , (27u8 , 0u8) => Some (ErrorDetails { pallet : "DeipVesting" , error : "ExistingVestingPlan" , docs : "" }) , (27u8 , 1u8) => Some (ErrorDetails { pallet : "DeipVesting" , error : "AmountLow" , docs : "" }) , (27u8 , 2u8) => Some (ErrorDetails { pallet : "DeipVesting" , error : "InvalidVestingPlan" , docs : "" }) , (27u8 , 3u8) => Some (ErrorDetails { pallet : "DeipVesting" , error : "NoVestingPlan" , docs : "" }) , _ => None }
            } else {
                None
            }
        }
    }
    /// The default storage entry from which to fetch an account nonce, required for
    /// constructing a transaction.
    pub enum DefaultAccountData {}
    impl ::subxt::AccountData for DefaultAccountData {
        type StorageEntry = self::system::storage::Account;
        type AccountId = ::subxt::sp_core::crypto::AccountId32;
        type Index = ::core::primitive::u32;
        fn nonce(result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value) -> Self::Index {
            result.nonce
        }
        fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
            self::system::storage::Account(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X, A = DefaultAccountData> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self { client, marker: ::core::marker::PhantomData }
        }
    }
    impl<'a, T, X, A> RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn constants(&'a self) -> ConstantsApi {
            ConstantsApi
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi { client: &self.client }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
            TransactionApi { client: &self.client, marker: ::core::marker::PhantomData }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi {
            babe::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::constants::ConstantsApi {
            octopus_appchain::constants::ConstantsApi
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::constants::ConstantsApi {
            octopus_lpos::constants::ConstantsApi
        }
        pub fn octopus_upward_messages(&self) -> octopus_upward_messages::constants::ConstantsApi {
            octopus_upward_messages::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
        pub fn parity_tech_assets(&self) -> parity_tech_assets::constants::ConstantsApi {
            parity_tech_assets::constants::ConstantsApi
        }
        pub fn parity_tech_uniques(&self) -> parity_tech_uniques::constants::ConstantsApi {
            parity_tech_uniques::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
        pub fn assets(&self) -> assets::constants::ConstantsApi {
            assets::constants::ConstantsApi
        }
        pub fn deip_proposal(&self) -> deip_proposal::constants::ConstantsApi {
            deip_proposal::constants::ConstantsApi
        }
        pub fn deip_dao(&self) -> deip_dao::constants::ConstantsApi {
            deip_dao::constants::ConstantsApi
        }
        pub fn deip_vesting(&self) -> deip_vesting::constants::ConstantsApi {
            deip_vesting::constants::ConstantsApi
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::storage::StorageApi<'a, T> {
            octopus_appchain::storage::StorageApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::storage::StorageApi<'a, T> {
            octopus_lpos::storage::StorageApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::storage::StorageApi<'a, T> {
            octopus_upward_messages::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn parity_tech_assets(&self) -> parity_tech_assets::storage::StorageApi<'a, T> {
            parity_tech_assets::storage::StorageApi::new(self.client)
        }
        pub fn parity_tech_uniques(&self) -> parity_tech_uniques::storage::StorageApi<'a, T> {
            parity_tech_uniques::storage::StorageApi::new(self.client)
        }
        pub fn mmr(&self) -> mmr::storage::StorageApi<'a, T> {
            mmr::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn deip(&self) -> deip::storage::StorageApi<'a, T> {
            deip::storage::StorageApi::new(self.client)
        }
        pub fn assets(&self) -> assets::storage::StorageApi<'a, T> {
            assets::storage::StorageApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::storage::StorageApi<'a, T> {
            uniques::storage::StorageApi::new(self.client)
        }
        pub fn deip_proposal(&self) -> deip_proposal::storage::StorageApi<'a, T> {
            deip_proposal::storage::StorageApi::new(self.client)
        }
        pub fn deip_dao(&self) -> deip_dao::storage::StorageApi<'a, T> {
            deip_dao::storage::StorageApi::new(self.client)
        }
        pub fn deip_portal(&self) -> deip_portal::storage::StorageApi<'a, T> {
            deip_portal::storage::StorageApi::new(self.client)
        }
        pub fn deip_vesting(&self) -> deip_vesting::storage::StorageApi<'a, T> {
            deip_vesting::storage::StorageApi::new(self.client)
        }
        pub fn deip_ecosystem_fund(&self) -> deip_ecosystem_fund::storage::StorageApi<'a, T> {
            deip_ecosystem_fund::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<'a, T, X, A> TransactionApi<'a, T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X, A> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X, A> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X, A> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_appchain(&self) -> octopus_appchain::calls::TransactionApi<'a, T, X, A> {
            octopus_appchain::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_lpos(&self) -> octopus_lpos::calls::TransactionApi<'a, T, X, A> {
            octopus_lpos::calls::TransactionApi::new(self.client)
        }
        pub fn octopus_upward_messages(
            &self,
        ) -> octopus_upward_messages::calls::TransactionApi<'a, T, X, A> {
            octopus_upward_messages::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X, A> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X, A> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T, X, A> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T, X, A> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T, X, A> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn deip(&self) -> deip::calls::TransactionApi<'a, T, X, A> {
            deip::calls::TransactionApi::new(self.client)
        }
        pub fn assets(&self) -> assets::calls::TransactionApi<'a, T, X, A> {
            assets::calls::TransactionApi::new(self.client)
        }
        pub fn uniques(&self) -> uniques::calls::TransactionApi<'a, T, X, A> {
            uniques::calls::TransactionApi::new(self.client)
        }
        pub fn deip_proposal(&self) -> deip_proposal::calls::TransactionApi<'a, T, X, A> {
            deip_proposal::calls::TransactionApi::new(self.client)
        }
        pub fn deip_dao(&self) -> deip_dao::calls::TransactionApi<'a, T, X, A> {
            deip_dao::calls::TransactionApi::new(self.client)
        }
        pub fn deip_portal(&self) -> deip_portal::calls::TransactionApi<'a, T, X, A> {
            deip_portal::calls::TransactionApi::new(self.client)
        }
        pub fn deip_vesting(&self) -> deip_vesting::calls::TransactionApi<'a, T, X, A> {
            deip_vesting::calls::TransactionApi::new(self.client)
        }
    }
}
