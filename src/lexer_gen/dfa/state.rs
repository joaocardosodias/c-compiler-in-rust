use std::collections::HashMap;

use crate::lexer_gen::nfa::TransitionSymbol;

/// DFA construido a partir do NFA por construcao de subconjuntos.
#[derive(Debug, Clone, Default)]
pub struct Dfa {
    /// Estado inicial do DFA.
    pub start_state: usize,
    /// Estados do DFA.
    pub states: Vec<DfaState>,
}

/// Estado deterministico derivado de um conjunto de estados do NFA.
#[derive(Debug, Clone, Default)]
pub struct DfaState {
    /// Conjunto de estados do NFA que este estado representa.
    pub nfa_states: Vec<usize>,
    /// Transicoes deterministicas por simbolo abstrato.
    pub transitions: HashMap<TransitionSymbol, usize>,
    /// Melhor regra de aceitacao (menor indice de regra) presente no conjunto.
    pub accept_rule_index: Option<usize>,
}
