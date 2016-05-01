#! /bin/bash
./build.sh
echo
echo "=== pcb ==="
echo
cargo run --manifest-path pcb/Cargo.toml --example compile
rm test.o
echo
echo "===  pcb-c  ==="
echo
./compile
rm test.o
