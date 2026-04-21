//! Representacao e construcao de AFD (DFA).

pub mod state;
pub mod subset;

pub use state::{Dfa, DfaState};
pub use subset::build_dfa_from_nfa;
