# WASM Proposal Checker

Minor CLI tool to check that integrity of a WASM code associated with a proposal.

#### How to build

```
cd check-onchain-wasm
cargo build
```

## How to use

```
check-onchain-wasm --help                                                                                                             ok  at 18:46:11
Usage: check-onchain-wasm [OPTIONS] --tendermint-url <TENDERMINT_URL> --proposal-id <PROPOSAL_ID>

Options:
      --tendermint-url <TENDERMINT_URL>  [env: TENDERMINT_URL=]
      --proposal-id <PROPOSAL_ID>        [env: PROPOSAL_ID=]
      --expected-hash <EXPECTED_HASH>    [env: EXPECTED_HASH=]
      --dump-to <DUMP_TO>                [env: DUMP_TO=]
  -h, --help                             Print help
```