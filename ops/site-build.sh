#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rustup target add $TARGET
if [ ! -f ./site.db ];
then
    cargo prisma db push
else
    cargo prisma generate
fi
LEPTOS_BIN_TARGET_TRIPLE=$TARGET cargo leptos build --release
precompress --brotli --deflate --gzip --zstd target/site/pkg