#!/usr/bin/env bash

echo 'Update gir...'
git submodule update --recursive --remote

echo 'Build gir...'
cd gir && cargo build --release && cd ..

echo 'Generate NM ffi bindings...'
./gir/target/release/gir -c Gir_NM.sys.toml -o nm-sys -d gir-files

echo 'Generate NM auto bindings...'
./gir/target/release/gir -c Gir_NM.toml

echo 'Replace 80211Mode with _80211Mode...'
grep -lr -w 80211Mode ./src/auto/*.rs | xargs sed -i 's/\b80211Mode\b/_80211Mode/g'

echo 'Run connectivity example...'
cargo run --example connectivity
