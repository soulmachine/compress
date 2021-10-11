# crypto-cli-tools

A multi-threaded compress tool written in Rust.

## How to build

First, install `rust`,

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly`

Second, build,

`cargo build`

Third, run,

`./target/debug/compress example.txt example.txt.xz`
