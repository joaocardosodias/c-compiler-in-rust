use crate::TokenKind;

/// Classifica o efeito de uma regra lexica no fluxo de tokens.
///
/// - `Emit(token)`: reconhece um lexema e emite esse token.
/// - `Skip`: reconhece um lexema que deve ser ignorado (espacos, comentarios etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuleKind {
    Emit(TokenKind),
    Skip,
}

impl RuleKind {
    /// Nome textual da acao para uso no codegen.
    pub fn as_generated_action(&self) -> &'static str {
        match self {
            Self::Emit(_) => "Emit",
            Self::Skip => "Skip",
        }
    }
}
