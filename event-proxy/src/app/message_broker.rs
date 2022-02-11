use super::{actor_io::*, BlocksReplay, EventsBuffer, SubscriptionBuffer};
use crate::{actor::*, config::MessageBrokerConfig};

use rdkafka::{
    producer::{future_producer::OwnedDeliveryResult, FutureProducer, FutureRecord},
    util::Timeout,
};

pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";

pub struct MessageBrokerActor {
    producer: Option<FutureProducer>,
}
impl MessageBrokerActor {
    pub fn new() -> Self {
        Self { producer: None }
    }
}

pub type MessageBrokerActorInput = ActorDirective<MessageBrokerActorInputData>;
impl MessageBrokerActorInput {
    pub fn configure(config: MessageBrokerConfig, ctx: MessageBrokerConfigureCtx) -> Self {
        MessageBrokerActorInputData::Configure(MessageBrokerConfigure { config, ctx }).into()
    }

    pub fn send_replayed_block_event(
        event: super::MaybeBlockEvent,
        remaining: usize,
        replay: super::BlocksReplay,
    ) -> Self {
        MessageBrokerActorInputData::SendReplayedBlockEvent(SendEvent {
            event,
            ctx: SendReplayedBlockEventCtx { remaining, replay },
        })
        .into()
    }

    pub fn send_block_event(
        event: super::MaybeBlockEvent,
        events_buffer: super::EventsBuffer,
        remaining: usize,
        subscription_buffer: super::SubscriptionBuffer,
    ) -> Self {
        MessageBrokerActorInputData::SendBlockEvent(SendEvent {
            event,
            ctx: SendBlockEventCtx { events_buffer, remaining, subscription_buffer },
        })
        .into()
    }
}

pub type Delivery = Result<OwnedDeliveryResult, codec::Error>;

pub struct SendEvent<Ctx> {
    pub event: super::MaybeBlockEvent,
    pub ctx: Ctx,
}

pub struct SendEventResult<Ctx> {
    pub delivery: Result<Delivery, serde_json::Error>,
    pub ctx: Ctx,
}

pub struct SendReplayedBlockEventCtx {
    pub remaining: usize,
    pub replay: BlocksReplay,
}

pub struct SendBlockEventCtx {
    pub events_buffer: EventsBuffer,
    pub remaining: usize,
    pub subscription_buffer: SubscriptionBuffer,
}

pub struct MessageBrokerConfigure<Ctx> {
    config: MessageBrokerConfig,
    ctx: Ctx,
}
pub struct MessageBrokerConfigureCtx {
    pub maybe_input: Box<Option<MessageBrokerActorInput>>,
}
impl Default for MessageBrokerConfigureCtx {
    fn default() -> Self {
        Self { maybe_input: Box::new(None) }
    }
}
impl From<MessageBrokerConfigureCtx> for MessageBrokerActorOutput {
    fn from(ctx: MessageBrokerConfigureCtx) -> Self {
        MessageBrokerActorOutput::NotConfigured(ctx)
    }
}
pub struct MessageBrokerConfigureResult<Ctx> {
    pub maybe_error: Option<rdkafka::error::KafkaError>,
    pub ctx: Ctx,
}

pub enum MessageBrokerActorInputData {
    Configure(MessageBrokerConfigure<MessageBrokerConfigureCtx>),
    SendReplayedBlockEvent(SendEvent<SendReplayedBlockEventCtx>),
    SendBlockEvent(SendEvent<SendBlockEventCtx>),
}
impl From<MessageBrokerActorInputData> for MessageBrokerActorInput {
    fn from(data: MessageBrokerActorInputData) -> Self {
        Self::Input(data)
    }
}

pub enum MessageBrokerActorOutput {
    NotConfigured(MessageBrokerConfigureCtx),
    Result(MessageBrokerActorOutputData),
}

pub enum MessageBrokerActorOutputData {
    Configure(MessageBrokerConfigureResult<MessageBrokerConfigureCtx>),
    SendReplayedBlockEvent(SendEventResult<SendReplayedBlockEventCtx>),
    SendBlockEvent(SendEventResult<SendBlockEventCtx>),
}

impl From<MessageBrokerActorOutputData> for MessageBrokerActorOutput {
    fn from(data: MessageBrokerActorOutputData) -> Self {
        Self::Result(data)
    }
}

pub type MessageBrokerActorIO = ActorJack<MessageBrokerActorInput, MessageBrokerActorOutput>;

fn configure(
    c: MessageBrokerConfigure<MessageBrokerConfigureCtx>,
) -> (Option<FutureProducer>, MessageBrokerActorOutput) {
    let MessageBrokerConfigure { config: c, ctx } = c;
    let mut config = rdkafka::ClientConfig::new();
    let key = "bootstrap.servers";
    config.set(key, &c.kafka_bootstrap_servers);
    info!("{}: {}", key, config.get(key).unwrap());

    match config.create::<FutureProducer>() {
        Ok(producer) => {
            let output = MessageBrokerActorOutput::Result(MessageBrokerActorOutputData::Configure(
                MessageBrokerConfigureResult { maybe_error: None, ctx },
            ));
            (Some(producer), output)
        },
        Err(e) => {
            let output = MessageBrokerActorOutput::Result(MessageBrokerActorOutputData::Configure(
                MessageBrokerConfigureResult { maybe_error: Some(e), ctx },
            ));
            (None, output)
        },
    }
}

#[async_trait::async_trait]
impl
    Actor<
        MessageBrokerActorInputData,
        MessageBrokerActorInput,
        MessageBrokerActorOutput,
        MessageBrokerActorIO,
    > for MessageBrokerActor
{
    async fn on_input(&mut self, data: MessageBrokerActorInputData) -> MessageBrokerActorOutput {
        if let MessageBrokerActorInputData::Configure(c) = data {
            let (producer, output) = configure(c);
            self.producer = producer;
            return output
        }

        if self.producer.is_none() {
            return MessageBrokerConfigureCtx { maybe_input: Box::new(Some(data.into())) }.into()
        }

        let producer = self.producer.as_ref().unwrap();
        match data {
            MessageBrokerActorInputData::Configure(_) => {
                unreachable!();
            },
            MessageBrokerActorInputData::SendReplayedBlockEvent(SendEvent { event, ctx }) => {
                if event.is_err() {
                    return MessageBrokerActorOutputData::SendReplayedBlockEvent(SendEventResult {
                        delivery: Ok(Err(event.err().unwrap())),
                        ctx,
                    })
                    .into()
                }
                let payload = match serde_json::to_string_pretty(&event.unwrap()) {
                    Ok(payload) => payload,
                    Err(e) =>
                        return MessageBrokerActorOutputData::SendReplayedBlockEvent(
                            SendEventResult { delivery: Err(e), ctx },
                        )
                        .into(),
                };
                let record = FutureRecord::to(TOPIC).key(EVENTS_KEY).payload(&payload);
                let delivery_result =
                    producer.send(record, Timeout::After(std::time::Duration::from_secs(5))).await;
                MessageBrokerActorOutputData::SendReplayedBlockEvent(SendEventResult {
                    delivery: Ok(Ok(delivery_result)),
                    ctx,
                })
                .into()
            },
            MessageBrokerActorInputData::SendBlockEvent(SendEvent { event, ctx }) => {
                if event.is_err() {
                    return MessageBrokerActorOutputData::SendBlockEvent(SendEventResult {
                        delivery: Ok(Err(event.err().unwrap())),
                        ctx,
                    })
                    .into()
                }
                let payload = match serde_json::to_string_pretty(&event.unwrap()) {
                    Ok(payload) => payload,
                    Err(e) =>
                        return MessageBrokerActorOutputData::SendBlockEvent(SendEventResult {
                            delivery: Err(e),
                            ctx,
                        })
                        .into(),
                };
                let record = FutureRecord::to(TOPIC).key(EVENTS_KEY).payload(&payload);
                let delivery_result =
                    producer.send(record, Timeout::After(std::time::Duration::from_secs(5))).await;
                MessageBrokerActorOutputData::SendBlockEvent(SendEventResult {
                    delivery: Ok(Ok(delivery_result)),
                    ctx,
                })
                .into()
            },
        }
    }
}
