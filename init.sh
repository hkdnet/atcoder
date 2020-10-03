#!/bin/bash

rustup target add x86_64-unknown-linux-musl

mkdir -p $HOME/.config
ln -sf $(pwd)/cargo-atcoder.toml $HOME/.config/cargo-atcoder.toml
