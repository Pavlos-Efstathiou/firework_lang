# Changelog
All noteworth changes to this project will be mentioned in this file.

[Latest Version](#v0.3.1)

# v0.1.1
### Changes
- Added [rustyline](https://crates.io/crates/rustyline/) to dependencies
- Added lexer REPL
- `Lexer::new` accepts an `&str` instead of a `&Cow<str>`
- Rewrote `Lexer::handle_multichar_tokens`
- Added the `Lexer::lex` and `Lexer::next_char` methods
- Added another test
- Followed Clippy's sugestions

# v0.1.2
### Changes 
- Added benches and configures tests properly
- Made `Lexer::lex` more functional
- Minor changes

# v0.1.3
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

# v0.2.1
### Changes
- Added keywords to Cargo.toml
- Updated README.MD

# v0.2.2
### Changes
- Added the repl subcommands which runs a REPL (Read-Eval-Print Loop)
- Added the build subcommand which build a project
- The run subcommand now builds and runs a project
- Fixed transpiling negative numbers
- Renamed build_system to firework_project
- Added more tests
- Switched back to Rust 2018

# v0.2.3
### Changes
- Added support for transpiling if statements, negative numbers and booleans
- Added some macros for logging
- Implemented the ```Drop``` trait for the ```Transpiler``` and ```Repl``` structs
- Changed some rules in ```src/firework.pest```
- Added ```colored``` and ```serde``` to this project's dependencies
- Improved the example a bit

# v0.2.4
### Changes
- Replaced all occurrences of ```unrecoverable_error!("{}", err)``` with ```unrecoverable_error(err)```
- Added the ```dump_ast`` subcommand which dumbs your codes AST as JSON to a file
- Added ```serde_json``` to this project's dependencies
- Added the ```dump_ast``` method to the ```FireworkProject```

# v0.2.5
### Changes
- Added suggesting subcommands when an unknown subcommand is supplied
- In the logging macros only the logging levels are colored
- ```FireworkProject::dump_ast``` creates the ```build/``` directory if it doesn't exist
- Added ```strsim``` to this project's dependencies


# v0.3.0
### Changes
- Removed the transpiler and added a compiler (Incomplete!) to LLVM IR and an interpreter

# v0.3.1
### Changes
- Added expressions in parenthenses
- Added better parsing error messages
- Added dumping assembly of code
- Changed the logging macros just a bit
- Added compiling if statements
- Changed strings and chars from vectors to ```i8``` pointers
- Added the ```==```, ```+```, ```-```, ```*``` and ```/``` functions
- Removed the ```warn``` and ```info``` macros
- Removed dumping the AST of code