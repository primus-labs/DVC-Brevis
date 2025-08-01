# DVC-Brevis


## Prerequisite

1. Install [Pico toolchains](https://pico-docs.brevis.network/getting-started/installation).

## Build

```sh
cd zktls/app
RUST_LOG=info cargo pico build

cd zktls/prover
RUST_LOG=info cargo build --release
```

## Run

```sh
cd zktls/prover
RUST_LOG=info cargo run --release
```

## localtest

```sh
bash ./build.sh
RUST_LOG=info ./target/release/zktls-prover \
  --elf ./zktls/app/elf/riscv32im-pico-zkvm-elf \
  --input ./zktls/prover/data/attestation_data.json \
  --output-dir pico_out
```
