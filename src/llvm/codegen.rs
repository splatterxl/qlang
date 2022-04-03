use std::process::exit;

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{FunctionType, BasicMetadataTypeEnum},
    values::FunctionValue, AddressSpace, execution_engine::ExecutionEngine, OptimizationLevel,
};

use crate::parser::{ast::ast::{NodeType, Node, Function}, TopLevel};

#[derive(Debug)]
pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl Clone for Codegen<'_> {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            module: self.module.clone(),
            builder: self.context.create_builder(),
            execution_engine: self.execution_engine.clone(),
        }
    }
}

macro_rules! coerce_node_type {
    ($ctx:tt, $ty:tt) => {
        match $ty {
            NodeType::Bool => $ctx.bool_type().into(),
            NodeType::Integer => $ctx.i32_type().into(),
            NodeType::Float => $ctx.f32_type().into(),
            NodeType::String => $ctx.i8_type().ptr_type(AddressSpace::Generic).into(),
            _ => panic!("Unsupported type"),
        }
    };
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        Self {
            context,
            module,
            builder: context.create_builder(),
            execution_engine,
        }
    } 

    pub fn interpret(&self, code: &str) {
        let ast: TopLevel = code.parse().unwrap();

        for func in ast.fns {
            self.create_function(func);
        }
    }

    fn create_function(&self, func: Node) {
        if let Node::Fn(func) = func {
            let ty = self.type_for_function(&func);

            let created = self.module.add_function(&func.name, ty, None);
            self.create_entry(&func, created);
        } else {
            panic!("Expected function");
        }
    }

    fn type_for_function(&self, func: &Function) -> FunctionType<'ctx> {
        let mut args: Vec<BasicMetadataTypeEnum> = Vec::new();
        let ctx = &self.context;

        for arg in &func.args { 
            let arg_ty = &arg.1;
            let ty = coerce_node_type!(ctx, arg_ty);

            args.push(ty);
        }

        match func.ret {
            NodeType::Bool => self.context.bool_type().fn_type(&args[..], false),
            NodeType::Integer => self.context.i32_type().fn_type(&args[..], false),
            NodeType::Float => self.context.f32_type().fn_type(&args[..], false),
            NodeType::String => self.context.i8_type().ptr_type(AddressSpace::Generic).fn_type(&args[..], false),
            _ => panic!("Unsupported type"),
        }
    }

    fn create_entry(&self, func: &Function, created: FunctionValue<'ctx>) {
        let entry = self.context.append_basic_block(created, "entry");
        self.builder.position_at_end(entry);

        let _body = match &func.body {
            Node::Block(body) => body,
            _ => panic!("Expected block"),
        };

        self.builder.build_return(None);
        
    }

    fn _emit_error(&self, msg: String) -> ! {
        eprintln!("{}", msg);
        exit(1)
    }
}
