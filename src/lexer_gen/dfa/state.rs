use std::collections::HashMap;

use crate::lexer_gen::nfa::TransitionSymbol;

#[derive(Debug, Clone, Default)]
pub struct Dfa {
    pub start_state: usize,

    pub states: Vec<DfaState>,
}

#[derive(Debug, Clone, Default)]
pub struct DfaState {
    pub nfa_states: Vec<usize>,

    pub transitions: HashMap<TransitionSymbol, usize>,

    pub accept_rule_index: Option<usize>,
}
