use std::process::exit;

use inkwell::{
    builder::Builder, 
    module::Module, 
    context::Context, 
    types::BasicMetadataTypeEnum, 
    values::{FunctionValue, BasicValueEnum}
};

use crate::parser::ast::ast;

struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    raw: String,
}

impl<'ctx> Codegen<'ctx> {
    pub fn add_constant(&self, name: String, value: ast::Node) {
        unimplemented!()
    }

    pub fn add_function(
        &'ctx self,
        name: String,
        args: Vec<(String, ast::NodeType)>,
        ret: ast::NodeType,
        body: Vec<ast::Node>
    ) {
        let args_type = &self.args_to_llvm(&args)[..];

        let ty = match ret {
            ast::NodeType::String => unimplemented!(),
            ast::NodeType::Integer => {
                let int_type = self.context.i32_type();
                int_type.fn_type(args_type, false)
            },
            ast::NodeType::Float => {
                let f32_type = self.context.f32_type();
                f32_type.fn_type(args_type, false)
            },
            ast::NodeType::Bool => {
                let bool_type = self.context.bool_type();
                bool_type.fn_type(args_type, false)
            },
            ast::NodeType::Void => {
                let void_type = self.context.void_type();
                void_type.fn_type(args_type, false)
            },
            _ => unreachable!()
        };

        let function = self.module.add_function(&name, ty, None);

        self.build_entry(function, args, body);
    }

    fn build_entry(
        &self,
        function: FunctionValue<'ctx>, 
        args: Vec<(String, ast::NodeType)>, 
        body: Vec<ast::Node>
    ) {
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        fn build_expr<T>(_: T) {}
        fn build_stmt<T>(_: T) {}

        for expr in body {
            match expr {
                ast::Node::Expr { .. } => {
                    build_expr(expr);
                },
                ast::Node::Stmt(stmt) => {
                    build_stmt(stmt);
                },
                // ignore single values
                _ => {}
            }
        }
    }

    fn args_to_llvm(&self, args: &Vec<(String, ast::NodeType)>) -> Vec<BasicMetadataTypeEnum> {
        let mut to_ret: Vec<BasicMetadataTypeEnum> = Vec::new();

        for arg in args {
            to_ret.push(match arg.1 {
                ast::NodeType::String => unimplemented!(),
                ast::NodeType::Integer => self.context.i32_type().into(),
                ast::NodeType::Float => self.context.f32_type().into(),
                ast::NodeType::Bool => self.context.bool_type().into(),
                _ => self.emit_error("Invalid argument type".into()),
            });
        };

        to_ret
    }

    fn get_param_for_arg(&self, function: FunctionValue<'ctx>, args: &Vec<(String, ast::NodeType)>, name: String) -> BasicValueEnum {
        for (i, (arg_name, _)) in args.iter().enumerate() {
            if arg_name == &name {
                return function.get_nth_param(i as u32).expect("invalid param");
            }
        }

        unreachable!()
    }

    fn emit_error(&self, msg: String) -> ! {
        eprintln!("{}", msg);
        exit(1)
    }
}
