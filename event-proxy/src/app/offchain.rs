use crate::actor::*;

use crate::{config::OffchainConfig, events::BlockMetadata, RuntimeT};

use super::actor_io::ActorJack;

pub type LastKnownBlock = Option<BlockMetadata<RuntimeT>>;
pub type MaybeLastKnownBlock = Result<LastKnownBlock, ()>;

pub struct OffchainClient {
    mock: MaybeLastKnownBlock,
}
impl OffchainClient {
    pub fn mock(mock: MaybeLastKnownBlock) -> Self {
        Self { mock }
    }
    pub async fn get_last_known_block(&self) -> MaybeLastKnownBlock {
        self.mock
    }
}

pub struct OffchainActor {
    client: Option<OffchainClient>,
}
impl OffchainActor {
    pub fn new() -> Self {
        Self { client: None }
    }
}

pub enum OffchainActorInputData {
    SetClient(OffchainClient),
    GetLastKnownBlock,
    BuildClient { mock: MaybeLastKnownBlock },
}

pub type OffchainActorInput = ActorDirective<OffchainActorInputData>;

impl OffchainActorInput {
    pub fn set_client(client: OffchainClient) -> Self {
        Self::Input(OffchainActorInputData::SetClient(client))
    }

    pub fn get_last_known_block() -> Self {
        Self::Input(OffchainActorInputData::GetLastKnownBlock)
    }

    pub fn build_client(config: OffchainConfig<LastKnownBlock>) -> Self {
        Self::Input(OffchainActorInputData::BuildClient { mock: Ok(config.last_known_block) })
    }
}
pub enum OffchainActorOutput {
    NoClient,
    Output(OffchainActorOutputData),
}
pub enum OffchainActorOutputData {
    SetClient,
    GetLastKnownBlock(MaybeLastKnownBlock),
    BuildClient(OffchainClient),
}
pub type OffchainActorIO = ActorJack<OffchainActorInput, OffchainActorOutput>;

#[async_trait::async_trait]
impl Actor<OffchainActorInputData, OffchainActorInput, OffchainActorOutput, OffchainActorIO>
    for OffchainActor
{
    async fn on_input(&mut self, data: OffchainActorInputData) -> OffchainActorOutput {
        if let OffchainActorInputData::BuildClient { mock } = data {
            return OffchainActorOutput::Output(OffchainActorOutputData::BuildClient(
                OffchainClient::mock(mock),
            ))
        }
        if self.client.is_none() {
            return match data {
                OffchainActorInputData::SetClient(c) => {
                    let _ = self.client.replace(c);
                    OffchainActorOutput::Output(OffchainActorOutputData::SetClient)
                },
                OffchainActorInputData::BuildClient { .. } => {
                    unreachable!();
                },
                _ => OffchainActorOutput::NoClient,
            }
        }
        let client = self.client.as_mut().unwrap();
        let output = match data {
            OffchainActorInputData::SetClient(c) => {
                let _ = std::mem::replace(client, c);
                OffchainActorOutputData::SetClient
            },
            OffchainActorInputData::GetLastKnownBlock =>
                OffchainActorOutputData::GetLastKnownBlock(client.get_last_known_block().await),
            OffchainActorInputData::BuildClient { .. } => {
                unreachable!();
            },
        };
        OffchainActorOutput::Output(output)
    }
}
