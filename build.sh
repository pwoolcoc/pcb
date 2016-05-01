#! /bin/bash
cargo build --manifest-path pcb-core/Cargo.toml || exit
cargo build --manifest-path pcb/Cargo.toml || exit
cargo build --manifest-path pcb/Cargo.toml --example compile

cargo build --manifest-path pcb-c/Cargo.toml || exit
clang -std=c11 -Wall -Wextra -pedantic -Werror -c -o compile.o \
  -I pcb-c/include pcb-c/examples/compile.c || exit
clang++ -o compile `llvm-config --cxxflags --ldflags --libs all --system-libs` \
  pcb-c/target/debug/libpcb_c.a compile.o || exit

rm compile.o
