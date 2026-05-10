#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLiteral(i32),
    Variable(String)
}
#[derive(Debug, PartialEq)]
pub enum Stmt {
    Return(Expr),
    VarDecl(String, Expr)
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
