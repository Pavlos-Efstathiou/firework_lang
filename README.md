## Firework programming language

[![Test Status](https://github.com/Pavlos-Efstathiou/firework_lang/workflows/Rust/badge.svg?event=push)](https://github.com/Pavlos-Efstathiou/firework_lang/actions)
![Lines of Code](https://tokei.rs/b1/github/Pavlos-Efstathiou/firework_lang)
[![Crates.io Version](https://img.shields.io/crates/v/firework_lang)](https://crates.io/crates/firework_lang)
[![Crates.io Downloads](https://img.shields.io/crates/d/firework_lang)](https://crates.io/crates/firework_lang)
[![Crates.io License](https://img.shields.io/crates/l/firework_lang)](https://crates.io/crates/firework_lang)

Pure functional programming language that compiles to [LLVM IR](https://llvm.org/docs/LangRef.html) (Very incomplete, be warned)

## Build Guide

### 1. Installing Rust

- *nix:
	```sh
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	```
	
- Windows:
	Install [rustup-init](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe) and run it
  
### 2a. Installing Firework
```sh
cargo install firework_lang
```
### 2b. Updating Firework
```sh
cargo install firework_lang --force
```

### 2b. Building from Source
```sh
git clone https://github.com/Pavlos-Efstathiou/firework_lang
cd firework_lang
cargo build --release
```


### 3. Installing LLVM 12.x

- [*nix](https://llvm.org/docs/CMake.html)
- [Windows Binaries](https://github.com/PLC-lang/llvm-package-windows/releases/tag/v12.0.1)

## Quickstart

```sh
firework_lang new hello_world
cd hello_world
firework_lang run
```

### 4. Running the example

```sh
git clone https://github.com/Pavlos-Efstathiou/firework_lang
cd firework_lang/example
firework_lang run
```

## Upcoming features

- [ ] Algebraic Data Types
- [ ] Custom Data Types
- [ ] Dependency management
- [ ] Docs
