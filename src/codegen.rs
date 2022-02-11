//! Compiler
extern crate inkwell_llvm12 as inkwell;

use std::borrow::Borrow;
use std::fs::{self};

use crate::core::{get_core_path, install_core, is_core_installed};
use crate::parser::{parse, AstNode, AST};
use crate::{todo_feature, unrecoverable_error};
use inkwell::attributes::AttributeLoc;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::{Linkage, Module};
use inkwell::support::LLVMString;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::{BasicMetadataTypeEnum, StringRadix};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};

macro_rules! any_value_enum_to_basic_value_enum {
    ( $x:expr ) => {
        match $x {
            AnyValueEnum::IntValue(x) => BasicValueEnum::IntValue(x),
            AnyValueEnum::PointerValue(x) => BasicValueEnum::PointerValue(x),
            a => todo_feature!(format!("Type `{:?}` not implemented or doesn't exist", a)),
        }
    };

    () => {};
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    core: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    #[inline]
    pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    pub fn new(
        context: &'ctx Context,
        module: Module<'ctx>,
        builder: Builder<'ctx>,
        execution_engine: ExecutionEngine<'ctx>,
    ) -> Self {
        let core = context.create_module("core");

        CodeGen {
            context,
            module,
            builder,
            execution_engine,
            core,
        }
    }

    fn available_name(&self, name: u8) -> String {
        let name_as_string = name.to_string();

        if self.module.get_global(&name_as_string).is_some() {
            self.available_name(name + 1)
        } else {
            name_as_string
        }
    }

    fn compile_astnode(&self, node: AstNode) -> Result<AnyValueEnum<'ctx>, &'static str> {
        // TODO: anonymous functions, module imports and declarations, enums and unescaping strings
        match node {
            AstNode::Int(int) => Ok(self
                .context
                .i64_type()
                .const_int_from_string(&int.to_string(), StringRadix::Decimal)
                .unwrap()
                .into()),
            AstNode::Str(str) => Ok(unsafe {
                self.builder
                    .build_global_string(&(str + "\0"), &self.available_name(0))
                    .as_pointer_value()
                    .into()
            }),
            AstNode::Boolean(bool) => Ok(self
                .context
                .bool_type()
                .const_int(bool as u64, false)
                .into()),
            AstNode::Char(char) => Ok(unsafe {
                self.builder
                    .build_global_string(&(char.to_string() + "\0"), &self.available_name(0))
                    .as_pointer_value()
                    .into()
            }),
            AstNode::Fn {
                name,
                return_type,
                args,
                value,
            } => Ok(self
                .compile_function(AstNode::Fn {
                    name,
                    return_type,
                    args,
                    value,
                })
                .into()),
            AstNode::IfElse {
                condition,
                stmt_false,
                stmt_true,
            } => {
                let bool_true = self.context.bool_type().const_int(1, false);

                let value = self.compile_astnode(*stmt_true).unwrap();

                let cond = self.compile_astnode(*condition).unwrap();

                let cond = self.builder.build_int_compare(
                    IntPredicate::EQ,
                    cond.into_int_value(),
                    bool_true,
                    "ifcond",
                );

                let else_branch = self.compile_astnode(*stmt_false).unwrap();

                let last_function = self.module.get_last_function().unwrap();

                let then_bb = self.context.append_basic_block(last_function, "then");
                let else_bb = self.context.append_basic_block(last_function, "else");
                let cont_bb = self.context.append_basic_block(last_function, "ifcont");

                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb);

                self.builder.position_at_end(then_bb);
                self.builder.build_unconditional_branch(cont_bb);

                let then_bb = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(else_bb);
                self.builder.build_unconditional_branch(cont_bb);

                let else_bb = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(cont_bb);

                let phi = self.builder.build_phi(self.context.i64_type(), "iftmp");

                phi.add_incoming(&[
                    (&any_value_enum_to_basic_value_enum!(value), then_bb),
                    (&any_value_enum_to_basic_value_enum!(else_branch), else_bb),
                ]);

                Ok(phi.as_basic_value().into())
            }
            AstNode::Identifier { name, args } => {
                let arg_values = args
                    .iter()
                    .cloned()
                    .map(|node| {
                        let a = self.compile_astnode(node).unwrap();
                        match a {
                            AnyValueEnum::IntValue(x) => BasicMetadataValueEnum::IntValue(x),
                            AnyValueEnum::ArrayValue(x) => BasicMetadataValueEnum::ArrayValue(x),
                            AnyValueEnum::PointerValue(x) => {
                                BasicMetadataValueEnum::PointerValue(x)
                            }
                            a => {
                                todo_feature!(format!("Compiling {:?}", a))
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                let last_function = self.module.get_last_function().unwrap();
                let attr = last_function.get_string_attribute(AttributeLoc::Return, &name);

                Ok(if let Some(arg) = attr {
                    last_function
                        .get_nth_param(
                            arg.get_string_value()
                                .to_str()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap(),
                        )
                        .unwrap()
                        .into()
                } else {
                    let called = self.builder.build_call(
                        self.get_function(&name).unwrap_or_else(|| {
                            unrecoverable_error!(format!("Function {} not found!", name))
                        }),
                        arg_values.as_slice(),
                        &name,
                    );
                    called.set_tail_call(true);
                    called.try_as_basic_value().left().unwrap().into()
                })
            }
            AstNode::InParens(node) => {
                let compiled_astnode = self.compile_astnode(*node).unwrap();

                let alloca = self.builder.build_alloca(
                    any_value_enum_to_basic_value_enum!(compiled_astnode).get_type(),
                    "inparens",
                );

                self.builder.build_store(
                    alloca,
                    any_value_enum_to_basic_value_enum!(compiled_astnode),
                );

                Ok(self
                    .builder
                    .build_load(alloca, alloca.get_name().to_string_lossy().borrow())
                    .into())
            }
            a => {
                todo_feature!(format!("Compiling {:?}", a))
            }
        }
    }

    fn compile_function(&self, node: AstNode) -> FunctionValue<'ctx> {
        match node {
            AstNode::Fn {
                name,
                return_type,
                args,
                value,
            } => {
                let args_vec: Vec<(AstNode, AstNode)> = match *args {
                    AstNode::FnArgs(x) => x,
                    _ => unreachable!(),
                };

                let fn_type = match self.compile_type(*return_type) {
                    BasicMetadataTypeEnum::IntType(x) => x.fn_type(
                        &args_vec
                            .iter()
                            .cloned()
                            .map(|(_, types)| self.compile_type(types))
                            .collect::<Vec<_>>()[..],
                        false,
                    ),
                    BasicMetadataTypeEnum::PointerType(x) => x.fn_type(
                        &args_vec
                            .iter()
                            .cloned()
                            .map(|(_, types)| self.compile_type(types))
                            .collect::<Vec<_>>()[..],
                        false,
                    ),
                    _ => panic!(),
                };
                let function = self.module.add_function(&name, fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");

                self.builder.position_at_end(basic_block);

                let params = function.get_params();

                for arg in &params {
                    let index = params.iter().position(|curr_arg| arg == curr_arg).unwrap();
                    let name = match &args_vec[index].0 {
                        AstNode::Identifier { name, args: _ } => name,
                        _ => unreachable!(),
                    };
                    let attr = self
                        .context
                        .create_string_attribute(name, &index.to_string());
                    function.add_attribute(AttributeLoc::Return, attr)
                }

                let basic_value =
                    any_value_enum_to_basic_value_enum!(self.compile_astnode(*value).unwrap());

                self.builder.build_return(Some(&basic_value));

                function
            }
            _ => panic!("Not a function!"),
        }
    }

    fn add_default_functions(&self) {
        let i64_type = self.context.i64_type();
        let bool_type = self.context.bool_type();

        // Puts
        let func_type = i64_type.fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into()],
            false,
        );

        self.module
            .add_function("puts", func_type, Some(Linkage::External));

        // Printf
        let func_type = i64_type.fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into()],
            true,
        );

        self.module
            .add_function("printf", func_type, Some(Linkage::External));

        // bool_eq
        let func_type = self
            .context
            .bool_type()
            .fn_type(&[bool_type.into(), bool_type.into()], false);
        let func = self.module.add_function("bool_eq", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                lhs,
                rhs,
                "eq",
            )));

        // ==
        let func_type = self
            .context
            .bool_type()
            .fn_type(&[i64_type.into(), i64_type.into()], false);
        let func = self.core.add_function("==", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                lhs,
                rhs,
                "eq",
            )));

        // +

        let func_type = self
            .context
            .i64_type()
            .fn_type(&[i64_type.into(), i64_type.into()], false);

        let func = self.core.add_function("+", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_add(lhs, rhs, "sum")));

        // -

        let func_type = self
            .context
            .i64_type()
            .fn_type(&[i64_type.into(), i64_type.into()], false);

        let func = self.core.add_function("-", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_sub(lhs, rhs, "sub")));

        // *

        let func_type = self
            .context
            .i64_type()
            .fn_type(&[i64_type.into(), i64_type.into()], false);

        let func = self.core.add_function("*", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_mul(lhs, rhs, "mul")));

        // /

        let func_type = self
            .context
            .i64_type()
            .fn_type(&[i64_type.into(), i64_type.into()], false);

        let func = self.core.add_function("/", func_type, None);

        self.builder
            .position_at_end(self.context.append_basic_block(func, "entry"));

        let lhs = func.get_nth_param(0).unwrap().into_int_value();
        let rhs = func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_signed_div(lhs, rhs, "div")));

        let core = if is_core_installed() {
            fs::read_to_string(get_core_path())
        } else {
            install_core().unwrap();
            fs::read_to_string(get_core_path())
        }
        .unwrap();

        let parsed = parse(&core).unwrap();

        parsed.iter().cloned().for_each(|n| {
            self.compile_astnode(n).unwrap();
        });

        self.module.link_in_module(self.core.clone()).unwrap();
    }

    fn compile_type(&self, node_type: AstNode) -> BasicMetadataTypeEnum<'ctx> {
        match node_type {
            AstNode::Type(typ) => match &typ[..] {
                "i64" => self.context.i64_type().into(),
                "bool" => self.context.bool_type().into(),
                "str" => self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
                "char" => self
                    .context
                    .i8_type()
                    .ptr_type(AddressSpace::Generic)
                    .into(),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn compile(&self, ast: AST) {
        self.add_default_functions();
        ast.iter().cloned().for_each(|n| {
            self.compile_astnode(n).unwrap();
        });
    }

    /// # Safety
    ///
    /// Should be called to run the main function after the program has been compiled
    pub(crate) unsafe fn call_main(&self) {
        self.execution_engine
            .run_function_as_main(self.get_function("main").unwrap(), &[]);
    }

    pub fn dump_ir(&self) -> Result<(), LLVMString> {
        self.module.print_to_file("ir.ll")
    }

    pub fn dump_asm(&self) -> Result<(), LLVMString> {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

        let triple = TargetMachine::get_default_triple();
        let cpu = TargetMachine::get_host_cpu_name().to_string();
        let features = TargetMachine::get_host_cpu_features().to_string();

        let target = Target::from_triple(&triple).unwrap();
        let machine = target
            .create_target_machine(
                &triple,
                &cpu,
                &features,
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        machine.write_to_file(&self.module, FileType::Assembly, "out.asm".as_ref())?;

        Ok(())
    }
}
