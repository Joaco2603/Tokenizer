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

Notes & recommendations

- `.gitignore` should include `target/`, `.soroban/`, `.stellar/`, editor folders (`.vscode/`, `.idea/`) and OS files (`.DS_Store`).
- Use `soroban-sdk` test utilities (feature `testutils`) when writing unit tests that mock auth and environment.
- For token metadata and safe token operations rely on the `stellar-tokens` base implementations and guard flows with `only_owner` / `when_not_paused` macros.
