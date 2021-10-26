### How to run

First, we need to up Apache Kafka. For testing purposes we use "bitnami/kafka:2" docker image (go to the image page in the docker-hub website and follow setup instructions).  



Second, we up the "event-proxy-client" - a simple program that read events from the kafka-topic and prints them to stdout:

`$ cd deip-substrate/event-proxy-client && cargo run --release`



Third, run "event-proxy":

`$ cd deip-substrate/event-proxy-client`

`$ RUST_LOG="error,librdkafka=error,rdkafka::client=error" cargo run --release -- --config=src/default_config.toml`



