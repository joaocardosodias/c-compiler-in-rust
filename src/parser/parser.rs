use crate::parser::ast::{
    BinaryOp,
    Expr,
    ForInit,
    Function,
    Param,
    Program,
    Stmt,
    TypeSpecifier,
    UnaryOp,
};
use crate::scanner::tokens::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub at: usize,
}

impl ParseError {
    fn new(message: impl Into<String>, at: usize) -> Self {
        Self {
            message: message.into(),
            at,
        }
    }
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut functions = Vec::new();
        while !self.is_eof() {
            functions.push(self.parse_function()?);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        let return_type = self.parse_type_specifier()?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::RParen)?;
        let body = self.parse_block_stmt()?;
        Ok(Function {
            return_type,
            name,
            params,
            body,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParseError> {
        let mut params = Vec::new();
        if self.check(TokenKind::RParen) {
            return Ok(params);
        }

        if self.check(TokenKind::KwVoid) && self.peek_n(1).map(|t| t.kind) == Some(TokenKind::RParen)
        {
            self.bump();
            return Ok(params);
        }

        loop {
            let ty = self.parse_type_specifier()?;
            let name = self.expect_identifier()?;
            params.push(Param { ty, name });

            if self.check(TokenKind::Comma) {
                self.bump();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.check(TokenKind::LBrace) {
            return self.parse_block_stmt();
        }
        if self.check(TokenKind::KwIf) {
            return self.parse_if_stmt();
        }
        if self.check(TokenKind::KwWhile) {
            return self.parse_while_stmt();
        }
        if self.check(TokenKind::KwFor) {
            return self.parse_for_stmt();
        }
        if self.check(TokenKind::KwReturn) {
            return self.parse_return_stmt();
        }
        if self.peek_is_type_specifier() {
            return self.parse_var_decl_stmt();
        }
        if self.check(TokenKind::Semicolon) {
            self.bump();
            return Ok(Stmt::Empty);
        }

        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_block_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while !self.check(TokenKind::RBrace) {
            if self.is_eof() {
                return Err(ParseError::new("unterminated block", self.pos));
            }
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Stmt::Block(stmts))
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::KwIf)?;
        self.expect(TokenKind::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(TokenKind::RParen)?;
        let then_branch = Box::new(self.parse_stmt()?);
        let else_branch = if self.check(TokenKind::KwElse) {
            self.bump();
            Some(Box::new(self.parse_stmt()?))
        } else {
            None
        };
        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::KwWhile)?;
        self.expect(TokenKind::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_stmt()?);
        Ok(Stmt::While { cond, body })
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::KwFor)?;
        self.expect(TokenKind::LParen)?;

        let init = if self.check(TokenKind::Semicolon) {
            self.bump();
            None
        } else if self.peek_is_type_specifier() {
            Some(self.parse_for_decl_init()?)
        } else {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Some(ForInit::Expr(expr))
        };

        let cond = if self.check(TokenKind::Semicolon) {
            self.bump();
            None
        } else {
            let cond = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Some(cond)
        };

        let step = if self.check(TokenKind::RParen) {
            None
        } else {
            Some(self.parse_expr()?)
        };

        self.expect(TokenKind::RParen)?;
        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::For {
            init,
            cond,
            step,
            body,
        })
    }

    fn parse_for_decl_init(&mut self) -> Result<ForInit, ParseError> {
        let ty = self.parse_type_specifier()?;
        let name = self.expect_identifier()?;
        let init = if self.check(TokenKind::Assign) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(ForInit::Decl { ty, name, init })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::KwReturn)?;
        if self.check(TokenKind::Semicolon) {
            self.bump();
            return Ok(Stmt::Return(None));
        }
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(Some(expr)))
    }

    fn parse_var_decl_stmt(&mut self) -> Result<Stmt, ParseError> {
        let ty = self.parse_type_specifier()?;
        let name = self.expect_identifier()?;
        let init = if self.check(TokenKind::Assign) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::VarDecl { ty, name, init })
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_logical_or()?;
        if self.check(TokenKind::Assign) {
            self.bump();
            let right = self.parse_assignment()?;
            return Ok(Expr::Assign {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        Ok(left)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_logical_and, &[TokenKind::OrOr], |_| BinaryOp::LogicalOr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_bit_or, &[TokenKind::AndAnd], |_| BinaryOp::LogicalAnd)
    }

    fn parse_bit_or(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_bit_xor, &[TokenKind::Pipe], |_| BinaryOp::BitOr)
    }

    fn parse_bit_xor(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_bit_and, &[TokenKind::Caret], |_| BinaryOp::BitXor)
    }

    fn parse_bit_and(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_equality, &[TokenKind::Amp], |_| BinaryOp::BitAnd)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_relational, &[TokenKind::EqEq, TokenKind::NotEq], |k| {
            match k {
                TokenKind::EqEq => BinaryOp::Equal,
                TokenKind::NotEq => BinaryOp::NotEqual,
                _ => unreachable!(),
            }
        })
    }

    fn parse_relational(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(
            Self::parse_shift,
            &[TokenKind::Lt, TokenKind::Le, TokenKind::Gt, TokenKind::Ge],
            |k| match k {
                TokenKind::Lt => BinaryOp::Less,
                TokenKind::Le => BinaryOp::LessEqual,
                TokenKind::Gt => BinaryOp::Greater,
                TokenKind::Ge => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            },
        )
    }

    fn parse_shift(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_additive, &[TokenKind::Shl, TokenKind::Shr], |k| {
            match k {
                TokenKind::Shl => BinaryOp::ShiftLeft,
                TokenKind::Shr => BinaryOp::ShiftRight,
                _ => unreachable!(),
            }
        })
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(Self::parse_multiplicative, &[TokenKind::Plus, TokenKind::Minus], |k| {
            match k {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            }
        })
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        self.parse_left_assoc(
            Self::parse_unary,
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Percent],
            |k| match k {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            },
        )
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.check(TokenKind::Plus) {
            self.bump();
            return Ok(Expr::Unary {
                op: UnaryOp::Plus,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.check(TokenKind::Minus) {
            self.bump();
            return Ok(Expr::Unary {
                op: UnaryOp::Minus,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.check(TokenKind::Not) {
            self.bump();
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                expr: Box::new(self.parse_unary()?),
            });
        }
        if self.check(TokenKind::Tilde) {
            self.bump();
            return Ok(Expr::Unary {
                op: UnaryOp::BitNot,
                expr: Box::new(self.parse_unary()?),
            });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.check(TokenKind::LParen) {
                self.bump();
                let mut args = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if self.check(TokenKind::Comma) {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
                continue;
            }
            break;
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if self.check(TokenKind::LParen) {
            self.bump();
            let expr = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            return Ok(expr);
        }

        if let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Identifier => {
                    let name = tok.lexeme.clone();
                    self.bump();
                    Ok(Expr::Identifier(name))
                }
                TokenKind::IntLiteral => {
                    let value = tok.lexeme.parse::<i64>().map_err(|_| {
                        ParseError::new(
                            format!("invalid integer literal '{}'", tok.lexeme),
                            self.pos,
                        )
                    })?;
                    self.bump();
                    Ok(Expr::IntLiteral(value))
                }
                _ => Err(ParseError::new(
                    format!("expected expression, found {:?}", tok.kind),
                    self.pos,
                )),
            }
        } else {
            Err(ParseError::new("unexpected end of input", self.pos))
        }
    }

    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ParseError> {
        let kind = self.peek().map(|t| t.kind);
        match kind {
            Some(TokenKind::KwInt) => {
                self.bump();
                Ok(TypeSpecifier::Int)
            }
            Some(TokenKind::KwChar) => {
                self.bump();
                Ok(TypeSpecifier::Char)
            }
            Some(TokenKind::KwVoid) => {
                self.bump();
                Ok(TypeSpecifier::Void)
            }
            _ => Err(ParseError::new("expected type specifier", self.pos)),
        }
    }

    fn parse_left_assoc<F>(
        &mut self,
        mut parse_next: F,
        operators: &[TokenKind],
        map_op: fn(TokenKind) -> BinaryOp,
    ) -> Result<Expr, ParseError>
    where
        F: FnMut(&mut Self) -> Result<Expr, ParseError>,
    {
        let mut left = parse_next(self)?;
        loop {
            let Some(tok) = self.peek() else {
                break;
            };
            if !operators.contains(&tok.kind) {
                break;
            }
            let op = tok.kind;
            self.bump();
            let right = parse_next(self)?;
            left = Expr::Binary {
                op: map_op(op),
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.peek() {
            Some(tok) if tok.kind == TokenKind::Identifier => {
                let name = tok.lexeme.clone();
                self.bump();
                Ok(name)
            }
            Some(tok) => Err(ParseError::new(
                format!("expected identifier, found {:?}", tok.kind),
                self.pos,
            )),
            None => Err(ParseError::new("expected identifier, found EOF", self.pos)),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        match self.peek() {
            Some(tok) if tok.kind == kind => {
                self.bump();
                Ok(())
            }
            Some(tok) => Err(ParseError::new(
                format!("expected {:?}, found {:?}", kind, tok.kind),
                self.pos,
            )),
            None => Err(ParseError::new(
                format!("expected {:?}, found EOF", kind),
                self.pos,
            )),
        }
    }

    fn peek(&self) -> Option<&'a Token> {
        self.tokens.get(self.pos)
    }

    fn peek_n(&self, n: usize) -> Option<&'a Token> {
        self.tokens.get(self.pos + n)
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().map(|t| t.kind == kind).unwrap_or(false)
    }

    fn peek_is_type_specifier(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Some(TokenKind::KwInt | TokenKind::KwChar | TokenKind::KwVoid)
        )
    }

    fn bump(&mut self) {
        self.pos += 1;
    }

    fn is_eof(&self) -> bool {
        self.peek().map(|t| t.kind == TokenKind::Eof).unwrap_or(true)
    }
}
