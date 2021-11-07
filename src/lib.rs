#[macro_use]
extern crate pest_derive;
pub mod firework_project;
pub mod parser;
pub mod repl;
pub mod transpiler;
pub mod typechecker;

#[cfg(test)]
mod tests {
    use crate::{
        parser::ast::{parse, parse_repl},
        transpiler::transpile::Transpiler,
    };

    #[test]
    fn generics() {
        let trans = Transpiler::new();

        trans.transpile_ast(parse("let id (a: b): b = a").unwrap());
    }

    #[test]
    fn anon_fn() {
        let trans = Transpiler::new();

        trans.transpile_ast(parse_repl("(): Int -> -1").unwrap());
    }

    #[test]
    fn enum_() {
        let trans = Transpiler::new();

        trans.transpile_ast(parse_repl("enum QubitState = Zero | One | Superposition").unwrap());
    }

    #[test]
    fn hello_world() {
        let trans = Transpiler::new();

        trans.transpile_ast(parse("let main: IO() = putStrLn \"Hello, World!\"").unwrap());
    }
}
