#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
extern crate pest;
use map::{DefinitionMap, Types};
use pest::error::Error;
use pest::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use std::sync::Mutex;
pub mod map;

lazy_static! {
    static ref DEFINITIONS: Mutex<DefinitionMap<'static>> = {
        let mut definition_map = DefinitionMap::new();

        definition_map.insert(
            "version".to_string(),
            Types::String(env!("CARGO_PKG_VERSION")),
        );
        definition_map.insert(
            "author".to_string(),
            Types::String(env!("CARGO_PKG_AUTHORS")),
        );
        definition_map.insert("license".to_string(), Types::String("GPL-3.0"));
        Mutex::new(definition_map)
    };
}

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
                    || -> Result<(), Error<Rule>> {
                        let pairs = FireworkParser::parse(Rule::file, &line)?;
                        for pair in pairs {
                            if let Some(x) =
                                DEFINITIONS.lock().unwrap().get(pair.as_str().to_string())
                            {
                                match x {
                                    Types::String(x) => println!("{}", x),
                                    Types::Int(x) => println!("{}", x),
                                    Types::Char(x) => println!("{}", x),
                                    Types::Boolean(x) => println!("{}", x),
                                }
                            } else {
                                println!("{:?}", pair);
                            }
                        }

                        Ok(())
                    }()
                    .unwrap_or_else(|x| println!("{}", x));
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
