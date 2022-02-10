use std::convert::Infallible;

use frame_system::Config;

use sp_core::{
    hashing::{self, twox_128},
    storage::StorageKey,
};

use subxt::{
    rpc::{RpcError, Subscription},
    BasicError, Client, ClientBuilder, EventsDecoder, GenericError, Phase, RawEvent,
};
use tokio::sync::mpsc;

use crate::{
    actor::{Actor, ActorDirective},
    config::BlockchainConfig,
    events::{known_domain_events, BlockMetadata, InfrastructureEvent, SpecializedEvent},
    RuntimeT,
};

use super::{actor_io::ActorJack, LastKnownBlock};

pub struct BlockchainActor {
    client: Option<Client<RuntimeT>>,
}

impl BlockchainActor {
    pub fn new() -> Self {
        Self { client: None }
    }
}

pub type BlocksReplay = (
    tokio::task::JoinHandle<()>,
    mpsc::Receiver<<RuntimeT as Config>::Header>,
    SubscriptionBuffer,
    EventsBuffer,
);
pub type MaybeBlockEvent = Result<SpecializedEvent<RuntimeT>, codec::Error>;
type Error = Box<dyn std::error::Error + Send>;
pub type BlockEvents = Result<Option<Vec<MaybeBlockEvent>>, BasicError>;

pub type SubscriptionBuffer = crate::Buffer<<RuntimeT as Config>::Header>;
pub type SubscriptionBufferIn = crate::BufferIn<<RuntimeT as Config>::Header>;

pub type EventsBuffer = crate::Buffer<MaybeBlockEvent>;

pub type FinalizedBlocksSubscription = Subscription<<RuntimeT as Config>::Header>;
pub type FinalizedBlocksSubscriptionItem = Option<Result<<RuntimeT as Config>::Header, RpcError>>;

pub enum BlockchainActorInputData {
    BuildClient(BlockchainConfig),
    SubscribeFinalizedBlocks(LastKnownBlock),
    SetClient(Client<RuntimeT>),
    GetBlockEvents(<RuntimeT as Config>::Hash, SubscriptionBuffer, EventsBuffer),
    ReplayBlocks(LastKnownBlock, <RuntimeT as Config>::Hash, SubscriptionBuffer, EventsBuffer),
    GetReplayedBlockEvents(<RuntimeT as Config>::Hash, BlocksReplay),
}

pub type BlockchainActorInput = ActorDirective<BlockchainActorInputData>;

impl BlockchainActorInput {
    pub fn build_client(config: BlockchainConfig) -> Self {
        Self::Input(BlockchainActorInputData::BuildClient(config))
    }

    pub fn subscribe_finalized_blocks(last_known_block: LastKnownBlock) -> Self {
        Self::Input(BlockchainActorInputData::SubscribeFinalizedBlocks(last_known_block))
    }

    pub fn set_client(client: Client<RuntimeT>) -> Self {
        Self::Input(BlockchainActorInputData::SetClient(client))
    }

    pub fn get_block_events(
        hash: <RuntimeT as Config>::Hash,
        subscription_buffer: SubscriptionBuffer,
        events_buffer: EventsBuffer,
    ) -> Self {
        Self::Input(BlockchainActorInputData::GetBlockEvents(
            hash,
            subscription_buffer,
            events_buffer,
        ))
    }

    pub fn replay_blocks(
        last_known_block: LastKnownBlock,
        head_block: <RuntimeT as Config>::Hash,
        subscription_buffer: SubscriptionBuffer,
        events_buffer: EventsBuffer,
    ) -> Self {
        let replay_blocks = BlockchainActorInputData::ReplayBlocks(
            last_known_block,
            head_block,
            subscription_buffer,
            events_buffer,
        );
        Self::Input(replay_blocks)
    }

    pub fn get_replayed_block_events(
        hash: <RuntimeT as Config>::Hash,
        replay: BlocksReplay,
    ) -> Self {
        Self::Input(BlockchainActorInputData::GetReplayedBlockEvents(hash, replay))
    }
}

pub enum BlockchainActorOutput {
    NoClient(BlockchainActorInputData),
    Ok(BlockchainActorOutputData),
}

pub enum BlockchainActorOutputData {
    BuildClient(Result<Client<RuntimeT>, BasicError>),
    SubscribeFinalizedBlocks(
        Result<FinalizedBlocksSubscription, BasicError>,
        LastKnownBlock,
        SubscriptionBuffer,
        EventsBuffer,
    ),
    SetClient,
    GetBlockEvents {
        maybe_events: BlockEvents,
        subscription_buffer: SubscriptionBuffer,
        events_buffer: EventsBuffer,
    },
    GetReplayedBlockEvents(BlockEvents, BlocksReplay),
    ReplayBlocks(BlocksReplay),
}

pub type BlockchainActorIO = ActorJack<BlockchainActorInput, BlockchainActorOutput>;

#[async_trait::async_trait]
impl Actor<BlockchainActorInputData, BlockchainActorInput, BlockchainActorOutput, BlockchainActorIO>
    for BlockchainActor
{
    async fn on_input(&mut self, data: BlockchainActorInputData) -> BlockchainActorOutput {
        //  If we receive the BuildClient directive we have only choose to build client:
        if let BlockchainActorInputData::BuildClient(ref conf) = data {
            // use crate::types::register_types;
            // let client = register_types(ClientBuilder::<RuntimeT>::new())
            let client = ClientBuilder::new().set_url(&conf.rpc).build().await;
            return BlockchainActorOutput::Ok(BlockchainActorOutputData::BuildClient(client))
        }

        // If client is not set we might only set client or raise an error:
        if self.client.is_none() {
            return if let BlockchainActorInputData::SetClient(c) = data {
                let _ = self.client.replace(c);
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient)
            } else {
                BlockchainActorOutput::NoClient(data)
            }
        }

        let client = self.client.as_mut().unwrap();

        let output = match data {
            BlockchainActorInputData::BuildClient(..) => {
                unreachable!();
            },
            BlockchainActorInputData::SubscribeFinalizedBlocks(last_known_block) =>
                BlockchainActorOutputData::SubscribeFinalizedBlocks(
                    client.rpc().subscribe_finalized_blocks().await,
                    last_known_block,
                    SubscriptionBuffer::new(),
                    EventsBuffer::new(),
                ),
            BlockchainActorInputData::SetClient(c) => {
                let _ = std::mem::replace(client, c);
                BlockchainActorOutputData::SetClient
            },
            BlockchainActorInputData::GetBlockEvents(hash, subscription_buffer, events_buffer) => {
                let block = match client.rpc().block(Some(hash)).await {
                    Ok(Some(block)) => block,
                    Ok(None) => {
                        // Block not found:
                        return BlockchainActorOutput::Ok(
                            BlockchainActorOutputData::GetBlockEvents {
                                maybe_events: Ok(None),
                                subscription_buffer,
                                events_buffer,
                            },
                        )
                    },
                    Err(e) =>
                        return BlockchainActorOutput::Ok(
                            BlockchainActorOutputData::GetBlockEvents {
                                maybe_events: Err(e),
                                subscription_buffer,
                                events_buffer,
                            },
                        ),
                };
                let portal_info = portal_info::fetch(client, &block.block.header).await.unwrap();

                let maybe_events =
                    get_block_events(client, client.events_decoder(), hash).await.map(|ok| {
                        let portal_info_lookup = portal_info::transpose(&portal_info);
                        // println!("$$$$ {:?}", portal_info_lookup);
                        let mut list: Vec<_> = ok
                            .into_iter()
                            .filter_map(|x| {
                                let portal_id =
                                    portal_info_lookup.get(&x.0).map(|x| **x).unwrap_or_default();
                                known_domain_events::<RuntimeT>(&x, &block.block, &portal_id)
                                    .transpose()
                            })
                            .collect();
                        list.push(Ok(InfrastructureEvent::block_created(
                            &block.block,
                            list.len() as u32,
                        )
                        .into()));
                        Some(list)
                    });
                BlockchainActorOutputData::GetBlockEvents {
                    maybe_events,
                    subscription_buffer,
                    events_buffer,
                }
            },
            BlockchainActorInputData::GetReplayedBlockEvents(hash, replay) => {
                let block = match client.rpc().block(Some(hash)).await {
                    Ok(Some(block)) => block,
                    Ok(None) => {
                        // Block not found:
                        return BlockchainActorOutput::Ok(
                            BlockchainActorOutputData::GetReplayedBlockEvents(Ok(None), replay),
                        )
                    },
                    Err(e) =>
                        return BlockchainActorOutput::Ok(
                            BlockchainActorOutputData::GetReplayedBlockEvents(Err(e), replay),
                        ),
                };
                let portal_info = portal_info::fetch(client, &block.block.header).await.unwrap();

                let events =
                    get_block_events(client, client.events_decoder(), hash).await.map(|ok| {
                        let portal_info_lookup = portal_info::transpose(&portal_info);
                        // println!("$$$$ {:?}", portal_info_lookup);
                        let mut list: Vec<_> = ok
                            .into_iter()
                            .filter_map(|x| {
                                let portal_id =
                                    portal_info_lookup.get(&x.0).map(|x| **x).unwrap_or_default();
                                known_domain_events::<RuntimeT>(&x, &block.block, &portal_id)
                                    .transpose()
                            })
                            .collect();
                        list.push(Ok(InfrastructureEvent::block_created(
                            &block.block,
                            list.len() as u32,
                        )
                        .into()));
                        Some(list)
                    });
                BlockchainActorOutputData::GetReplayedBlockEvents(events, replay)
            },
            BlockchainActorInputData::ReplayBlocks(
                last_known_block,
                head_block,
                subscription_buffer,
                events_buffer,
            ) => {
                let client2 = client.clone();
                let (tx, rx) = mpsc::channel(1);
                let replay_blocks_task = tokio::spawn(async move {
                    let client = client2;

                    let head = client.rpc().header(Some(head_block)).await.unwrap().unwrap();

                    let mut number = if let Some(BlockMetadata { number, hash, parent_hash }) =
                        last_known_block
                    {
                        let known_hash =
                            client.rpc().block_hash(Some(number.into())).await.unwrap().unwrap();
                        let known = client.rpc().header(Some(known_hash)).await.unwrap().unwrap();
                        if !(known.hash() == hash && known.parent_hash == parent_hash) {
                            unimplemented!();
                        }
                        if number > head.number {
                            unimplemented!();
                        }
                        number
                    } else {
                        0
                    };

                    while number != head.number {
                        let current_hash =
                            client.rpc().block_hash(Some(number.into())).await.unwrap().unwrap();
                        let current =
                            client.rpc().header(Some(current_hash)).await.unwrap().unwrap();
                        if tx.send(current).await.is_err() {
                            break
                        }
                        number += 1;
                    }
                });
                BlockchainActorOutputData::ReplayBlocks((
                    replay_blocks_task,
                    rx,
                    subscription_buffer,
                    events_buffer,
                ))
            },
        };
        BlockchainActorOutput::Ok(output)
    }
}

struct SystemEvents(StorageKey);
impl SystemEvents {
    pub(crate) fn new() -> Self {
        let mut storage_key = twox_128(b"System").to_vec();
        storage_key.extend(twox_128(b"Events").to_vec());
        log::debug!("Events storage key {:?}", hex::encode(&storage_key));
        Self(StorageKey(storage_key))
    }
}

impl From<SystemEvents> for StorageKey {
    fn from(key: SystemEvents) -> Self {
        key.0
    }
}

async fn get_block_events(
    client: &Client<RuntimeT>,
    decoder: &EventsDecoder<RuntimeT>,
    hash: <RuntimeT as Config>::Hash,
) -> Result<Vec<(u32, RawEvent)>, BasicError> {
    let change_set =
        client.rpc().query_storage_at(&[SystemEvents::new().into()], Some(hash)).await?;

    let mut events = Vec::new();

    for (_key, data) in change_set.into_iter().flat_map(|x| x.changes) {
        if let Some(data) = data {
            let raw_events = match decoder.decode_events(&mut &data.0[..]) {
                Ok(events) => events,
                Err(error) => return Err(error),
            };
            for (phase, raw) in raw_events {
                if let Phase::ApplyExtrinsic(i) = phase {
                    events.push((i, raw));
                }
            }
        }
    }
    Ok(events)
}

mod portal_info {
    use super::*;
    use std::{collections::HashMap, iter::FromIterator};

    fn twox_64_concat(x: &[u8]) -> Vec<u8> {
        let mut y = hashing::twox_64(x).to_vec();
        y.extend_from_slice(x);
        y
    }

    use crate::events::{ExtrinsicIndex, PortalId};

    pub type ExtrinsicIdList = Vec<ExtrinsicIndex>;
    pub type PortalInfo = Vec<(PortalId, ExtrinsicIdList)>;

    pub fn transpose(source: &PortalInfo) -> HashMap<&ExtrinsicIndex, &PortalId> {
        HashMap::from_iter(source.iter().flat_map(|(x, y)| y.iter().map(move |z| (z, x))))
    }

    pub async fn fetch(
        client: &Client<RuntimeT>,
        at: &<RuntimeT as Config>::Header,
    ) -> Result<PortalInfo, BasicError> {
        // let mut prefix = twox_128(b"DeipPortal").to_vec();
        // prefix.extend(twox_128(b"PortalTagOfTransaction").to_vec());
        // prefix.extend(at.number.using_encoded(twox_64_concat));
        // let prefix_len = prefix.len();
        // client
        //     .storage_pairs(StorageKey(prefix), Some(at.hash()))
        //     .await?
        //     .into_iter()
        //     .map(|(key, data)| -> Result<_, _> {
        //         Ok((
        //             PortalId::decode(&mut &key.0[prefix_len + 16..])?,
        //             ExtrinsicIdList::decode(&mut &data.0[..])?,
        //         ))
        //     })
        //     .collect()
        todo!()
    }
}
