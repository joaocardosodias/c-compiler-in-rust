use crate::lexer_gen::regex::CharClass;

/// NFA completo usado como entrada da construcao de subconjuntos (DFA).
#[derive(Debug, Clone)]
pub struct Nfa {
    /// Estado inicial global.
    pub start_state: usize,
    /// Todos os estados do automato.
    pub states: Vec<NfaState>,
}

impl Nfa {
    /// Cria NFA vazio (sem estados).
    pub fn new() -> Self {
        Self {
            start_state: 0,
            states: Vec::new(),
        }
    }

    /// Adiciona um estado e retorna seu indice.
    pub fn add_state(&mut self) -> usize {
        let idx = self.states.len();
        self.states.push(NfaState::default());
        idx
    }

    /// Adiciona transicao epsilon entre dois estados.
    pub fn add_epsilon(&mut self, from: usize, to: usize) {
        self.states[from].epsilon.push(to);
    }

    /// Adiciona transicao consumindo simbolo entre dois estados.
    pub fn add_symbol_transition(&mut self, from: usize, to: usize, symbol: TransitionSymbol) {
        self.states[from]
            .transitions
            .push(SymbolTransition { symbol, to });
    }
}

impl Default for Nfa {
    fn default() -> Self {
        Self::new()
    }
}

/// Estado do NFA.
#[derive(Debug, Clone, Default)]
pub struct NfaState {
    /// Fechos epsilon.
    pub epsilon: Vec<usize>,
    /// Transicoes com consumo de simbolo.
    pub transitions: Vec<SymbolTransition>,
    /// Regra aceita por este estado (indice em `UnifiedRegexSpec.rules`).
    pub accept_rule_index: Option<usize>,
}

/// Transicao com consumo de um simbolo (ou classe de simbolos).
#[derive(Debug, Clone)]
pub struct SymbolTransition {
    pub symbol: TransitionSymbol,
    pub to: usize,
}

/// Alfabeto abstrato usado no NFA.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransitionSymbol {
    /// Um caractere literal.
    Literal(char),
    /// Coringa `.`
    AnyChar,
    /// Classe de caracteres `[ ... ]`.
    CharClass(CharClass),
}

impl TransitionSymbol {
    /// Verifica se um caractere e' aceito por este simbolo.
    pub fn matches(&self, ch: char) -> bool {
        match self {
            TransitionSymbol::Literal(c) => *c == ch,
            TransitionSymbol::AnyChar => true,
            TransitionSymbol::CharClass(class) => class.matches(ch),
        }
    }

    /// Retorna um caractere representativo deste simbolo (para uso na particao).
    pub fn representative(&self) -> Option<char> {
        match self {
            TransitionSymbol::Literal(ch) => Some(*ch),
            TransitionSymbol::AnyChar => Some('\x01'),
            TransitionSymbol::CharClass(class) => {
                (0u32..=127).filter_map(char::from_u32).find(|&ch| class.matches(ch))
            }
        }
    }
}
