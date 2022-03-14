//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use appchain_deip_runtime::{
    opaque::Block, AccountId, AssetBalance, AssetExtra, AssetId, Balance, BlockNumber, DeipAssetId,
    Hash, Index, InstanceId, Moment, NftClassId, TransactionCtxId,
};
use sc_client_api::{
    AuxStore, BlockBackend, BlockchainEvents, ExecutorProvider, ProofProvider, StorageProvider,
};
use sc_consensus_babe::{Config, Epoch};
use sc_consensus_babe_rpc::BabeRpcHandler;
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
    FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{CallApiAt, Metadata, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::SyncCryptoStorePtr;

use beefy_gadget::notification::BeefySignedCommitmentStream;
use sp_runtime::traits::Block as BlockT;

use jsonrpc_pubsub::manager::SubscriptionManager;

/// Extra dependencies for BEEFY
pub struct BeefyDeps<B: BlockT> {
    /// Receives notifications about signed commitments from BEEFY.
    pub signed_commitment_stream: BeefySignedCommitmentStream<B>,
    /// Executor to drive the subscription manager in the BEEFY RPC handler.
    pub subscription_executor: SubscriptionTaskExecutor,
}

/// Extra dependencies for BABE.
pub struct BabeDeps {
    /// BABE protocol config.
    pub babe_config: Config,
    /// BABE pending epoch changes.
    pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
    /// The keystore that manages the keys of the node.
    pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
    /// Voting round info.
    pub shared_voter_state: SharedVoterState,
    /// Authority set info.
    pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
    /// Receives notifications about justification events from Grandpa.
    pub justification_stream: GrandpaJustificationStream<Block>,
    /// Executor to drive the subscription manager in the Grandpa RPC handler.
    pub subscription_executor: SubscriptionTaskExecutor,
    /// Finality proof provider.
    pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B, BT: BlockT> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// The SelectChain Strategy
    pub select_chain: SC,
    /// A copy of the chain spec.
    pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// BABE specific dependencies.
    pub babe: BabeDeps,
    /// GRANDPA specific dependencies.
    pub grandpa: GrandpaDeps<B>,
    /// BEEFY specific dependencies.
    pub beefy: BeefyDeps<BT>,
}

/// A IO handler that uses all Full RPC extensions.
pub type IoHandler = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B, BT>(
    deps: FullDeps<C, P, SC, B, BT>,
) -> Result<jsonrpc_core::IoHandler<sc_rpc_api::Metadata>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Sync
        + Send
        + 'static,
    C: ExecutorProvider<Block>,
    C: StorageProvider<Block, B>,
    C: ProofProvider<Block>,
    C: BlockchainEvents<Block>,
    C: CallApiAt<Block>,
    C: BlockBackend<Block>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_mmr_rpc::MmrRuntimeApi<Block, <Block as sp_runtime::traits::Block>::Hash>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BabeApi<Block>,
    C::Api: BlockBuilder<Block>,
    C::Api: Metadata<Block>,
    C::Api: deip_dao_rpc::DeipDaoRuntimeApi<Block, AccountId>,
    C::Api: deip_rpc::DeipStorageRuntimeApi<
        Block,
        AccountId,
        Moment,
        DeipAssetId,
        AssetBalance,
        Hash,
        TransactionCtxId,
    >,
    P: TransactionPool + 'static,
    SC: SelectChain<Block> + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
    BT: BlockT,
{
    use pallet_mmr_rpc::{Mmr, MmrApi};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps { client, pool, select_chain, chain_spec, deny_unsafe, babe, grandpa, beefy } =
        deps;

    let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;
    let GrandpaDeps {
        shared_voter_state,
        shared_authority_set,
        justification_stream,
        subscription_executor,
        finality_provider,
    } = grandpa;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));
    // Making synchronous calls in light client freezes the browser currently,
    // more context: https://github.com/paritytech/substrate/pull/3480
    // These RPCs should use an asynchronous caller instead.
    io.extend_with(MmrApi::to_delegate(Mmr::new(client.clone())));
    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));
    io.extend_with(sc_consensus_babe_rpc::BabeApi::to_delegate(BabeRpcHandler::new(
        client.clone(),
        shared_epoch_changes.clone(),
        keystore,
        babe_config,
        select_chain,
        deny_unsafe,
    )));
    io.extend_with(sc_finality_grandpa_rpc::GrandpaApi::to_delegate(GrandpaRpcHandler::new(
        shared_authority_set.clone(),
        shared_voter_state,
        justification_stream,
        subscription_executor.clone(),
        finality_provider,
    )));

    io.extend_with(sc_sync_state_rpc::SyncStateRpcApi::to_delegate(
        sc_sync_state_rpc::SyncStateRpcHandler::new(
            chain_spec,
            client.clone(),
            shared_authority_set,
            shared_epoch_changes,
            deny_unsafe,
        )?,
    ));

    io.extend_with(beefy_gadget_rpc::BeefyApi::to_delegate(
        beefy_gadget_rpc::BeefyRpcHandler::new(
            beefy.signed_commitment_stream,
            beefy.subscription_executor,
        ),
    ));

    let subscriptions = SubscriptionManager::new(Arc::new(subscription_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe, None);

    io.extend_with(deip_assets_rpc::DeipAssetsRpc::<
        <Block as BlockT>::Hash,
        AssetId,
        AssetBalance,
        AccountId,
        Balance,
        AssetExtra,
        DeipAssetId,
    >::to_delegate(deip_assets_rpc::DeipAssetsRpcObj::<
        sc_rpc::state::State<Block, C>,
        Block,
    >::new(state)));

    let subscriptions = SubscriptionManager::new(Arc::new(subscription_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe, None);

    io.extend_with(deip_uniques_rpc::DeipUniquesRpc::<
        <Block as BlockT>::Hash,
        NftClassId,
        InstanceId,
        AccountId,
        Balance,
        AssetExtra,
        DeipAssetId,
    >::to_delegate(deip_uniques_rpc::DeipUniquesRpcObj::<
        sc_rpc::state::State<Block, C>,
        Block,
    >::new(state)));

    let subscriptions = SubscriptionManager::new(Arc::new(subscription_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe, None);

    io.extend_with(deip_uniques_rpc::DeipUniquesRpc::<
        <Block as BlockT>::Hash,
        NftClassId,
        InstanceId,
        AccountId,
        Balance,
        AssetExtra,
        DeipAssetId,
    >::to_delegate(deip_uniques_rpc::DeipUniquesRpcObj::<
        sc_rpc::state::State<Block, C>,
        Block,
    >::new(state)));

    let subscriptions = SubscriptionManager::new(Arc::new(subscription_executor.clone()));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe, None);

    io.extend_with(deip_dao_rpc::DeipDaoRpcApi::to_delegate(deip_dao_rpc::DeipDaoRpcApiObj::new(
        client.clone(),
        state,
    )));

    let subscriptions = SubscriptionManager::new(Arc::new(subscription_executor));
    let (state, _) = sc_rpc::state::new_full(client.clone(), subscriptions, deny_unsafe, None);

    io.extend_with(deip_rpc::DeipStorageApi::to_delegate(deip_rpc::DeipStorage::new(
        client, state,
    )));

    Ok(io)
}

// @TODO light mod removed in later version of sc_client_api
// /// Instantiate all Light RPC extensions.
// pub fn create_light<C, P, M, F>(deps: LightDeps<C, F, P>) -> jsonrpc_core::IoHandler<M>
// where
//     C: sp_blockchain::HeaderBackend<Block>,
//     C: Send + Sync + 'static,
//     F: sc_client_api::light::Fetcher<Block> + 'static,
//     P: TransactionPool + 'static,
//     M: jsonrpc_core::Metadata + Default,
// {
//     use substrate_frame_rpc_system::SystemApi;

//     let LightDeps { client, pool, remote_blockchain, fetcher } = deps;
//     let mut io = jsonrpc_core::IoHandler::default();
//     io.extend_with(SystemApi::<Hash, AccountId, Index>::to_delegate(LightSystem::new(
//         client,
//         remote_blockchain,
//         fetcher,
//         pool,
//     )));

//     io
// }
