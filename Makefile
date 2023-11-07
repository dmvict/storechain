app_conf_dir=~/.storechain

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)
#

run-debug:
	RUST_LOG=DEBUG cargo run -- run --verbose

run:
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- run

test:
	cargo test

install:
	cargo install --path ./storechain

init:
	set -eux
	rm -rf ${app_conf_dir}
	mkdir -p ${app_conf_dir}/config
	echo "pg_url = \"postgresql://pg:pg@localhost:5432/pg\"" > ${app_conf_dir}/config/app_conf.toml
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- init test
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- add-genesis-account cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux 34uatom

tendermint-start:
	tendermint start --home ~/.storechain

.PHONY: run run-debug test install init tendermint-start
