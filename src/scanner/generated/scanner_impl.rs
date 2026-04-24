// This file is generated. Do not edit by hand.
use crate::scanner::tokens::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedAcceptAction {
    Emit(TokenKind),
    Skip,
}

pub fn dfa_accept_action(state: usize) -> Option<GeneratedAcceptAction> {
    match state {
        1 => Some(GeneratedAcceptAction::Skip),
        2 => Some(GeneratedAcceptAction::Emit(TokenKind::Slash)),
        3 => Some(GeneratedAcceptAction::Emit(TokenKind::Star)),
        4 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        5 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        6 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        7 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        8 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        9 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        10 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        11 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        12 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        13 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        14 => Some(GeneratedAcceptAction::Emit(TokenKind::IntLiteral)),
        15 => Some(GeneratedAcceptAction::Emit(TokenKind::IntLiteral)),
        16 => Some(GeneratedAcceptAction::Emit(TokenKind::Dot)),
        17 => Some(GeneratedAcceptAction::Emit(TokenKind::Minus)),
        18 => Some(GeneratedAcceptAction::Emit(TokenKind::Gt)),
        19 => Some(GeneratedAcceptAction::Emit(TokenKind::Plus)),
        20 => Some(GeneratedAcceptAction::Emit(TokenKind::Lt)),
        21 => Some(GeneratedAcceptAction::Emit(TokenKind::Assign)),
        22 => Some(GeneratedAcceptAction::Emit(TokenKind::Percent)),
        23 => Some(GeneratedAcceptAction::Emit(TokenKind::Amp)),
        24 => Some(GeneratedAcceptAction::Emit(TokenKind::Pipe)),
        25 => Some(GeneratedAcceptAction::Emit(TokenKind::Caret)),
        26 => Some(GeneratedAcceptAction::Emit(TokenKind::Not)),
        27 => Some(GeneratedAcceptAction::Emit(TokenKind::Tilde)),
        28 => Some(GeneratedAcceptAction::Emit(TokenKind::Question)),
        29 => Some(GeneratedAcceptAction::Emit(TokenKind::Colon)),
        30 => Some(GeneratedAcceptAction::Emit(TokenKind::Comma)),
        31 => Some(GeneratedAcceptAction::Emit(TokenKind::Semicolon)),
        32 => Some(GeneratedAcceptAction::Emit(TokenKind::LParen)),
        33 => Some(GeneratedAcceptAction::Emit(TokenKind::RParen)),
        34 => Some(GeneratedAcceptAction::Emit(TokenKind::LBrace)),
        35 => Some(GeneratedAcceptAction::Emit(TokenKind::RBrace)),
        36 => Some(GeneratedAcceptAction::Emit(TokenKind::LBracket)),
        37 => Some(GeneratedAcceptAction::Emit(TokenKind::RBracket)),
        38 => Some(GeneratedAcceptAction::Emit(TokenKind::Hash)),
        39 => Some(GeneratedAcceptAction::Skip),
        41 => Some(GeneratedAcceptAction::Emit(TokenKind::SlashAssign)),
        42 => Some(GeneratedAcceptAction::Emit(TokenKind::StarAssign)),
        43 => Some(GeneratedAcceptAction::Emit(TokenKind::KwIf)),
        44 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        45 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        46 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        47 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        48 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        49 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        50 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        51 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        52 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        54 => Some(GeneratedAcceptAction::Emit(TokenKind::Decrement)),
        55 => Some(GeneratedAcceptAction::Emit(TokenKind::Arrow)),
        56 => Some(GeneratedAcceptAction::Emit(TokenKind::MinusAssign)),
        57 => Some(GeneratedAcceptAction::Emit(TokenKind::Shr)),
        58 => Some(GeneratedAcceptAction::Emit(TokenKind::Ge)),
        59 => Some(GeneratedAcceptAction::Emit(TokenKind::Increment)),
        60 => Some(GeneratedAcceptAction::Emit(TokenKind::PlusAssign)),
        61 => Some(GeneratedAcceptAction::Emit(TokenKind::Shl)),
        62 => Some(GeneratedAcceptAction::Emit(TokenKind::Le)),
        63 => Some(GeneratedAcceptAction::Emit(TokenKind::EqEq)),
        64 => Some(GeneratedAcceptAction::Emit(TokenKind::PercentAssign)),
        65 => Some(GeneratedAcceptAction::Emit(TokenKind::AndAssign)),
        66 => Some(GeneratedAcceptAction::Emit(TokenKind::AndAnd)),
        67 => Some(GeneratedAcceptAction::Emit(TokenKind::OrAssign)),
        68 => Some(GeneratedAcceptAction::Emit(TokenKind::OrOr)),
        69 => Some(GeneratedAcceptAction::Emit(TokenKind::XorAssign)),
        70 => Some(GeneratedAcceptAction::Emit(TokenKind::NotEq)),
        71 => Some(GeneratedAcceptAction::Emit(TokenKind::HashHash)),
        73 => Some(GeneratedAcceptAction::Emit(TokenKind::KwInt)),
        74 => Some(GeneratedAcceptAction::Emit(TokenKind::KwFor)),
        75 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        76 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        77 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        78 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        79 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        80 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        81 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        82 => Some(GeneratedAcceptAction::Emit(TokenKind::Ellipsis)),
        83 => Some(GeneratedAcceptAction::Emit(TokenKind::ShrAssign)),
        84 => Some(GeneratedAcceptAction::Emit(TokenKind::ShlAssign)),
        85 => Some(GeneratedAcceptAction::Skip),
        86 => Some(GeneratedAcceptAction::Emit(TokenKind::KwElse)),
        87 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        88 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        89 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        90 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        91 => Some(GeneratedAcceptAction::Emit(TokenKind::KwChar)),
        92 => Some(GeneratedAcceptAction::Emit(TokenKind::KwVoid)),
        93 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        94 => Some(GeneratedAcceptAction::Emit(TokenKind::KwWhile)),
        95 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        96 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        97 => Some(GeneratedAcceptAction::Emit(TokenKind::KwStruct)),
        98 => Some(GeneratedAcceptAction::Emit(TokenKind::KwReturn)),
        99 => Some(GeneratedAcceptAction::Emit(TokenKind::Identifier)),
        100 => Some(GeneratedAcceptAction::Emit(TokenKind::KwTypedef)),
        _ => None,
    }
}

pub fn dfa_next_state(state: usize, ch: char) -> Option<usize> {
    match state {
        0 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(15)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if (ch == '\t' || ch == '\r' || ch == ' ') {
                Some(1)
            } else if ch == '\n' {
                Some(1)
            } else if ch == '!' {
                Some(26)
            } else if ch == '#' {
                Some(38)
            } else if ch == '%' {
                Some(22)
            } else if ch == '&' {
                Some(23)
            } else if ch == '(' {
                Some(32)
            } else if ch == ')' {
                Some(33)
            } else if ch == '*' {
                Some(3)
            } else if ch == '+' {
                Some(19)
            } else if ch == ',' {
                Some(30)
            } else if ch == '-' {
                Some(17)
            } else if ch == '.' {
                Some(16)
            } else if ch == '/' {
                Some(2)
            } else if ch == '0' {
                Some(14)
            } else if ch == ':' {
                Some(29)
            } else if ch == ';' {
                Some(31)
            } else if ch == '<' {
                Some(20)
            } else if ch == '=' {
                Some(21)
            } else if ch == '>' {
                Some(18)
            } else if ch == '?' {
                Some(28)
            } else if ch == '[' {
                Some(36)
            } else if ch == ']' {
                Some(37)
            } else if ch == '^' {
                Some(25)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(12)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(6)
            } else if ch == 'f' {
                Some(5)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(4)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(10)
            } else if ch == 's' {
                Some(8)
            } else if ch == 't' {
                Some(11)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(13)
            } else if ch == 'w' {
                Some(9)
            } else if ch == 'y' {
                Some(7)
            } else if ch == '{' {
                Some(34)
            } else if ch == '|' {
                Some(24)
            } else if ch == '}' {
                Some(35)
            } else if ch == '~' {
                Some(27)
            } else {
                None
            }
        }
        1 => {
            if (ch == '\t' || ch == '\r' || ch == ' ') {
                Some(1)
            } else if ch == '\n' {
                Some(1)
            } else {
                None
            }
        }
        2 => {
            if ch == '*' {
                Some(40)
            } else if ch == '/' {
                Some(39)
            } else if ch == '=' {
                Some(41)
            } else {
                None
            }
        }
        3 => {
            if ch == '=' {
                Some(42)
            } else {
                None
            }
        }
        4 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(43)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(44)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        5 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(45)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        6 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(46)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        7 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        8 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(47)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        9 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(48)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        10 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(49)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        11 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(50)
            } else {
                None
            }
        }
        12 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(51)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        13 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(52)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        14 => None,
        15 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(15)
            } else if ch == '0' {
                Some(15)
            } else {
                None
            }
        }
        16 => {
            if ch == '.' {
                Some(53)
            } else {
                None
            }
        }
        17 => {
            if ch == '-' {
                Some(54)
            } else if ch == '=' {
                Some(56)
            } else if ch == '>' {
                Some(55)
            } else {
                None
            }
        }
        18 => {
            if ch == '=' {
                Some(58)
            } else if ch == '>' {
                Some(57)
            } else {
                None
            }
        }
        19 => {
            if ch == '+' {
                Some(59)
            } else if ch == '=' {
                Some(60)
            } else {
                None
            }
        }
        20 => {
            if ch == '<' {
                Some(61)
            } else if ch == '=' {
                Some(62)
            } else {
                None
            }
        }
        21 => {
            if ch == '=' {
                Some(63)
            } else {
                None
            }
        }
        22 => {
            if ch == '=' {
                Some(64)
            } else {
                None
            }
        }
        23 => {
            if ch == '&' {
                Some(66)
            } else if ch == '=' {
                Some(65)
            } else {
                None
            }
        }
        24 => {
            if ch == '=' {
                Some(67)
            } else if ch == '|' {
                Some(68)
            } else {
                None
            }
        }
        25 => {
            if ch == '=' {
                Some(69)
            } else {
                None
            }
        }
        26 => {
            if ch == '=' {
                Some(70)
            } else {
                None
            }
        }
        27 => None,
        28 => None,
        29 => None,
        30 => None,
        31 => None,
        32 => None,
        33 => None,
        34 => None,
        35 => None,
        36 => None,
        37 => None,
        38 => {
            if ch == '#' {
                Some(71)
            } else {
                None
            }
        }
        39 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(39)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(39)
            } else if (ch == '\0'
                || ch == '\u{1}'
                || ch == '\u{2}'
                || ch == '\u{3}'
                || ch == '\u{4}'
                || ch == '\u{5}'
                || ch == '\u{6}'
                || ch == '\u{7}'
                || ch == '\u{8}'
                || ch == '\u{b}'
                || ch == '\u{c}'
                || ch == '\u{e}'
                || ch == '\u{f}'
                || ch == '\u{10}'
                || ch == '\u{11}'
                || ch == '\u{12}'
                || ch == '\u{13}'
                || ch == '\u{14}'
                || ch == '\u{15}'
                || ch == '\u{16}'
                || ch == '\u{17}'
                || ch == '\u{18}'
                || ch == '\u{19}'
                || ch == '\u{1a}'
                || ch == '\u{1b}'
                || ch == '\u{1c}'
                || ch == '\u{1d}'
                || ch == '\u{1e}'
                || ch == '\u{1f}'
                || ch == '"'
                || ch == '$'
                || ch == '\''
                || ch == '@'
                || ch == '\\'
                || ch == '`'
                || ch == '\u{7f}')
            {
                Some(39)
            } else if (ch == '\t' || ch == '\r' || ch == ' ') {
                Some(39)
            } else if ch == '!' {
                Some(39)
            } else if ch == '#' {
                Some(39)
            } else if ch == '%' {
                Some(39)
            } else if ch == '&' {
                Some(39)
            } else if ch == '(' {
                Some(39)
            } else if ch == ')' {
                Some(39)
            } else if ch == '*' {
                Some(39)
            } else if ch == '+' {
                Some(39)
            } else if ch == ',' {
                Some(39)
            } else if ch == '-' {
                Some(39)
            } else if ch == '.' {
                Some(39)
            } else if ch == '/' {
                Some(39)
            } else if ch == '0' {
                Some(39)
            } else if ch == ':' {
                Some(39)
            } else if ch == ';' {
                Some(39)
            } else if ch == '<' {
                Some(39)
            } else if ch == '=' {
                Some(39)
            } else if ch == '>' {
                Some(39)
            } else if ch == '?' {
                Some(39)
            } else if ch == '[' {
                Some(39)
            } else if ch == ']' {
                Some(39)
            } else if ch == '^' {
                Some(39)
            } else if ch == 'a' {
                Some(39)
            } else if ch == 'c' {
                Some(39)
            } else if ch == 'd' {
                Some(39)
            } else if ch == 'e' {
                Some(39)
            } else if ch == 'f' {
                Some(39)
            } else if ch == 'h' {
                Some(39)
            } else if ch == 'i' {
                Some(39)
            } else if ch == 'l' {
                Some(39)
            } else if ch == 'n' {
                Some(39)
            } else if ch == 'o' {
                Some(39)
            } else if ch == 'p' {
                Some(39)
            } else if ch == 'r' {
                Some(39)
            } else if ch == 's' {
                Some(39)
            } else if ch == 't' {
                Some(39)
            } else if ch == 'u' {
                Some(39)
            } else if ch == 'v' {
                Some(39)
            } else if ch == 'w' {
                Some(39)
            } else if ch == 'y' {
                Some(39)
            } else if ch == '{' {
                Some(39)
            } else if ch == '|' {
                Some(39)
            } else if ch == '}' {
                Some(39)
            } else if ch == '~' {
                Some(39)
            } else {
                None
            }
        }
        40 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(40)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(40)
            } else if (ch == '\0'
                || ch == '\u{1}'
                || ch == '\u{2}'
                || ch == '\u{3}'
                || ch == '\u{4}'
                || ch == '\u{5}'
                || ch == '\u{6}'
                || ch == '\u{7}'
                || ch == '\u{8}'
                || ch == '\u{b}'
                || ch == '\u{c}'
                || ch == '\u{e}'
                || ch == '\u{f}'
                || ch == '\u{10}'
                || ch == '\u{11}'
                || ch == '\u{12}'
                || ch == '\u{13}'
                || ch == '\u{14}'
                || ch == '\u{15}'
                || ch == '\u{16}'
                || ch == '\u{17}'
                || ch == '\u{18}'
                || ch == '\u{19}'
                || ch == '\u{1a}'
                || ch == '\u{1b}'
                || ch == '\u{1c}'
                || ch == '\u{1d}'
                || ch == '\u{1e}'
                || ch == '\u{1f}'
                || ch == '"'
                || ch == '$'
                || ch == '\''
                || ch == '@'
                || ch == '\\'
                || ch == '`'
                || ch == '\u{7f}')
            {
                Some(40)
            } else if (ch == '\t' || ch == '\r' || ch == ' ') {
                Some(40)
            } else if ch == '\n' {
                Some(40)
            } else if ch == '!' {
                Some(40)
            } else if ch == '#' {
                Some(40)
            } else if ch == '%' {
                Some(40)
            } else if ch == '&' {
                Some(40)
            } else if ch == '(' {
                Some(40)
            } else if ch == ')' {
                Some(40)
            } else if ch == '*' {
                Some(72)
            } else if ch == '+' {
                Some(40)
            } else if ch == ',' {
                Some(40)
            } else if ch == '-' {
                Some(40)
            } else if ch == '.' {
                Some(40)
            } else if ch == '/' {
                Some(40)
            } else if ch == '0' {
                Some(40)
            } else if ch == ':' {
                Some(40)
            } else if ch == ';' {
                Some(40)
            } else if ch == '<' {
                Some(40)
            } else if ch == '=' {
                Some(40)
            } else if ch == '>' {
                Some(40)
            } else if ch == '?' {
                Some(40)
            } else if ch == '[' {
                Some(40)
            } else if ch == ']' {
                Some(40)
            } else if ch == '^' {
                Some(40)
            } else if ch == 'a' {
                Some(40)
            } else if ch == 'c' {
                Some(40)
            } else if ch == 'd' {
                Some(40)
            } else if ch == 'e' {
                Some(40)
            } else if ch == 'f' {
                Some(40)
            } else if ch == 'h' {
                Some(40)
            } else if ch == 'i' {
                Some(40)
            } else if ch == 'l' {
                Some(40)
            } else if ch == 'n' {
                Some(40)
            } else if ch == 'o' {
                Some(40)
            } else if ch == 'p' {
                Some(40)
            } else if ch == 'r' {
                Some(40)
            } else if ch == 's' {
                Some(40)
            } else if ch == 't' {
                Some(40)
            } else if ch == 'u' {
                Some(40)
            } else if ch == 'v' {
                Some(40)
            } else if ch == 'w' {
                Some(40)
            } else if ch == 'y' {
                Some(40)
            } else if ch == '{' {
                Some(40)
            } else if ch == '|' {
                Some(40)
            } else if ch == '}' {
                Some(40)
            } else if ch == '~' {
                Some(40)
            } else {
                None
            }
        }
        41 => None,
        42 => None,
        43 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        44 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(73)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        45 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(74)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        46 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(75)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        47 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(76)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        48 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(77)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        49 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(78)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        50 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(79)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        51 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(80)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        52 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(81)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        53 => {
            if ch == '.' {
                Some(82)
            } else {
                None
            }
        }
        54 => None,
        55 => None,
        56 => None,
        57 => {
            if ch == '=' {
                Some(83)
            } else {
                None
            }
        }
        58 => None,
        59 => None,
        60 => None,
        61 => {
            if ch == '=' {
                Some(84)
            } else {
                None
            }
        }
        62 => None,
        63 => None,
        64 => None,
        65 => None,
        66 => None,
        67 => None,
        68 => None,
        69 => None,
        70 => None,
        71 => None,
        72 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(40)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(40)
            } else if (ch == '\0'
                || ch == '\u{1}'
                || ch == '\u{2}'
                || ch == '\u{3}'
                || ch == '\u{4}'
                || ch == '\u{5}'
                || ch == '\u{6}'
                || ch == '\u{7}'
                || ch == '\u{8}'
                || ch == '\u{b}'
                || ch == '\u{c}'
                || ch == '\u{e}'
                || ch == '\u{f}'
                || ch == '\u{10}'
                || ch == '\u{11}'
                || ch == '\u{12}'
                || ch == '\u{13}'
                || ch == '\u{14}'
                || ch == '\u{15}'
                || ch == '\u{16}'
                || ch == '\u{17}'
                || ch == '\u{18}'
                || ch == '\u{19}'
                || ch == '\u{1a}'
                || ch == '\u{1b}'
                || ch == '\u{1c}'
                || ch == '\u{1d}'
                || ch == '\u{1e}'
                || ch == '\u{1f}'
                || ch == '"'
                || ch == '$'
                || ch == '\''
                || ch == '@'
                || ch == '\\'
                || ch == '`'
                || ch == '\u{7f}')
            {
                Some(40)
            } else if (ch == '\t' || ch == '\r' || ch == ' ') {
                Some(40)
            } else if ch == '\n' {
                Some(40)
            } else if ch == '!' {
                Some(40)
            } else if ch == '#' {
                Some(40)
            } else if ch == '%' {
                Some(40)
            } else if ch == '&' {
                Some(40)
            } else if ch == '(' {
                Some(40)
            } else if ch == ')' {
                Some(40)
            } else if ch == '*' {
                Some(72)
            } else if ch == '+' {
                Some(40)
            } else if ch == ',' {
                Some(40)
            } else if ch == '-' {
                Some(40)
            } else if ch == '.' {
                Some(40)
            } else if ch == '/' {
                Some(85)
            } else if ch == '0' {
                Some(40)
            } else if ch == ':' {
                Some(40)
            } else if ch == ';' {
                Some(40)
            } else if ch == '<' {
                Some(40)
            } else if ch == '=' {
                Some(40)
            } else if ch == '>' {
                Some(40)
            } else if ch == '?' {
                Some(40)
            } else if ch == '[' {
                Some(40)
            } else if ch == ']' {
                Some(40)
            } else if ch == '^' {
                Some(40)
            } else if ch == 'a' {
                Some(40)
            } else if ch == 'c' {
                Some(40)
            } else if ch == 'd' {
                Some(40)
            } else if ch == 'e' {
                Some(40)
            } else if ch == 'f' {
                Some(40)
            } else if ch == 'h' {
                Some(40)
            } else if ch == 'i' {
                Some(40)
            } else if ch == 'l' {
                Some(40)
            } else if ch == 'n' {
                Some(40)
            } else if ch == 'o' {
                Some(40)
            } else if ch == 'p' {
                Some(40)
            } else if ch == 'r' {
                Some(40)
            } else if ch == 's' {
                Some(40)
            } else if ch == 't' {
                Some(40)
            } else if ch == 'u' {
                Some(40)
            } else if ch == 'v' {
                Some(40)
            } else if ch == 'w' {
                Some(40)
            } else if ch == 'y' {
                Some(40)
            } else if ch == '{' {
                Some(40)
            } else if ch == '|' {
                Some(40)
            } else if ch == '}' {
                Some(40)
            } else if ch == '~' {
                Some(40)
            } else {
                None
            }
        }
        73 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        74 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        75 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(86)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        76 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(87)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        77 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(88)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        78 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(89)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        79 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(90)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        80 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(91)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        81 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(92)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        82 => None,
        83 => None,
        84 => None,
        85 => None,
        86 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        87 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(93)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        88 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(94)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        89 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(95)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        90 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(96)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        91 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        92 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        93 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(97)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        94 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        95 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(98)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        96 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(99)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        97 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        98 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        99 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(100)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        100 => {
            if (ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9')
            {
                Some(7)
            } else if (ch == 'A'
                || ch == 'B'
                || ch == 'C'
                || ch == 'D'
                || ch == 'E'
                || ch == 'F'
                || ch == 'G'
                || ch == 'H'
                || ch == 'I'
                || ch == 'J'
                || ch == 'K'
                || ch == 'L'
                || ch == 'M'
                || ch == 'N'
                || ch == 'O'
                || ch == 'P'
                || ch == 'Q'
                || ch == 'R'
                || ch == 'S'
                || ch == 'T'
                || ch == 'U'
                || ch == 'V'
                || ch == 'W'
                || ch == 'X'
                || ch == 'Y'
                || ch == 'Z'
                || ch == '_'
                || ch == 'b'
                || ch == 'g'
                || ch == 'j'
                || ch == 'k'
                || ch == 'm'
                || ch == 'q'
                || ch == 'x'
                || ch == 'z')
            {
                Some(7)
            } else if ch == '0' {
                Some(7)
            } else if ch == 'a' {
                Some(7)
            } else if ch == 'c' {
                Some(7)
            } else if ch == 'd' {
                Some(7)
            } else if ch == 'e' {
                Some(7)
            } else if ch == 'f' {
                Some(7)
            } else if ch == 'h' {
                Some(7)
            } else if ch == 'i' {
                Some(7)
            } else if ch == 'l' {
                Some(7)
            } else if ch == 'n' {
                Some(7)
            } else if ch == 'o' {
                Some(7)
            } else if ch == 'p' {
                Some(7)
            } else if ch == 'r' {
                Some(7)
            } else if ch == 's' {
                Some(7)
            } else if ch == 't' {
                Some(7)
            } else if ch == 'u' {
                Some(7)
            } else if ch == 'v' {
                Some(7)
            } else if ch == 'w' {
                Some(7)
            } else if ch == 'y' {
                Some(7)
            } else {
                None
            }
        }
        _ => None,
    }
}
