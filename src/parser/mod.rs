//! Parser do mini-C.

pub mod ast;
pub mod parser;

pub use ast::{
    BinaryOp,
    Expr,
    ForInit,
    Function,
    Param,
    Program,
    Stmt,
    TypeSpecifier,
    UnaryOp,
};
pub use parser::{ParseError, parse_program};
