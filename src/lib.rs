#[macro_use]
extern crate pest_derive;
pub mod codegen;
pub mod core;
pub mod firework_project;
pub mod parser;

/// Used when code encounters an unrecoverable error.
///
/// Displays all errors given then panics
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

/// Used as a placeholder for unfinished code.
///
/// Calls the ```unrecoverable_error``` macro for every to-do feature given
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
    mod parser_tests {}
    mod codegen_tests {}
}
