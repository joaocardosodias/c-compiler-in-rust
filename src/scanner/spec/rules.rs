//! Regras lexicas do scanner em formato de expressao regular.
//!
//! Este arquivo e' a "fonte de verdade" da microssintaxe:
//! - cada regra tem um padrao RE
//! - cada regra define se emite token ou e' ignorada
//! - cada regra tem prioridade para desempate
//!
//! A ordem e a prioridade sao importantes para modelar o comportamento classico de
//! scanners: longest match + desempate deterministico.

use crate::lexer_gen::regex::{Regex, parse_regex};
use crate::scanner::spec::tokens::RuleKind;
use crate::TokenKind;

/// Regra lexica atomica antes da construcao de automatos.
#[derive(Debug, Clone)]
pub struct RegexRule {
    /// Nome auxiliar para debug/logs/erros.
    pub name: &'static str,
    /// Expressao regular textual que sera parseada para AST.
    pub pattern: &'static str,
    /// Acao da regra no fluxo de tokens (emitir ou ignorar).
    pub kind: RuleKind,
    /// Prioridade de desempate (menor numero = mais forte).
    pub priority: usize,
}

impl RegexRule {
    /// Cria regra que emite um token.
    pub const fn emit(
        name: &'static str,
        pattern: &'static str,
        token: TokenKind,
        priority: usize,
    ) -> Self {
        Self {
            name,
            pattern,
            kind: RuleKind::Emit(token),
            priority,
        }
    }

    /// Cria regra que reconhece e ignora lexema.
    pub const fn skip(name: &'static str, pattern: &'static str, priority: usize) -> Self {
        Self {
            name,
            pattern,
            kind: RuleKind::Skip,
            priority,
        }
    }
}

/// Lista de regras RE do scanner.
///
/// Racional da ordenacao:
/// - primeiro regras de `Skip` para remover ruido lexical
/// - depois keywords para vencer `Identifier` em empate de comprimento
/// - depois compostos (ex: `>>=`) antes de simples (ex: `>`)
pub fn regex_rules() -> Vec<RegexRule> {
    vec![
        // Ignorados pelo scanner.
        RegexRule::skip("Whitespace", "[ \t\n\r]+", 0),
        RegexRule::skip("LineComment", "//[^\n]*", 1),
        RegexRule::skip("BlockComment", "/\\*([^*]|\\*+[^*/])*\\*+/", 2),
        // Keywords (antes de Identifier para desempate por prioridade).
        RegexRule::emit("KwIf", "if", TokenKind::KwIf, 10),
        RegexRule::emit("KwElse", "else", TokenKind::KwElse, 11),
        RegexRule::emit("KwWhile", "while", TokenKind::KwWhile, 12),
        RegexRule::emit("KwFor", "for", TokenKind::KwFor, 13),
        RegexRule::emit("KwReturn", "return", TokenKind::KwReturn, 14),
        RegexRule::emit("KwInt", "int", TokenKind::KwInt, 15),
        RegexRule::emit("KwChar", "char", TokenKind::KwChar, 16),
        RegexRule::emit("KwVoid", "void", TokenKind::KwVoid, 17),
        RegexRule::emit("KwStruct", "struct", TokenKind::KwStruct, 18),
        RegexRule::emit("KwTypedef", "typedef", TokenKind::KwTypedef, 19),
        // Literais e identificadores.
        RegexRule::emit(
            "Identifier",
            "[A-Za-z_][A-Za-z0-9_]*",
            TokenKind::Identifier,
            40,
        ),
        RegexRule::emit("IntLiteral", "0|[1-9][0-9]*", TokenKind::IntLiteral, 41),
        // Operadores compostos (antes dos simples).
        RegexRule::emit("Ellipsis", "\\.\\.\\.", TokenKind::Ellipsis, 60),
        RegexRule::emit("Arrow", "->", TokenKind::Arrow, 61),
        RegexRule::emit("Increment", "\\+\\+", TokenKind::Increment, 62),
        RegexRule::emit("Decrement", "--", TokenKind::Decrement, 63),
        RegexRule::emit("ShlAssign", "<<=", TokenKind::ShlAssign, 64),
        RegexRule::emit("ShrAssign", ">>=", TokenKind::ShrAssign, 65),
        RegexRule::emit("PlusAssign", "\\+=", TokenKind::PlusAssign, 66),
        RegexRule::emit("MinusAssign", "-=", TokenKind::MinusAssign, 67),
        RegexRule::emit("StarAssign", "\\*=", TokenKind::StarAssign, 68),
        RegexRule::emit("SlashAssign", "/=", TokenKind::SlashAssign, 69),
        RegexRule::emit("PercentAssign", "%=", TokenKind::PercentAssign, 70),
        RegexRule::emit("AndAssign", "&=", TokenKind::AndAssign, 71),
        RegexRule::emit("OrAssign", "\\|=", TokenKind::OrAssign, 72),
        RegexRule::emit("XorAssign", "\\^=", TokenKind::XorAssign, 73),
        RegexRule::emit("EqEq", "==", TokenKind::EqEq, 74),
        RegexRule::emit("NotEq", "!=", TokenKind::NotEq, 75),
        RegexRule::emit("Le", "<=", TokenKind::Le, 76),
        RegexRule::emit("Ge", ">=", TokenKind::Ge, 77),
        RegexRule::emit("AndAnd", "&&", TokenKind::AndAnd, 78),
        RegexRule::emit("OrOr", "\\|\\|", TokenKind::OrOr, 79),
        RegexRule::emit("Shl", "<<", TokenKind::Shl, 80),
        RegexRule::emit("Shr", ">>", TokenKind::Shr, 81),
        // Operadores e pontuacao simples.
        RegexRule::emit("Assign", "=", TokenKind::Assign, 100),
        RegexRule::emit("Plus", "\\+", TokenKind::Plus, 101),
        RegexRule::emit("Minus", "-", TokenKind::Minus, 102),
        RegexRule::emit("Star", "\\*", TokenKind::Star, 103),
        RegexRule::emit("Slash", "/", TokenKind::Slash, 104),
        RegexRule::emit("Percent", "%", TokenKind::Percent, 105),
        RegexRule::emit("Not", "!", TokenKind::Not, 106),
        RegexRule::emit("Lt", "<", TokenKind::Lt, 107),
        RegexRule::emit("Gt", ">", TokenKind::Gt, 108),
        RegexRule::emit("Amp", "&", TokenKind::Amp, 109),
        RegexRule::emit("Pipe", "\\|", TokenKind::Pipe, 110),
        RegexRule::emit("Caret", "\\^", TokenKind::Caret, 111),
        RegexRule::emit("Tilde", "~", TokenKind::Tilde, 112),
        RegexRule::emit("Question", "\\?", TokenKind::Question, 113),
        RegexRule::emit("Colon", ":", TokenKind::Colon, 114),
        RegexRule::emit("Dot", "\\.", TokenKind::Dot, 115),
        RegexRule::emit("Comma", ",", TokenKind::Comma, 116),
        RegexRule::emit("Semicolon", ";", TokenKind::Semicolon, 117),
        RegexRule::emit("LParen", "\\(", TokenKind::LParen, 118),
        RegexRule::emit("RParen", "\\)", TokenKind::RParen, 119),
        RegexRule::emit("LBrace", "\\{", TokenKind::LBrace, 120),
        RegexRule::emit("RBrace", "\\}", TokenKind::RBrace, 121),
        RegexRule::emit("LBracket", "\\[", TokenKind::LBracket, 122),
        RegexRule::emit("RBracket", "\\]", TokenKind::RBracket, 123),
        RegexRule::emit("HashHash", "##", TokenKind::HashHash, 124),
        RegexRule::emit("Hash", "#", TokenKind::Hash, 125),
    ]
}

/// Parseia todas as regras para AST de RE.
///
/// Esta funcao e' um ponto de validacao adiantado: se uma RE estiver invalida,
/// falha cedo com mensagem contextualizada pelo nome da regra.
pub fn parsed_regex_rules() -> Result<Vec<(RegexRule, Regex)>, String> {
    regex_rules()
        .into_iter()
        .map(|rule| {
            parse_regex(rule.pattern)
                .map(|regex| (rule.clone(), regex))
                .map_err(|err| {
                    format!(
                        "failed parsing rule '{}' at {}: {}",
                        rule.name, err.position, err.message
                    )
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parsed_regex_rules, regex_rules};

    #[test]
    fn all_rule_patterns_parse() {
        parsed_regex_rules().expect("all rules should parse");
    }

    #[test]
    fn priorities_are_non_decreasing() {
        let rules = regex_rules();
        for pair in rules.windows(2) {
            assert!(pair[0].priority <= pair[1].priority);
        }
    }
}
