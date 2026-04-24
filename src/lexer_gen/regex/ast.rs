/// No da AST de expressao regular.
///
/// Esta estrutura e' a entrada do algoritmo de Thompson. Cada variante mapeia
/// diretamente para uma construcao classica de automato:
/// - `Concat` para sequencia
/// - `Alternation` para uniao (`|`)
/// - `Star`/`Plus`/`Optional` para repeticoes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// `Clone` (nao `Copy`): Regex e' uma arvore recursiva com Vec e Box — nao e' copiavel sem clone.
// `PartialEq, Eq`: permite comparar duas ASTs (util para testes de parser).
// `Hash`: permite guardar Regex em HashSet (usado na construcao de subconjuntos do DFA).
pub enum Regex {
    /// Conjunto vazio (nao aceita nenhuma string).
    EmptySet,
    // Representa a linguagem vazia L = {}. Resulta em nenhuma transicao no NFA.

    /// String vazia.
    Epsilon,
    // Representa a string de tamanho zero. No NFA de Thompson, vira uma e-transicao direta.

    /// Caractere literal.
    Literal(char),
    // Ex: o 'a' na RE "abc". Thompson cria um NFA de dois estados com transicao pelo char.

    /// Coringa `.`
    AnyChar,
    // Corresponde a qualquer caractere exceto newline (convencao POSIX).
    // No NFA, gera transicoes para todos os caracteres do alfabeto exceto '\n'.

    /// Classe de caracteres `[ ... ]`.
    CharClass(CharClass),
    // Ex: [a-z], [0-9], [^abc]. Armazena a definicao da classe para o Thompson expandir.

    /// Concatenacao de subexpressoes.
    Concat(Vec<Regex>),
    // Ex: "abc" -> Concat([Literal('a'), Literal('b'), Literal('c')]).
    // Thompson liga o estado de aceitacao de cada parte ao estado inicial da proxima.
    // Vec em vez de Box<(Regex, Regex)> permite concatenacoes longas sem encadeamento profundo.

    /// Alternacao de subexpressoes.
    Alternation(Vec<Regex>),
    // Ex: "a|b|c" -> Alternation([Literal('a'), Literal('b'), Literal('c')]).
    // Thompson cria um novo estado inicial com e-transicoes para cada ramo, e um
    // novo estado de aceitacao recebendo e-transicoes de todos os estados finais dos ramos.

    /// Fechamento de Kleene (`*`).
    Star(Box<Regex>),
    // Ex: "a*" -> Star(Literal('a')). Zero ou mais repeticoes.
    // Box e' necessario porque o enum seria auto-referencial (tamanho infinito) sem ponteiro.
    // Thompson adiciona um loop de e-transicao do estado final de volta ao inicial,
    // e uma e-transicao direta do inicio para o fim (permitindo zero ocorrencias).

    /// Fechamento positivo (`+`).
    Plus(Box<Regex>),
    // Ex: "a+" -> Plus(Literal('a')). Uma ou mais repeticoes.
    // Equivalente a Concat([r, Star(r)]) mas representado diretamente para evitar duplicar a subarvore.

    /// Opcional (`?`).
    Optional(Box<Regex>),
    // Ex: "a?" -> Optional(Literal('a')). Zero ou uma ocorrencia.
    // Thompson adiciona uma e-transicao direta do estado inicial ao final (bypass).
}

/// Classe de caracteres da forma `[abc]`, `[a-z]` ou `[^0-9]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharClass {
    /// Se verdadeiro, inverte o resultado da classe (`[^ ... ]`).
    pub negated: bool,
    // Ao inves de expandir todos os caracteres do complemento (potencialmente 65536+),
    // guardamos apenas a flag de negacao e invertemos na hora de checar.

    /// Itens que compoem a classe.
    pub items: Vec<CharClassItem>,
    // Lista de caracteres ou intervalos que fazem parte desta classe.
    // Ex: [a-z0-9_] -> [Range('a','z'), Range('0','9'), Char('_')]
}

/// Unidade interna de uma classe de caracteres.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharClassItem {
    /// Um unico caractere.
    Char(char),
    // Ex: 'a' em [abc].

    /// Intervalo inclusivo, por exemplo `a-z`.
    Range(char, char),
    // Ex: 'a'-'z' em [a-z]. Os dois extremos sao inclusos na verificacao.
}

impl CharClass {
    /// Verifica se um caractere pertence a esta classe.
    pub fn matches(&self, ch: char) -> bool {
        let hit = self.items.iter().any(|item| match item {
            // Itera pelos itens e checa se `ch` satisfaz algum deles.
            CharClassItem::Char(c) => *c == ch,
            // Comparacao simples: o caractere e' exatamente este?
            CharClassItem::Range(lo, hi) => *lo <= ch && ch <= *hi,
            // Comparacao de intervalo: usa a ordenacao Unicode natural dos chars.
        });
        if self.negated { !hit } else { hit }
        // Se `negated` e' true ([^...]), inverte o resultado.
        // Isso evita ter que expandir o complemento inteiro do alfabeto.
    }
}
