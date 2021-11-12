use std::process::Command;

use indoc::formatdoc;

use crate::{
    info,
    parser::ast::{AstNode, AST},
    unrecoverable_error,
};

impl Drop for Transpiler {
    fn drop(&mut self) {}
}
#[derive(Debug)]
pub struct Transpiler {}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transpiler {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_args(&self, args: Vec<(self::AstNode, self::AstNode)>) -> (Vec<String>, Vec<String>) {
        args.iter()
            .cloned()
            .fold((Vec::new(), Vec::new()), |mut vec, (name, return_type)| {
                vec.0.push(self.transpile_ast_node(name));
                vec.1.push(self.transpile_ast_node(return_type));
                vec
            })
    }

    fn transpile_ast_node(&self, ast_node: AstNode) -> String {
        match ast_node {
            AstNode::Str(x) => format!("{:?}", x),
            AstNode::Eoi => "".to_string(),
            AstNode::Boolean(x) => {
                let to_string = x.to_string();
                let mut chars = to_string.chars();

                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            }
            AstNode::Char(x) => format!("{:?}", x),
            AstNode::Type(x) => x,
            AstNode::Int(x) => {
                if x.is_negative() {
                    format!("({})", x)
                } else {
                    x.to_string()
                }
            }
            AstNode::Module(x) => x,
            AstNode::ModuleDeclaration(x) => {
                format!("module {} where", self.transpile_ast_node(*x))
            }
            AstNode::ModuleImport(x) => format!("import {}", self.transpile_ast_node(*x)),
            AstNode::Identifier { name, args } => format!(
                "{} {}",
                name,
                args.iter()
                    .cloned()
                    .map(|node| self.transpile_ast_node(node))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            AstNode::Fn {
                name,
                return_type,
                args,
                value,
            } => {
                let (names, types) = if let AstNode::FnArgs(args) = *args {
                    self.handle_args(args)
                } else {
                    unreachable!()
                };

                formatdoc! {
                    "{name} :: {types} {return_type}
                    {name} {arg_names} = {value}",
                    return_type = self.transpile_ast_node(*return_type),
                    value = self.transpile_ast_node(*value),
                    types = if types.is_empty() {"".to_string()} else {types.join(" ") + " ->"},
                    arg_names = names.join(" "),
                    name = name,
                }
            }
            AstNode::AnonFn {
                args,
                return_type,
                value,
            } => {
                let (names, _) = if let AstNode::FnArgs(args) = *args {
                    self.handle_args(args)
                } else {
                    unreachable!()
                };

                formatdoc! {
                    "\\{args} -> ({return_value} :: {return_type})",
                    args = if names.is_empty() { "()".to_string() } else { names.join(" ") },
                    return_type = self.transpile_ast_node(*return_type),
                    return_value = self.transpile_ast_node(*value),
                }
            }
            AstNode::Enum { name, variants } => {
                formatdoc! {
                    "data {name} = {variants}",
                    name = self.transpile_ast_node(*name),
                    variants = variants.into_iter().map(|node| self.transpile_ast_node(node)).collect::<Vec<_>>().join("|"),
                }
            }
            AstNode::IfElse {
                condition,
                stmt_false,
                stmt_true,
            } => {
                formatdoc!(
                    "if {condition} then {stmt_true} else {stmt_false}",
                    condition = self.transpile_ast_node(*condition),
                    stmt_true = self.transpile_ast_node(*stmt_true),
                    stmt_false = self.transpile_ast_node(*stmt_false)
                )
            }
            _ => todo!(),
        }
    }

    pub fn transpile_ast(&self, ast: AST) -> String {
        info!("Transpiling");
        ast.iter()
            .cloned()
            .map(|node| self.transpile_ast_node(node) + "\n")
            .collect::<String>()
            .trim()
            .to_string()
    }

    pub fn compile(&self) {
        info!("Running GHC");
        Command::new("ghc")
            .args(["build/Main.hs"])
            .status()
            .unwrap_or_else(|err| unrecoverable_error!("{}", err));
    }
}
