#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    LetStmt {
        ident: Ident,
        expr: Expr,
    },
    RetStmt(Expr),
    FnStmt {
        ident: Ident,
        args: Vec<Ident>,
        body: Vec<Stmt>,
    },
    BlockStmt(Vec<Stmt>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Variable(Ident),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: String,
    pub type_: Type,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    UserDefine(String),
    Inference,
}
