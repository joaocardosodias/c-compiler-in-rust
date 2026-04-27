use crate::scanner::lexer_gen::regex::CharClass;

#[derive(Debug, Clone)]
pub struct Nfa {
    pub start_state: usize,

    pub states: Vec<NfaState>,
}

impl Nfa {
    pub fn new() -> Self {
        Self {
            start_state: 0,
            states: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        let idx = self.states.len();
        self.states.push(NfaState::default());
        idx
    }

    pub fn add_epsilon(&mut self, from: usize, to: usize) {
        self.states[from].epsilon.push(to);
    }

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

#[derive(Debug, Clone, Default)]

pub struct NfaState {
    pub epsilon: Vec<usize>,

    pub transitions: Vec<SymbolTransition>,

    pub accept_rule_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SymbolTransition {
    pub symbol: TransitionSymbol,

    pub to: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransitionSymbol {
    Literal(char),

    AnyChar,

    CharClass(CharClass),
}

impl TransitionSymbol {
    pub fn matches(&self, ch: char) -> bool {
        match self {
            TransitionSymbol::Literal(c) => *c == ch,
            TransitionSymbol::AnyChar => true,
            TransitionSymbol::CharClass(class) => class.matches(ch),
        }
    }

    pub fn representative(&self) -> Option<char> {
        match self {
            TransitionSymbol::Literal(ch) => Some(*ch),

            TransitionSymbol::AnyChar => Some('\x01'),

            TransitionSymbol::CharClass(class) => (0u32..=127)
                .filter_map(char::from_u32)
                .find(|&ch| class.matches(ch)),
        }
    }
}
