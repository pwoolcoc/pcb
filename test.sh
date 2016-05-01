#! /bin/bash
./build.sh
cargo run --manifest-path pcb/Cargo.toml --example compile
rm test.o
