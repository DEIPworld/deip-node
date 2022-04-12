use codec::Codec;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
pub use pallet_deip::api::DeipApi as DeipStorageRuntimeApi;
use pallet_deip::{investment_opportunity::*, *};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
    generic::BlockId,
    traits::{AtLeast32BitUnsigned, Block as BlockT},
};
use std::sync::Arc;

use common_rpc::{
    get_list_by_index, to_rpc_error, BoxFutureResult, Error, HashOf, ListResult, StorageMap,
};

use frame_support::{Blake2_128Concat, Identity, Twox64Concat};

mod types;

#[rpc]
pub trait DeipStorageApi<BlockHash, AccountId, Moment, AssetId, AssetBalance, Hash, TransactionCtx>
where
    AssetBalance: Clone + AtLeast32BitUnsigned,
    TransactionCtx: Default,
{
    #[rpc(name = "deip_getProjectList")]
    fn get_project_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectId, Project<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProject")]
    fn get_project(
        &self,
        at: Option<BlockHash>,
        project_id: ProjectId,
    ) -> Result<Option<Project<Hash, AccountId>>>;

    #[rpc(name = "deip_getProjectListByTeam")]
    fn get_project_list_by_team(
        &self,
        at: Option<BlockHash>,
        team_id: AccountId,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectId, Project<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProjectContentList")]
    fn get_project_content_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProjectContentListByProject")]
    fn get_project_content_list_by_project(
        &self,
        at: Option<BlockHash>,
        project_id: ProjectId,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>>;

    #[rpc(name = "deip_getProjectContent")]
    fn get_project_content(
        &self,
        at: Option<BlockHash>,
        id: ProjectContentId,
    ) -> Result<Option<ProjectContent<Hash, AccountId>>>;

    #[rpc(name = "deip_getDomainList")]
    fn get_domains(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> BoxFutureResult<Vec<ListResult<DomainId, Domain>>>;

    #[rpc(name = "deip_getDomain")]
    fn get_domain(&self, at: Option<BlockHash>, domain_id: DomainId) -> Result<Option<Domain>>;

    #[rpc(name = "deip_getNdaList")]
    fn get_nda_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<NdaId>,
    ) -> BoxFutureResult<Vec<ListResult<NdaId, Nda<Hash, AccountId, Moment>>>>;

    #[rpc(name = "deip_getNda")]
    fn get_nda(
        &self,
        at: Option<BlockHash>,
        nda_id: NdaId,
    ) -> Result<Option<Nda<Hash, AccountId, Moment>>>;

    #[rpc(name = "deip_getReviewList")]
    fn get_review_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>>;

    #[rpc(name = "deip_getReviewListByProject")]
    fn get_review_list_by_project(
        &self,
        at: Option<BlockHash>,
        project_id: ProjectId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>>;

    #[rpc(name = "deip_getReviewListByProjectContent")]
    fn get_review_list_by_project_content(
        &self,
        at: Option<BlockHash>,
        key: ProjectContentId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>>;

    #[rpc(name = "deip_getReviewListByReviewer")]
    fn get_review_list_by_reviewer(
        &self,
        at: Option<BlockHash>,
        key: AccountId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>>;

    #[rpc(name = "deip_getReview")]
    fn get_review(
        &self,
        at: Option<BlockHash>,
        review_id: ReviewId,
    ) -> Result<Option<Review<Hash, AccountId>>>;

    #[rpc(name = "deip_getContractAgreement")]
    fn get_contract_agreement(
        &self,
        at: Option<BlockHash>,
        id: ContractAgreementId,
    ) -> Result<Option<contract::Agreement<AccountId, Hash, Moment>>>;

    #[rpc(name = "deip_getContractAgreementList")]
    fn get_contract_agreement_list(
        &self,
        at: Option<BlockHash>,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> BoxFutureResult<
        Vec<ListResult<ContractAgreementId, contract::Agreement<AccountId, Hash, Moment>>>,
    >;

    #[rpc(name = "deip_getContractAgreementListByType")]
    fn get_contract_agreement_list_by_type(
        &self,
        at: Option<BlockHash>,
        key: ContractAgreementIndexTerms,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> BoxFutureResult<
        Vec<ListResult<ContractAgreementId, contract::Agreement<AccountId, Hash, Moment>>>,
    >;

    #[rpc(name = "deip_getReviewUpvoteListByReview")]
    fn get_review_upvote_list_by_review(
        &self,
        at: Option<BlockHash>,
        key: ReviewId,
        count: u32,
        start_id: Option<(ReviewId, AccountId, DomainId)>,
    ) -> BoxFutureResult<
        Vec<ListResult<(ReviewId, AccountId, DomainId), DeipReviewVote<AccountId, Moment>>>,
    >;

    #[rpc(name = "deip_getReviewUpvoteListByUpvoter")]
    fn get_review_upvote_list_by_upvoter(
        &self,
        at: Option<BlockHash>,
        key: AccountId,
        count: u32,
        start_id: Option<(ReviewId, AccountId, DomainId)>,
    ) -> BoxFutureResult<
        Vec<ListResult<(ReviewId, AccountId, DomainId), DeipReviewVote<AccountId, Moment>>>,
    >;
}

/// A struct that implements the `DeipStorage`.
pub struct DeipStorage<C, State, M> {
    // If you have more generics, no need to DeipStorage<C, M, N, P, ...>
    // just use a tuple like DeipStorage<C, (M, N, P, ...)>
    client: Arc<C>,
    state: State,
    _marker: std::marker::PhantomData<M>,
}

impl<C, State, M> DeipStorage<C, State, M> {
    /// Create new `DeipStorage` instance with the given reference to the client.
    pub fn new(client: Arc<C>, state: State) -> Self {
        Self { client, state, _marker: Default::default() }
    }
}

impl<C, State, Block, AccountId, Moment, AssetId, AssetBalance, Hash, TransactionCtx>
    DeipStorageApi<HashOf<Block>, AccountId, Moment, AssetId, AssetBalance, Hash, TransactionCtx>
    for DeipStorage<C, State, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: DeipStorageRuntimeApi<
        Block,
        AccountId,
        Moment,
        AssetId,
        AssetBalance,
        Hash,
        TransactionCtx,
    >,
    State: sc_rpc_api::state::StateApi<HashOf<Block>>,
    AccountId: 'static + Codec + Send,
    Moment: 'static + Codec + Send,
    AssetId: 'static + Codec + Send,
    AssetBalance: 'static + Codec + Send + Clone + AtLeast32BitUnsigned,
    Hash: 'static + Codec + Send,
    TransactionCtx: 'static + Codec + Send + Default,
{
    fn get_project_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectId, Project<Hash, AccountId>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ProjectMap",
            count,
            start_id.map(types::ProjectKeyValue::new),
        )
    }

    fn get_project(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        project_id: ProjectId,
    ) -> Result<Option<Project<Hash, AccountId>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_project(&at, &project_id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::ProjectApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_project_list_by_team(
        &self,
        at: Option<HashOf<Block>>,
        key: AccountId,
        count: u32,
        start_id: Option<ProjectId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectId, Project<Hash, AccountId>>>> {
        get_list_by_index::<Blake2_128Concat, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ProjectIdByTeamId",
            b"ProjectMap",
            count,
            &key,
            start_id.map(types::ProjectKeyValue::new),
        )
    }

    fn get_domains(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<DomainId>,
    ) -> BoxFutureResult<Vec<ListResult<DomainId, Domain>>> {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"Deip",
            b"Domains",
            count,
            start_id.map(types::DomainKeyValue::new),
        )
    }

    fn get_domain(
        &self,
        at: Option<<Block as BlockT>::Hash>,
        domain_id: DomainId,
    ) -> Result<Option<Domain>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_domain(&at, &domain_id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::DomainApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_project_content_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ProjectContentMap",
            count,
            start_id.map(types::ProjectContentKeyValue::new),
        )
    }

    fn get_project_content_list_by_project(
        &self,
        at: Option<HashOf<Block>>,
        key: ProjectId,
        count: u32,
        start_id: Option<ProjectContentId>,
    ) -> BoxFutureResult<Vec<ListResult<ProjectContentId, ProjectContent<Hash, AccountId>>>> {
        get_list_by_index::<Identity, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ContentIdByProjectId",
            b"ProjectContentMap",
            count,
            &key,
            start_id.map(types::ProjectContentKeyValue::new),
        )
    }

    fn get_project_content(
        &self,
        at: Option<HashOf<Block>>,
        id: ProjectContentId,
    ) -> Result<Option<ProjectContent<Hash, AccountId>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_project_content(&at, &id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::ProjectContentApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_nda_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<NdaId>,
    ) -> BoxFutureResult<Vec<ListResult<NdaId, Nda<Hash, AccountId, Moment>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"NdaMap",
            count,
            start_id.map(types::NdaKeyValue::new),
        )
    }

    fn get_nda(
        &self,
        at: Option<HashOf<Block>>,
        nda_id: NdaId,
    ) -> Result<Option<Nda<Hash, AccountId, Moment>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_nda(&at, &nda_id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::NdaApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_review_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>> {
        StorageMap::<Identity>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ReviewMap",
            count,
            start_id.map(types::ReviewKeyValue::new),
        )
    }

    fn get_review_list_by_project(
        &self,
        at: Option<HashOf<Block>>,
        key: ProjectId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>> {
        get_list_by_index::<Identity, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ReviewIdByProjectId",
            b"ReviewMap",
            count,
            &key,
            start_id.map(types::ReviewKeyValue::new),
        )
    }

    fn get_review_list_by_project_content(
        &self,
        at: Option<HashOf<Block>>,
        key: ProjectContentId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>> {
        get_list_by_index::<Identity, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ReviewIdByContentId",
            b"ReviewMap",
            count,
            &key,
            start_id.map(types::ReviewKeyValue::new),
        )
    }

    fn get_review_list_by_reviewer(
        &self,
        at: Option<HashOf<Block>>,
        key: AccountId,
        count: u32,
        start_id: Option<ReviewId>,
    ) -> BoxFutureResult<Vec<ListResult<ReviewId, Review<Hash, AccountId>>>> {
        get_list_by_index::<Blake2_128Concat, Identity, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ReviewIdByAccountId",
            b"ReviewMap",
            count,
            &key,
            start_id.map(types::ReviewKeyValue::new),
        )
    }

    fn get_review(
        &self,
        at: Option<HashOf<Block>>,
        id: ReviewId,
    ) -> Result<Option<Review<Hash, AccountId>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_review(&at, &id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::ReviewApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_contract_agreement(
        &self,
        at: Option<HashOf<Block>>,
        id: ContractAgreementId,
    ) -> Result<Option<contract::Agreement<AccountId, Hash, Moment>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_contract_agreement(&at, &id);
        runtime_api_result
            .map_err(|e| to_rpc_error(Error::AgreementApiGetFailed, Some(format!("{:?}", e))))
    }

    fn get_contract_agreement_list(
        &self,
        at: Option<HashOf<Block>>,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> BoxFutureResult<
        Vec<ListResult<ContractAgreementId, contract::Agreement<AccountId, Hash, Moment>>>,
    > {
        StorageMap::<Blake2_128Concat>::get_list(
            &self.state,
            at,
            b"Deip",
            b"ContractAgreementMap",
            count,
            start_id.map(types::AgreementKeyValue::new),
        )
    }

    fn get_contract_agreement_list_by_type(
        &self,
        at: Option<HashOf<Block>>,
        key: ContractAgreementIndexTerms,
        count: u32,
        start_id: Option<ContractAgreementId>,
    ) -> BoxFutureResult<
        Vec<ListResult<ContractAgreementId, contract::Agreement<AccountId, Hash, Moment>>>,
    > {
        get_list_by_index::<Twox64Concat, Blake2_128Concat, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"ContractAgreementIdByType",
            b"ContractAgreementMap",
            count,
            &key,
            start_id.map(types::AgreementKeyValue::new),
        )
    }

    fn get_review_upvote_list_by_review(
        &self,
        at: Option<HashOf<Block>>,
        key: ReviewId,
        count: u32,
        start_id: Option<(ReviewId, AccountId, DomainId)>,
    ) -> BoxFutureResult<
        Vec<ListResult<(ReviewId, AccountId, DomainId), DeipReviewVote<AccountId, Moment>>>,
    > {
        get_list_by_index::<Identity, Blake2_128Concat, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"VoteIdByReviewId",
            b"ReviewVoteMap",
            count,
            &key,
            start_id.map(types::UpvoteKeyValue::new),
        )
    }

    fn get_review_upvote_list_by_upvoter(
        &self,
        at: Option<HashOf<Block>>,
        key: AccountId,
        count: u32,
        start_id: Option<(ReviewId, AccountId, DomainId)>,
    ) -> BoxFutureResult<
        Vec<ListResult<(ReviewId, AccountId, DomainId), DeipReviewVote<AccountId, Moment>>>,
    > {
        get_list_by_index::<Blake2_128Concat, Blake2_128Concat, _, _, _, _>(
            &self.state,
            at,
            b"Deip",
            b"VoteIdByAccountId",
            b"ReviewVoteMap",
            count,
            &key,
            start_id.map(types::UpvoteKeyValue::new),
        )
    }
}
