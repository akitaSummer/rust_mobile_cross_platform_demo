#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

export SWIFT_BRIDGE_OUT_DIR="$(pwd)/generated"

BUILD_API=0 cargo build --target x86_64-apple-darwin
BUILD_API=0 cargo build --target aarch64-apple-darwin
mkdir -p ../target/universal-macos/debug

lipo \
    ../target/aarch64-apple-darwin/debug/libios_code.a \
    ../target/x86_64-apple-darwin/debug/libios_code.a -create -output \
    ../target/universal-macos/debug/libios_code.a

BUILD_API=0 cargo build --target aarch64-apple-ios
BUILD_API=0 cargo build --target x86_64-apple-ios
BUILD_API=0 cargo build --target aarch64-apple-ios-sim
mkdir -p ../target/universal-ios/debug

lipo \
    ../target/aarch64-apple-ios-sim/debug/libios_code.a \
    ../target/x86_64-apple-ios/debug/libios_code.a -create -output \
    ../target/universal-ios/debug/libios_code.a

BUILD_API=1 cargo build