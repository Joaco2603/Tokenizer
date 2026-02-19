
# Configurable variables
WASM ?= target/wasm32-unknown-unknown/release/my_contract.wasm
SOURCE ?= alice
NETWORK ?= testnet

default: build

all: test

test: build
	cargo test

build:
	stellar contract build
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

# Deploy the built wasm to Stellar (uses `stellar` CLI)
# Usage: make deploy WASM=path/to/wasm SOURCE=alice NETWORK=testnet
deploy: build
	stellar contract deploy --wasm $(WASM) --source $(SOURCE) --network $(NETWORK)

# Convenience target: deploy and show contract id (stdout from CLI)
deploy-show: deploy
	@echo "Deployed. Check output above for CONTRACT_ID"

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets -- -D warnings

clean:
	cargo clean
