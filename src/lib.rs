//! Biblioteca principal do projeto.
//!
//! Neste momento, o foco da arquitetura e' o scanner gerado a partir de REs.
//! A API publica exporta:
//! - tipos de token e erro lexico
//! - tipos e parser de expressao regular usados pelo gerador lexico

pub mod lexer_gen;
pub mod pipeline;
pub mod scanner;

pub use lexer_gen::codegen::{
    generate_dfa_table_module, generate_rust_match_scanner, write_generated_scanner_to,
};
pub use lexer_gen::dfa::{Dfa, DfaState, build_dfa_from_nfa};
pub use lexer_gen::minimize::minimize_dfa_hopcroft;
pub use lexer_gen::nfa::{
    Nfa, NfaState, SymbolTransition, TransitionSymbol, build_nfa_from_unified_spec,
};
pub use lexer_gen::regex::{CharClass, CharClassItem, Regex, RegexParseError, parse_regex};
pub use pipeline::{
    AnnotatedRuleRegex, UnifiedRegexSpec, build_unified_regex_spec, generate_scanner_artifacts,
};
pub use scanner::scan_source;
pub use scanner::{LexError, LexErrorKind, Position, Span, Token, TokenKind};
