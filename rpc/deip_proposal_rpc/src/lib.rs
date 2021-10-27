use jsonrpc_derive::rpc;

use std::vec::Vec;

use codec::Codec;

use sp_runtime::traits::Block as BlockT;

use pallet_deip_proposal::proposal::ProposalId;

use frame_support::Blake2_128Concat;

use common_rpc::{get_list_by_index, FutureResult, HashOf, ListResult, StorageMap};

mod types;

pub use types::Call;

#[rpc]
pub trait DeipProposalRpcApi<BlockHash, AccountId, Moment, CallT>
where
    AccountId: Ord,
{
    #[rpc(name = "deipProposal_get")]
    fn get(
        &self,
        at: Option<BlockHash>,
        id: ProposalId,
    ) -> FutureResult<Option<types::DeipProposal<AccountId, Moment, CallT>>>;

    #[rpc(name = "deipProposal_getList")]
    fn get_proposal_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ProposalId>,
    ) -> FutureResult<Vec<ListResult<ProposalId, types::DeipProposal<AccountId, Moment, CallT>>>>;

    #[rpc(name = "deipProposal_getListByCreator")]
    fn get_proposal_list_by_creator(
        &self,
        at: Option<BlockHash>,
        creator: AccountId,
        count: u32,
        start_id: Option<ProposalId>,
    ) -> FutureResult<Vec<ListResult<ProposalId, types::DeipProposal<AccountId, Moment, CallT>>>>;
}

pub struct DeipProposalRpcApiObj<State, Block> {
    state: State,
    _marker: std::marker::PhantomData<Block>,
}

impl<State, Block> DeipProposalRpcApiObj<State, Block> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            _marker: Default::default(),
        }
    }
}

impl<State, Block, AccountId, Moment, Call>
    DeipProposalRpcApi<HashOf<Block>, AccountId, Moment, Call>
    for DeipProposalRpcApiObj<State, Block>
where
    Block: BlockT,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    AccountId: 'static + Codec + Send + Ord,
    Moment: 'static + Codec + Send,
    Call: 'static + Codec + Send,
{
    fn get(
        &self,
        at: Option<HashOf<Block>>,
        id: ProposalId,
    ) -> FutureResult<Option<types::DeipProposal<AccountId, Moment, Call>>> {
        StorageMap::<Blake2_128Concat>::get_value(
            &self.state,
            at,
            b"DeipProposal",
            b"ProposalRepository",
            &id,
        )
    }

    fn get_proposal_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ProposalId>,
    ) -> FutureResult<Vec<ListResult<ProposalId, types::DeipProposal<AccountId, Moment, Call>>>>
    {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"DeipProposal",
            b"ProposalRepository",
            count,
            start_id.map(types::ProposalKeyValue::new),
        )
    }

    fn get_proposal_list_by_creator(
        &self,
        at: Option<HashOf<Block>>,
        key: AccountId,
        count: u32,
        start_id: Option<ProposalId>,
    ) -> FutureResult<Vec<ListResult<ProposalId, types::DeipProposal<AccountId, Moment, Call>>>>
    {
        get_list_by_index::<Blake2_128Concat, Blake2_128Concat, _, _, _, _>(
            &self.state,
            at,
            b"DeipProposal",
            b"ProposalIdByAccountId",
            b"ProposalRepository",
            count,
            &key,
            start_id.map(types::ProposalKeyValue::new),
        )
    }
}
