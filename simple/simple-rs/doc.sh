#!/usr/bin/env bash
rm -r target/doc
mkdir -p target/doc
cp -rl ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/* target/doc
cargo doc $@
