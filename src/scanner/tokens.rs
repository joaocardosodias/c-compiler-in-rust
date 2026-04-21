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
pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    /// Construtor simples para facilitar criacao em testes e no scanner.
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
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
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// Construtor de span.
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Categoria sintatica de token reconhecida pelo scanner.
///
/// O enum cobre um conjunto amplo de tokens de C para que o pipeline RE->NFA->DFA
/// possa evoluir sem quebrar assinatura publica.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
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
    Identifier,
    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Increment,
    Decrement,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    AndAnd,
    OrOr,
    Not,
    Amp,
    Pipe,
    Caret,
    Tilde,
    AndAssign,
    OrAssign,
    XorAssign,
    Shl,
    Shr,
    ShlAssign,
    ShrAssign,
    Question,
    Colon,
    Arrow,
    Dot,
    Ellipsis,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Hash,
    HashHash,
    Eof,
}

/// Token produzido pelo scanner.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    /// Classe sintatica do token.
    pub kind: TokenKind,
    /// Texto bruto reconhecido no fonte.
    pub lexeme: String,
    /// Intervalo do lexema no arquivo de entrada.
    pub span: Span,
}

impl Token {
    /// Construtor de token.
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }
}

/// Categoria de erro lexico.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    UnexpectedChar,
    UnterminatedComment,
    UnterminatedString,
    UnterminatedChar,
    InvalidEscape,
    InvalidNumber,
}

/// Erro lexico com localizacao e mensagem amigavel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LexError {
    /// Tipo estruturado do erro.
    pub kind: LexErrorKind,
    /// Faixa no fonte onde o erro ocorreu.
    pub span: Span,
    /// Texto pronto para diagnostico.
    pub message: String,
}

impl LexError {
    /// Construtor de erro lexico.
    pub fn new(kind: LexErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }
}
