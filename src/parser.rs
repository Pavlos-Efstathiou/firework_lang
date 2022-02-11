//! Parser
#[derive(Parser)]
#[grammar = "firework.pest"]
pub(crate) struct FireworkParser;

use self::AstNode::*;
use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;

/// Represents a Firework program's Abstract Syntax Tree
pub type AST = Vec<AstNode>;

/// Represents an Abstract Syntax Tree's node
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Str(String),
    Int(i64),
    Char(char),
    Boolean(bool),
    List(Vec<AstNode>),
    InParens(Box<AstNode>),
    Type(String),
    FnArgs(Vec<(self::AstNode, self::AstNode)>),
    ModuleImport(Box<self::AstNode>),
    ModuleDeclaration(Box<self::AstNode>),
    Module(String),
    Identifier {
        name: String,
        args: Vec<self::AstNode>,
    },
    Enum {
        name: String,
        variants: Vec<self::AstNode>,
    },
    AnonFn {
        args: Box<self::AstNode>,
        return_type: Box<self::AstNode>,
        value: Box<self::AstNode>,
    },
    Fn {
        name: String,
        return_type: Box<self::AstNode>,
        args: Box<self::AstNode>,
        value: Box<self::AstNode>,
    },
    IfElse {
        condition: Box<self::AstNode>,
        stmt_true: Box<self::AstNode>,
        stmt_false: Box<self::AstNode>,
    },
    Eoi,
}

/// Parses a Firework program and transforms pest's output to a custom AST
pub fn parse(input: &str) -> Result<AST, Error<Rule>> {
    Ok(FireworkParser::parse(Rule::program, input)?
        .into_iter()
        .map(build_ast)
        .filter(|node| !matches!(node, Eoi))
        .collect::<AST>())
}

/// Builds a custom AST from pest's output
fn build_ast(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::name => Identifier {
            name: pair.as_str().to_string(),
            args: vec![],
        },
        Rule::firework_type => Type(pair.as_str().to_string()),
        Rule::type_signature => build_ast(pair.into_inner().next().unwrap()),
        Rule::int => Int(pair.as_str().parse().unwrap()),
        Rule::boolean => Boolean(pair.as_str().parse().unwrap()),
        Rule::string => Str(pair.as_str().to_string().replace("\"", "")),
        Rule::char => Char(pair.as_str().chars().nth(1).unwrap()),
        Rule::literal => build_ast(pair.into_inner().next().unwrap()),
        Rule::identifier => {
            let mut inner_pair = pair.into_inner();
            Identifier {
                name: inner_pair.next().unwrap().as_str().to_string(),
                args: inner_pair.map(build_ast).collect::<Vec<AstNode>>(),
            }
        }
        Rule::enum_type => {
            let mut inner_pair = pair.into_inner();

            Enum {
                name: inner_pair.next().unwrap().to_string(),
                variants: inner_pair.map(build_ast).collect::<Vec<_>>(),
            }
        }
        Rule::fn_args => {
            let mut args: Vec<AstNode> = vec![];
            let mut types: Vec<AstNode> = vec![];

            pair.into_inner().for_each(|x| match x.as_rule() {
                Rule::name => args.push(build_ast(x)),
                Rule::type_signature => types.push(build_ast(x)),
                _ => unreachable!(),
            });

            FnArgs(
                args.iter()
                    .cloned()
                    .zip(types.iter().cloned())
                    .collect::<Vec<(_, _)>>(),
            )
        }
        Rule::declaration => {
            let mut inner_pair = pair.into_inner();
            let name = inner_pair.next().unwrap();
            let args_or_type = inner_pair.next().unwrap();

            if let Rule::fn_args = args_or_type.as_rule() {
                Fn {
                    name: name.as_str().to_string(),
                    args: Box::new(build_ast(args_or_type)),
                    return_type: Box::new(build_ast(inner_pair.next().unwrap())),
                    value: Box::new(build_ast(inner_pair.next().unwrap())),
                }
            } else {
                Fn {
                    name: name.as_str().to_string(),
                    args: Box::new(AstNode::FnArgs(Vec::new())),
                    return_type: Box::new(build_ast(args_or_type)),
                    value: Box::new(build_ast(inner_pair.next().unwrap())),
                }
            }
        }
        Rule::anon_fn => {
            let mut inner_pair = pair.into_inner();
            let args_or_type = inner_pair.next().unwrap();

            if let Rule::fn_args = args_or_type.as_rule() {
                AnonFn {
                    args: Box::new(build_ast(args_or_type)),
                    return_type: Box::new(build_ast(inner_pair.next().unwrap())),
                    value: Box::new(build_ast(inner_pair.next().unwrap())),
                }
            } else {
                AnonFn {
                    args: Box::new(FnArgs(Vec::new())),
                    return_type: Box::new(build_ast(args_or_type)),
                    value: Box::new(build_ast(inner_pair.next().unwrap())),
                }
            }
        }
        Rule::module_name => Identifier {
            name: pair.as_str().to_string(),
            args: vec![],
        },
        Rule::module_import => ModuleImport(Box::new(Module(
            pair.into_inner().next().unwrap().as_str().to_string(),
        ))),
        Rule::module_declaration => ModuleDeclaration(Box::new(Module(
            pair.into_inner().next().unwrap().as_str().to_string(),
        ))),

        Rule::if_statement => {
            let mut inner_pair = pair.into_inner();
            IfElse {
                condition: Box::new(build_ast(inner_pair.next().unwrap())),
                stmt_true: Box::new(build_ast(inner_pair.next().unwrap())),
                stmt_false: Box::new(build_ast(inner_pair.next().unwrap())),
            }
        }
        Rule::repl => build_ast(pair.into_inner().next().unwrap()),
        Rule::precedence => InParens(Box::new(build_ast(pair.into_inner().next().unwrap()))),
        Rule::list => {
            todo!()
        }
        Rule::EOI => Eoi,
        _ => unreachable!(),
    }
}
