FROM paritytech/ci-linux:production

RUN rustup toolchain install nightly-2021-12-15-x86_64-unknown-linux-gnu
RUN rustup default nightly-2021-12-15-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

WORKDIR /home/build_source
VOLUME /home/build_source
VOLUME /home/build_cache

COPY cargo.sh /usr/bin
