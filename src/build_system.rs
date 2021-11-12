use indoc::formatdoc;
use std::fs;

use crate::parser::ast::parse;
use crate::transpiler::transpile::Transpiler;

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
    pub fn new_project(&self, project_name: &str) {
        self.create_project(project_name)
            .unwrap_or_else(|err| println!("{}", err));
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Running src/main.firework");

        let main = fs::read_to_string("src/main.firework")
            .unwrap_or_else(|_| panic!("Couldn't read src/main.firework or project not found"));
        let transpiler = Transpiler::default();

        fs::create_dir_all("build")?;

        let parsed = parse(&main).unwrap();

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
}
