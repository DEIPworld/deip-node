use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;

use std::{sync::Arc, vec::Vec};

use codec::Codec;

use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;

pub use pallet_deip_dao::api::DeipDaoRuntimeApi;
use pallet_deip_dao::{
    api::{GetMultiResult, GetResult},
    dao::{Dao, DaoId},
};

use frame_support::Blake2_128Concat;

use common_rpc::{BoxFutureResult, HashOf, ListResult, StorageMap};

mod types;

#[rpc]
pub trait DeipDaoRpcApi<BlockHash, AccountId> {
    #[rpc(name = "deipDao_get")]
    fn get(&self, at: Option<BlockHash>, id: DaoId) -> RpcResult<GetResult<AccountId>>;

    #[rpc(name = "deipDao_getMulti")]
    fn get_multi(
        &self,
        at: Option<BlockHash>,
        ids: Vec<DaoId>,
    ) -> RpcResult<GetMultiResult<AccountId>>;

    #[rpc(name = "deipDao_getList")]
    fn list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<DaoId>,
    ) -> BoxFutureResult<Vec<ListResult<DaoId, Dao<AccountId, DaoId>>>>;
}

pub struct DeipDaoRpcApiObj<C, State, Block> {
    client: Arc<C>,
    state: State,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, State, Block> DeipDaoRpcApiObj<C, State, Block> {
    pub fn new(client: Arc<C>, state: State) -> Self {
        Self { client, state, _marker: Default::default() }
    }
}

impl<C, State, Block, AccountId> DeipDaoRpcApi<HashOf<Block>, AccountId>
    for DeipDaoRpcApiObj<C, State, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: DeipDaoRuntimeApi<Block, AccountId>,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    AccountId: 'static + Codec + std::marker::Send,
{
    fn get(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        name: DaoId,
    ) -> RpcResult<GetResult<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get(&at, name);
        runtime_api_result.map_err(|e| {
            common_rpc::to_rpc_error(common_rpc::Error::DaoApiGetFailed, Some(format!("{:?}", e)))
        })
    }

    fn get_multi(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        names: Vec<DaoId>,
    ) -> RpcResult<GetMultiResult<AccountId>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_multi(&at, names);
        runtime_api_result.map_err(|e| {
            common_rpc::to_rpc_error(
                common_rpc::Error::DaoApiGetMultiFailed,
                Some(format!("{:?}", e)),
            )
        })
    }

    fn list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<DaoId>,
    ) -> BoxFutureResult<Vec<ListResult<DaoId, Dao<AccountId, DaoId>>>> {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"DeipDao",
            b"DaoRepository",
            count,
            start_id.map(types::DaoKeyValue::new),
        )
    }
}
