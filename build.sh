#! /bin/bash
cargo build || exit
cargo build --example compile

clang -std=c11 -Wall -Wextra -pedantic -Werror -c -o compile.o -I include \
  examples/compile.c || exit
clang++ -o compile `llvm-config --cxxflags --ldflags --libs all --system-libs` \
  target/debug/libpcb.a compile.o || exit

rm compile.o
