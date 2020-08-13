#!/usr/bin/env sh


# move into the directory this script is in
cd "${0%/*}"

mkdir -p dist

# test as binary
cargo build -p humm_jwt_check --release

# remove symbols
strip target/release/humm_jwt_check

# show size
stat -c %s target/release/humm_jwt_check
