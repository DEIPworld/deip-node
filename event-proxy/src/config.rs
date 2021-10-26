use std::path::Path;
use std::io;
use std::fs;
use serde::{Deserialize, de::DeserializeOwned};

pub fn load<Offchain: DeserializeOwned, P: AsRef<Path>>(path: P) -> io::Result<Config<Offchain>> {
    toml::from_slice(fs::read(path)?.as_slice()).map_err(Into::into)
}

#[derive(Deserialize)]
pub struct Config<Offchain> {
    pub blockchain: BlockchainConfig,
    pub message_broker: MessageBrokerConfig,
    pub offchain: Offchain
}

#[derive(Deserialize, Clone)]
pub struct BlockchainConfig {
    pub rpc: String
}

#[derive(Deserialize, Clone)]
pub struct MessageBrokerConfig {
    pub kafka_bootstrap_servers: String,
}

#[derive(Deserialize, Clone)]
pub struct OffchainConfig<LastKnownBlock> {
    pub last_known_block: LastKnownBlock
}
