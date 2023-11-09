app_conf_dir=~/.storechain
# docker_compose=docker-compose
ifeq (, $(shell which docker-compose))
docker_compose=docker compose
else
docker_compose=docker-compose
endif
# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)
#

init:
	set -eux
	rm -rf ${app_conf_dir}
	mkdir -p ${app_conf_dir}/config
	echo "pg_url = \"postgresql://pg:pg@localhost:5432/pg\"" > ${app_conf_dir}/config/app_conf.toml
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- init test
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- add-genesis-account cosmos1syavy2npfyt9tcncdtsdzf7kny9lh777pahuux 34uatom

run:
	cargo run $(if $(call eq,$(features),),,--features $(features)) -- run

run-debug:
	RUST_LOG=DEBUG cargo run $(if $(call eq,$(features),),,--features $(features)) -- run --verbose

tendermint-start:
	tendermint start --home ~/.storechain

db.init:
	sqlx migrate run

add.test.acc:
	echo "race draft rival universe maid cheese steel logic crowd fork comic easy truth drift tomorrow eye buddy head time cash swing swift midnight borrow" | cargo run $(if $(call eq,$(features),),,--features $(features)) -- keys add alice --recover

test:
	cargo test

install:
	cargo install --path ${app_conf_dir}

# Docker commands

docker.start:
	${docker_compose} up -d

docker.stop:
	${docker_compose} down

docker.exec:
	${docker_compose} exec app /bin/bash

docker.init:
	${docker_compose} exec app make init features=$(features)

docker.run:
	${docker_compose} exec app make run features=$(features)

docker.tendermint-start:
	${docker_compose} exec app make tendermint-start

docker.db.init:
	${docker_compose} exec app make db.init

docker.add.test.acc:
	${docker_compose} exec app make add.test.acc features=$(features)

.PHONY: run run-debug test install init tendermint-start
