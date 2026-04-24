use super::ast::{CharClass, CharClassItem, Regex};

/// Erro sintatico ao interpretar uma RE textual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexParseError {
    /// Mensagem de erro para exibicao ao usuario.
    pub message: String,
    /// Posicao (em caracteres) onde o parser detectou a falha.
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

/// Parseia uma expressao regular textual e produz a AST correspondente.
///
/// Precedencia adotada (da maior para a menor):
/// 1. agrupamento por parenteses `( )`
/// 2. operadores postfix (`*`, `+`, `?`)
/// 3. concatenacao implicita (ex: `ab` é `a` seguido de `b`)
/// 4. alternacao (`|`)
pub fn parse_regex(input: &str) -> Result<Regex, RegexParseError> {
    let mut parser = Parser::new(input);
    let regex = parser.parse_regex()?;
    
    // Se o parser terminou de ler uma expressão válida mas ainda há caracteres
    // no input, significa que algo estava mal formado (ex: "a)b", o parser lê "a" e acha um ")" sobrando).
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

/// Parser recursivo descendente para RE.
///
/// Mantemos uma implementacao pequena e direta para facilitar debug e evolucao.
/// "Recursivo Descendente" significa que há uma função para cada regra gramatical,
/// e elas chamam umas às outras obedecendo à ordem de precedência.
struct Parser {
    /// Buffer de caracteres para acesso indexado por posicao logica.
    chars: Vec<char>,
    /// Cursor atual no buffer de caracteres.
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    /// Regra base (mais baixa precedência): Inicia o parse chamando a Alternação.
    fn parse_regex(&mut self) -> Result<Regex, RegexParseError> {
        self.parse_alternation()
    }

    /// alternation := concatenation ('|' concatenation)*
    /// A alternação tem a precedência mais baixa. Ela junta vários blocos de concatenação.
    fn parse_alternation(&mut self) -> Result<Regex, RegexParseError> {
        // Lê o primeiro braço da alternação.
        let mut arms = vec![self.parse_concatenation()?];

        // Se ver um '|', consome e lê o próximo braço.
        while self.peek_char() == Some('|') {
            self.advance_char();
            arms.push(self.parse_concatenation()?);
        }

        // `fold_alternation` simplifica: se só houver 1 braço, retorna direto o braço (não cria nó `Alternation`).
        Ok(Self::fold_alternation(arms))
    }

    /// concatenation := postfix*
    ///
    /// A concatenacao e' implicita: tokens adjacentes formam sequencia.
    fn parse_concatenation(&mut self) -> Result<Regex, RegexParseError> {
        let mut factors = Vec::new();

        while let Some(ch) = self.peek_char() {
            // Parêntese fechando ou pipe indicam o fim deste bloco concatenado.
            // O fechamento de parêntese será consumido pelo método que o abriu (parse_atom).
            if ch == ')' || ch == '|' {
                break;
            }
            // Parseia o próximo elemento, que pode ter um *, +, ou ?.
            factors.push(self.parse_postfix()?);
        }

        Ok(Self::fold_concat(factors))
    }

    /// postfix := atom ('*' | '+' | '?')*
    /// Trata repetições. Têm precedência muito alta, colam direto no "átomo" à esquerda.
    fn parse_postfix(&mut self) -> Result<Regex, RegexParseError> {
        // Lê o átomo base (um caractere literal, uma classe, ou um grupo entre parênteses).
        let mut node = self.parse_atom()?;

        // Pode haver múltiplos operadores postfix encadeados (ex: `a*?+`), o que é inútil mas sintaticamente válido.
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
                _ => break, // Se não for postfix, acaba o loop.
            }
        }

        Ok(node)
    }

    /// atom := '(' alternation ')' | '[' class ']' | '.' | literal | escape
    /// Átomos são as unidades indivisíveis da Regex (maior precedência).
    fn parse_atom(&mut self) -> Result<Regex, RegexParseError> {
        match self.peek_char() {
            None => Err(RegexParseError::new("unexpected end of regex", self.pos)),
            Some('(') => {
                // Parênteses alteram a precedência delegando de volta pro início (alternation).
                self.advance_char();
                let inner = self.parse_alternation()?;
                self.expect_char(')')?;
                Ok(inner)
            }
            Some('[') => {
                // Classe de caracteres
                self.advance_char();
                let class = self.parse_char_class()?;
                Ok(Regex::CharClass(class))
            }
            Some('.') => {
                // Coringa
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
            // Metacaracteres não podem aparecer soltos como átomos.
            Some(')') | Some('|') | Some('*') | Some('+') | Some('?') => {
                Err(RegexParseError::new("unexpected operator", self.pos))
            }
            Some('\\') => {
                // Escape (ex: \*, \n)
                self.advance_char();
                let escaped = self.parse_escape()?;
                Ok(Regex::Literal(escaped))
            }
            Some(ch) => {
                // Qualquer outro caractere é interpretado literalmente.
                self.advance_char();
                Ok(Regex::Literal(ch))
            }
        }
    }

    /// Le o conteudo de uma classe de caracteres `[ ... ]`.
    fn parse_char_class(&mut self) -> Result<CharClass, RegexParseError> {
        let mut negated = false;
        let mut items = Vec::new();

        // Se o primeiro char for `^`, a classe é invertida (ex: [^0-9]).
        if self.peek_char() == Some('^') {
            self.advance_char();
            negated = true;
        }

        // Cobre o erro de escrever `[]`.
        if self.peek_char() == Some(']') {
            return Err(RegexParseError::new("empty char class", self.pos));
        }

        loop {
            let Some(ch) = self.peek_char() else {
                return Err(RegexParseError::new("unterminated char class", self.pos));
            };

            // Se for o fechamento `]`, saímos do loop de classe.
            if ch == ']' {
                self.advance_char();
                break;
            }

            // Lê um caractere isolado (ou escapado) para iniciar o item.
            let start = self.parse_class_char()?;

            // Verifica se é um intervalo (ex: `a-z`).
            if self.peek_char() == Some('-') {
                let dash_pos = self.pos;
                self.advance_char(); // Consome o '-'
                
                match self.peek_char() {
                    Some(']') => {
                        // Caso especial: `-` no final da classe (ex: `[a-z-]`) significa hífen literal.
                        items.push(CharClassItem::Char(start));
                        items.push(CharClassItem::Char('-'));
                    }
                    Some(_) => {
                        // Lê o caractere final do intervalo.
                        let end = self.parse_class_char()?;
                        if start > end {
                            // Não faz sentido intervalo invertido tipo z-a.
                            return Err(RegexParseError::new("invalid class range", dash_pos));
                        }
                        items.push(CharClassItem::Range(start, end));
                    }
                    None => {
                        return Err(RegexParseError::new("unterminated char class", self.pos));
                    }
                }
            } else {
                // Se não tinha `-` depois, era apenas um caractere isolado na classe.
                items.push(CharClassItem::Char(start));
            }
        }

        Ok(CharClass { negated, items })
    }

    /// Le um item de classe de caractere, com suporte a escape (dentro dos colchetes `[]`).
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

    /// Interpreta sequencias de escape basicas em contexto de RE (como \n, \t).
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
            other => other, // Se for \* ou \+, apenas retorna * ou + literal.
        };

        Ok(escaped)
    }

    /// Consome um caractere esperado ou retorna erro localizado.
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

    /// Remove nos desnecessarios ao construir concatenacao.
    fn fold_concat(parts: Vec<Regex>) -> Regex {
        match parts.as_slice() {
            [] => Regex::Epsilon, // Concatenação de nada é apenas transitar vazio.
            [single] => single.clone(), // Concatenação de 1 é só ele próprio.
            _ => Regex::Concat(parts),
        }
    }

    /// Remove nos desnecessarios ao construir alternacao.
    fn fold_alternation(arms: Vec<Regex>) -> Regex {
        match arms.as_slice() {
            [] => Regex::Epsilon, // Alternação de nada não faz sentido, resolvemos pra Epsilon (vazio).
            [single] => single.clone(), // Alternação com 1 opção é só a própria opção.
            _ => Regex::Alternation(arms),
        }
    }

    /// Observa o caractere atual sem consumir.
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// Consome um caractere e avanca o cursor.
    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += 1;
        Some(ch)
    }

    /// Indica se todo input ja foi consumido.
    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::parse_regex;
    use crate::lexer_gen::regex::ast::{CharClassItem, Regex};

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
