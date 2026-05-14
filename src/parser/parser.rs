use crate::lexer::token_kind::{self, TokenKind};
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<TokenKind>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn peek(&self) -> TokenKind {
        self.tokens.get(self.pos).cloned().unwrap_or(TokenKind::EOF)
    }
    fn advance(&mut self) -> TokenKind {
        let token = self.peek();
        self.pos += 1;
        token
    }
    fn expect(&mut self, expected: TokenKind) {
        if self.peek() == expected {
            self.advance();
        } else {
            panic!(
                "Erro de sintaxe! Esperava {:?}, mas encontrou {:?}",
                expected,
                self.peek()
            );
        }
    }
    pub fn parser_primary(&mut self) -> Expr {
        let token = self.advance();
        match token {
            TokenKind::Integer(number) => Expr::IntLiteral(number),
            TokenKind::Identifier(word) => Expr::Variable(word),

            _ => panic!("Invalid sintax"),
        }
    }
    pub fn parse_statement(&mut self) -> Stmt {
        let token = self.advance();
        match token {
            TokenKind::Return => {
                let expr = self.parser_expression(0);
                self.expect(TokenKind::Semicolon);
                Stmt::Return(expr)
            }
            TokenKind::Int => {
                let name_expr = match self.advance() {
                    TokenKind::Identifier(name_expr) => name_expr,
                    _ => panic!("Invalid sintax"),
                };
                self.expect(TokenKind::Equal);
                let number_expr = self.parser_expression(0);
                self.expect(TokenKind::Semicolon);
                Stmt::VarDecl(name_expr, number_expr)
            }
            TokenKind::While => {
                let mut block: Vec<Stmt> = Vec::new();

                self.expect(TokenKind::LParen);
                let condition = self.parser_expression(0);
                self.expect(TokenKind::RParen);

                self.expect(TokenKind::LBrace);
                while self.peek() != TokenKind::RBrace {
                    let statement = self.parse_statement();
                    block.push(statement);
                }
                self.expect(TokenKind::RBrace);

                let blocks = Stmt::Block(block);

                Stmt::While {
                    condition,
                    body: Box::new(blocks),
                }
            }
            TokenKind::For => {
                let mut block: Vec<Stmt> = Vec::new();
                self.expect(TokenKind::LParen);
                let init = self.parse_statement();
                let condition = self.parser_expression(0);
                self.expect(TokenKind::Semicolon);
                let post = self.parser_expression(0);
                self.expect(TokenKind::RParen);
                self.expect(TokenKind::LBrace);
                while self.peek() != TokenKind::RBrace {
                    let statement = self.parse_statement();
                    block.push(statement);
                }
                self.expect(TokenKind::RBrace);

                let blocks = Stmt::Block(block);
                Stmt::For {
                    init: Box::new(init),
                    condition,
                    post,
                    body: Box::new(blocks),
                }
            }
            TokenKind::If => {
                let mut block: Vec<Stmt> = Vec::new();
                let mut else_block: Vec<Stmt> = Vec::new();
                self.expect(TokenKind::LParen);
                let condition = self.parser_expression(0);
                self.expect(TokenKind::RParen);
                self.expect(TokenKind::LBrace);
                while self.peek() != TokenKind::RBrace {
                    let statement = self.parse_statement();
                    block.push(statement);
                }
                self.expect(TokenKind::RBrace);
                if self.peek() == TokenKind::Else {
                    self.advance();
                    self.expect(TokenKind::LBrace);
                    while self.peek() != TokenKind::RBrace {
                        let statement = self.parse_statement();
                        else_block.push(statement);
                    }
                    self.expect(TokenKind::RBrace);
                }
                let blocks = Stmt::Block(block);
                let else_blocks = Stmt::Block(else_block);
                Stmt::If {
                    condition,
                    action: Box::new(blocks),
                    else_block: Box::new(else_blocks),
                }
            }
            _ => panic!("Invalid sintax"),
        }
    }
    pub fn parse_function(&mut self) -> FunctionDecl {
        self.expect(TokenKind::Int);
        let name = self.advance();
        let mut body: Vec<Stmt> = Vec::new();
        let function_name = match name {
            TokenKind::Identifier(identifier) => identifier,
            _ => panic!("Invalid Sintax"),
        };
        self.expect(TokenKind::LParen);
        self.expect(TokenKind::RParen);
        self.expect(TokenKind::LBrace);

        while self.peek() != TokenKind::RBrace {
            let statement = self.parse_statement();
            body.push(statement);
        }
        self.expect(TokenKind::RBrace);

        FunctionDecl {
            name: function_name,
            body,
        }
    }
    pub fn parse_program(&mut self) -> Program {
        let mut functions: Vec<FunctionDecl> = Vec::new();
        let token = TokenKind::EOF;
        while self.peek() != token {
            let function = self.parse_function();
            functions.push(function);
        }
        Program { functions }
    }
    pub fn get_precedence(token: &TokenKind) -> u8 {
        match token {
            TokenKind::LParen=>100,
            TokenKind::Plus | TokenKind::Minus => 10,
            TokenKind::Star | TokenKind::Slash => 20,
            TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::Greater => 5,
            _ => 0,
        }
    }
    pub fn parser_expression(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parser_primary();
        loop {
            let peek_token = self.peek();
            let prec = Self::get_precedence(&peek_token);
            if prec == 0 || prec <= min_prec {
                break;
            }
            let op_token = self.advance();
            if op_token == TokenKind::LParen{
                let function_name=match left{
                    Expr::Variable(name)=>name,
                    _=>panic!("Invalid Sintax"),
                };
                let mut args:Vec<Expr>=Vec::new();
                if self.peek() !=TokenKind::RParen{
                    loop{
                        args.push(self.parser_expression(0));
                        if self.peek() == TokenKind::Comma{
                            self.advance();
                        }
                        else{break};
                    }
                    
                }
                self.expect(TokenKind::RParen);
                left = Expr::Call { name: function_name, args };
                continue;
            }
            let op = match op_token {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Less => BinaryOp::LessThan,
                TokenKind::LessEqual => BinaryOp::LessThanOrEqual,
                TokenKind::Greater => BinaryOp::GreaterThan,
                TokenKind::GreaterEqual => BinaryOp::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.parser_expression(prec);
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_program() {
        let tokens = vec![
            TokenKind::Int,
            TokenKind::Identifier("main".to_string()),
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Integer(42),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        let expected_ast = Program {
            functions: vec![FunctionDecl {
                name: "main".to_string(),
                body: vec![Stmt::Return(Expr::IntLiteral(42))],
            }],
        };
        assert_eq!(program, expected_ast);
    }

    #[test]
    fn test_parse_variables() {
        let tokens = vec![
            TokenKind::Int,
            TokenKind::Identifier("main".to_string()),
            TokenKind::LParen,
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Int,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(42),
            TokenKind::Semicolon,
            TokenKind::Return,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program();
        let expected_ast = Program {
            functions: vec![FunctionDecl {
                name: "main".to_string(),
                body: vec![
                    Stmt::VarDecl("x".to_string(), Expr::IntLiteral(42)),
                    Stmt::Return(Expr::Variable("x".to_string())),
                ],
            }],
        };
        assert_eq!(program, expected_ast);
    }

    #[test]
    fn test_parse_math_expression() {
        let tokens = vec![
            TokenKind::Return,
            TokenKind::Integer(2),
            TokenKind::Plus,
            TokenKind::Integer(3),
            TokenKind::Star,
            TokenKind::Integer(4),
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();
        let expected_ast = Stmt::Return(Expr::BinOp(
            BinaryOp::Add,
            Box::new(Expr::IntLiteral(2)),
            Box::new(Expr::BinOp(
                BinaryOp::Multiply,
                Box::new(Expr::IntLiteral(3)),
                Box::new(Expr::IntLiteral(4)),
            )),
        ));
        assert_eq!(statement, expected_ast);
    }

    #[test]
    fn test_parse_while_loop() {
        let tokens = vec![
            TokenKind::While,
            TokenKind::LParen,
            TokenKind::Identifier("x".to_string()),
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Integer(42),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();

        let expected_ast = Stmt::While {
            condition: Expr::Variable("x".to_string()),
            body: Box::new(Stmt::Block(vec![Stmt::Return(Expr::IntLiteral(42))])),
        };

        assert_eq!(statement, expected_ast);
    }
    #[test]
    fn test_parse_for_loop() {
        let tokens = vec![
            TokenKind::For,
            TokenKind::LParen,
            TokenKind::Int,
            TokenKind::Identifier("i".to_string()),
            TokenKind::Equal,
            TokenKind::Integer(0),
            TokenKind::Semicolon,
            TokenKind::Identifier("i".to_string()),
            TokenKind::Less,
            TokenKind::Integer(10),
            TokenKind::Semicolon,
            TokenKind::Identifier("i".to_string()),
            TokenKind::Plus,
            TokenKind::Integer(1),
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Identifier("i".to_string()),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];

        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();

        let expected_ast = Stmt::For {
            init: Box::new(Stmt::VarDecl("i".to_string(), Expr::IntLiteral(0))),

            condition: Expr::BinOp(
                BinaryOp::LessThan,
                Box::new(Expr::Variable("i".to_string())),
                Box::new(Expr::IntLiteral(10)),
            ),

            post: Expr::BinOp(
                BinaryOp::Add,
                Box::new(Expr::Variable("i".to_string())),
                Box::new(Expr::IntLiteral(1)),
            ),

            body: Box::new(Stmt::Block(vec![Stmt::Return(Expr::Variable(
                "i".to_string(),
            ))])),
        };
        assert_eq!(statement, expected_ast);
    }
    #[test]
    fn test_parse_if_statement() {
        let tokens = vec![
            TokenKind::If,
            TokenKind::LParen,
            TokenKind::Identifier("x".to_string()),
            TokenKind::Less,
            TokenKind::Integer(10),
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Integer(1),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();
        let expected_ast = Stmt::If {
            condition: Expr::BinOp(
                BinaryOp::LessThan,
                Box::new(Expr::Variable("x".to_string())),
                Box::new(Expr::IntLiteral(10)),
            ),
            action: Box::new(Stmt::Block(vec![Stmt::Return(Expr::IntLiteral(1))])),
            else_block: Box::new(Stmt::Block(vec![])), // Sem else, bloco vazio!
        };
        assert_eq!(statement, expected_ast);
    }
    #[test]
    fn test_parse_if_else_statement() {
        let tokens = vec![
            TokenKind::If,
            TokenKind::LParen,
            TokenKind::Identifier("x".to_string()),
            TokenKind::RParen,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Integer(1),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::Else,
            TokenKind::LBrace,
            TokenKind::Return,
            TokenKind::Integer(0),
            TokenKind::Semicolon,
            TokenKind::RBrace,
            TokenKind::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();
        let expected_ast = Stmt::If {
            condition: Expr::Variable("x".to_string()),
            action: Box::new(Stmt::Block(vec![Stmt::Return(Expr::IntLiteral(1))])),
            else_block: Box::new(Stmt::Block(vec![Stmt::Return(Expr::IntLiteral(0))])),
        };
        assert_eq!(statement, expected_ast);
    }
    #[test]
    fn test_parse_function_call() {
        // Simulando: return soma(x, 5 * 2);
        let tokens = vec![
            TokenKind::Return,
            TokenKind::Identifier("soma".to_string()),
            TokenKind::LParen,
            
            // Arg 1: x
            TokenKind::Identifier("x".to_string()),
            TokenKind::Comma,
            
            // Arg 2: 5 * 2
            TokenKind::Integer(5), TokenKind::Star, TokenKind::Integer(2),
            
            TokenKind::RParen,
            TokenKind::Semicolon,
            TokenKind::EOF,
        ];
        
        let mut parser = Parser::new(tokens);
        let statement = parser.parse_statement();
        let expected_ast = Stmt::Return(Expr::Call {
            name: "soma".to_string(),
            args: vec![
                Expr::Variable("x".to_string()), // Arg 1
                Expr::BinOp(                     // Arg 2
                    BinaryOp::Multiply,
                    Box::new(Expr::IntLiteral(5)),
                    Box::new(Expr::IntLiteral(2))
                )
            ]
        });
        assert_eq!(statement, expected_ast);
    }
}
