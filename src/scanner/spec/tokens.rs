use crate::TokenKind;
// Importa TokenKind da raiz da crate (definido em scanner/tokens.rs e reexportado em lib.rs).
// Usar `crate::` em vez de `super::super::` deixa o caminho explicito e resistente a refatoracoes.

/// Classifica o efeito de uma regra lexica no fluxo de tokens.
///
/// - `Emit(token)`: reconhece um lexema e emite esse token.
/// - `Skip`: reconhece um lexema que deve ser ignorado (espacos, comentarios etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// `Copy`: RuleKind e' tiny — ou uma tag vazia (Skip) ou uma tag com um TokenKind (que tambem e' Copy).
// Copiar RuleKind nunca aloca memoria, entao nao ha custo em derivar Copy aqui.
pub enum RuleKind {
    Emit(TokenKind), // Variante com dado: carrega QUAL TokenKind deve ser emitido.
    Skip,            // Variante sem dado: o lexema reconhecido deve ser descartado.
}

impl RuleKind {
    /// Nome textual da acao para uso no codegen.
    pub fn as_generated_action(&self) -> &'static str {
        // Retorna &'static str (string literal) em vez de String para evitar alocacao.
        // O codegen usa essa string para escrever "Emit" ou "Skip" no arquivo .rs gerado.
        // O lifetime 'static garante que o valor vive para sempre (e' uma string literal).
        match self {
            Self::Emit(_) => "Emit", // Nao precisamos do TokenKind aqui, so do nome da acao.
            Self::Skip => "Skip",
        }
    }
}
