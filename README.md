# ink-contracts

Some toy contracts with ink! on substrate

  > Now substrate contract only support nightly rust compiler, see: <https://github.com/paritytech/cargo-contract#build-requires-the-nightly-toolchain>

## Run Node

```bash
./target/release/substrate-contracts-node --dev --tmp -lerror,runtime::contracts=debug
```

## Frontend UI

* <https://polkadot.js.org/apps/>
* <https://paritytech.github.io/canvas-ui/>


## Build

```bash
cd <contract_dir>
cargo +nightly build
```

## Test

```bash
cargo +nightly test
```
