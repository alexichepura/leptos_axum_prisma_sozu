#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -rvC --progress target/server/$TARGET/release/$NAME $SOZU_HOST:$SERVER_DIR/$NAME
rsync -rvC --progress target/site $SOZU_HOST:$SERVER_DIR