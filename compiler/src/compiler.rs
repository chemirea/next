use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::FunctionValue;
use next_parser::ast::*;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub program: &'a Program,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile(&self) {
        for stmt in &self.program.0 {
            match stmt {
                Stmt::ExprStmt(expr) => self.compile_expr(&expr),
                // Todo
                _ => (),
            }
        }
    }

    // 式コンパイラ
    fn compile_expr(&self, expr: &Expr) {
        match expr {
            Expr::Primitive(v) => self.compile_primitive(v),
            // Todo
            _ => (),
        }
    }

    fn compile_primitive(&self, primitive: &Primitive) {
        match primitive {
            Primitive::Int(n) => self
                .context
                .i64_type()
                .const_int(*n, false)
                // Todo
                .print_to_stderr(),
            _ => panic!(),
        };
    }
}
