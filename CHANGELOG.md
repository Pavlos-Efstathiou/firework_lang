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