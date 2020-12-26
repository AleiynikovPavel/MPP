#!/bin/bash
cd rust_lib
cargo clean
RUSTFLAGS="--emit asm -C no-vectorize-loops -C no-vectorize-slp" cargo build --release
cp target/release/liblibmpp.so ../java/
cd ../java/src
javac com/company/RustJni.java
java -Djava.library.path=../ com.company.RustJni
