#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

ssh $ROOT_HOST "
systemctl daemon-reload
systemctl enable sozu.service
systemctl start sozu.service
"