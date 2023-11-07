#!/usr/bin/env bash

set -eux

APP_CONF_DIR=~/.storechain

rm -rf $APP_CONF_DIR
mkdir -p $APP_CONF_DIR/config
echo "pg_url = \"postgresql://pg:pg@localhost:5432/pg\"" > $APP_CONF_DIR/config/app_conf.toml

cargo run -- init test

cargo run -- add-genesis-account cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux 34uatom
