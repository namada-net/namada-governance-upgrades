# Namada Governance Upgrades

A repository and toolkit for using Rust code to build WASM files that are executable by Namada governance. 

## Overview

This repository contains two different classes of proposals. First, the proposals that were used to advance Namada through its [5-phase rollout](https://namada.net/mainnet-launch) of its mainnet launch are provided here for future reference.

Second, there are some template / example proposal codes that are useful for commonly desired governance actions, such as changing the values of protocol parameters, updating wasm hashes on-chain, increasing IBC rate limits, etc.

### Phase Progression Proposals

- [Phase 2](./phase2/): Enabled staking rewards and Public Goods Funding (PGF) inflation
- [Phase 3](./phase3/): Enabled IBC transfers, the MASP, and all transfer functionality of non-native tokens
- [Phase 4](./phase4/): Enabled shielded rewards for select incentivized assets
- [Phase 5a](./phase5a/): Enabled transfer of the native token internally within Namada only
- [Phase 5b](./phase5b/): Enabled transferrability of the native token over IBC out of Namada

### Other Proposals

- [Pre-Phase 4](./pre-phase4/): Prepared for Phase 4 shielding rewards by resetting MASP conversions and precision for various tokens
- [Increase Target Staked Ratio](./increase_target_staked_ratio/): Updates the proof-of-stake target staked ratio parameter. Template for updating any PoS parameter.
- [Update WASM Code](./update-wasm/): Template for updating WASM code hashes (for transactions or validity predicates) on-chain
- [Update IBC Rate Limits](./update_ibc_rate_limits/): Adjusts IBC transfer rate limits for specified tokens


## 🛠️ Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.85.1 with `wasm32-unknown-unknown` target
- [Earthly](https://earthly.dev/get-earthly) for containerized builds
- Python 3.6+ for proposal generation
- `protobuf-compiler` and `clang` for compilation

### Building WASM Artifacts

Generate all phase transition WASM files:

```bash
earthly +build
```

This creates an `artifacts/` directory containing optimized WASM binaries ready for governance proposals.

### Alternative: Local Build

```bash
# Install WASM target (if not already installed)
rustup target add wasm32-unknown-unknown

# Build all proposals
cargo build --release --target wasm32-unknown-unknown

# Manually optimize with wasm-opt (optional)
./docker/download-wasmopt.sh
./docker/run-wasmopt.sh
```

## 📋 Creating Governance Proposals

### Generate a Proposal

Use the proposal builder to create governance-ready JSON:

```bash
python3 builder/build_proposal.py -d <parameters_file> -o <output_file>
```

### Example: Phase 2 Proposal

```bash
python3 builder/build_proposal.py -d builder/parameters/phase2.json -o phase2_proposal.json
```

## 🔍 Verification & Validation

### Check On-Chain WASM Integrity

Verify that deployed WASM matches your local artifacts using the included verification tool:

```bash
cd check-onchain-wasm
cargo build --release

# Check a specific proposal
./target/release/check-onchain-wasm \
  --tendermint-url <RPC_URL> \
  --proposal-id $PROPOSAL_ID \
  [--expected-hash $HASH]
```

See [`check-onchain-wasm/README.md`](./check-onchain-wasm/README.md) for detailed usage.

### Development & Testing

Use Earthly for comprehensive development workflows:

```bash
# Run all checks (recommended)
earthly +all

# Individual targets:
earthly +test      # Run unit tests
earthly +fmt       # Check code formatting  
earthly +lint      # Run clippy lints
earthly +check     # Compilation check
```

Alternatively, use Cargo directly:

```bash
# Format code
cargo fmt

# Run lints  
cargo clippy --all-features --all-targets -- -D warnings

# Test changes
cargo test

# Check compilation
cargo check
```
## ⚠️ Important Notes

> **Parameter Values:** Current parameter values in template files are placeholders and subject to change based on network conditions and governance decisions.

> **Security:** Always verify WASM hashes and proposal content before voting. Use the provided verification tools.

> **Testing:** This code directly affects mainnet governance. Thoroughly test all changes in testnet environments.

## 📄 License

GPL-3.0 - See [LICENSE](LICENSE) for details.

---

**Maintainer:** Heliax AG <hello@heliax.dev>