use jsonrpc_core::{
    futures::{
        self,
        future::{self, Future},
        stream, Stream,
    },
    futures_util::{stream::FuturesOrdered, StreamExt},
    BoxFuture,
};

pub use sp_core::{
    hashing::twox_128_into,
    storage::{StorageData, StorageKey},
};

use frame_support::{ReversibleStorageHasher, StorageHasher};

use codec::{Decode, Encode};

use std::{convert::TryFrom, iter::FromIterator};

pub mod error;
pub use error::*;

pub type HashOf<T> = <T as sp_runtime::traits::Block>::Hash;
pub type BoxFutureResult<T> = BoxFuture<Result<T, RpcError>>;

pub fn prefix(pallet: &[u8], storage: &[u8]) -> Vec<u8> {
    let mut prefix = Vec::new();
    prefix.resize(32, 0u8);

    twox_128_into(pallet, <&mut [u8; 16]>::try_from(&mut prefix[..16]).unwrap());
    twox_128_into(storage, <&mut [u8; 16]>::try_from(&mut prefix[16..]).unwrap());

    prefix
}

pub struct HashedKey<Hasher: StorageHasher>(Hasher::Output);
pub struct HashedKeyRef<'a, Hasher: StorageHasher>(&'a [u8], std::marker::PhantomData<Hasher>);

pub trait HashedKeyTrait {
    fn as_ref(&self) -> &[u8];
}

impl<Hasher: StorageHasher> HashedKey<Hasher> {
    pub fn new<Key: Encode>(key: &Key) -> Self {
        Self(key.using_encoded(Hasher::hash))
    }

    pub fn unsafe_from_encoded(encoded: &[u8]) -> Self {
        Self(Hasher::hash(encoded))
    }
}

impl<Hasher: StorageHasher> HashedKeyTrait for HashedKey<Hasher> {
    fn as_ref(&self) -> &[u8] {
        return self.0.as_ref()
    }
}

impl<'a, Hasher: StorageHasher> HashedKeyTrait for HashedKeyRef<'a, Hasher> {
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'a, Hasher: StorageHasher> HashedKeyRef<'a, Hasher> {
    pub fn unsafe_from_hashed(hashed: &'a [u8]) -> Self {
        Self(hashed, Default::default())
    }
}

// The trait is designed to map tuple with data and composite key
// to a tuple with data and a Result containing composite key.
// Check `get_list_by_keys` and the second implementation for details.
pub trait CompositeKeyTrait<Key, Hasher> {
    type KeyType;

    fn decompose(self) -> (Option<StorageData>, Result<Self::KeyType, Vec<u8>>);
}

// This implementation is for ordinary use when we just retrieve data
// from the map. Check this file for usage.
impl<Key, Hasher> CompositeKeyTrait<Key, Hasher> for (Option<StorageData>, StorageKey)
where
    Key: Decode,
    Hasher: StorageHasher + ReversibleStorageHasher,
{
    type KeyType = Key;

    fn decompose(self) -> (Option<StorageData>, Result<Self::KeyType, Vec<u8>>) {
        let key = self.1;
        let no_prefix = Hasher::reverse(&key.0[32..]);
        (self.0, Key::decode(&mut &no_prefix[..]).map_err(|_| key.0))
    }
}

// This implementation is for case when index-map is used to get the
// key which is used to retrieve data from the map, but the index key
// should also be kept.
impl<Key, Hasher, IndexKey> CompositeKeyTrait<Key, Hasher>
    for (Option<StorageData>, StorageKey, IndexKey)
where
    Key: Decode,
    Hasher: StorageHasher + ReversibleStorageHasher,
{
    type KeyType = (IndexKey, Key);

    fn decompose(self) -> (Option<StorageData>, Result<Self::KeyType, Vec<u8>>) {
        let (data, key, index) = self;
        let x = CompositeKeyTrait::<Key, Hasher>::decompose((data, key));
        (x.0, x.1.map(|k| (index, k)))
    }
}

pub fn chain_key_hash_map<T: HashedKeyTrait>(prefix: &[u8], key: &T) -> StorageKey {
    StorageKey(prefix.iter().chain(key.as_ref()).map(|b| *b).collect())
}

pub fn key_hash_map<Key: Encode, Hasher: StorageHasher>(
    pallet: &[u8],
    map: &[u8],
    key: &Key,
) -> StorageKey {
    chain_key_hash_map(prefix(pallet, map).as_ref(), &HashedKey::<Hasher>::new(key))
}

pub fn chain_key_hash_double_map<KeyFirst: HashedKeyTrait, KeySecond: HashedKeyTrait>(
    prefix: &[u8],
    key_first: &KeyFirst,
    key_second: &KeySecond,
) -> StorageKey {
    StorageKey(
        prefix
            .iter()
            .chain(key_first.as_ref())
            .chain(key_second.as_ref())
            .map(|b| *b)
            .collect(),
    )
}

pub fn key_hash_double_map<KeyFirst, KeySecond, HasherFirst, HasherSecond>(
    pallet: &[u8],
    map: &[u8],
    key_first: &KeyFirst,
    key_second: &KeySecond,
) -> StorageKey
where
    KeyFirst: Encode,
    KeySecond: Encode,
    HasherFirst: StorageHasher,
    HasherSecond: StorageHasher,
{
    chain_key_hash_double_map(
        prefix(pallet, map).as_ref(),
        &HashedKey::<HasherFirst>::new(key_first),
        &HashedKey::<HasherSecond>::new(key_second),
    )
}

pub fn get_value<R, State, Hash>(
    state: &State,
    key: StorageKey,
    at: Option<Hash>,
) -> BoxFutureResult<Option<R>>
where
    R: 'static + Decode + GetError + Send,
    State: sc_rpc_api::state::StateApi<Hash>,
    Hash: Copy,
{
    // let x = state
    //     .storage(key, at)
    //     .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
    //     .and_then(|d| match d {
    //         None => future::ok(None),
    //         Some(data) => match R::decode(&mut &data.0[..]) {
    //             Err(_) => future::err(to_rpc_error(R::get_error(), Some(format!("{:?}", data)))),
    //             Ok(decoded) => future::ok(Some(decoded)),
    //         },
    //     });
    // Box::new(x)
    todo!()
}

pub fn get_list_by_keys<KeyValue, Hasher, State, BlockHash, KeyMap, T, Item>(
    state: &State,
    at: Option<BlockHash>,
    prefix_key: StorageKey,
    count: u32,
    start_key: Option<StorageKey>,
    key_map: KeyMap,
) -> BoxFutureResult<
    Vec<ListResult<<Item as CompositeKeyTrait<KeyValue::Key, Hasher>>::KeyType, KeyValue::Value>>,
>
where
    KeyValue: KeyValueInfo,
    Hasher: StorageHasher + ReversibleStorageHasher,
    State: sc_rpc_api::state::StateApi<BlockHash>,
    BlockHash: Copy,
    KeyMap: FnMut(StorageKey) -> T,
    T: Future<Output = Result<Item, RpcError>> + Send + 'static,
    Item: CompositeKeyTrait<KeyValue::Key, Hasher> + 'static + Send,
    <Item as CompositeKeyTrait<KeyValue::Key, Hasher>>::KeyType: 'static + Send,
{
    // let keys = match state.storage_keys_paged(Some(prefix_key), count, start_key, at).wait() {
    //     Ok(k) => k,
    //     Err(e) =>
    //         return Box::new(future::err(to_rpc_error(
    //             Error::ScRpcApiError,
    //             Some(format!("{:?}", e)),
    //         ))),
    // };
    // if keys.is_empty() {
    //     return Box::new(future::ok(vec![]))
    // }

    // let key_futures: Vec<_> = keys.into_iter().map(key_map).collect();

    // StorageMap::<Hasher>::get_list_by_keys::<KeyValue, _>(key_futures)
    todo!()
}

/// The function gets list of keys from the first map (i.e. index) and
/// then retrieves the data from the second map (storage itself).
///
/// Hashing type of the second key in the index has to be the same
/// used for the first key in the second map.
///
/// The index map has to be StorageDoubleMap.
pub fn get_list_by_index<IndexKeyHasher, Hasher, State, BlockHash, Key, KeyValue>(
    state: &State,
    at: Option<BlockHash>,
    pallet: &[u8],
    index: &[u8],
    storage: &[u8],
    count: u32,
    key: &Key,
    start_key: Option<KeyValue>,
) -> BoxFutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
where
    State: sc_rpc_api::state::StateApi<BlockHash>,
    BlockHash: Copy,
    Key: Encode,
    KeyValue: KeyValueInfo,
    IndexKeyHasher: StorageHasher + ReversibleStorageHasher,
    Hasher: StorageHasher + ReversibleStorageHasher,
{
    // let key_encoded = key.encode();
    // let key_encoded_size = key_encoded.len();

    // let map = |k: StorageKey| {
    //     // below we retrieve key in the other map from the index map key
    //     let no_prefix = IndexKeyHasher::reverse(&k.0[32..]);
    //     let key_hashed =
    //         HashedKeyRef::<'_, Hasher>::unsafe_from_hashed(&no_prefix[key_encoded_size..]);

    //     let key = chain_key_hash_map(&prefix(pallet, storage), &key_hashed);

    //     state
    //         .storage(key.clone(), at)
    //         .map(|v| (v, key))
    //         .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
    // };

    // let prefix = prefix(pallet, index);
    // let key = HashedKey::<IndexKeyHasher>::unsafe_from_encoded(&key_encoded);
    // let start_key = start_key
    //     .map(|id| chain_key_hash_double_map(&prefix, &key, &HashedKey::<Hasher>::new(&id.key())));

    // get_list_by_keys::<KeyValue, Hasher, _, _, _, _>(
    //     state,
    //     at,
    //     chain_key_hash_map(&prefix, &key),
    //     count,
    //     start_key,
    //     map,
    // )
    todo!()
}

pub struct StorageMap<Hasher>(std::marker::PhantomData<Hasher>);

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResult<Key, Value> {
    pub key: KeyWrapper<Key>,
    pub value: Value,
}

pub trait KeyValueInfo {
    type Key: 'static + Encode + Decode + Send;
    type KeyError: GetError;
    type Value: 'static + Decode + Send;
    type ValueError: GetError;

    fn key(&self) -> &Self::Key;
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(transparent)]
pub struct KeyWrapper<Key> {
    pub key: Key,
}

impl<Key> From<Key> for KeyWrapper<Key> {
    fn from(key: Key) -> Self {
        Self { key }
    }
}

impl<Hasher: StorageHasher + ReversibleStorageHasher> StorageMap<Hasher> {
    pub fn get_value<R, State, BlockHash, Key>(
        state: &State,
        at: Option<BlockHash>,
        pallet: &[u8],
        map: &[u8],
        key: &Key,
    ) -> BoxFutureResult<Option<R>>
    where
        R: 'static + Decode + GetError + Send,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        Key: Encode,
        BlockHash: Copy,
    {
        get_value(state, key_hash_map::<_, Hasher>(pallet, map, key), at)
    }

    pub fn get_list<KeyValue, State, BlockHash>(
        state: &State,
        at: Option<BlockHash>,
        pallet: &[u8],
        map: &[u8],
        count: u32,
        start_id: Option<KeyValue>,
    ) -> BoxFutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
    where
        KeyValue: KeyValueInfo,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        BlockHash: Copy,
    {
        // let prefix = prefix(pallet, map);
        // let start_key =
        //     start_id.map(|id| chain_key_hash_map(&prefix, &HashedKey::<Hasher>::new(id.key())));

        // let map = |k: StorageKey| {
        //     state
        //         .storage(k.clone(), at)
        //         .map(|v| (v, k))
        //         .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
        // };

        // get_list_by_keys::<KeyValue, Hasher, _, _, _, _>(
        //     state,
        //     at,
        //     StorageKey(prefix),
        //     count,
        //     start_key,
        //     map,
        // )
        todo!()
    }

    pub fn get_list_by_keys<KeyValue, T, Item>(
        keys: Vec<T>,
    ) -> BoxFutureResult<
        Vec<
            ListResult<
                <Item as CompositeKeyTrait<KeyValue::Key, Hasher>>::KeyType,
                KeyValue::Value,
            >,
        >,
    >
    where
        KeyValue: KeyValueInfo,
        T: Future<Output = Result<Item, RpcError>> + Send + 'static,
        Item: 'static + Send + CompositeKeyTrait<KeyValue::Key, Hasher>,
        <Item as CompositeKeyTrait<KeyValue::Key, Hasher>>::KeyType: 'static + Send,
    {
        // let result = Vec::with_capacity(keys.len());
        // Box::new(FuturesOrdered::from_iter(keys.into_iter()).fold(result, |mut result, kv| {
        //     let (value, composite_key) = kv.decompose();
        //     let data = match value {
        //         None => return future::err(to_rpc_error(Error::NoneForReturnedKey, None)),
        //         Some(d) => d,
        //     };

        //     let key = match composite_key {
        //         Err(data) =>
        //             return future::err(to_rpc_error(
        //                 KeyValue::KeyError::get_error(),
        //                 Some(format!("{:?}", &data)),
        //             )),
        //         Ok(k) => KeyWrapper::from(k),
        //     };

        //     match KeyValue::Value::decode(&mut &data.0[..]) {
        //         Err(_) => future::err(to_rpc_error(
        //             KeyValue::ValueError::get_error(),
        //             Some(format!("{:?}", data)),
        //         )),
        //         Ok(value) => {
        //             result.push(ListResult { key, value });
        //             future::ok(result)
        //         },
        //     }
        // }))
        todo!()
    }
}

pub struct StorageDoubleMap<HasherFirst, HasherSecond>(
    std::marker::PhantomData<(HasherFirst, HasherSecond)>,
);

impl<HasherFirst: StorageHasher, HasherSecond: StorageHasher>
    StorageDoubleMap<HasherFirst, HasherSecond>
{
    pub fn get_value<R, State, BlockHash, KeyFirst, KeySecond>(
        state: &State,
        at: Option<BlockHash>,
        pallet: &[u8],
        map: &[u8],
        key_first: &KeyFirst,
        key_second: &KeySecond,
    ) -> BoxFutureResult<Option<R>>
    where
        R: 'static + Decode + GetError + Send,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        KeyFirst: Encode,
        KeySecond: Encode,
        BlockHash: Copy,
    {
        get_value(
            state,
            key_hash_double_map::<_, _, HasherFirst, HasherSecond>(
                pallet, map, key_first, key_second,
            ),
            at,
        )
    }
}
