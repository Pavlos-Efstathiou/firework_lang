use clap::{App, AppSettings, Arg, SubCommand};

use firework_lang::firework_project::FireworkProject;
use firework_lang::repl::Repl;

fn main() {
    let clap_app = App::new("Firework")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("project").takes_value(true)))
        .subcommand(SubCommand::with_name("build").help("Builds a firework project"))
        .subcommand(SubCommand::with_name("run").help("Runs a firework project"))
        .subcommand(SubCommand::with_name("repl").help("Runs the firework repl"));

    let matches = clap_app.get_matches();
    let project = FireworkProject::new();

    match matches.subcommand() {
        ("run", _) => project.run().unwrap_or_else(|err| panic!("{}", err)),
        ("new", Some(matches)) => {
            if let Some(project_name) = matches.value_of("project") {
                project.new_project(project_name)
            }
        }
        ("repl", _) => Repl::new().repl_loop(),
        ("build", _) => project.build().unwrap_or_else(|err| panic!("{}", err)),
        _ => {}
    }
}
