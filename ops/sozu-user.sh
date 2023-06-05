#!/usr/bin/env sh
set -o allexport; source .env; set +o allexport

SSH_PUB_KEY=$(cat $SSH_PUB_KEY_PATH)

ssh $ROOT_HOST "
useradd --create-home $SOZU_USER;
passwd -d $SOZU_USER;
mkdir -p $SOZU_HOME/.ssh;
echo $SSH_PUB_KEY > $SOZU_HOME/.ssh/authorized_keys;
chmod 400 .ssh/authorized_keys
chown -R $SOZU_USER: $SOZU_HOME/.ssh
"