#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -rvC --progress $SOZU_HOST:$SERVER_DIR/site.db ./site.db