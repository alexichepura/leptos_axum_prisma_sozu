#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -rvC --progress ./site.db $SOZU_HOST:$SERVER_DIR/site.db