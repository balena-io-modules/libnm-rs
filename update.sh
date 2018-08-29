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

echo 'Replace 80211ApFlags with _80211ApFlags...'
grep -lr -w 80211ApFlags ./src/auto/*.rs | xargs sed -i 's/\b80211ApFlags\b/_80211ApFlags/g'

echo 'Replace 80211ApSecurityFlags with _80211ApSecurityFlags...'
grep -lr -w 80211ApSecurityFlags ./src/auto/*.rs | xargs sed -i 's/\b80211ApSecurityFlags\b/_80211ApSecurityFlags/g'

echo 'Replace 4WAY_HANDSHAKE with _4WAY_HANDSHAKE...'
grep -lr -w 4WAY_HANDSHAKE ./src/auto/*.rs | xargs sed -i 's/\b4WAY_HANDSHAKE\b/_4WAY_HANDSHAKE/g'

echo 'Purge unused glib from auto/client.rs...'
sed -i '/use glib;/d' src/auto/client.rs

echo 'Formatting code...'
cargo fmt

echo 'Run connectivity example...'
cargo run --example connectivity
