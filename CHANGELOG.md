# Changelog
All noteworth changes to this project will be mentioned in this file.

## v0.1.1 | 17/10/2021
### Changes
- Added [rustyline](https://crates.io/crates/rustyline/) to dependencies
- Added lexer REPL
- `Lexer::new` accepts an `&str` instead of a `&Cow<str>`
- Rewrote `Lexer::handle_multichar_tokens`
- Added the `Lexer::lex` and `Lexer::next_char` methods
- Added another test
- Followed Clippy's sugestions

## v0.1.2 | 18/10/2021
### Changes 
- Added benches and configures tests properly
- Made `Lexer::lex` more functional
- Minor changes

## v0.1.3 | 18/10/2021
### Changes
- Switched to pest for parsing and lexing