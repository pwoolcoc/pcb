#! /bin/bash
./build.sh || exit
echo
echo "=== pcb ==="
echo
cargo run --manifest-path pcb/Cargo.toml --example compile
echo
echo "===  pcb-c  ==="
echo
./compile || exit
rm test.o
