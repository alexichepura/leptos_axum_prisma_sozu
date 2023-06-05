#!/usr/bin/env sh

certbot certonly \
  --manual \
  --preferred-challenges dns \
  --work-dir ./ops/cert --logs-dir ./ops/cert --config-dir ./ops/cert