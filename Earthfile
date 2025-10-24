VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

ARG RUST_VERSION
ARG PACKAGE

install:
  ARG RUST_VERSION
  FROM rust:${RUST_VERSION}-bookworm

  RUN apt-get update && apt-get install -y protobuf-compiler build-essential clang-tools-14

  RUN rustup component add clippy rustfmt
  RUN rustup target add wasm32-unknown-unknown

  DO rust+INIT --keep_fingerprints=true

source:
  FROM +install

  COPY --keep-ts Cargo.toml Cargo.lock ./
  COPY --keep-ts --chmod 755 docker/run-wasmopt.sh ./run-wasmopt.sh
  COPY --keep-ts --chmod 755 docker/download-wasmopt.sh ./download-wasmopt.sh
  COPY --keep-ts --dir proposals ./

# lint runs cargo clippy on the source code
lint:
  FROM +source
  DO rust+CARGO --args="clippy --all-features --all-targets -- -D warnings"

# compilation check
check:
  FROM +lint
  DO rust+CARGO --args="check"

# builds with the Cargo release profile
build:
  ARG PACKAGE

  FROM +lint
  DO rust+CARGO --args="build --package ${PACKAGE} --release --target wasm32-unknown-unknown" --output="wasm32-unknown-unknown\/release\/[a-zA-Z_1-9]+\.wasm"
  RUN ./download-wasmopt.sh
  RUN ./run-wasmopt.sh
  SAVE ARTIFACT ./target/wasm32-unknown-unknown/release AS LOCAL artifacts/wasms

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