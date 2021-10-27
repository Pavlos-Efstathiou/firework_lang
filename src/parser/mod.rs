pub mod ast;

#[derive(Parser)]
#[grammar = "firework.pest"]
pub(crate) struct FireworkParser;
