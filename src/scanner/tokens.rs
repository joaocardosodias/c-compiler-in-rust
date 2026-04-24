#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub struct Position {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        Self {
            offset,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Token {
    pub kind: TokenKind,

    pub lexeme: String,

    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

pub enum LexErrorKind {
    UnexpectedChar,
    UnterminatedComment,
    UnterminatedString,
    UnterminatedChar,
    InvalidEscape,
    InvalidNumber,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct LexError {
    pub kind: LexErrorKind,

    pub span: Span,

    pub message: String,
}

impl LexError {
    pub fn new(kind: LexErrorKind, span: Span, message: impl Into<String>) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }
}
