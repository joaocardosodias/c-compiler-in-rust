#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i32),
    Variable(String),
    BinOp(BinaryOp, Box<Expr>, Box<Expr>),
}
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Return(Expr),
    VarDecl(String, Expr),
    Block(Vec<Stmt>),
    While{
        condition:Expr,
        body: Box<Stmt>
    },
    For{
        init:Box<Stmt>,
        condition:Expr,
        post:Expr,
        body:Box<Stmt>
    }

}
#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub body: Vec<Stmt>,
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