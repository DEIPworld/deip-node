#!/bin/bash

set -e

if [ -z "$CARGO_HOME" ]; then
  echo "Please specify \$CARGO_HOME"
  exit 1
fi
if [ -z "$CARGO_TARGET_DIR" ]; then
  echo "Please specify \$CARGO_HOME"
  exit 1
fi
if [ -z "$CARGO_ARGS" ]; then
  echo "Please specify \$CARGO_ARGS"
  exit 1
fi

echo "+--- Cargo env ---------------------------------+"
echo "CARGO_ARGS=$CARGO_ARGS"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "CARGO_HOME=$CARGO_HOME"

echo "+--- Run Cargo ---------------------------------+"
mkdir -p $CARGO_HOME
mkdir -p $CARGO_TARGET_DIR
cargo $CARGO_ARGS
