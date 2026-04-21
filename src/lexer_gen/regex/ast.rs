/// No da AST de expressao regular.
///
/// Esta estrutura e' a entrada do algoritmo de Thompson. Cada variante mapeia
/// diretamente para uma construcao classica de automato:
/// - `Concat` para sequencia
/// - `Alternation` para uniao (`|`)
/// - `Star`/`Plus`/`Optional` para repeticoes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Regex {
    /// Conjunto vazio (nao aceita nenhuma string).
    EmptySet,
    /// String vazia.
    Epsilon,
    /// Caractere literal.
    Literal(char),
    /// Coringa `.`
    AnyChar,
    /// Classe de caracteres `[ ... ]`.
    CharClass(CharClass),
    /// Concatenacao de subexpressoes.
    Concat(Vec<Regex>),
    /// Alternacao de subexpressoes.
    Alternation(Vec<Regex>),
    /// Fechamento de Kleene (`*`).
    Star(Box<Regex>),
    /// Fechamento positivo (`+`).
    Plus(Box<Regex>),
    /// Opcional (`?`).
    Optional(Box<Regex>),
}

/// Classe de caracteres da forma `[abc]`, `[a-z]` ou `[^0-9]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharClass {
    /// Se verdadeiro, inverte o resultado da classe (`[^ ... ]`).
    pub negated: bool,
    /// Itens que compoem a classe.
    pub items: Vec<CharClassItem>,
}

/// Unidade interna de uma classe de caracteres.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharClassItem {
    /// Um unico caractere.
    Char(char),
    /// Intervalo inclusivo, por exemplo `a-z`.
    Range(char, char),
}

impl CharClass {
    /// Verifica se um caractere pertence a esta classe.
    pub fn matches(&self, ch: char) -> bool {
        let hit = self.items.iter().any(|item| match item {
            CharClassItem::Char(c) => *c == ch,
            CharClassItem::Range(lo, hi) => *lo <= ch && ch <= *hi,
        });
        if self.negated { !hit } else { hit }
    }
}
