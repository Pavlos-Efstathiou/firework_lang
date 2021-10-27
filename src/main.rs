#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate pest;
use map::{DefinitionMap, Types};
use parser::ast;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::sync::Mutex;

mod map;
mod parser;

lazy_static! {
    static ref DEFINITIONS: Mutex<DefinitionMap> = {
        let mut definition_map = DefinitionMap::new();

        definition_map.insert(
            "version".to_string(),
            Types::String(env!("CARGO_PKG_VERSION").to_string()),
        );
        definition_map.insert(
            "author".to_string(),
            Types::String(env!("CARGO_PKG_AUTHORS").to_string()),
        );
        definition_map.insert("license".to_string(), Types::String("GPL-3.0".to_string()));
        Mutex::new(definition_map)
    };
}

fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("🎆 >> ");
        match readline {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(&line);
                    println!("{:?}", ast::parse(&line));
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
