# Leptos Axum Prisma Sōzu (LAPS)

100% rust fullstack starter \
Demo <https://leptos-axum-prisma-sozu.chepura.space>

## Features
- SSR+SPA with Leptos <https://github.com/leptos-rs/leptos>
- API with Axum <https://github.com/tokio-rs/axum>
- DB integration with Prisma <https://github.com/Brendonovich/prisma-client-rust>
- proxy Sōzu <https://github.com/sozu-proxy/sozu>
- letsencrypt cert gen in manual mode and proxy config
- cross compilation to server architecture, ARM64 used
- precompression of frontend files

Initially based on official leptos starter <https://github.com/leptos-rs/start-axum>

## Dev
<https://leptos-rs.github.io/leptos/02_getting_started.html>
```sh
cargo install cargo-leptos
cargo prisma db push
cargo leptos watch
```

## Prod prerequisites
### macOS
Cross compiler toolchains, supports both Apple Silicon & Intel Macs. 
https://github.com/messense/homebrew-macos-cross-toolchains
```sh
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-gnu # install x86_64-unknown-linux-gnu toolchain
brew install aarch64-unknown-linux-gnu # install aarch64-unknown-linux-gnu toolchain

# setup the environment variables as below to use it with Cargo
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

brew install protobuf # sōzu requires protobuf installed
```
### Common
```sh
rustup target add wasm32-unknown-unknown # leptos SPA requirement
cargo install precompres # precompression tool for site public folder
cp .env.example .env # copy and edit .env
```

## Prod build, deploy and run
### Sōzu
<https://github.com/sozu-proxy/sozu/blob/main/doc/recipes.md>
```sh 
./ops/sozu-install.sh
./ops/sozu-config.sh
./ops/sozu-user.sh
```

### Cert
<https://eff-certbot.readthedocs.io/en/stable/using.html#manual>
```sh
./ops/cert-bot.sh # Letsencrypt certbot manual certificates generation into ./ops/cert
./ops/cert-rsync.sh # Upload certificates
```

### Site
```sh
./ops/site-build.sh # Prisma generate, leptos build, precompress pkg
./ops/site-rsync.sh # Upload site binary and public
./ops/site-rsync-db-to-server.sh
```

### Run
```sh
./ops/site-run.sh # Stop previous site instance, start with nohup
./ops/sozu-start.sh
```
