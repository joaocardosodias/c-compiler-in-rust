//! Artefatos gerados para o scanner.

pub mod dfa_table;
pub mod scanner_impl;

pub use dfa_table::{DFA_ACCEPT_RULE_INDEX, DFA_START_STATE, DFA_STATE_COUNT};
pub use scanner_impl::{GeneratedAcceptAction, dfa_accept_action, dfa_next_state};
