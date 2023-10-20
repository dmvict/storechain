run-debug:
	RUST_LOG=DEBUG cargo run -- run --verbose

run:
	cargo run -- run

test:
	cargo test

install:
	cargo install --path ./storechain

init:
	./storechain/scripts/init.sh

tendermint-start:
	tendermint start --home ~/.storechain

.PHONY: run run-debug test install init tendermint-start
