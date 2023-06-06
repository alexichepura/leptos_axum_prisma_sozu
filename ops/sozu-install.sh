#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

git clone git@github.com:sozu-proxy/sozu.git $SOZU_SOURCE_DIR
cd $SOZU_SOURCE_DIR
rustup target add $TARGET
cargo build --bin sozu --release --locked --target=$TARGET --config profile.release.debug=false
rsync -rvC --progress target/$TARGET/release/sozu $ROOT_HOST:/usr/bin/sozu
cd -
rsync -rvC --progress ops/sozu.service $ROOT_HOST:/etc/systemd/system/