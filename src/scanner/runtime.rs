use crate::scanner::generated::{GeneratedAcceptAction, dfa_accept_action, dfa_next_state};

use crate::scanner::tokens::{LexError, LexErrorKind, Position, Span, Token, TokenKind};

pub fn scan_source(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let mut offset = 0usize;

    let mut line = 1usize;

    let mut column = 1usize;

    while offset < source.len() {
        let start = Position::new(offset, line, column);

        let mut state = 0usize;

        let mut cursor_line = line;
        let mut cursor_column = column;

        let mut last_accept: Option<(usize, usize, usize, GeneratedAcceptAction)> = None;

        let mut iter = source[offset..].char_indices();

        while let Some((rel, ch)) = iter.next() {
            let Some(next_state) = dfa_next_state(state, ch) else {
                break;
            };

            state = next_state;

            let cursor = offset + rel + ch.len_utf8();

            if ch == '\n' {
                cursor_line += 1;
                cursor_column = 1;
            } else {
                cursor_column += 1;
            }

            if let Some(action) = dfa_accept_action(state) {
                last_accept = Some((cursor, cursor_line, cursor_column, action));
            }
        }

        let Some((end_offset, end_line, end_column, action)) = last_accept else {
            let ch = source[offset..].chars().next().unwrap_or('\0');

            return Err(LexError::new(
                LexErrorKind::UnexpectedChar,
                Span::new(start, start),
                format!("unexpected character '{}'", ch),
            ));
        };

        match action {
            GeneratedAcceptAction::Emit(kind) => {
                let end = Position::new(end_offset, end_line, end_column);
                let lexeme = source[offset..end_offset].to_string();

                tokens.push(Token::new(kind, lexeme, Span::new(start, end)));
            }
            GeneratedAcceptAction::Skip => {}
        }

        offset = end_offset;

        line = end_line;
        column = end_column;
    }

    let eof_pos = Position::new(offset, line, column);

    tokens.push(Token::new(
        TokenKind::Eof,
        String::new(),
        Span::new(eof_pos, eof_pos),
    ));

    Ok(tokens)
}
