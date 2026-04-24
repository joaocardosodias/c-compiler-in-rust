//! Tipos fundamentais do scanner.
//!
//! Estes tipos sao compartilhados por todas as fases seguintes do compilador.
//! A ideia e' manter um contrato estavel para que parser/sema/codegen nao
//! dependam de detalhes internos da implementacao do lexer.

/// Posicao absoluta de um ponto no arquivo-fonte.
///
/// Mantemos tres medidas porque cada uma resolve um problema diferente:
/// - `offset`: index linear (bom para slices e diagnosticos internos)
/// - `line`: linha humana para mensagens de erro
/// - `column`: coluna humana para mensagens de erro
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// `Debug`: permite imprimir com {:?} em testes e logs.
// `Clone, Copy`: Position e' pequena (3 usizes) e barata de copiar — sem necessidade de Box/Rc.
// `PartialEq, Eq`: permite comparar posicoes com ==, util em testes e asserts.
// `Hash`: permite usar Position como chave em HashMap/HashSet (pode ser util para tabelas de simbolos).
pub struct Position {
    pub offset: usize, // Byte-offset no arquivo. Usado para fatiar strings: &source[offset..].
    pub line: usize,   // Linha (1-indexed). Exibida em mensagens de erro para o usuario.
    pub column: usize, // Coluna (1-indexed). Exibida junto com a linha no diagnostico.
}

impl Position {
    /// Construtor simples para facilitar criacao em testes e no scanner.
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        // `const fn`: pode ser chamada em tempo de compilacao (ex: em constantes estaticas).
        // Isso evita alocar qualquer coisa em runtime para posicoes fixas.
        Self {
            offset,
            line,
            column,
        }
    }
}

/// Intervalo semiaberto de um lexema no fonte: `[start, end)`.
///
/// Usar intervalo semiaberto evita ambiguidades ao concatenar spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// Mesma logica de derives de Position: Span e' pequeno (2x Position = 6 usizes), copiavel e comparavel.
pub struct Span {
    pub start: Position, // Primeira posicao do lexema (inclusive).
    pub end: Position,   // Primeira posicao APOS o lexema (exclusive). Convencao semiaberta [start, end).
}

impl Span {
    /// Construtor de span.
    pub const fn new(start: Position, end: Position) -> Self {
        // Tambem `const fn` pelo mesmo motivo de Position::new.
        Self { start, end }
    }
}

/// Categoria sintatica de token reconhecida pelo scanner.
///
/// O enum cobre um conjunto amplo de tokens de C para que o pipeline RE->NFA->DFA
/// possa evoluir sem quebrar assinatura publica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// `Copy`: TokenKind e' apenas uma tag numerica (discriminante do enum). Copiar e' gratuito.
// Usar enum ao inves de string evita comparacoes de string em toda a fase de parsing.
pub enum TokenKind {
    // --- Keywords da linguagem C (C89 + C99 + C11) ---
    // Cada variante representa exatamente uma palavra reservada.
    // Dividir em variantes distintas (ao inves de um Keyword(String)) permite
    // que o parser use `match` com zero custo de comparacao de string.
    KwAuto,
    KwBreak,
    KwCase,
    KwChar,
    KwConst,
    KwContinue,
    KwDefault,
    KwDo,
    KwDouble,
    KwElse,
    KwEnum,
    KwExtern,
    KwFloat,
    KwFor,
    KwGoto,
    KwIf,
    KwInline,
    KwInt,
    KwLong,
    KwRegister,
    KwRestrict,
    KwReturn,
    KwShort,
    KwSigned,
    KwSizeof,
    KwStatic,
    KwStruct,
    KwSwitch,
    KwTypedef,
    KwUnion,
    KwUnsigned,
    KwVoid,
    KwVolatile,
    KwWhile,
    // Keywords adicionadas em C11 (_Alignas, _Atomic, etc.).
    KwAlignas,
    KwAlignof,
    KwAtomic,
    KwBool,
    KwComplex,
    KwGeneric,
    KwImaginary,
    KwNoreturn,
    KwStaticAssert,
    KwThreadLocal,

    // --- Tokens de valor variavel ---
    Identifier,    // Nome de variavel, funcao, tipo definido pelo usuario. Ex: "minha_var".
    IntLiteral,    // Numero inteiro literal. Ex: 42, 0x1F.
    FloatLiteral,  // Numero de ponto flutuante. Ex: 3.14, 1e-5.
    CharLiteral,   // Literal de caractere. Ex: 'a', '\n'.
    StringLiteral, // Literal de string. Ex: "hello world".

    // --- Operadores aritmeticos ---
    Plus,    // +
    Minus,   // -
    Star,    // * (tambem usado como ponteiro/deref no contexto do parser)
    Slash,   // /
    Percent, // %

    // --- Operadores de incremento/decremento ---
    Increment, // ++
    Decrement, // --

    // --- Operadores de atribuicao ---
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=
    StarAssign,    // *=
    SlashAssign,   // /=
    PercentAssign, // %=

    // --- Operadores de comparacao ---
    EqEq,  // ==
    NotEq, // !=
    Lt,    // <
    Le,    // <=
    Gt,    // >
    Ge,    // >=

    // --- Operadores logicos ---
    AndAnd, // &&
    OrOr,   // ||
    Not,    // !

    // --- Operadores bitwise ---
    Amp,       // & (tambem usado como operador de referencia no parser)
    Pipe,      // |
    Caret,     // ^
    Tilde,     // ~
    AndAssign, // &=
    OrAssign,  // |=
    XorAssign, // ^=
    Shl,       // <<
    Shr,       // >>
    ShlAssign, // <<=
    ShrAssign, // >>=

    // --- Operadores e pontuacao especial ---
    Question,  // ? (operador ternario)
    Colon,     // :
    Arrow,     // -> (acesso a membro via ponteiro)
    Dot,       // . (acesso a membro direto)
    Ellipsis,  // ... (varargs)
    Comma,     // ,
    Semicolon, // ;

    // --- Delimitadores ---
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // --- Preprocessador ---
    Hash,     // #  (diretiva de preprocessador, ex: #include)
    HashHash, // ## (concatenacao de tokens no preprocessador)

    // Marcador de fim de arquivo. O parser usa este token para saber que nao ha mais input.
    Eof,
}

/// Token produzido pelo scanner.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// `Clone` (nao `Copy`) porque `lexeme` e' uma String alocada no heap — copiar exige clone explicito.
pub struct Token {
    /// Classe sintatica do token.
    pub kind: TokenKind, // O "tipo" do token: o que ele representa semanticamente.
    /// Texto bruto reconhecido no fonte.
    pub lexeme: String, // O texto exato que o scanner leu. Ex: para KwIf, e' a string "if".
    /// Intervalo do lexema no arquivo de entrada.
    pub span: Span, // Onde no arquivo este token esta. Usado para mensagens de erro no parser.
}

impl Token {
    /// Construtor de token.
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Self {
        // `impl Into<String>`: aceita &str, String ou qualquer tipo convertivel.
        // Isso evita que o chamador precise fazer .to_string() explicitamente.
        Self {
            kind,
            lexeme: lexeme.into(), // Converte o argumento para String aqui, uma unica vez.
            span,
        }
    }
}

/// Categoria de erro lexico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// `Copy`: assim como TokenKind, e' apenas uma tag. Nao tem dados alocados.
pub enum LexErrorKind {
    UnexpectedChar,      // Caractere que nao inicia nenhum token valido. Ex: '@' no meio do codigo.
    UnterminatedComment, // Comentario de bloco (/* ... ) que nunca foi fechado.
    UnterminatedString,  // String literal que nao foi fechada com aspas duplas.
    UnterminatedChar,    // Literal de char que nao foi fechado com aspas simples.
    InvalidEscape,       // Sequencia de escape invalida dentro de string/char. Ex: '\q'.
    InvalidNumber,       // Numero malformado. Ex: 0x (hexadecimal sem digitos).
}

/// Erro lexico com localizacao e mensagem amigavel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// `Clone` (nao `Copy`) pela mesma razao de Token: `message` e' uma String no heap.
pub struct LexError {
    /// Tipo estruturado do erro.
    pub kind: LexErrorKind, // Permite que o chamador trate cada categoria de erro diferentemente.
    /// Faixa no fonte onde o erro ocorreu.
    pub span: Span, // Indica ao usuario exatamente onde no arquivo o erro esta.
    /// Texto pronto para diagnostico.
    pub message: String, // Mensagem legivel por humano. Ex: "unexpected character '@'".
}

impl LexError {
    /// Construtor de erro lexico.
    pub fn new(kind: LexErrorKind, span: Span, message: impl Into<String>) -> Self {
        // Mesmo padrao de `impl Into<String>` do Token::new: flexivel sem custo.
        Self {
            kind,
            span,
            message: message.into(),
        }
    }
}
