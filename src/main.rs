#[macro_use]
extern crate pest_derive;
extern crate pest;
use pest::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Parser)]
#[grammar = "firework.pest"]
pub struct FireworkParser;

fn main() {
    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("🎆 >> ");
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str());
                    println!("{:?}", FireworkParser::parse(Rule::file, &line));
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
