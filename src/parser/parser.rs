use crate::lexer::token_kind::TokenKind;
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
    pub fn parser_expression(&mut self) -> Expr {
        let token = self.advance();
        match token {
            TokenKind::Integer(number) => Expr::IntLiteral(number),
            TokenKind::Identifier(world)=>Expr::Variable(world),
            _ => panic!("Invalid sintax"),
        }
    }
    pub fn parse_statement(&mut self) -> Stmt {
        let token=self.advance();
        match token {
            TokenKind::Return=>{
                
        let expr = self.parser_expression();
        self.expect(TokenKind::Semicolon);
        Stmt::Return(expr)
            },
            TokenKind::Int=>{;
                let name_expr=match self.advance() {
                    TokenKind::Identifier(name_expr)=>name_expr,
                    _=>panic!("Invalid sintax")
  
                };
                self.expect(TokenKind::Equal);
                let number_expr=self.parser_expression();
                self.expect(TokenKind::Semicolon);
                Stmt::VarDecl(name_expr, number_expr)
            },
            _=>panic!("Invalid sintax")
            
        }
        
    }
    pub fn parse_function(&mut self) -> FunctionDecl {
        self.expect(TokenKind::Int);
        let name = self.advance();
        let mut body:Vec<Stmt>=Vec::new();
        let function_name = match name {
            TokenKind::Identifier(identifier) => identifier,
            _ => panic!("Invalid Sintax"),
        };
        self.expect(TokenKind::LParen);
        self.expect(TokenKind::RParen);
        self.expect(TokenKind::LBrace);

        while self.peek() != TokenKind::RBrace{
            let statement=self.parse_statement();
            body.push(statement);
        };

        
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
            self.advance();
        }
        Program {
            functions,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
#[test]
    fn test_parse_simple_program() {
        // Simulando a saída do nosso Lexer para: int main() { return 42; }
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
        // O que esperamos que a árvore seja?
        let expected_ast = Program {
            functions: vec![
                FunctionDecl {
                    name: "main".to_string(),
                    body: vec![
                        Stmt::Return(Expr::IntLiteral(42))
                    ]
                }
            ]
        };
        // Verifica se a árvore construída é igual à árvore esperada
        assert_eq!(program, expected_ast);
    }




}
 #[test]
    fn test_parse_variables() {
        // Simulando: int main() { int x = 42; return x; }
        let tokens = vec![
            TokenKind::Int, TokenKind::Identifier("main".to_string()),
            TokenKind::LParen, TokenKind::RParen, TokenKind::LBrace,
            TokenKind::Int, TokenKind::Identifier("x".to_string()), TokenKind::Equal, TokenKind::Integer(42), TokenKind::Semicolon,
            TokenKind::Return, TokenKind::Identifier("x".to_string()), TokenKind::Semicolon,
            TokenKind::RBrace, TokenKind::EOF,
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