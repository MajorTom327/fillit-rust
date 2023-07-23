#!/bin/zsh

cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target aarch64-apple-darwin --release

et -c -i -l 1 -s size-rev ./target/aarch64-apple-darwin/release
