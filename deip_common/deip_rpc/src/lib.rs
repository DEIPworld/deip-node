use jsonrpc_core::{
    futures::future::{self, Future},
    futures::{self, stream, Stream},
};

pub use sp_core::{hashing::twox_128_into, storage::StorageData, storage::StorageKey};

use frame_support::{ReversibleStorageHasher, StorageHasher};

use codec::{Decode, Encode};

use std::convert::TryFrom;

pub mod error;
pub use error::*;

pub type HashOf<T> = <T as sp_runtime::traits::Block>::Hash;
pub type FutureResult<T> = Box<dyn Future<Item = T, Error = RpcError> + Send>;

pub fn prefix(pallet: &[u8], storage: &[u8]) -> Vec<u8> {
    let mut prefix = Vec::new();
    prefix.resize(32, 0u8);

    twox_128_into(
        pallet,
        <&mut [u8; 16]>::try_from(&mut prefix[..16]).unwrap(),
    );
    twox_128_into(
        storage,
        <&mut [u8; 16]>::try_from(&mut prefix[16..]).unwrap(),
    );

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
        return self.0.as_ref();
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

fn get_value<R, State, Hash>(
    state: &State,
    key: StorageKey,
    at: Option<Hash>,
) -> FutureResult<Option<R>>
where
    R: 'static + Decode + GetError + Send,
    State: sc_rpc_api::state::StateApi<Hash>,
    Hash: Copy,
{
    Box::new(
        state
            .storage(key, at)
            .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
            .and_then(|d| match d {
                None => future::ok(None),
                Some(data) => match R::decode(&mut &data.0[..]) {
                    Err(_) => {
                        future::err(to_rpc_error(R::get_error(), Some(format!("{:?}", data))))
                    }
                    Ok(decoded) => future::ok(Some(decoded)),
                },
            }),
    )
}

pub fn get_list_by_keys<KeyValue, Hasher, State, BlockHash, KeyMap, T>(
    state: &State,
    at: Option<BlockHash>,
    prefix_key: StorageKey,
    count: u32,
    start_key: Option<StorageKey>,
    key_map: KeyMap,
) -> FutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
where
    KeyValue: KeyValueInfo,
    Hasher: StorageHasher + ReversibleStorageHasher,
    State: sc_rpc_api::state::StateApi<BlockHash>,
    BlockHash: Copy,
    KeyMap: FnMut(StorageKey) -> T,
    T: futures::future::IntoFuture<Item = (StorageKey, Option<StorageData>), Error = RpcError>,
    T::Future: 'static + Send,
{
    let keys = match state
        .storage_keys_paged(Some(prefix_key), count, start_key, at)
        .wait()
    {
        Ok(k) => k,
        Err(e) => {
            return Box::new(future::err(to_rpc_error(
                Error::ScRpcApiError,
                Some(format!("{:?}", e)),
            )))
        }
    };
    if keys.is_empty() {
        return Box::new(future::ok(vec![]));
    }

    let key_futures: Vec<_> = keys.into_iter().map(key_map).collect();

    StorageMap::<Hasher>::get_list_by_keys::<KeyValue, _>(key_futures)
}

/// The function gets list of keys from the first map (i.e. index) and
/// then retrieves the data from the second map (storage itself).
///
/// Hashing type of the second key in the index has to be the same
/// used for the first key in the second map.
///
/// The index map has to be StorageDoubleMap.
///
pub fn get_list_by_index<IndexKeyHasher, Hasher, State, BlockHash, Key, KeyValue>(
    state: &State,
    at: Option<BlockHash>,
    pallet: &[u8],
    index: &[u8],
    storage: &[u8],
    count: u32,
    key: &Key,
    start_key: Option<KeyValue>,
) -> FutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
where
    State: sc_rpc_api::state::StateApi<BlockHash>,
    BlockHash: Copy,
    Key: Encode,
    KeyValue: KeyValueInfo,
    IndexKeyHasher: StorageHasher + ReversibleStorageHasher,
    Hasher: StorageHasher + ReversibleStorageHasher,
{
    let key_encoded = key.encode();
    let key_encoded_size = key_encoded.len();

    let map = |k: StorageKey| {
        // below we retrieve key in the other map from the index map key
        let no_prefix = IndexKeyHasher::reverse(&k.0[32..]);
        let key_hashed =
            HashedKeyRef::<'_, Hasher>::unsafe_from_hashed(&no_prefix[key_encoded_size..]);

        let key = chain_key_hash_map(&prefix(pallet, storage), &key_hashed);

        state
            .storage(key.clone(), at)
            .map(|v| (key, v))
            .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
    };

    let prefix = prefix(pallet, index);
    let key = HashedKey::<IndexKeyHasher>::unsafe_from_encoded(&key_encoded);
    let start_key = start_key
        .map(|id| chain_key_hash_double_map(&prefix, &key, &HashedKey::<Hasher>::new(&id.key())));

    get_list_by_keys::<KeyValue, Hasher, _, _, _, _>(
        state,
        at,
        chain_key_hash_map(&prefix, &key),
        count,
        start_key,
        map,
    )
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
    ) -> FutureResult<Option<R>>
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
    ) -> FutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
    where
        KeyValue: KeyValueInfo,
        State: sc_rpc_api::state::StateApi<BlockHash>,
        BlockHash: Copy,
    {
        let prefix = prefix(pallet, map);
        let start_key =
            start_id.map(|id| chain_key_hash_map(&prefix, &HashedKey::<Hasher>::new(id.key())));

        let map = |k: StorageKey| {
            state
                .storage(k.clone(), at)
                .map(|v| (k, v))
                .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
        };

        get_list_by_keys::<KeyValue, Hasher, _, _, _, _>(
            state,
            at,
            StorageKey(prefix),
            count,
            start_key,
            map,
        )
    }

    pub fn get_list_by_keys<KeyValue, T>(
        keys: Vec<T>,
    ) -> FutureResult<Vec<ListResult<KeyValue::Key, KeyValue::Value>>>
    where
        KeyValue: KeyValueInfo,
        T: futures::future::IntoFuture<Item = (StorageKey, Option<StorageData>), Error = RpcError>,
        T::Future: 'static + Send,
    {
        let result = Vec::with_capacity(keys.len());
        Box::new(
            stream::futures_ordered(keys.into_iter()).fold(result, |mut result, kv| {
                let (key, value) = kv;
                let data = match value {
                    None => return future::err(to_rpc_error(Error::NoneForReturnedKey, None)),
                    Some(d) => d,
                };

                let no_prefix = Hasher::reverse(&key.0[32..]);
                let key = match KeyValue::Key::decode(&mut &no_prefix[..]) {
                    Err(_) => {
                        return future::err(to_rpc_error(
                            KeyValue::KeyError::get_error(),
                            Some(format!("{:?}", &key.0)),
                        ))
                    }
                    Ok(k) => KeyWrapper::from(k),
                };

                match KeyValue::Value::decode(&mut &data.0[..]) {
                    Err(_) => future::err(to_rpc_error(
                        KeyValue::ValueError::get_error(),
                        Some(format!("{:?}", data)),
                    )),
                    Ok(value) => {
                        result.push(ListResult { key, value });
                        future::ok(result)
                    }
                }
            }),
        )
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
    ) -> FutureResult<Option<R>>
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
