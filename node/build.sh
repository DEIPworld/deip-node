#!/bin/bash

set -e

mode=""
if [ "$BUILD_MODE" == "release" ]; then
  mode="--release"
fi

features=""
if [ -n "$FEATURES" ]; then
  features="--features $FEATURES"
fi

cd $WORKSPACE
CARGO_ARGS="build $mode $features --bin=$CARGO_BIN" cargo.sh
