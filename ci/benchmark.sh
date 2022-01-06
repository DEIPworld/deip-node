#!/bin/bash

set -e

export BUILD_SOURCE=${BUILD_SOURCE?Please specify BUILD_SOURCE directory}
export BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}
export PALLETS=${PALLETS?Please specify PALLETS to benchmarking}

export REVISION=$(git -C $BUILD_SOURCE rev-parse --short HEAD)

export BUILD_SCRIPT="node/build.sh"
export BUILD_WORKDIR="."
export BINARY_NAME="appchain-deip"
export FEATURES="runtime-benchmarks"
export APP_IMAGE="$BINARY_NAME-benchmarking:$REVISION"
./build_img.sh
echo $APP_IMAGE
docker run --rm --init -t $APP_IMAGE \
  bash -c "appchain-deip benchmark \
    --chain dev \
    --execution wasm \
    --wasm-execution compiled \
    --pallet $PALLETS \
    --extrinsic '*' \
    --steps 50 \
    --repeat 20 \
    --heap-pages 4096 \
    --output /home/weights.rs \
  && cat /home/weights.rs"
