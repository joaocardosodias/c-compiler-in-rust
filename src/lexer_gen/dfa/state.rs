use std::collections::HashMap;

use crate::lexer_gen::nfa::TransitionSymbol;

/// DFA construido a partir do NFA por construcao de subconjuntos.
#[derive(Debug, Clone, Default)]
pub struct Dfa {
    /// Estado inicial do DFA.
    // Assim como no NFA, a execução começa por aqui.
    pub start_state: usize,
    
    /// Estados do DFA.
    // Mantemos a mesma arquitetura plana baseada em Vec e IDs (índices).
    pub states: Vec<DfaState>,
}

/// Estado deterministico derivado de um conjunto de estados do NFA.
#[derive(Debug, Clone, Default)]
pub struct DfaState {
    /// Conjunto de estados do NFA que este estado representa.
    // O algoritmo de Subset Construction agrupa vários estados do NFA em um único estado do DFA.
    // Essa lista (geralmente ordenada) serve como a "identidade" deste DfaState.
    // Ex: O DfaState atual equivale aos estados {1, 4, 5, 8} do NFA estarem ativos ao mesmo tempo.
    pub nfa_states: Vec<usize>,
    
    /// Transicoes deterministicas por simbolo abstrato.
    // DIFERENÇA CRUCIAL PARA O NFA: 
    // 1. Não existe epsilon-transição (`epsilon: Vec<usize>` sumiu).
    // 2. Usamos `HashMap` garantindo que para CADA símbolo haja no MÁXIMO UMA transição de saída.
    // Isso torna a simulação O(1) por caractere consumido.
    pub transitions: HashMap<TransitionSymbol, usize>,
    
    /// Melhor regra de aceitacao (menor indice de regra) presente no conjunto.
    // Se o conjunto de `nfa_states` contiver um ou mais estados de aceitação do NFA,
    // este estado do DFA também será de aceitação.
    // Se houver conflito (ex: casou com Keyword e Identifier ao mesmo tempo), 
    // armazenamos o `menor índice` (a regra com maior prioridade definida em spec/rules.rs).
    pub accept_rule_index: Option<usize>,
}
