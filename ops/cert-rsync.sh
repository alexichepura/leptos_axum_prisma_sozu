#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

rsync -arvC --progress --copy-links ops/cert/live/$DOMAIN/*.pem $SOZU_HOST:$SERVER_DIR