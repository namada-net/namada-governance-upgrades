VERSION --global-cache 0.7

IMPORT github.com/earthly/lib/rust AS rust

install:
  FROM rust:1.78.0-bookworm
  RUN rustup component add clippy rustfmt
  RUN rustup target add wasm32-unknown-unknown

  # Call +INIT before copying the source file to avoid installing function depencies every time source code changes
  # This parametrization will be used in future calls to functions of the library
  DO rust+INIT --keep_fingerprints=true
  RUN apt-get update && apt-get install -y protobuf-compiler build-essential clang-tools-14

source:
  FROM +install
  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --chmod 755 docker/run-washopt.sh ./run-washopt.sh
  COPY --keep-ts --chmod 755 docker/download-wasmopt.sh ./download-wasmopt.sh
  COPY --keep-ts --dir block_party shielding_party staking_party shielding_reward_party  ./

# lint runs cargo clippy on the source code
lint:
  FROM +source
  DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

# compilation check
check:
  FROM +lint
  DO rust+CARGO --args="check"

# build builds with the Cargo release profile
build:
  FROM +lint
  DO rust+CARGO --args="build --release --target wasm32-unknown-unknown" --output="wasm32-unknown-unknown\/release\/[a-zA-Z_]+\.wasm"
  RUN ./download-wasmopt.sh
  RUN ./run-washopt.sh
  SAVE ARTIFACT ./target/wasm32-unknown-unknown/release AS LOCAL artifacts

# test executes all unit and integration tests via Cargo
test:
  FROM +lint
  DO rust+CARGO --args="test"

# fmt checks whether Rust code is formatted according to style guidelines
fmt:
  FROM +lint
  DO rust+CARGO --args="fmt --check"

# all runs all other targets in parallel
all:
  BUILD +build
  BUILD +test
  BUILD +fmt
  BUILD +check