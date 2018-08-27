#!/usr/bin/env bash

git submodule update --recursive --remote

cd gir && cargo build --release && cd ..

./gir/target/release/gir -c Gir.sys.toml -o nm-sys -d gir-files

./gir/target/release/gir -c Gir.toml
