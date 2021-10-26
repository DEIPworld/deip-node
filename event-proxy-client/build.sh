#!/bin/bash

set -e

mode=""
if [ "$BUILD_MODE" == "release" ]; then
  mode="--release"
fi

cd $WORKSPACE
CARGO_ARGS="build $mode" cargo.sh
