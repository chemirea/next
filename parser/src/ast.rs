#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
}

#[macro_export]
macro_rules! int {
    ($e:expr) => {
        ast::Program(vec![ast::Stmt::ExprStmt(ast::Expr::Int($e))])
    };
}
