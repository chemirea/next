#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Stmt>);

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

#[derive(Debug, PartialEq)]
pub enum Expr {
    Primitive(Primitive),
    Variable(Ident),
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Primitive,
    UserDefine(String),
    Inference,
}

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Int(u64),
    String(String),
}
