use clap::{App, AppSettings, Arg, SubCommand};

use firework_lang::codegen::CodeGen;
use firework_lang::firework_project::FireworkProject;
use firework_lang::{todo_feature, unrecoverable_error};
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::error::Error;
use strsim::damerau_levenshtein;

const SUBCOMMANDS: [&str; 5] = ["new", "build", "dump_ast", "dump_ir", "repl"];

fn main() -> Result<(), Box<dyn Error>> {
    let clap_app = App::new("Firework")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("project").takes_value(true)))
        .subcommand(SubCommand::with_name("run").help("Runs a firework project"))
        .subcommand(
            SubCommand::with_name("dump_ast").help("Dumps the AST of your code as JSON to a file"),
        )
        .subcommand(SubCommand::with_name("dump_ir").help("Dumps LLVM's output to ir.ll"))
        .subcommand(SubCommand::with_name("repl").help("Runs the firework repl"));

    let matches = clap_app.get_matches();
    let context = Context::create();
    let module = context.create_module("main");
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();

    let codegen = CodeGen::new(&context, module, context.create_builder(), execution_engine);
    let project = FireworkProject::new(codegen);

    match matches.subcommand() {
        ("new", Some(matches)) => {
            if let Some(project_name) = matches.value_of("project") {
                project.new_project(project_name)
            } else {
                unrecoverable_error!("No project name supplied!")
            }
        }
        ("run", _) => project.run()?,
        ("dump_ast", _) => project.dump_ast()?,
        ("dump_ir", _) => project.dump_ir()?,
        ("repl", _) => todo_feature!("The REPL"),
        (other, _) => {
            if !other.chars().map(|c| c.is_numeric()).all(|x| x) {
                let words: Vec<usize> = SUBCOMMANDS
                    .iter()
                    .map(|command| damerau_levenshtein(other, command))
                    .collect();

                let closest_word_index: usize = words
                    .iter()
                    .position(|a| a == words.iter().min().unwrap())
                    .unwrap();

                unrecoverable_error!(format!(
                    "Subcommand `{}` doesn't exist! Did you mean `{}`?",
                    other, SUBCOMMANDS[closest_word_index]
                ));
            } else {
                unrecoverable_error!(format!("Subcommand `{}` doesn't exist!", other));
            }
        }
    }
    Ok(())
}
