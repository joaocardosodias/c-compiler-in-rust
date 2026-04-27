use crate::TokenKind;

use crate::scanner::lexer_gen::regex::{Regex, parse_regex};

use crate::scanner::spec::tokens::RuleKind;

#[derive(Debug, Clone)]

pub struct RegexRule {
    pub name: &'static str,
    // `&'static str`: string literal com vida util de todo o programa.
    pub pattern: &'static str,
    // O texto da RE exatamente como escrito. Ex: "[A-Za-z_][A-Za-z0-9_]*" para Identifier.
    // Sera convertido em Regex (AST) pela funcao `parsed_regex_rules`.
    /// Acao da regra no fluxo de tokens (emitir ou ignorar).
    pub kind: RuleKind,
    // Determina o que o scanner faz quando esta regra vence: emite um Token ou descarta o lexema.
    /// Prioridade de desempate (menor numero = mais forte).
    pub priority: usize,
    // Quando dois lexemas de mesmo tamanho casam ao mesmo tempo (ex: "if" como KwIf e Identifier),
    // a regra com MENOR prioridade vence. Isso permite que keywords vençam identificadores.
}

impl RegexRule {
    /// Cria regra que emite um token.
    pub const fn emit(
        // `const fn`: a funcao pode ser avaliada em tempo de compilacao.
        // Isso e' opcional aqui (nao usamos em constantes estaticas), mas e' uma boa pratica
        // para funcoes simples que so constroem structs — documenta que nao ha side effects.
        name: &'static str,
        pattern: &'static str,
        token: TokenKind,
        priority: usize,
    ) -> Self {
        Self {
            name,
            pattern,
            kind: RuleKind::Emit(token), // Envolve o token na variante Emit do RuleKind.
            priority,
        }
    }

    /// Cria regra que reconhece e ignora lexema.
    pub const fn skip(name: &'static str, pattern: &'static str, priority: usize) -> Self {
        Self {
            name,
            pattern,
            kind: RuleKind::Skip, // Nao ha token associado: o lexema sera descartado.
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
    // Retorna Vec (alocado no heap) porque a lista cresce e muda conforme o compilador evolui.
    // Usar &[RegexRule] estatico exigiria que todas as regras fossem constantes — mais rigido.
    vec![
        // --- Regras ignoradas pelo scanner (whitespace e comentarios) ---
        // Prioridades 0, 1, 2: sao as mais fortes para garantir que espaco/comentario
        // nunca "vazem" para uma regra de token mais especifica.
        RegexRule::skip("Whitespace", "[ \t\n\r]+", 0),
        // RE: um ou mais caracteres de espaco, tab, newline ou carriage return.
        // Priority 0: a mais alta possivel — espaco branco sempre vence.
        RegexRule::skip("LineComment", "//[^\n]*", 1),
        // RE: "//" seguido de qualquer coisa que NAO seja newline, ate o fim da linha.
        // `[^\n]*`: classe negada — casa qualquer char exceto '\n'. O '*' aceita linha vazia.
        RegexRule::skip("BlockComment", "/\\*([^*]|\\*+[^*/])*\\*+/", 2),
        // RE: a famosa regex de comentario de bloco do C (/* ... */).
        // `/\\*`: literal "/*" (o * precisa ser escapado na RE pois e' metacaractere).
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
        RegexRule::emit(
            "Identifier",
            "[A-Za-z_][A-Za-z0-9_]*",
            TokenKind::Identifier,
            40,
        ),
        RegexRule::emit("IntLiteral", "0|[1-9][0-9]*", TokenKind::IntLiteral, 41),
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
