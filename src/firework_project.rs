extern crate inkwell_llvm12 as inkwell;

use indoc::formatdoc;
use inkwell::support::LLVMString;
use std::fs::{self, File};
use std::io::BufRead;
use std::io::BufReader;

use crate::codegen::CodeGen;
use crate::parser::ast::parse;
use crate::unrecoverable_error;

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
            .unwrap_or_else(|err| unrecoverable_error!(err.to_string()));
    }

    pub fn compile(&self) -> std::io::Result<()> {
        let main = fs::read_to_string("src/main.firework").unwrap_or_else(|_| {
            unrecoverable_error!("Couldn't read src/main.firework or project not found")
        });

        let parsed = parse(&main).unwrap_or_else(|err| {
            let col = match err.line_col {
                pest::error::LineColLocation::Pos(a) => a,
                _ => unreachable!(),
            };

            let location = match err.location {
                pest::error::InputLocation::Pos(a) => a,
                _ => unreachable!(),
            };

            let file = File::open("src/main.firework").unwrap();

            let reader = BufReader::new(file);
            let line = reader
                .lines()
                .enumerate()
                .filter(|(index, _)| index == &(col.0 - 1))
                .map(|(_, line)| line.unwrap())
                .collect::<String>();

            unrecoverable_error!(formatdoc!(
                "Syntax error at src/main.firework:{line_number}:{col}
                {empty_space}| 
                {empty_space}| {line}
                {empty_space}|
                ",
                line_number = col.0,
                col = location,
                line = line,
                empty_space = "  ",
            ));
        });

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
            "let main: i64 = printf \"Hello World!\n\"",
        )?;

        Ok(())
    }

    pub fn dump_ir(&self) -> Result<(), LLVMString> {
        self.compile().unwrap();
        self.compiler.dump_ir()
    }

    pub fn dump_asm(&self) -> Result<(), LLVMString> {
        self.compile().unwrap();
        self.compiler.dump_asm()
    }
}
