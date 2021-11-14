use clap::{App, AppSettings, Arg, SubCommand};

use firework_lang::firework_project::FireworkProject;
use firework_lang::repl::Repl;
use firework_lang::unrecoverable_error;
use strsim::damerau_levenshtein;

const SUBCOMMANDS: [&str; 5] = ["new", "build", "run", "repl", "dump_ast"];

fn main() {
    let clap_app = App::new("Firework")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("project").takes_value(true)))
        .subcommand(SubCommand::with_name("build").help("Builds a firework project"))
        .subcommand(SubCommand::with_name("run").help("Runs a firework project"))
        .subcommand(SubCommand::with_name("repl").help("Runs the firework repl"))
        .subcommand(
            SubCommand::with_name("dump_ast").help("Dumps the AST of your code as JSON to a file"),
        );

    let matches = clap_app.get_matches();
    let project = FireworkProject::new();

    match matches.subcommand() {
        ("run", _) => project
            .run()
            .unwrap_or_else(|err| unrecoverable_error!(err)),
        ("new", Some(matches)) => {
            if let Some(project_name) = matches.value_of("project") {
                project.new_project(project_name)
            }
        }
        ("repl", _) => Repl::new().repl_loop(),
        ("build", _) => project
            .build()
            .unwrap_or_else(|err| unrecoverable_error!(err)),
        ("dump_ast", _) => project
            .dump_ast()
            .unwrap_or_else(|err| unrecoverable_error!(err)),
        (other, _) => {
            let words = SUBCOMMANDS
                .iter()
                .map(|command| damerau_levenshtein(other, command))
                .collect::<Vec<usize>>();
            let closest_word_index = words
                .iter()
                .position(|a| a == words.iter().min().unwrap())
                .unwrap();
            unrecoverable_error!(format!(
                "Subcommand `{}` doesn't exist! Did you mean `{}`?",
                other, SUBCOMMANDS[closest_word_index]
            ));
        }
    }
}
