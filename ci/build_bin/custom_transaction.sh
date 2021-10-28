#!/bin/bash

set -e

export BUILD_SOURCE=${BUILD_SOURCE?Please specify BUILD_SOURCE directory}
export BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}
export BUILD_OUTPUT=${BUILD_OUTPUT?Please specify BUILD_OUTPUT directory}

export BUILD_SCRIPT="bin/custom-transaction/build.sh"
export BUILD_WORKDIR="."
export BINARY_NAME="custom-transaction"
export ARTIFACT=$BUILD_OUTPUT/$BINARY_NAME
./build_bin.sh
