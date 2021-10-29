use jsonrpc_core::{
    futures::future::{self, Future},
    futures::Stream,
};
use jsonrpc_derive::rpc;

use std::vec::Vec;

use codec::{Codec, Decode, Encode};

use sp_runtime::traits::{Block as BlockT, AtLeast32BitUnsigned};

use sp_core::storage::StorageKey;

use frame_support::{Blake2_128Concat, ReversibleStorageHasher, StorageHasher, Identity};

use common_rpc::{
    chain_key_hash_map, chain_key_hash_double_map, prefix, to_rpc_error, get_list_by_keys,
    Error, FutureResult, HashOf, HashedKey,
    ListResult, StorageDoubleMap, StorageMap, HashedKeyRef,
};

mod types;
use types::*;

#[rpc]
pub trait DeipAssetsRpc<BlockHash, AssetId, Balance, AccountId, DepositBalance, Extra, DeipAssetId>
where
    AssetId: Encode + Decode,
    DeipAssetId: Encode + Decode,
    Balance: Decode + AtLeast32BitUnsigned + Clone,
    AccountId: Decode,
    DepositBalance: Decode + AtLeast32BitUnsigned + Clone,
    Extra: Decode,
{
    #[rpc(name = "assets_getAsset")]
    fn get_asset(
        &self,
        at: Option<BlockHash>,
        id: DeipAssetId,
    ) -> FutureResult<Option<AssetDetails<Balance, AccountId, DepositBalance>>>;

    #[rpc(name = "assets_getAssetList")]
    fn get_asset_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<AssetId>,
    ) -> FutureResult<Vec<ListResult<AssetId, AssetDetails<Balance, AccountId, DepositBalance>>>>;

    #[rpc(name = "assets_getAssetBalanceList")]
    fn get_asset_balance_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<(AssetId, AccountId)>,
    ) -> FutureResult<Vec<AssetBalanceWithIds<AssetId, Balance, AccountId, Extra>>>;

    #[rpc(name = "assets_getAssetBalanceByOwner")]
    fn get_asset_balance_by_owner(
        &self,
        at: Option<BlockHash>,
        owner: AccountId,
        asset: AssetId,
    ) -> FutureResult<Option<AssetBalance<Balance, Extra>>>;

    #[rpc(name = "assets_getAssetBalanceListByAsset")]
    fn get_asset_balance_list_by_asset(
        &self,
        at: Option<BlockHash>,
        asset: AssetId,
        count: u32,
        start_id: Option<AccountId>,
    ) -> FutureResult<Vec<AssetBalanceWithOwner<Balance, AccountId, Extra>>>;
}

pub struct DeipAssetsRpcObj<State, B> {
    state: State,
    _marker: std::marker::PhantomData<B>,
}

impl<State, B> DeipAssetsRpcObj<State, B> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            _marker: Default::default(),
        }
    }
}

impl<State, Block, AssetId, Balance, AccountId, DepositBalance, Extra, DeipAssetId>
    DeipAssetsRpc<HashOf<Block>, AssetId, Balance, AccountId, DepositBalance, Extra, DeipAssetId>
    for DeipAssetsRpcObj<State, Block>
where
    AssetId: 'static + Codec + Send,
    DeipAssetId: 'static + Send + Codec + Clone,
    Balance: 'static + Decode + AtLeast32BitUnsigned + Clone + Send,
    AccountId: 'static + Codec + Send,
    DepositBalance: 'static + Send + Encode + Decode + AtLeast32BitUnsigned + Clone,
    Extra: 'static + Send + Decode,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    Block: BlockT,
{
    fn get_asset(
        &self,
        at: Option<HashOf<Block>>,
        id: DeipAssetId,
    ) -> FutureResult<Option<AssetDetails<Balance, AccountId, DepositBalance>>> {
        let key_encoded = id.encode();
        let key_encoded_size = key_encoded.len();

        let map = |k: StorageKey| {
            // below we retrieve key in the other map from the index map key
            let no_prefix = Identity::reverse(&k.0[32..]);
            let key_hashed =
                HashedKeyRef::<'_, Blake2_128Concat>::unsafe_from_hashed(&no_prefix[key_encoded_size..]);

            let key = chain_key_hash_map(&prefix(b"Assets", b"Asset"), &key_hashed);

            self.state
                .storage(key.clone(), at)
                .map(|v| (v, key))
                .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
        };

        let index_prefix = prefix(b"DeipAssets", b"AssetIdByDeipAssetId");
        let index_key = HashedKey::<Identity>::unsafe_from_encoded(&key_encoded);

        Box::new(get_list_by_keys::<types::AssetKeyValue<AssetId, Balance, AccountId, DepositBalance>, Identity, _, _, _, _>(
            &self.state,
            at,
            chain_key_hash_map(&index_prefix, &index_key),
            1,
            None,
            map,
        ).map(|mut result| {
            result.pop().map(|item| item.value)
        }))
    }

    fn get_asset_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<AssetId>,
    ) -> FutureResult<Vec<ListResult<AssetId, AssetDetails<Balance, AccountId, DepositBalance>>>>
    {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"Assets",
            b"Asset",
            count,
            start_id.map(types::AssetKeyValue::new),
        )
    }

    fn get_asset_balance_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<(AssetId, AccountId)>,
    ) -> FutureResult<Vec<AssetBalanceWithIds<AssetId, Balance, AccountId, Extra>>> {
        let prefix = prefix(b"Assets", b"Account");

        let start_key = start_id.map(|(first, second)| {
            chain_key_hash_double_map(
                &prefix,
                &HashedKey::<Blake2_128Concat>::new(&first),
                &HashedKey::<Blake2_128Concat>::new(&second),
            )
        });

        let state = &self.state;
        let keys = match state
            .storage_keys_paged(Some(StorageKey(prefix)), count, start_key, at)
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

        let key_futures: Vec<_> = keys
            .into_iter()
            .map(|k| {
                state
                    .storage(k.clone(), at)
                    .map(|v| (k, v))
                    .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
            })
            .collect();

        let result = Vec::with_capacity(key_futures.len());
        Box::new(
            jsonrpc_core::futures::stream::futures_ordered(key_futures.into_iter()).fold(
                result,
                |mut result, kv| {
                    let (key, value) = kv;
                    let data = match value {
                        None => return future::ok(result),
                        Some(d) => d,
                    };

                    let no_prefix = Blake2_128Concat::reverse(&key.0[32..]);
                    let asset = match AssetId::decode(&mut &no_prefix[..]) {
                        Err(_) => {
                            return future::err(to_rpc_error(
                                Error::AssetIdDecodeFailed,
                                Some(format!("{:?}", &key.0)),
                            ))
                        }
                        Ok(id) => id,
                    };

                    let no_prefix = Blake2_128Concat::reverse(&no_prefix[asset.encoded_size()..]);
                    let account = match AccountId::decode(&mut &no_prefix[..]) {
                        Err(_) => {
                            return future::err(to_rpc_error(
                                Error::AccountIdDecodeFailed,
                                Some(format!("{:?}", &key.0)),
                            ))
                        }
                        Ok(id) => id,
                    };

                    match AssetBalance::<Balance, Extra>::decode(&mut &data.0[..]) {
                        Err(_) => future::err(to_rpc_error(
                            Error::AssetBalanceDecodeFailed,
                            Some(format!("{:?}", data)),
                        )),
                        Ok(balance) => {
                            result.push(AssetBalanceWithIds {
                                asset,
                                account,
                                balance,
                            });
                            future::ok(result)
                        }
                    }
                },
            ),
        )
    }

    fn get_asset_balance_by_owner(
        &self,
        at: Option<HashOf<Block>>,
        owner: AccountId,
        asset: AssetId,
    ) -> FutureResult<Option<AssetBalance<Balance, Extra>>> {
        StorageDoubleMap::<Blake2_128Concat, Blake2_128Concat>::get_value(
            &self.state,
            at,
            b"Assets",
            b"Account",
            &asset,
            &owner,
        )
    }

    fn get_asset_balance_list_by_asset(
        &self,
        at: Option<HashOf<Block>>,
        asset: AssetId,
        count: u32,
        start_id: Option<AccountId>,
    ) -> FutureResult<Vec<AssetBalanceWithOwner<Balance, AccountId, Extra>>> {
        let prefix = prefix(b"Assets", b"Account");

        let asset_encoded = asset.encode();
        let asset_encoded_size = asset_encoded.len();
        let asset_hashed = Blake2_128Concat::hash(&asset_encoded);
        let start_key = start_id.map(|account_id| {
            StorageKey(
                prefix
                    .iter()
                    .chain(&asset_hashed)
                    .chain(&account_id.using_encoded(Blake2_128Concat::hash))
                    .map(|b| *b)
                    .collect(),
            )
        });

        let prefix = prefix.iter().chain(&asset_hashed).map(|b| *b).collect();

        let state = &self.state;
        let keys = match state
            .storage_keys_paged(Some(StorageKey(prefix)), count, start_key, at)
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

        let key_futures: Vec<_> = keys
            .into_iter()
            .map(|k| {
                state
                    .storage(k.clone(), at)
                    .map(|v| (k, v))
                    .map_err(|e| to_rpc_error(Error::ScRpcApiError, Some(format!("{:?}", e))))
            })
            .collect();

        let result = Vec::with_capacity(key_futures.len());
        Box::new(
            jsonrpc_core::futures::stream::futures_ordered(key_futures.into_iter()).fold(
                result,
                move |mut result, kv| {
                    let (key, value) = kv;
                    let data = match value {
                        None => return future::ok(result),
                        Some(d) => d,
                    };

                    let no_prefix = Blake2_128Concat::reverse(&key.0[32..]);
                    let no_prefix = Blake2_128Concat::reverse(&no_prefix[asset_encoded_size..]);
                    let account = match AccountId::decode(&mut &no_prefix[..]) {
                        Err(_) => {
                            return future::err(to_rpc_error(
                                Error::AccountIdDecodeFailed,
                                Some(format!("{:?}", &key.0)),
                            ))
                        }
                        Ok(id) => id,
                    };

                    match AssetBalance::<Balance, Extra>::decode(&mut &data.0[..]) {
                        Err(_) => future::err(to_rpc_error(
                            Error::AssetBalanceDecodeFailed,
                            Some(format!("{:?}", data)),
                        )),
                        Ok(balance) => {
                            result.push(AssetBalanceWithOwner { account, balance });
                            future::ok(result)
                        }
                    }
                },
            ),
        )
    }
}
