mod frame;
mod events;
mod types;
mod runtime;
mod actor;
mod app;
mod config;

use std::time::Duration;
use std::process::exit;

use substrate_subxt::{system::System};
use substrate_subxt::NodeTemplateRuntime;

use tokio::sync::mpsc;
use futures::stream::{FuturesOrdered, StreamExt};
use futures::{Future};

type RuntimeT = NodeTemplateRuntime;

use app::{
    Actor, ActorI, ActorO, ActorIO,
    MessageBrokerActor, MessageBrokerActorIO, MessageBrokerActorInput, MessageBrokerActorOutput, MessageBrokerActorOutputData,
    BlockchainActor, BlockchainActorIO, BlockchainActorInput, BlockchainActorOutput, BlockchainActorOutputData, BlocksReplay,
    OffchainActor, OffchainActorIO, OffchainActorInput, OffchainActorOutput, OffchainActorOutputData,
};

macro_rules! reset {
    ($actor_task_queue:ident, $_released_actor_queue:ident, $offchain_config:expr) => {
        $actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
            OffchainActorInput::build_client($offchain_config.clone()),
            &mut $_released_actor_queue
        ).await);
    };
}

macro_rules! reset_blockchain_actor {
    ($actor_task_queue:ident, $_released_actor_queue:ident, $blockchain_config:expr) => {
        $actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
            BlockchainActorInput::build_client($blockchain_config.clone()),
            &mut $_released_actor_queue
        ).await);
    };
}

macro_rules! offchain_actor_SetClient {
    ($client:ident, $actor_task_queue:ident, $_released_actor_queue:ident) => {
        $actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
            OffchainActorInput::set_client($client),
            &mut $_released_actor_queue
        ).await);
    };
}

macro_rules! offchain_actor_GetLastKnownBlock {
    ($actor_task_queue:ident, $_released_actor_queue:ident) => {
        $actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
            OffchainActorInput::get_last_known_block(),
            &mut $_released_actor_queue
        ).await);
    };
}

macro_rules! blockchain_actor_SubscribeFinalizedBlocks {
    ($last_known_block:ident, $actor_task_queue:ident, $_released_actor_queue:ident) => {
        $actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
            BlockchainActorInput::subscribe_finalized_blocks($last_known_block),
            &mut $_released_actor_queue
        ).await);
    };
}

use clap::value_t_or_exit;

#[tokio::main]
async fn main() {
    
    flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    let args = clap::App::new("event-proxy")
        .arg(clap::Arg::with_name("config")
            .long("config")
            .value_name("FILE")
            .help("Config file")
            .takes_value(true)
            .empty_values(false)
            .required(true)
        ).get_matches();
    
    let config_file = clap::value_t_or_exit!(args, "config", String);
    
    let config = config::load::<config::OffchainConfig<app::LastKnownBlock>, _>(config_file)
        .unwrap_or_else(|e| {
            log::error!("{}\n EXIT(-1)", e.to_string());
            exit(-1);
        });
    
    // Init blockchain-actor:
    let mut blockchain_actor = BlockchainActor::new();
    let (blockchain_actor_io, blockchain_actor_io2) = BlockchainActorIO::pair();
    tokio::spawn(async move { blockchain_actor.actor_loop(blockchain_actor_io).await });
    
    // Init message-broker-actor:
    let mut message_broker_actor = MessageBrokerActor::new();
    let (message_broker_actor_io, message_broker_actor_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move { message_broker_actor.actor_loop(message_broker_actor_io).await });
    
    // Init offchain-actor:
    let mut offchain_actor = OffchainActor::new();
    let (offchain_actor_io, offchain_actor_io2) = OffchainActorIO::pair();
    tokio::spawn(async move { offchain_actor.actor_loop(offchain_actor_io).await });
    
    let mut subscription_task_queue = FuturesOrdered::new();
    let mut subscription_buffer_task_queue = FuturesOrdered::new();
    
    let mut blocks_replay_task_queue = FuturesOrdered::new();
    
    let mut events_buffer_task_queue = FuturesOrdered::new();
    let mut replayed_block_events_buffer_task_queue = FuturesOrdered::new();
    
    let mut blockchain_actor_task_queue = FuturesOrdered::new();
    let mut message_broker_actor_task_queue = FuturesOrdered::new();
    let mut offchain_actor_task_queue = FuturesOrdered::new();
    
    let mut released_blockchain_actor_queue = released_actor_queue::<_, _, BlockchainActorIO>();
    let mut released_message_broker_actor_queue = released_actor_queue::<_, _, MessageBrokerActorIO>();
    let mut released_offchain_actor_queue = released_actor_queue::<_, _, OffchainActorIO>();

    release_actor(blockchain_actor_io2, &mut released_blockchain_actor_queue).await;
    release_actor(message_broker_actor_io2, &mut released_message_broker_actor_queue).await;
    release_actor(offchain_actor_io2, &mut released_offchain_actor_queue).await;
    
    // Put the initial task to trigger main workflow:
    reset!(offchain_actor_task_queue, released_offchain_actor_queue, config.offchain);
    
    loop { tokio::select! {
        Some(offchain_actor_task_result) = offchain_actor_task_queue.next() => {
            let (maybe_output, io) = offchain_actor_task_result;
            release_actor(io, &mut released_offchain_actor_queue).await;
            let output = if maybe_output.is_none() { unreachable!(); } else { maybe_output.unwrap() };
            match output {
                OffchainActorOutput::NoClient => {
                    reset!(offchain_actor_task_queue, released_offchain_actor_queue, config.offchain);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::BuildClient(client)) => {
                    offchain_actor_SetClient!(client,
                        offchain_actor_task_queue, released_offchain_actor_queue);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::SetClient) => {
                    offchain_actor_GetLastKnownBlock!(offchain_actor_task_queue, released_offchain_actor_queue);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::GetLastKnownBlock(maybe_last_known_block)) => {
                    match maybe_last_known_block {
                        Ok(last_known_block) => {
                            blockchain_actor_SubscribeFinalizedBlocks!(last_known_block,
                                blockchain_actor_task_queue, released_blockchain_actor_queue);
                        },
                        Err(e) => {
                            log::error!("{:?}", e);
                            reset!(offchain_actor_task_queue, released_offchain_actor_queue, config.offchain);
                        },
                    }
                },
            }
        },
        Some(subscription_task_result) = subscription_task_queue.next() => {
            let (maybe_finalized_block_header, subscription, buf) = subscription_task_result;
            // println!("BUFFERED SUBSCRIPTION TASK QUEUE");
            match maybe_finalized_block_header {
                Ok(Some(finalized_block_header)) => {
                    // Put subscription item into buffer:
                    // println!("PUT SUBSCRIPTION ITEM INTO BUFFER");
                    let buf: SubscriptionBufferIn = buf;
                    buf.push(finalized_block_header);
                    subscription_task_queue.push(subscription_task(subscription, buf));
                },
                err => {
                    match err {
                        Ok(Some(_)) => unreachable!(),
                        Ok(None) => { log::error!("Subscription termination unexpected"); },
                        Err(e) => { log::error!("{}", e); },
                    }
                    reset_blockchain_actor!(
                        blockchain_actor_task_queue,
                        released_blockchain_actor_queue,
                        config.blockchain
                    );
                },
            }
        },
        Some(SubscriptionBufferTaskResult {
            subscription_item, subscription_buffer, events_buffer
        }) = subscription_buffer_task_queue.next() => {
            // println!("NEXT SUBSCRIPTION TASK");
            let finalized_block_header: <RuntimeT as System>::Header = subscription_item;
            // println!("GOT FINALIZED BLOCK HEADER");
            blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                BlockchainActorInput::get_block_events(
                    finalized_block_header.hash(),
                    subscription_buffer,
                    events_buffer
                ),
                &mut released_blockchain_actor_queue
            ).await);
        },
        Some(blockchain_actor_task_result) = blockchain_actor_task_queue.next() => {
            let (maybe_output, io) = blockchain_actor_task_result;
            release_actor(io, &mut released_blockchain_actor_queue).await;
            let output = if maybe_output.is_none() { unreachable!(); } else { maybe_output.unwrap() };
            match output {
                BlockchainActorOutput::NoClient(_input) => {
                    reset_blockchain_actor!(
                        blockchain_actor_task_queue,
                        released_blockchain_actor_queue,
                        config.blockchain
                    );
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::BuildClient(maybe_client)) => {
                    match maybe_client {
                        Ok(client) => {
                            blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                                BlockchainActorInput::set_client(client),
                                &mut released_blockchain_actor_queue
                            ).await);
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            tokio::time::sleep(Duration::from_secs(5)).await;
                            reset_blockchain_actor!(
                                blockchain_actor_task_queue,
                                released_blockchain_actor_queue,
                                config.blockchain
                            );
                        },
                    }
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient) => {
                    reset!(offchain_actor_task_queue, released_offchain_actor_queue, config.offchain);
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::ReplayBlocks(replay)) => {
                    blocks_replay_task_queue.push(blocks_replay_task(replay));
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SubscribeFinalizedBlocks(
                    maybe_subscription, last_known_block, subscription_buffer, events_buffer
                )) => {
                    match maybe_subscription {
                        Ok(subscription) => {
                            let (maybe_head_block, subscription, subscription_buffer_in)
                            = subscription_task(subscription, subscription_buffer.detach_in()).await;
                            match maybe_head_block {
                                Ok(Some(head_block)) => {
                                    blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                                        BlockchainActorInput::replay_blocks(
                                            last_known_block,
                                            head_block.hash(),
                                            subscription_buffer,
                                            events_buffer
                                        ),
                                        &mut released_blockchain_actor_queue
                                    ).await);
                                    // Accumulate subscription items in the buffer until blocks replay ends:
                                    // println!("ACCUMULATE SUBSCRIPTION ITEMS");
                                    subscription_task_queue.push(subscription_task(subscription, subscription_buffer_in));
                                },
                                err => {
                                    match err {
                                        Ok(Some(_)) => unreachable!(),
                                        Ok(None) => { log::error!("Subscription termination unexpected"); },
                                        Err(e) => { log::error!("{}", e); },
                                    }
                                    reset_blockchain_actor!(
                                        blockchain_actor_task_queue,
                                        released_blockchain_actor_queue,
                                        config.blockchain
                                    );
                                },
                            }
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(
                                blockchain_actor_task_queue,
                                released_blockchain_actor_queue,
                                config.blockchain
                            );
                        },
                    }
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlockEvents {
                    maybe_events, subscription_buffer, events_buffer
                }) => {
                    // println!("GET BLOCK EVENTS: {:?}", &maybe_events);
                    match maybe_events {
                        Ok(events) => {
                            let events = events.expect("EXISTENT BLOCK");
                            // println!("EVENTS: {:?}", &events);
                            let remaining = events.len();
                            for x in events.into_iter() {
                                events_buffer.push(x);
                            }
                            events_buffer_task_queue.push(
                                events_buffer_task(events_buffer, remaining, subscription_buffer));
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(
                                blockchain_actor_task_queue,
                                released_blockchain_actor_queue,
                                config.blockchain
                            );
                        },
                    }
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetReplayedBlockEvents(maybe_events, replay)) => {
                    // println!("GET REPLAYED EVENTS: {:?}", &maybe_events);
                    match maybe_events {
                        Ok(events) => {
                            let events = events.expect("EXISTENT BLOCK");
                            // println!("REPLAYED EVENTS: {:?}", &events);
                            let remaining = events.len();
                            for x in events.into_iter() {
                                replay.3.push(x);
                            }
                            replayed_block_events_buffer_task_queue.push(
                                replayed_block_events_buffer_task(remaining, replay));
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(
                                blockchain_actor_task_queue,
                                released_blockchain_actor_queue,
                                config.blockchain
                            );
                        },
                    }
                },
            }
        },
        Some(blocks_replay_task_result) = blocks_replay_task_queue.next() => {
            let (replay, maybe_header) = blocks_replay_task_result;
            if let Some(header) = maybe_header {
                blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                    BlockchainActorInput::get_replayed_block_events(header.hash(), replay),
                    &mut released_blockchain_actor_queue
                ).await);
            } else {
                // End of blocks replay. Just start consume items from subscription buffer:
                // println!("END OF BLOCKS REPLAY");
                subscription_buffer_task_queue.push(subscription_buffer_task(replay.2, replay.3));
            }
        },
        
        Some(EventsBufferTaskResult {
            event, events_buffer, remaining, subscription_buffer
        }) = events_buffer_task_queue.next() => {
            // println!("EventsBufferTaskResult");
            message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                MessageBrokerActorInput::send_block_event(event, events_buffer, remaining, subscription_buffer),
                &mut released_message_broker_actor_queue
            ).await);
        },
        Some(ReplayedBlockEventsBufferTaskResult {
            event, remaining, replay
        }) = replayed_block_events_buffer_task_queue.next() => {
            // println!("ReplayedBlockEventsBufferTaskResult: {:?}", &event);
            message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                MessageBrokerActorInput::send_replayed_block_event(event, remaining, replay),
                &mut released_message_broker_actor_queue
            ).await);
        },
        
        Some(message_broker_actor_task_result) = message_broker_actor_task_queue.next() => {
            let (maybe_output, io) = message_broker_actor_task_result;
            let output = if maybe_output.is_some() { maybe_output.unwrap() } else { unreachable!(); };
            release_actor(io, &mut released_message_broker_actor_queue).await;
            let delivery_status = match output {
                MessageBrokerActorOutput::NotConfigured(ctx) => {
                    message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                        MessageBrokerActorInput::configure(config.message_broker.clone(), ctx),
                        &mut released_message_broker_actor_queue
                    ).await);
                    continue;
                },
                MessageBrokerActorOutput::Result(MessageBrokerActorOutputData::Configure(app::MessageBrokerConfigureResult { maybe_error, ctx })) => {
                    match maybe_error {
                        None => if ctx.maybe_input.is_some() {
                            message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                                ctx.maybe_input.unwrap(),
                                &mut released_message_broker_actor_queue
                            ).await);
                        },
                        Some(e) => {
                            log::error!("{}", e);
                            tokio::time::sleep(Duration::from_secs(5)).await;
                            message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                                MessageBrokerActorInput::configure(config.message_broker.clone(), ctx),
                                &mut released_message_broker_actor_queue
                            ).await);
                        },
                    }
                    continue;
                },
                MessageBrokerActorOutput::Result(MessageBrokerActorOutputData::SendReplayedBlockEvent(app::SendEventResult { delivery, ctx })) => {
                    if ctx.remaining > 0 {
                        // println!("replayed_block_events_buffer_task AGAIN: remaining={:?}", ctx.remaining);
                        replayed_block_events_buffer_task_queue.push(
                            replayed_block_events_buffer_task(ctx.remaining, ctx.replay));
                    } else {
                        // println!("PUSH BLOCK REPLAY TASK");
                        // Replay next block:
                        blocks_replay_task_queue.push(blocks_replay_task(ctx.replay));
                    }
                    delivery
                },
                MessageBrokerActorOutput::Result(MessageBrokerActorOutputData::SendBlockEvent(app::SendEventResult { delivery, ctx })) => {
                    if ctx.remaining > 0 {
                        // println!("events_buffer_task_queue AGAIN: remaining={:?}", ctx.remaining);
                        events_buffer_task_queue.push(
                            events_buffer_task(ctx.events_buffer, ctx.remaining, ctx.subscription_buffer));
                    } else {
                        // Process the next finalized block:
                        // println!("SEND NEXT SUBSCRIPTION TASK");
                        subscription_buffer_task_queue.push(
                        subscription_buffer_task(ctx.subscription_buffer, ctx.events_buffer));
                    }
                    delivery
                },
            };
            log::debug!("DELIVERY STATUS: {:?}", delivery_status);
        },
    }; }
}

#[derive(Clone)]
pub struct BufferIn<T>(mpsc::UnboundedSender<T>);
impl<T> BufferIn<T> {
    fn push(&self, item: T) {
        if self.0.send(item).is_err() { panic!("NEVER GONE"); }
    }
}
pub struct Buffer<T>(BufferIn<T>, mpsc::UnboundedReceiver<T>);
impl<T> Buffer<T> {
    fn new() -> Self {
        let (i, o) = mpsc::unbounded_channel::<T>();
        Self(BufferIn(i), o)
    }
    fn detach_in(&self) -> BufferIn<T> where T: Clone { self.0.clone() }
    
    fn push(&self, item: T) {
        self.0.push(item);
    }
    async fn pop(&mut self) -> T {
        if let Some(item) = self.1.recv().await { item } else { panic!("NEVER GONE"); }
    }
}

type ReleasedActorQueue<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);

fn released_actor_queue<I, O, IO: ActorIO<I, O>>() -> ReleasedActorQueue<IO::Pair> { mpsc::channel(1) }

async fn release_actor<T>(io: T, q: &mut ReleasedActorQueue<T>) {
    if q.0.send(io).await.is_err() {
        panic!("NEVER GONE");
    }
}

async fn wait_released_actor<T>(q: &mut ReleasedActorQueue<T>) -> T {
    match q.1.recv().await {
        Some(x) => x,
        _ => panic!("NEVER GONE"),
    }
} 

use app::{FinalizedBlocksSubscription, FinalizedBlocksSubscriptionItem, SubscriptionBufferIn};

async fn subscription_task(mut subscription: FinalizedBlocksSubscription, buf: SubscriptionBufferIn)
    -> (FinalizedBlocksSubscriptionItem, FinalizedBlocksSubscription, SubscriptionBufferIn)
{
    (subscription.next().await, subscription, buf)
}

struct SubscriptionBufferTaskResult<T, U> {
    subscription_item: T,
    subscription_buffer: Buffer<T>,
    events_buffer: Buffer<U>
}
async fn subscription_buffer_task<T, U>(
    mut subscription_buffer: Buffer<T>,
    events_buffer: Buffer<U>,
)
    -> SubscriptionBufferTaskResult<T, U>
{
    SubscriptionBufferTaskResult {
        subscription_item: subscription_buffer.pop().await,
        subscription_buffer,
        events_buffer,
    }
}

struct EventsBufferTaskResult {
    event: app::MaybeBlockEvent,
    events_buffer: app::EventsBuffer,
    remaining: usize,
    subscription_buffer: app::SubscriptionBuffer
}
async fn events_buffer_task(
    mut events_buffer: app::EventsBuffer,
    remaining: usize,
    subscription_buffer: app::SubscriptionBuffer
)
    -> EventsBufferTaskResult
{
    if remaining == 0 { panic!("UNEXPECTED"); }
    EventsBufferTaskResult {
        event: events_buffer.pop().await,
        events_buffer,
        remaining: remaining - 1,
        subscription_buffer
    }
}

struct ReplayedBlockEventsBufferTaskResult {
    event: app::MaybeBlockEvent,
    remaining: usize,
    replay: app::BlocksReplay
}
async fn replayed_block_events_buffer_task(
    remaining: usize,
    mut replay: app::BlocksReplay
)
    -> ReplayedBlockEventsBufferTaskResult
{
    if remaining == 0 { panic!("UNEXPECTED"); }
    let events_buffer = &mut replay.3;
    ReplayedBlockEventsBufferTaskResult {
        event: events_buffer.pop().await,
        remaining: remaining - 1,
        replay
    }
}

async fn blocks_replay_task(replay: BlocksReplay) -> (BlocksReplay, Option<<RuntimeT as System>::Header>)
{
    let (task, mut rx, subscription_buffer, events_buffer) = replay;
    let maybe_header = rx.recv().await;
    ((task, rx, subscription_buffer, events_buffer), maybe_header)
}

type ActorTaskOutput<O, IO2> = (Option<O>, IO2);

async fn actor_task<I, O, IO>(input: I, mut io: IO::Pair) -> ActorTaskOutput<O, IO::Pair>
    where 
        I: Send, O: Send, IO: ActorIO<I, O>
{
    if io.send(input).await.is_err() { return (None, io) }
    (io.recv().await, io)
}

async fn init_actor_task<I: Send, O: Send, IO: ActorIO<I, O>>(
    input: I,
    io: &mut ReleasedActorQueue<IO::Pair>
)
    -> impl Future<Output = ActorTaskOutput<O, IO::Pair>>
{
    actor_task::<I, O, IO>(input, wait_released_actor(io).await)
}
