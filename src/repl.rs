use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::error;
use crate::{parser::ast::parse_repl, transpiler::transpile::Transpiler};

pub struct Repl {}

impl Drop for Repl {
    fn drop(&mut self) {}
}

impl Default for Repl {
    fn default() -> Self {
        Repl::new()
    }
}

impl Repl {
    pub fn new() -> Self {
        Repl {}
    }

    pub fn repl_loop(&self) {
        let mut rl = Editor::<()>::new();

        if rl.load_history("history.txt").is_err() {}

        loop {
            let readline = rl.readline(">> ");

            match readline {
                Ok(line) => {
                    Repl::print(&Repl::eval(&line, Transpiler::default()));
                    rl.add_history_entry(line.as_str());
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history("history.txt").unwrap();
    }

    fn eval(input: &str, transpiler: Transpiler) -> String {
        let parsed = parse_repl(input);
        parsed
            .map(|ast| transpiler.transpile_ast(ast).trim().to_string())
            .map_err(|err| {
                error!(err.to_string());
                String::from("")
            })
            .unwrap_or_else(|_| String::from(""))
    }

    fn print(output: &str) {
        if output.is_empty() {
            print!("")
        } else {
            println!("{}", output)
        }
    }
}
