#!/usr/bin/env bash
find . -iname "*.rs" -exec touch {} \; || exit 1
cargo clippy -- -W clippy::all
