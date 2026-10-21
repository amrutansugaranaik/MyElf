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

# Things not supported deliberately.
- ABI version field and padding field are always ignored. If any implementation requires them, the library cannot be used.


# To check later
- In elf identifier, will this line work?
result[4] = identifier.class.into();

- 