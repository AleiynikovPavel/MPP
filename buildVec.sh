#!/bin/bash
cd rust_lib
cargo clean
RUSTFLAGS="--emit asm" cargo build --release
cp target/release/liblibmpp.so ../java/
cd ../java/src
javac com/company/RustJni.java
java -Djava.library.path=../ com.company.RustJni
