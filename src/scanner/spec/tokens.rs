use crate::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum RuleKind {
    Emit(TokenKind),
    Skip,
}

impl RuleKind {
    pub fn as_generated_action(&self) -> &'static str {
        // Retorna &'static str (string literal) em vez de String para evitar alocacao.

        match self {
            Self::Emit(_) => "Emit",
            Self::Skip => "Skip",
        }
    }
}
