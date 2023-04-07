#!/bin/sh

cargo build --release --target aarch64-apple-darwin
cbindgen --config cbindgen.toml --crate random-color-generator --output include/random_color_generator.h

cp target/aarch64-apple-darwin/release/librandom_color_generator.a macOS/RandomColorGenerator/Frameworks/librandom_color_generator.a