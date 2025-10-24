# Namada Governance Upgrades

A comprehensive toolkit for building and managing WASM-based governance proposals for the Namada blockchain. This repository provides templates, build tools, and examples for creating governance proposals that can be executed through Namada's on-chain governance system.

## 📁 Repository Structure

### `proposals/mainnet_phases/`
Contains historical mainnet upgrade phases that have been deployed:
- `phase2/` - Phase 2 mainnet upgrade proposal
- `phase3/` - Phase 3 mainnet upgrade proposal  
- `phase4/` - Phase 4 mainnet upgrade proposal
- `phase5a/` - Phase 5a mainnet upgrade proposal
- `phase5b/` - Phase 5b mainnet upgrade proposal

These serve as real-world examples and reference implementations for complex governance proposals.

### `proposals/template/`
Template code for common governance proposal patterns:
- `update-wasm/` - Template for updating WASM code via governance
- `update-params/` - Template for updating chain parameters
- `whitelist-token/` - Template for whitelisting new tokens

Use these as starting points for new proposals.

### `proposals/mainnet/` & `proposals/testnet/`
Use these folders for development or to hold actual mainnet and testnet proposals

## 🔧 Building Proposals

### Local Development with Cargo

For quick local development and testing:

```bash
# Build a specific proposal locally
cargo build --package $PROPOSAL_NAME --release --target wasm32-unknown-unknown

# Example: Build the update-wasm proposal
cargo build --package update-wasm --release --target wasm32-unknown-unknown

# The compiled WASM will be in target/wasm32-unknown-unknown/release/
```

**Prerequisites:**
- Rust toolchain (see `rust-toolchain.toml` for version)
- `wasm32-unknown-unknown` target installed: `rustup target add wasm32-unknown-unknown`

### Production Builds with Earthly

For optimized, production-ready builds use Earthly (recommended for final proposals):

#### Using the Python Builder Script

```bash
# Build a specific proposal
python3 builder/build_proposals.py --directory proposals/template/update-wasm

# Enable debug output for troubleshooting
python3 builder/build_proposals.py --directory proposals/template/update-wasm --debug
```

#### Using Earthly Directly

```bash
# Build with specific rust version and package name
earthly --build-arg=RUST_VERSION=1.85.1 --build-arg=PACKAGE=update-wasm +build
```

**Prerequisites:**
- [Earthly](https://earthly.dev/) installed
- Docker installed and running

The Earthly build process:
1. Sets up the Rust environment with the specified version
2. Installs required dependencies (protobuf, clang, etc.)
3. Runs cargo clippy for linting
4. Builds the WASM with release optimizations
5. Applies `wasm-opt` for size optimization
6. Outputs optimized WASM files to `artifacts/wasms/`
7. Generates proposal JSON files in `artifacts/`

## 📦 Adding a New Proposal

### 1. Create a new cargo directory

```bash
# Create directory structure
cargo new --lib proposals/tobenamed/my-new-proposal
```

### 2. Implement proposal code

- Write code into `proposals/my-new-proposal/src/lib.rs`
- Edit `proposals/my-new-proposal/Cargo.toml` appropriately

### 4. Create data.json

- Create `proposals/mainnet/my-new-proposal/data.json`
- An example is [here](./proposals/template/update-wasm/data.json)

### 6. Build

You have several options here, which include building the wasm alone locally with `cargo`,
building the wasm and proposal locally with Earthly, or opening a PR and having the CI build the wasm and proposal with Earthly.

To build your proposal in GitHub Actions, add it to `.github/workflows/build.yml`:

```yaml
jobs:
  build:
    strategy:
      matrix:
        proposal:
          # ... existing proposals ...
          - path: proposals/mainnet/my-new-proposal
            id: my-new-proposal
```

## 🔍 Proposal Anatomy

Each proposal contains:

- **`Cargo.toml`** - Rust package configuration
- **`data.json`** - Proposal metadata and governance parameters
- **`src/lib.rs`** - The actual proposal implementation in Rust
- **Generated artifacts:**
  - `artifacts/wasms/*.wasm` - Optimized WASM bytecode
  - `artifacts/*.json` - Complete proposal JSON with embedded WASM data

## 🐛 Troubleshooting

- **"earthly command not found"**: Install [Earthly](https://earthly.dev/)
- **Rust version conflicts**: Check `rust-toolchain.toml` and ensure correct version
- **WASM target missing**: Run `rustup target add wasm32-unknown-unknown`
- **Build failures**: Use `--debug` flag with build_proposals.py for verbose output

## 📚 Additional Resources

- [Namada Documentation](https://docs.namada.net/)
- [Namada Governance Guide](https://docs.namada.net/users/governance)
- [Earthly Documentation](https://docs.earthly.dev/)