use crate::parser::ast::{AstNode, AST};
use crate::unrecoverable_error;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::{Linkage, Module};
use inkwell::support::LLVMString;
use inkwell::types::{BasicMetadataTypeEnum, StringRadix};
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue};
use inkwell::AddressSpace;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
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
        CodeGen {
            context,
            module,
            builder,
            execution_engine,
        }
    }

    fn available_global_name(&self, name: u8) -> String {
        let name_as_string = name.to_string();

        if self.module.get_global(&name_as_string).is_some() {
            self.available_global_name(name + 1)
        } else {
            name_as_string
        }
    }

    fn compile_astnode(&self, node: AstNode) -> Result<AnyValueEnum<'ctx>, &'static str> {
        match node {
            AstNode::Int(int) => Ok(self
                .context
                .i64_type()
                .const_int_from_string(&int.to_string(), StringRadix::Decimal)
                .unwrap()
                .into()),
            AstNode::Str(str) => Ok(unsafe {
                self.builder
                    .build_global_string(&(str + "\0"), &self.available_global_name(0))
                    .as_pointer_value()
                    .into()
            }),
            AstNode::Boolean(bool) => Ok(self
                .context
                .bool_type()
                .const_int(bool as u64, false)
                .into()),
            AstNode::Char(char) => Ok(self.context.const_string(&[char as u8], false).into()),
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
            AstNode::Identifier { name, args } => {
                let arg_values = args
                    .iter()
                    .cloned()
                    .map(|node| match self.compile_astnode(node).unwrap() {
                        AnyValueEnum::IntValue(x) => BasicMetadataValueEnum::IntValue(x),
                        AnyValueEnum::VectorValue(x) => BasicMetadataValueEnum::VectorValue(x),
                        AnyValueEnum::PointerValue(x) => BasicMetadataValueEnum::PointerValue(x),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();

                let ret_val = self
                    .builder
                    .build_call(
                        self.get_function(&name).unwrap_or_else(|| {
                            unrecoverable_error!(format!("Function {} not found!", name))
                        }),
                        arg_values.as_slice(),
                        &name,
                    )
                    .try_as_basic_value()
                    .left()
                    .unwrap();

                Ok(ret_val.into())
            }
            _ => todo!(),
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
                    BasicMetadataTypeEnum::VectorType(x) => x.fn_type(
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

                let basic_value = match self.compile_astnode(*value).unwrap() {
                    AnyValueEnum::IntValue(x) => BasicValueEnum::IntValue(x),
                    AnyValueEnum::VectorValue(x) => BasicValueEnum::VectorValue(x),
                    _ => todo!(),
                };

                self.builder.build_return(Some(&basic_value));

                self.get_function(&name).unwrap()
            }
            _ => panic!("Not a function!"),
        }
    }

    fn add_default_functions(&self) {
        // Printf
        let printf_type = self.context.i64_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .into()],
            true,
        );

        self.module
            .add_function("printf", printf_type, Some(Linkage::External));

        // eq
        let eq_type = self.context.bool_type().fn_type(
            &[
                self.context.i64_type().into(),
                self.context.i64_type().into(),
            ],
            false,
        );
        let eq_func = self.module.add_function("eq", eq_type, None);
        let eq_block = self.context.append_basic_block(eq_func, "entry");

        self.builder.position_at_end(eq_block);

        let eq_lhs = eq_func.get_nth_param(0).unwrap().into_int_value();
        let eq_rhs = eq_func.get_nth_param(1).unwrap().into_int_value();

        self.builder
            .build_return(Some(&self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                eq_lhs,
                eq_rhs,
                "eq",
            )));
    }

    fn compile_type(&self, node_type: AstNode) -> BasicMetadataTypeEnum<'ctx> {
        match node_type {
            AstNode::Type(typ) => match &typ[..] {
                "i64" => self.context.i64_type().into(),
                "bool" => self.context.bool_type().into(),
                "str" => self
                    .context
                    .const_string("a".repeat(50).as_bytes(), false)
                    .get_type()
                    .into(),
                "char" => self
                    .context
                    .const_string("A".as_bytes(), false)
                    .get_type()
                    .into(),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn compile(&self, ast: AST) {
        self.add_default_functions();
        ast.iter()
            .filter(|n| !matches!(n, AstNode::Eoi))
            .cloned()
            .collect::<Vec<_>>()
            .iter()
            .cloned()
            .for_each(|n| {
                self.compile_astnode(n).unwrap();
            });
    }

    pub unsafe fn call_main(&self) {
        self.execution_engine
            .run_function_as_main(self.get_function("main").unwrap(), &[]);
    }
    pub fn dump_ir(&self) -> Result<(), LLVMString> {
        self.module.print_to_file("ir.ll")
    }
}
