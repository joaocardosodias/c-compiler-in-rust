//! Infraestrutura do gerador lexico.
//!
//! Aqui ficam os modulos que implementam o pipeline teorico:
//! RE -> NFA -> DFA -> DFA minimo -> codigo Rust.

pub mod regex;
pub mod nfa;
pub mod dfa;
pub mod minimize;
pub mod codegen;
