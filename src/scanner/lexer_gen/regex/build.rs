use super::ast::{CharClass, CharClassItem, Regex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexParseError {
    pub message: String,

    pub position: usize,
}

impl RegexParseError {
    fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}

pub fn parse_regex(input: &str) -> Result<Regex, RegexParseError> {
    let mut parser = Parser::new(input);
    let regex = parser.parse_regex()?;

    if !parser.is_eof() {
        return Err(RegexParseError::new(
            format!(
                "unexpected character '{}'",
                parser.peek_char().unwrap_or('\0')
            ),
            parser.pos,
        ));
    }
    Ok(regex)
}

struct Parser {
    chars: Vec<char>,

    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse_regex(&mut self) -> Result<Regex, RegexParseError> {
        self.parse_alternation()
    }

    fn parse_alternation(&mut self) -> Result<Regex, RegexParseError> {
        let mut arms = vec![self.parse_concatenation()?];

        while self.peek_char() == Some('|') {
            self.advance_char();
            arms.push(self.parse_concatenation()?);
        }

        Ok(Self::fold_alternation(arms))
    }

    fn parse_concatenation(&mut self) -> Result<Regex, RegexParseError> {
        let mut factors = Vec::new();

        while let Some(ch) = self.peek_char() {
            if ch == ')' || ch == '|' {
                break;
            }

            factors.push(self.parse_postfix()?);
        }

        Ok(Self::fold_concat(factors))
    }

    fn parse_postfix(&mut self) -> Result<Regex, RegexParseError> {
        let mut node = self.parse_atom()?;

        loop {
            match self.peek_char() {
                Some('*') => {
                    self.advance_char();
                    node = Regex::Star(Box::new(node));
                }
                Some('+') => {
                    self.advance_char();
                    node = Regex::Plus(Box::new(node));
                }
                Some('?') => {
                    self.advance_char();
                    node = Regex::Optional(Box::new(node));
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_atom(&mut self) -> Result<Regex, RegexParseError> {
        match self.peek_char() {
            None => Err(RegexParseError::new("unexpected end of regex", self.pos)),
            Some('(') => {
                self.advance_char();
                let inner = self.parse_alternation()?;
                self.expect_char(')')?;
                Ok(inner)
            }
            Some('[') => {
                self.advance_char();
                let class = self.parse_char_class()?;
                Ok(Regex::CharClass(class))
            }
            Some('.') => {
                self.advance_char();
                Ok(Regex::AnyChar)
            }
            Some('ε') => {
                self.advance_char();
                Ok(Regex::Epsilon)
            }
            Some('∅') => {
                self.advance_char();
                Ok(Regex::EmptySet)
            }

            Some(')') | Some('|') | Some('*') | Some('+') | Some('?') => {
                Err(RegexParseError::new("unexpected operator", self.pos))
            }
            Some('\\') => {
                self.advance_char();
                let escaped = self.parse_escape()?;
                Ok(Regex::Literal(escaped))
            }
            Some(ch) => {
                self.advance_char();
                Ok(Regex::Literal(ch))
            }
        }
    }

    fn parse_char_class(&mut self) -> Result<CharClass, RegexParseError> {
        let mut negated = false;
        let mut items = Vec::new();

        if self.peek_char() == Some('^') {
            self.advance_char();
            negated = true;
        }

        if self.peek_char() == Some(']') {
            return Err(RegexParseError::new("empty char class", self.pos));
        }

        loop {
            let Some(ch) = self.peek_char() else {
                return Err(RegexParseError::new("unterminated char class", self.pos));
            };

            if ch == ']' {
                self.advance_char();
                break;
            }

            let start = self.parse_class_char()?;

            if self.peek_char() == Some('-') {
                let dash_pos = self.pos;
                self.advance_char();

                match self.peek_char() {
                    Some(']') => {
                        items.push(CharClassItem::Char(start));
                        items.push(CharClassItem::Char('-'));
                    }
                    Some(_) => {
                        let end = self.parse_class_char()?;
                        if start > end {
                            return Err(RegexParseError::new("invalid class range", dash_pos));
                        }
                        items.push(CharClassItem::Range(start, end));
                    }
                    None => {
                        return Err(RegexParseError::new("unterminated char class", self.pos));
                    }
                }
            } else {
                items.push(CharClassItem::Char(start));
            }
        }

        Ok(CharClass { negated, items })
    }

    fn parse_class_char(&mut self) -> Result<char, RegexParseError> {
        match self.peek_char() {
            Some('\\') => {
                self.advance_char();
                self.parse_escape()
            }
            Some(ch) => {
                self.advance_char();
                Ok(ch)
            }
            None => Err(RegexParseError::new("unterminated char class", self.pos)),
        }
    }

    fn parse_escape(&mut self) -> Result<char, RegexParseError> {
        let Some(ch) = self.advance_char() else {
            return Err(RegexParseError::new("dangling escape", self.pos));
        };

        let escaped = match ch {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '\\' => '\\',
            '0' => '\0',
            other => other,
        };

        Ok(escaped)
    }

    fn expect_char(&mut self, expected: char) -> Result<(), RegexParseError> {
        if self.peek_char() == Some(expected) {
            self.advance_char();
            Ok(())
        } else {
            Err(RegexParseError::new(
                format!("expected '{}'", expected),
                self.pos,
            ))
        }
    }

    fn fold_concat(parts: Vec<Regex>) -> Regex {
        match parts.as_slice() {
            [] => Regex::Epsilon,
            [single] => single.clone(),
            _ => Regex::Concat(parts),
        }
    }

    fn fold_alternation(arms: Vec<Regex>) -> Regex {
        match arms.as_slice() {
            [] => Regex::Epsilon,
            [single] => single.clone(),
            _ => Regex::Alternation(arms),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += 1;
        Some(ch)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::parse_regex;
    use crate::scanner::lexer_gen::regex::ast::{CharClassItem, Regex};

    #[test]
    fn parses_alternation_concat_and_postfix() {
        let re = parse_regex("ab|c*").expect("should parse");
        assert_eq!(
            re,
            Regex::Alternation(vec![
                Regex::Concat(vec![Regex::Literal('a'), Regex::Literal('b')]),
                Regex::Star(Box::new(Regex::Literal('c'))),
            ])
        );
    }

    #[test]
    fn parses_char_class_ranges() {
        let re = parse_regex("[a-zA-Z0-9_]").expect("should parse");
        let Regex::CharClass(class) = re else {
            panic!("expected char class");
        };

        assert!(!class.negated);
        assert_eq!(
            class.items,
            vec![
                CharClassItem::Range('a', 'z'),
                CharClassItem::Range('A', 'Z'),
                CharClassItem::Range('0', '9'),
                CharClassItem::Char('_'),
            ]
        );
    }

    #[test]
    fn parses_grouping_plus_and_optional() {
        let re = parse_regex("(ab)+c?").expect("should parse");
        assert_eq!(
            re,
            Regex::Concat(vec![
                Regex::Plus(Box::new(Regex::Concat(vec![
                    Regex::Literal('a'),
                    Regex::Literal('b')
                ]))),
                Regex::Optional(Box::new(Regex::Literal('c'))),
            ])
        );
    }

    #[test]
    fn rejects_unbalanced_parens() {
        let err = parse_regex("(ab").expect_err("should fail");
        assert!(err.message.contains("expected ')'"));
    }
}
