#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

CONFIG="[[listeners]]
address = \"0.0.0.0:443\"
protocol = \"https\"

[clusters]
[clusters.laps]
protocol = \"http\"
https_redirect = true
frontends = [
    { address = \"0.0.0.0:80\", hostname = \"$DOMAIN\" },
]
backends = [{ address = \"127.0.0.1:3000\" }]
[clusters.laps_s]
protocol = \"http\"
https_redirect = false
frontends = [
    { address = \"0.0.0.0:443\", hostname = \"$DOMAIN\", certificate = \"$SERVER_DIR/cert.pem\", key = \"$SERVER_DIR/privkey.pem\", certificate_chain = \"$SERVER_DIR/chain.pem\" },
]
backends = [{ address = \"127.0.0.1:3000\" }]
"
printf "%s" "$CONFIG" > "ops/sozu.toml"

rsync -rvC --progress --rsync-path="mkdir -p /etc/sozu/ && rsync" ops/sozu.toml $ROOT_HOST:/etc/sozu/config.toml