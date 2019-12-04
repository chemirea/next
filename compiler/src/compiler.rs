use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::values::{FunctionValue, PointerValue};
use next_parser::ast::*;

use std::collections::HashMap;

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub program: &'a Program,

    // 新たな環境ができるたびにHashMapを作成してVecの最後尾に追加する。
    // 環境が終わるたびにpopして削除する。
    // その環境で使える値はenvに入っている全てのものである。
    // 変数のスコープを表現している。
    pub env: Vec<HashMap<String, PointerValue<'ctx>>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn compile(&mut self) {
        // 環境の作成
        self.env.push(HashMap::new());
        for stmt in &self.program.0 {
            match stmt {
                Stmt::ExprStmt(expr) => self.compile_expr(&expr),
                Stmt::LetStmt { ident, expr } => self.compile_let_stmt(ident, expr),
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

    // Todo
    fn compile_let_stmt(&self, ident: &Ident, expr: &Expr) {
        // let value = self.compile_expr(expr);

        // let tail_index = self.env.len() - 1;
        // self.env[tail_index].insert(ident.name, alloca);
    }

    // プリミティブをコンパイルする
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
