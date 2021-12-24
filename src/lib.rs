#[macro_use]
extern crate pest_derive;
pub mod codegen;
pub mod firework_project;
pub mod parser;

#[macro_export]
macro_rules! error {
    ( $( $errors:expr ),+ ) => {{
        use colored::Colorize;
        $(
            println!("{}: {}", "error".red(), $errors.white());
        )*
    }};

    () => {};
}

#[macro_export]
macro_rules! unrecoverable_error {
    ( $( $errors:expr ),+ ) => {{
        use colored::Colorize;
        $(
            println!("{}: {}", "error".red().bold(), $errors.white());
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
mod tests {}
