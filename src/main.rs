#[macro_use]
extern crate pest_derive;
extern crate pest;
use parser::ast;
use rustyline::error::ReadlineError;
use rustyline::Editor;

mod parser;

fn main() {
    let mut rl = Editor::<()>::new();

    rl.load_history("history.txt")
        .unwrap_or_else(|_| println!("Coudn't run history"));

    loop {
        let readline = rl.readline("🎆 >> ");
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(&line);
                    println!("{:#?}", ast::parse(&line));
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
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
