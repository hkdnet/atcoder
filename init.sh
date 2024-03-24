#!/bin/bash

rustup target add x86_64-unknown-linux-musl
rustup override set 1.70.0

mkdir -p $HOME/.config
ln -sf $(pwd)/cargo-atcoder.toml $HOME/.config/cargo-atcoder.toml
