#!/bin/bash

set -e

ENV=${ENV?Please specify target ENV}

export BUILD_SOURCE=${BUILD_SOURCE?Please specify BUILD_SOURCE directory}
export BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}

export REVISION=$(git -C $BUILD_SOURCE rev-parse --short HEAD)

export BUILD_SCRIPT="node/build.sh"
export BUILD_WORKDIR="."
export BINARY_NAME="appchain-deip"

TAG="deipworld/$ENV-vedai-substrate-$BINARY_NAME"
LATEST_TAG="$TAG:latest"
REVISION_TAG="$TAG:$REVISION"

APP_IMAGE="$REVISION_TAG" ./build_img.sh && docker tag $REVISION_TAG $LATEST_TAG

docker push "$REVISION_TAG"
docker push "$LATEST_TAG"

docker rmi "$REVISION_TAG"
docker rmi "$LATEST_TAG"
