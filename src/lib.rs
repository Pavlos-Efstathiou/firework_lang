#[macro_use]
extern crate pest_derive;
pub mod codegen;
pub mod firework_project;
pub mod parser;

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
mod tests {}
