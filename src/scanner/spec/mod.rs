//! Especificacao lexica.
//!
//! Esta camada descreve "o que" o scanner deve reconhecer:
//! - tipo da regra (emite token ou ignora lexema)
//! - lista de regras RE + prioridade
//!
//! A camada de geracao (Thompson/subset/Hopcroft/codegen) usa este modulo como
//! entrada para construir o automato final.

pub mod rules;
pub mod tokens;

pub use rules::{RegexRule, parsed_regex_rules, regex_rules};
pub use tokens::RuleKind;
