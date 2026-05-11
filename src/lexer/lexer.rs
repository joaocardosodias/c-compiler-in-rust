use crate::lexer::token_kind::TokenKind;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            line: 1,
        }
    }
    pub fn next_token(&mut self) -> TokenKind {
        self.skip_whitespace();
        match self.chars.next() {
            Some('(') => TokenKind::LParen,
            Some(')') => TokenKind::RParen,
            Some('{') => TokenKind::LBrace,
            Some('}') => TokenKind::RBrace,
            Some(';') => TokenKind::Semicolon,
            Some('+') => TokenKind::Plus,
            Some('-') => TokenKind::Minus,
            Some('*') => TokenKind::Star,
            Some('/') => TokenKind::Slash,
            Some('=') => TokenKind::Equal,
            Some('>')=>{
                if let Some(&'=')=self.chars.peek(){
                    self.chars.next();
                    TokenKind::LessEqual
                } 
                else{TokenKind::Less}
            },Some('<')=>{
                if let Some(&'=')=self.chars.peek(){
                    self.chars.next();
                    TokenKind::LessEqual
                } 
                else{TokenKind::Less}
            }
            

            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut text = String::new();
                text.push(c);
                while let Some(&next_t) = self.chars.peek() {
                    if next_t.is_ascii_alphanumeric() || next_t == '_' {
                        text.push(self.chars.next().unwrap());
                    } else {
                        break;
                    };
                }
                match text.as_str() {
                    "int" => TokenKind::Int,
                    "return" => TokenKind::Return,
                    "while" => TokenKind::While,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "for" => TokenKind::For,
                    _ => TokenKind::Identifier(text),
                }
            }
            Some(c) if c.is_ascii_digit() => {
                let mut text = String::new();
                text.push(c);
                while let Some(&next_t) = self.chars.peek() {
                    if next_t.is_ascii_digit() {
                        text.push(self.chars.next().unwrap())
                    } else {
                        break;
                    };
                }

                TokenKind::Integer(text.parse::<i32>().unwrap())
            }
            Some(other) => TokenKind::Invalid(other.to_string()),
            None => TokenKind::EOF,
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.chars.next();
            } else if c == '\n' {
                self.chars.next();
                self.line += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace() {
        let source = "   \n  \t \n  a";
        let mut lexer = Lexer::new(source);
        lexer.skip_whitespace();
        assert_eq!(lexer.line, 3);
        assert_eq!(lexer.chars.next(), Some('a'));
    }
    #[test]
    fn test_single_char_tokens() {
        let source = "(){};+-*/=int return if else while var 123";
        let mut lexer = Lexer::new(source);

        assert_eq!(lexer.next_token(), TokenKind::LParen);
        assert_eq!(lexer.next_token(), TokenKind::RParen);
        assert_eq!(lexer.next_token(), TokenKind::LBrace);
        assert_eq!(lexer.next_token(), TokenKind::RBrace);
        assert_eq!(lexer.next_token(), TokenKind::Semicolon);
        assert_eq!(lexer.next_token(), TokenKind::Plus);
        assert_eq!(lexer.next_token(), TokenKind::Minus);
        assert_eq!(lexer.next_token(), TokenKind::Star);
        assert_eq!(lexer.next_token(), TokenKind::Slash);
        assert_eq!(lexer.next_token(), TokenKind::Equal);
        assert_eq!(lexer.next_token(), TokenKind::Int);
        assert_eq!(lexer.next_token(), TokenKind::Return);
        assert_eq!(lexer.next_token(), TokenKind::If);
        assert_eq!(lexer.next_token(), TokenKind::Else);
        assert_eq!(lexer.next_token(), TokenKind::While);
        assert_eq!(lexer.next_token(), TokenKind::Identifier("var".to_string()));
        assert_eq!(lexer.next_token(), TokenKind::Integer(123));
        assert_eq!(lexer.next_token(), TokenKind::EOF);
    }
}
