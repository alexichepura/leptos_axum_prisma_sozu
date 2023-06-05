# Leptos Axum Prisma Sozu (LAPS)

Boilerplate based on oficial leptos axum example https://github.com/leptos-rs/start-axum

https://github.com/leptos-rs/leptos
https://github.com/tokio-rs/axum
https://github.com/Brendonovich/prisma-client-rust
https://github.com/sozu-proxy/sozu

## Prerequisites
```sh
rustup target add wasm32-unknown-unknown

# https://github.com/messense/homebrew-macos-cross-toolchains
# rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# precompression tool for site public folder
cargo install precompress

# copy and edit .env
cp .env.example .env
```

## Production
```sh
./ops/site-build.sh # Prisma generate, leptos build, precompress site/pkg
./ops/site-rsync.sh # Upload server binary and site public folder
./ops/site-rsync-db-to-server.sh
./ops/site-run.sh # Stop previous server instance, start with nohup
```

## Certificate
```sh
./ops/cert-bot.sh # Letsencrypt certbot manual certificates generation into ./ops/cert
./ops/cert-rsync.sh # Upload certificates to server project folder
```

## Sozu proxy
```sh 
./ops/sozu-install.sh
./ops/sozu-config.sh
./ops/sozu-user.sh
./ops/sozu-start.sh
```

