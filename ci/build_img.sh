#!/bin/bash

set -e


export REVISION=${REVISION?Please specify vcs REVISION}
export BINARY_NAME=${BINARY_NAME?Please specify BINARY_NAME}
export BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}
APP_IMAGE=${APP_IMAGE?Please specify APP_IMAGE}

CTX="${BUILD_CACHE}/ctx/${BINARY_NAME}_${REVISION}"
export ARTIFACT="$CTX/$BINARY_NAME"

mkdir -p $CTX && ./build_bin.sh

cp Dockerfile $CTX
docker build -t "$APP_IMAGE" \
  --build-arg=ARTIFACT=$BINARY_NAME \
  $CTX
rm -rf $CTX
