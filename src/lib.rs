#[macro_use]
extern crate pest_derive;
pub mod build_system;
pub mod parser;
pub mod transpiler;

#[cfg(test)]
mod tests {
    use crate::{parser::ast::parse, transpiler::transpile::Transpiler};

    #[test]
    fn generics() {
        let trans = Transpiler::new();

        trans.transpile_ast(parse("let id (a: b): b = a").unwrap());
    }
}
