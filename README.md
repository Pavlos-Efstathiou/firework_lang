## Firework programming language

[![Test Status](https://github.com/Pavlos-Efstathiou/firework_lang/workflows/Rust/badge.svg?event=push)](https://github.com/Pavlos-Efstathiou/firework_lang/actions)
[![Crates.io Version](https://img.shields.io/crates/v/firework_lang)](https://crates.io/crates/firework_lang)
[![Crates.io Downloads](https://img.shields.io/crates/d/firework_lang)](https://crates.io/crates/firework_lang)
[![Crates.io License](https://img.shields.io/crates/l/firework_lang)](https://crates.io/crates/firework_lang)

Programming language with accidental OCaml inspired syntax™

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

### 2b. Building from Source
```sh
git clone https://github.com/Pavlos-Efstathiou/firework_lang
cd firework_lang
cargo build --release
```


### 3. Installing GHC

#### [Instructions](https://www.haskell.org/platform/)

## Quickstart

```sh
firework_lang new hello_world
cd hello_world
firework_lang run
```

## Upcoming features

- [ ] Operators
- [ ] Algebraic Data Types
- [ ] An actually good transpiler
- [ ] Custom Data Types
- [ ] Dependency management
- [ ] Infix and prefix functions
- [ ] A Prelude written in Firework
- [ ] Type inference
- [ ] Operators
- [ ] Better parsing error messages
- [ ] Docs