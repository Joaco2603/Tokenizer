# Tokenizer Contracts (Soroban)

This repository contains Soroban smart-contracts centered on ERC-20 style fungible tokens for Stellar/Soroban. The main goals are:

- Provide an ERC-20-like token contract (mintable, burnable, transfer) compatible with common `stellar-tokens` primitives.
- Support a simple `sell` flow (user-initiated burn/sell operation) and owner-only `mint` operations.
- Organize the contract code following a modular architecture (core logic / events / storage / contract API) inspired by larger Soroban projects.

Repository layout (relevant parts):

```text
contracts/
	tokens/               # token contract package
		Cargo.toml
		src/
			contract.rs       # public contract entry (MyToken)
			lib.rs            # module exports
			core/             # business logic (TokenManager: mint, sell, transfer)
				token.rs
			events/           # event definitions and publishing
				handler.rs
			storage/          # contract storage types and keys
				types.rs
			tests.rs          # unit tests (TDD first)
	... other contracts ...
Cargo.toml              # workspace manifest
```

Key concepts

- `MyToken` (contract): public API and trait implementations (FungibleToken, Ownable, Pausable, UpgradeableInternal).
- `TokenManager` (core): encapsulates mint/transfer/burn/sell logic and emits events via `events::handler`.
- `events::handler`: centralized event publishing helpers (mint, transfer, burn, sell).
- `storage::types`: `contracttype` structs and custom data keys if needed.

Build & test (local)

Run unit tests for the token package only:

```bash
cargo test -p my-token
```

Or run workspace tests:

```bash
cargo test --workspace
```

Makefile
--------

- **Descripción:** Proporciona atajos para compilar, testear y desplegar contratos.
- **Archivo:** [Makefile](Makefile)
- **Comandos importantes:**
	- `make build` : compila el contrato optimizado (WASM).
	- `make test`  : ejecuta pruebas con `cargo test`.
	- `make fmt`   : formatea el código con `cargo fmt`.
	- `make clippy`: ejecuta `cargo clippy` con warnings como errores.
	- `make deploy WASM=path/to/wasm SOURCE=alice NETWORK=testnet` : despliega el WASM al network especificado.

Despliegue en Stellar
---------------------

Sigue estos pasos para desplegar un contrato en la red Stellar usando el CLI de Soroban:

1. Genera o importa una clave y fondéala (para `testnet`):

```bash
stellar keys generate --global alice --network testnet --fund
```

2. Construye el WASM optimizado (o usa `make build`):

```bash
make build
# o directamente
stellar contract build
```

3. Despliega el contrato con el `stellar` CLI (o `make deploy`):

```bash
# usando make (más cómodo)
make deploy WASM=target/wasm32-unknown-unknown/release/my_contract.wasm \
		SOURCE=alice NETWORK=testnet

# o usando el CLI directamente
stellar contract deploy \
	--wasm target/wasm32-unknown-unknown/release/my_contract.wasm \
	--source alice \
	--network testnet
```

4. (Opcional) Si tu contrato requiere inicialización con argumentos (constructor), invoca la función `__constructor` o tu `initialize` usando `stellar contract invoke`:

```bash
stellar contract invoke \
	--id CONTRACT_ID \
	--source alice \
	--network testnet \
	-- \
	__constructor --admin alice --initial_value 100
```

Notas
- Reemplaza `my_contract.wasm` por el nombre real del archivo WASM generado.
- Para despliegues en `mainnet`, cambia `NETWORK=mainnet` y asegúrate de usar claves y fondos adecuados


