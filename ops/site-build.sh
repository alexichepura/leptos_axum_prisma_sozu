#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

cargo prisma generate
LEPTOS_BIN_TARGET_TRIPLE=$TARGET cargo leptos build --release
precompress --brotli --deflate --gzip --zstd target/site/pkg