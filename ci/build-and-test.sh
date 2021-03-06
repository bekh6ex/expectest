#!/usr/bin/env bash

cargo build --verbose
cargo test --verbose

if [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
    cargo build --verbose --features nightly
    cargo test --verbose --features nightly
fi
