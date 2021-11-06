# Changelog
All noteworth changes to this project will be mentioned in this file.

[Latest Version](#v0.2.0)

## v0.1.1
### Changes
- Added [rustyline](https://crates.io/crates/rustyline/) to dependencies
- Added lexer REPL
- `Lexer::new` accepts an `&str` instead of a `&Cow<str>`
- Rewrote `Lexer::handle_multichar_tokens`
- Added the `Lexer::lex` and `Lexer::next_char` methods
- Added another test
- Followed Clippy's sugestions

## v0.1.2
### Changes 
- Added benches and configures tests properly
- Made `Lexer::lex` more functional
- Minor changes

## v0.1.3
### Changes
- Switched to pest for parsing and lexing

# v0.1.4
### Changes
- Added a `DefinitionMap` type for storing function definitions
- You can access global function definitions (sort of)
- Added types
- Added negative ints
- Changed the syntax for declarations
- Changed the rules for strings and chars
- Added anonymous functions

# v0.1.5
### Changes
- Removed DefinitionMap
- Added a custom AST
- The ```parser``` and ```compiler``` crates
- Remove ```lazy_static``` ```Cargo.toml```'s dependencies
- Added ```inkwell``` to ```Cargo.toml```'s dependencies
- Switched to Rust 2021

# v0.2.0
### Changes
- Added a primitive build system
- Added Enums, If Statements, Module Imports and Declarations, Function Arguments
- Added a ~~```compiler```~~ ```transpiler``` crate
- Added some tests
- Remove ```rustyline``` and ```inkwell``` from this projects dependencies
- Many more changes!