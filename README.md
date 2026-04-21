# C-compiler-in-Rust

Projeto de estudo para construir um compilador de C em Rust, com foco em entender o pipeline de scanner/automatos na pratica.

## Status atual

Ja implementado no projeto:

- definicao dos tipos de token (`TokenKind`, `Token`, `Span`, erros lexicos)
- definicao das regras lexicas (uma RE por token/regra)
- parser de RE textual para AST de regex
- uniao das regras em uma especificacao unica (`UnifiedRegexSpec`)
- Thompson (`RE -> NFA`)
- subset construction (`NFA -> DFA`)
- Hopcroft (`DFA -> DFA minimo`)
- codegen Rust (`match state { ... }`) para gerar scanner automaticamente

## Fluxo que esta funcionando

1. Definir os tipos de tokens em `src/scanner/tokens.rs`.
2. Criar uma RE para cada token/regra em `src/scanner/spec/rules.rs`.
3. Parsear as REs para AST de regex.
4. Juntar tudo em uma especificacao unica (uniao gigante + prioridade).
5. Rodar Thompson para gerar NFA.
6. Rodar subset construction para gerar DFA.
7. Rodar Hopcroft para minimizar o DFA.
8. Gerar arquivos `.rs` do scanner em `src/scanner/generated/`.

## Gerar scanner

Comando principal:

```bash
cargo run -- generate-scanner
```

Isso gera/sobrescreve:

- `src/scanner/generated/scanner_impl.rs`
- `src/scanner/generated/dfa_table.rs`

Tambem e possivel informar caminhos customizados:

```bash
cargo run -- generate-scanner caminho/scanner_impl.rs caminho/dfa_table.rs
```

## Testar o scanner em um arquivo C

Existe um comando de teste rapido para tokenizar um `.c`:

```bash
cargo run -- scan examples/sample.c
```

Arquivo de exemplo:

- `examples/sample.c`

## Estrutura principal

- `src/scanner/`
  - `tokens.rs`: tipos de token, span e erro
  - `spec/`: regras RE por token
  - `generated/`: scanner e tabelas geradas automaticamente
  - `runtime.rs`: executor do scanner gerado sobre texto-fonte
- `src/lexer_gen/`
  - `regex/`: AST + parser de RE
  - `nfa/`: Thompson
  - `dfa/`: subset construction
  - `minimize/`: Hopcroft
  - `codegen/`: geracao do Rust final do scanner
- `src/pipeline/`
  - orquestracao do pipeline completo e escrita dos artefatos

## Estudos
- Exercicios do livro: `docs/exercicios/README.md`
