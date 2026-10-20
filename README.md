# MyElf
An ELF (Executable and Linking Format) library and tools in Rust


# Basic setup
Coverage tool
```
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
cargo llvm-cov
cargo llvm-cov --text
cargo llvm-cov --html
open target/llvm-cov/html/index.html
```