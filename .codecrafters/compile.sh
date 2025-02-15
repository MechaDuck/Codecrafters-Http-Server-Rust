#!/bin/sh
#
# This script is used to compile your program on CodeCrafters
#
# This runs before .codecrafters/run.sh
#
# Learn more: https://codecrafters.io/program-interface

set -e # Exit on failure
set RUST_BACKTRACE=full
cargo build --release --target-dir=/tmp/codecrafters-http-server-target --manifest-path Cargo.toml 
