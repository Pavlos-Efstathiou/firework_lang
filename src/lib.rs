#[macro_use]
extern crate pest_derive;
pub mod firework_project;
pub mod parser;
pub mod repl;
pub mod transpiler;

#[macro_export]
macro_rules! info {
    ( $( $info:expr ),+ ) => {{
        $(
            println!("[INFO] {}", $info);
        )*
    }};

    () => {};
}

#[macro_export]
macro_rules! warn {
    ( $( $warnings:expr ),+ ) => {{
        use colored::Colorize;
        $(
            println!("[{}] {}", "WARNING".yellow(), $warnings);
        )*
    }};

    () => {};
}
#[macro_export]
macro_rules! error {
    ( $( $errors:expr ),+ ) => {{
        use colored::Colorize;
        $(
            println!("[{}] {}", "ERROR".red(), $errors);
        )*
    }};

    () => {};
}

#[macro_export]
macro_rules! unrecoverable_error {
    ( $( $errors:expr ),+ ) => {{
        use colored::Colorize;
        $(
            println!("[{}] {}", "ERROR".red(), $errors);
        )*
        panic!();
    }};

    () => {};
}

#[macro_export]
macro_rules! todo_feature {
    ( $( $features:expr ),+ ) => {{
        $(
            crate::unrecoverable_error!(format!("{} is not implemented yet!", $features));
        )*
    }}
}

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
