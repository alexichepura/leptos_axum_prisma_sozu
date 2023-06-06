#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

ssh $SOZU_HOST "
lsof -t -i:3000 | xargs -r kill
DATABASE_URL=file:$SERVER_DIR/site.db \
LEPTOS_SITE_ADDR=127.0.0.1:3000 \
LEPTOS_SITE_ROOT=$SERVER_DIR/site \
LEPTOS_OUTPUT_NAME=$NAME \
nohup $SERVER_DIR/$NAME >$SERVER_DIR/nohup.out 2>./nohup.err &
"
