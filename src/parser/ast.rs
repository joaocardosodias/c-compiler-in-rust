#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i32),
    Variable(String),
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
    Call { name: String, args: Vec<Expr> },
}
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Return(Expr),
    VarDecl {
        name: String,
        expr: Option<Expr>,
        var_type: Type,
    },
    Block(Vec<Stmt>),
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        init: Box<Stmt>,
        condition: Expr,
        post: Expr,
        body: Box<Stmt>,
    },
    If {
        condition: Expr,
        action: Box<Stmt>,
        else_block: Box<Stmt>,
    },
}
#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub body: Vec<Stmt>,
    pub function_type: Type,
}
#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}
#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Void,
    Float,
    Double,
}
