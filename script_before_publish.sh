#!/usr/bin/env bash
cargo test
cargo clippy --workspace --profile test -- -Dclippy::all

# coverage
# cargo +nightly test --color=always --package url_encoded_data --lib test_qs --no-fail-fast -- --format=json -Z unstable-options --show-output