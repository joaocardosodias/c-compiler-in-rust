//! Geracao de codigo Rust para o scanner final.
//!
//! O backend atual produz uma implementacao baseada em `match state { ... }`
//! a partir do DFA minimizado.

pub mod rust_match;

pub use rust_match::{
    generate_dfa_table_module, generate_rust_match_scanner, write_generated_scanner_to,
};
