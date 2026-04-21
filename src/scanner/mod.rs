//! Modulos relacionados ao scanner da linguagem C.
//!
//! O scanner final sera gerado a partir de regras em RE, mas os tipos de token
//! e a especificacao das regras ficam aqui para servir de contrato entre o
//! gerador e o restante do compilador.

pub mod tokens;
pub mod spec;
pub mod generated;
pub mod runtime;

pub use tokens::{LexError, LexErrorKind, Position, Span, Token, TokenKind};
pub use spec::{RegexRule, RuleKind, parsed_regex_rules, regex_rules};
pub use generated::{
    DFA_ACCEPT_RULE_INDEX,
    DFA_START_STATE,
    DFA_STATE_COUNT,
    GeneratedAcceptAction,
    dfa_accept_action,
    dfa_next_state,
};
pub use runtime::scan_source;
