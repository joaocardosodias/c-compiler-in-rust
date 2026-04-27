pub mod parser;
pub mod scanner;

pub use scanner::lexer_gen::codegen::{
    generate_dfa_table_module, generate_rust_match_scanner, write_generated_scanner_to,
};
pub use scanner::lexer_gen::dfa::{Dfa, DfaState, build_dfa_from_nfa};
pub use scanner::lexer_gen::minimize::minimize_dfa_hopcroft;
pub use scanner::lexer_gen::nfa::{
    Nfa, NfaState, SymbolTransition, TransitionSymbol, build_nfa_from_unified_spec,
};
pub use scanner::lexer_gen::regex::{CharClass, CharClassItem, Regex, RegexParseError, parse_regex};
pub use scanner::pipeline::{
    AnnotatedRuleRegex, UnifiedRegexSpec, build_unified_regex_spec, generate_scanner_artifacts,
};
pub use parser::{
    BinaryOp,
    Expr,
    ForInit,
    Function,
    Param,
    ParseError,
    Program,
    Stmt,
    TypeSpecifier,
    UnaryOp,
    parse_program,
};
pub use scanner::scan_source;
pub use scanner::{LexError, LexErrorKind, Position, Span, Token, TokenKind};
