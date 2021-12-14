use inkwell::support::LLVMString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::codegen::CodeGen;
use crate::parser::ast::{parse, AST};
use crate::{info, unrecoverable_error};

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

pub struct FireworkProject<'ctx> {
    compiler: CodeGen<'ctx>,
}

#[allow(dead_code)]
impl<'ctx> FireworkProject<'ctx> {
    pub fn new(compiler: CodeGen<'ctx>) -> Self {
        Self { compiler }
    }

    pub fn new_project(&self, project_name: &str) {
        self.create_project(project_name)
            .unwrap_or_else(|err| unrecoverable_error!(err));
    }

    pub fn compile(&self) -> std::io::Result<()> {
        let main = fs::read_to_string("src/main.firework").unwrap_or_else(|_| {
            unrecoverable_error!("Couldn't read src/main.firework or project not found")
        });

        info!("Parsing");

        let parsed = parse(&main).unwrap();

        info!("Interpreting");

        self.compiler.compile(parsed);

        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.compile()?;
        unsafe {
            self.compiler.call_main();
        };
        Ok(())
    }

    fn create_project(&self, project_name: &str) -> std::io::Result<()> {
        fs::create_dir_all(project_name)?;

        fs::write(format!("{}/.gitignore", project_name), "ast.json\nir.ll")?;
        fs::create_dir_all(format!("{}/src", project_name))?;
        fs::write(
            format!("{}/src/main.firework", project_name),
            "let main: i64 = 0",
        )?;

        Ok(())
    }

    pub fn dump_ast(&self) -> Result<(), Box<dyn std::error::Error>> {
        let main = fs::read_to_string("src/main.firework").unwrap_or_else(|_| {
            unrecoverable_error!("Couldn't read src/main.firework or project not found")
        });

        fs::write(
            "ast.json",
            serde_json::to_string_pretty(&Program {
                ast: parse(&main).unwrap(),
            })
            .unwrap(),
        )?;

        info!("Writing AST to file");

        Ok(())
    }

    pub fn dump_ir(&self) -> Result<(), LLVMString> {
        self.compile().unwrap();
        self.compiler.dump_ir()
    }
}
