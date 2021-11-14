use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::{env, fs};
use toml::de::Error;

use crate::parser::ast::{parse, AST};
use crate::transpiler::transpile::Transpiler;
use crate::{info, todo_feature, unrecoverable_error};

#[derive(Deserialize, Debug)]
struct Config {
    project: Project,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
    version: String,
    author: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Program {
    ast: AST,
}

pub struct FireworkProject {}

impl Default for FireworkProject {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl FireworkProject {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_config(&self, _config_file_contents: &str) -> Result<Config, Error> {
        todo_feature!("parsing TOML configs")
    }

    pub fn new_project(&self, project_name: &str) {
        self.create_project(project_name)
            .unwrap_or_else(|err| unrecoverable_error!(err));
    }

    pub fn build(&self) -> std::io::Result<()> {
        let main = fs::read_to_string("src/main.firework").unwrap_or_else(|_| {
            unrecoverable_error!("Couldn't read src/main.firework or project not found")
        });
        let transpiler = Transpiler::default();

        fs::create_dir_all("build")?;

        info!("Parsing");
        let parsed = parse(&main).unwrap();

        info!("Building");

        fs::write(
            "build/Main.hs",
            formatdoc! {
                "module Main where

                {transpiled}", 
                transpiled = transpiler.transpile_ast(parsed)
            },
        )?;
        transpiler.compile();

        Ok(())
    }

    pub fn run(&self) -> std::io::Result<()> {
        self.build().unwrap_or_else(|err| unrecoverable_error!(err));
        env::set_current_dir("build")?;

        if cfg!(windows) {
            Command::new("Main.exe")
                .status()
                .unwrap_or_else(|err| unrecoverable_error!(err));
        } else {
            Command::new("./Main")
                .status()
                .unwrap_or_else(|err| unrecoverable_error!(err));
        };

        Ok(())
    }

    fn create_project(&self, project_name: &str) -> std::io::Result<()> {
        fs::create_dir_all(project_name)?;

        fs::write(
            format!("{}/config.toml", project_name),
            formatdoc! {
                "[project]
                name = \"{project_name}\"
                version = \"{version_number}\"

                [dependencies]
                ",
                project_name = project_name,
                version_number = "0.1.0"
            },
        )?;

        fs::write(format!("{}/.gitignore", project_name), "/build")?;
        fs::create_dir_all(format!("{}/src", project_name))?;
        fs::write(
            format!("{}/src/main.firework", project_name),
            "let main: IO() = putStrLn \"Hello, World!\"",
        )?;

        Ok(())
    }

    pub fn dump_ast(&self) -> Result<(), Box<dyn std::error::Error>> {
        let main = fs::read_to_string("src/main.firework").unwrap_or_else(|_| {
            unrecoverable_error!("Couldn't read src/main.firework or project not found")
        });

        fs::write(
            "build/ast.json",
            serde_json::to_string_pretty(&Program {
                ast: parse(&main).unwrap(),
            })
            .unwrap(),
        )?;

        info!("Writing AST to file");

        Ok(())
    }
}
