# Anchor Escrow — Q3 2026

A Solana smart contract project built with **Rust and Anchor Framework**, implementing a token vault and a token escrow system with on-chain state management, PDA-based account validation, SPL Token transfers, and LiteSVM integration tests.

## Overview

This project implements two major components:

1. **Vault Program** — Handles token deposits, withdrawals, and account closure.
2. **Escrow Program** — Enables two parties to exchange SPL tokens through a trust-minimized, PDA-controlled escrow mechanism.

The escrow program supports creating an escrow, executing a token exchange, refunding deposited tokens, and updating escrow terms.

## Tech Stack

- **Blockchain:** Solana
- **Smart Contracts:** Rust
- **Framework:** Anchor
- **Token Standard:** SPL Token
- **Testing:** LiteSVM
- **Build Tool:** Cargo / Anchor CLI

## Project Features

### 1. Vault Program

The vault program provides functionality for managing deposited tokens.

- **Withdraw:** Transfers tokens from the vault to the authorized user.
- **Close:** Closes the vault account and handles the remaining account balance according to the program logic.

### 2. Escrow Program

The escrow program facilitates token exchanges between a maker and a taker.

| Instruction | Description |
|---|---|
| `make` | Creates an escrow account and deposits Token A into the vault. |
| `take` | Allows a taker to exchange Token B for Token A. |
| `refund` | Returns deposited Token A to the maker. |
| `update` | Allows the maker to update the requested amount and expiration. |

### Escrow Workflow

1. The maker initializes an escrow with the required exchange terms.
2. Token A is transferred into a vault controlled by the escrow PDA.
3. A taker can execute the exchange by providing the requested Token B.
4. The escrow transfers Token A to the taker and closes the vault.
5. Alternatively, the maker can reclaim the deposited tokens through the refund instruction when the expiration condition is satisfied.

## Solana Architecture

The project uses Program Derived Addresses (PDAs) to manage escrow state and token custody.

### Escrow PDA

The escrow account is derived using:

```rust
[
    b"escrow",
    maker.key().as_ref(),
    seed.to_le_bytes().as_ref()
]
```

This provides a deterministic address for each maker and seed combination.

### Vault Authority

The escrow PDA acts as the authority of the associated token account holding Token A.

The program uses PDA signer seeds when transferring tokens from the vault.

### Token Transfers

SPL Token transfers are performed through Anchor's token interface and checked transfers, using mint decimals where required.

### Time-based Conditions

The escrow stores an expiration timestamp and uses Solana's Clock Sysvar to evaluate the expiration condition during escrow operations.

## Testing

The project uses **LiteSVM** to execute program instructions in a lightweight Solana virtual environment.

Current escrow integration tests:

| Test | Coverage |
|---|---|
| `test_make_and_refund` | Escrow creation and refund flow |
| `test_make_and_take` | Escrow creation and token exchange |
| `test_make_and_update` | Updating and verifying escrow terms |

All three escrow integration tests are passing.

## Getting Started

### Prerequisites

Install the following tools:

- Rust
- Solana CLI
- Anchor CLI
- Cargo

### Clone the Repository

```bash
git clone <YOUR_REPOSITORY_URL>
cd anchor-escrow-q2-2026
```

### Build the Programs

```bash
anchor build --arch v2
```

### Run Tests

Run all Rust tests:

```bash
cargo test
```

Run the escrow LiteSVM integration tests:

```bash
cargo test --test mod -- --nocapture
```

## Project Structure

```text
anchor-escrow-q2-2026/
│
├── programs/
│   └── anchor-escrow-q2-2026/
│       ├── src/
│       │   ├── instructions/
│       │   │   ├── make.rs
│       │   │   ├── take.rs
│       │   │   ├── refund.rs
│       │   │   └── update.rs
│       │   │
│       │   ├── constants.rs
│       │   ├── error.rs
│       │   ├── instructions.rs
│       │   ├── lib.rs
│       │   └── state.rs
│       │
│       ├── tests/
│       │   └── mod.rs
│       │
│       └── Cargo.toml
│
├── Anchor.toml
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Learning Objectives

This project explores:

- Solana account model and ownership.
- Anchor account validation and constraints.
- PDA derivation and PDA signing.
- Associated Token Accounts.
- SPL Token CPI operations.
- Account initialization and closure.
- On-chain state management.
- Clock Sysvar and timestamp-based conditions.
- Integration testing with LiteSVM.

## Current Status

- [x] Vault withdraw instruction
- [x] Vault close instruction
- [x] Escrow make instruction
- [x] Escrow take instruction
- [x] Escrow refund instruction
- [x] Escrow update instruction
- [x] LiteSVM integration tests for escrow instructions

## Author

**Yatharth Singh**

Solana | Rust | Anchor | Blockchain Development
