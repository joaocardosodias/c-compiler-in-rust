//! Componentes de expressao regular (RE).
//!
//! Este modulo separa duas responsabilidades:
//! - `ast`: representacao estruturada da RE
//! - `build`: parser da sintaxe textual para montar a AST

pub mod ast;
pub mod build;

pub use ast::{CharClass, CharClassItem, Regex};
pub use build::{RegexParseError, parse_regex};
