use crate::scanner::generated::{GeneratedAcceptAction, dfa_accept_action, dfa_next_state};
// Importa as funcoes geradas automaticamente pelo codegen (scanner_impl.rs).
// - `dfa_next_state(state, char) -> Option<usize>`: dado estado atual + char, retorna proximo estado.
// - `dfa_accept_action(state) -> Option<GeneratedAcceptAction>`: retorna a acao se o estado aceita.
// - `GeneratedAcceptAction`: enum Emit(TokenKind) | Skip gerado automaticamente.
use crate::scanner::tokens::{LexError, LexErrorKind, Position, Span, Token, TokenKind};
// Importa os tipos de dados do contrato publico do scanner.

pub fn scan_source(source: &str) -> Result<Vec<Token>, LexError> {
    // Funcao principal do scanner em tempo de execucao.
    // Recebe o codigo-fonte como uma &str e retorna ou um vetor de tokens ou um erro lexico.
    // `Result<Vec<Token>, LexError>` e' o padrao Rust para operacoes que podem falhar:
    // o chamador e' obrigado a tratar o erro (nao ha excecoes silenciosas).

    let mut tokens = Vec::new();
    // Vetor que acumula todos os tokens reconhecidos. Cresce dinamicamente conforme o scan avanca.

    let mut offset = 0usize;
    // Byte-offset atual no `source`. Aponta para o inicio do proximo lexema a ser reconhecido.
    // Usamos bytes (nao chars) para poder fatiar strings UTF-8 diretamente com &source[a..b].

    let mut line = 1usize;
    // Numero da linha atual (1-indexed, convencao de editores e compiladores).
    // Atualizado sempre que um '\n' e' consumido.

    let mut column = 1usize;
    // Numero da coluna atual (1-indexed). Resetada para 1 apos cada '\n'.

    while offset < source.len() {
        // Loop principal: enquanto houver bytes nao consumidos no fonte.
        // Cada iteracao reconhece exatamente um lexema (o maior possivel — Maximal Munch).

        let start = Position::new(offset, line, column);
        // Salva a posicao de inicio do lexema atual ANTES de avancar o cursor.
        // Usada para construir o Span do token no final da iteracao.

        let mut state = 0usize;
        // Estado inicial do DFA. Por convencao, o estado 0 e' sempre o estado inicial.
        // A cada caractere lido, `state` sera atualizado por `dfa_next_state`.

        let mut cursor = offset;
        // Cursor interno que avanca byte a byte durante o reconhecimento do lexema.
        // Ao final, `cursor` aponta para o byte SEGUINTE ao ultimo caractere aceito.

        let mut cursor_line = line;
        let mut cursor_column = column;
        // Copias de `line` e `column` que avancam junto com o cursor.
        // Guardamos separado do `line`/`column` principal para poder "voltar"
        // ao ultimo ponto de aceitacao (Maximal Munch) sem perder a posicao correta.

        let mut last_accept: Option<(usize, usize, usize, GeneratedAcceptAction)> = None;
        // A variavel chave do algoritmo de Maximal Munch (maior casamento possivel).
        // Guarda: (byte-offset-do-fim, linha, coluna, acao) do ULTIMO estado de aceitacao visitado.
        // Se o DFA "morrer" (nenhuma transicao para o proximo char), usamos este valor salvo
        // como o resultado final do lexema — mesmo que pudessemos ter lido mais chars.
        // Ex: para "int_var", o scanner la e aceita "int" como KwInt mas continua ate ver '_'
        // (que nao e' transicao valida saindo do estado final de "int"). Como "int_var" e' maior
        // e e' um Identifier valido, ele vence. O last_accept e' atualizado enquanto ha estados.

        let mut iter = source[offset..].char_indices();
        // Itera sobre os chars do fonte a partir da posicao atual.
        // `char_indices()` retorna (byte_offset_relativo, char). O offset e' relativo ao slice.

        while let Some((rel, ch)) = iter.next() {
            // `rel`: offset do char relativo ao inicio do slice (source[offset..]).
            // `ch`: o caractere Unicode atual.

            let Some(next_state) = dfa_next_state(state, ch) else {
                // Chama a funcao gerada que implementa a funcao de transicao do DFA.
                // Se retorna `None`, nao ha transicao valida para `ch` a partir de `state`.
                // Isso significa que o DFA "morreu" — o lexema atual chegou ao seu fim maximo.
                break;
                // Sai do loop interno. O resultado sera o que esta em `last_accept`.
            };

            state = next_state;
            // Atualiza o estado atual do DFA para o proximo estado.

            cursor = offset + rel + ch.len_utf8();
            // Avanca o cursor para APOS o caractere atual.
            // `offset`: inicio do lexema atual no source global.
            // `rel`: posicao relativa do char dentro do slice.
            // `ch.len_utf8()`: tamanho em bytes do caractere (1-4 bytes em UTF-8).
            // A soma da esses tres valores e' o proximo byte a ser lido apos `ch`.

            if ch == '\n' {
                cursor_line += 1; // Incrementa a linha ao encontrar uma quebra de linha.
                cursor_column = 1; // Reseta a coluna para o inicio da nova linha.
            } else {
                cursor_column += 1; // Avanca a coluna para o proximo caractere.
            }

            if let Some(action) = dfa_accept_action(state) {
                // Verifica se o estado atual e' um estado de aceitacao.
                // `dfa_accept_action` e' gerada automaticamente e mapeia estados de aceitacao
                // para suas acoes (Emit ou Skip).
                last_accept = Some((cursor, cursor_line, cursor_column, action));
                // Salva este ponto como o ultimo ponto de aceitacao valido.
                // Se o DFA morrer no proximo passo, usaremos este como resultado.
            }
        }

        let Some((end_offset, end_line, end_column, action)) = last_accept else {
            // Se `last_accept` ainda e' `None` apos o loop interno, significa que
            // o DFA nunca passou por nenhum estado de aceitacao — nenhum token valido
            // pode comecar com o caractere atual.
            let ch = source[offset..].chars().next().unwrap_or('\0');
            // Pega o caractere problematico para incluir na mensagem de erro.
            // `unwrap_or('\0')`: se de alguma forma o slice estiver vazio, usa o char nulo.
            return Err(LexError::new(
                LexErrorKind::UnexpectedChar,
                Span::new(start, start),
                // O span e' pontual (start == end) pois nao conseguimos consumir nenhum char.
                format!("unexpected character '{}'", ch),
            ));
        };

        match action {
            GeneratedAcceptAction::Emit(kind) => {
                // O estado de aceitacao diz para emitir um token.
                let end = Position::new(end_offset, end_line, end_column);
                let lexeme = source[offset..end_offset].to_string();
                // Fatiamos o source original para extrair o texto bruto do lexema.
                // `offset`: inicio do lexema. `end_offset`: fim (exclusive).
                // `.to_string()` aloca uma String nova para o lexema.
                // Poderiamos usar &str aqui para evitar alocacao, mas String simplifica
                // o ciclo de vida (o Token pode viver independente do source original).
                tokens.push(Token::new(kind, lexeme, Span::new(start, end)));
                // Adiciona o token ao vetor de resultado.
            }
            GeneratedAcceptAction::Skip => {}
            // O estado de aceitacao diz para ignorar este lexema (espaco, comentario etc.).
            // Simplesmente nao adicionamos nenhum token ao vetor e continuamos.
        }

        offset = end_offset;
        // Avanca o offset principal para logo apos o lexema reconhecido.
        // A proxima iteracao do loop externo comecara deste ponto.
        line = end_line;
        column = end_column;
        // Sincroniza a posicao de linha/coluna com o fim do ultimo lexema aceito.
    }

    let eof_pos = Position::new(offset, line, column);
    // Cria a posicao final do arquivo (logo apos o ultimo caractere).
    tokens.push(Token::new(
        TokenKind::Eof,
        String::new(),
        // O token EOF nao tem lexeme — e' um marcador logico, nao um texto real.
        Span::new(eof_pos, eof_pos),
        // Span pontual: o EOF ocupa zero caracteres.
    ));
    // O parser depende de receber um Eof para saber que o input acabou.
    // Sem ele, o parser teria que checar `tokens.is_empty()` em varios lugares.

    Ok(tokens)
    // Retorna o vetor completo de tokens envolvido em Ok.
    // Se chegamos aqui, nenhum erro lexico ocorreu.
}
