//! Etapas de alto nivel do pipeline lexico.
//!
//! Este modulo concentra o passo de agregacao das regras: ele le as REs por token,
//! ordena por prioridade e constrói uma RE unica (alternacao gigante) que servira
//! de entrada para Thompson.

use std::path::Path;

use crate::lexer_gen::codegen::{generate_dfa_table_module, write_generated_scanner_to};
use crate::lexer_gen::dfa::build_dfa_from_nfa;
use crate::lexer_gen::minimize::minimize_dfa_hopcroft;
use crate::lexer_gen::nfa::build_nfa_from_unified_spec;
use crate::lexer_gen::regex::Regex;
use crate::scanner::{RegexRule, RuleKind, parsed_regex_rules};

/// Regra lexical ja parseada e pronta para participar da RE unificada.
#[derive(Debug, Clone)]
pub struct AnnotatedRuleRegex {
    /// Metadados da regra (nome, prioridade e acao Emit/Skip).
    pub rule: RegexRule,
    /// AST de RE correspondente ao padrao textual da regra.
    pub regex: Regex,
}

/// Resultado do passo "uniao gigante".
///
/// A anotacao por regra e' mantida separada para que, ao construir o NFA/DFA,
/// seja possivel marcar estados de aceitacao com `RuleKind` e prioridade.
#[derive(Debug, Clone)]
pub struct UnifiedRegexSpec {
    /// Regras ordenadas por prioridade (e ordem de declaracao em caso de empate).
    pub rules: Vec<AnnotatedRuleRegex>,
    /// Alternacao unica de todas as REs das regras acima.
    pub unified_regex: Regex,
}

impl UnifiedRegexSpec {
    /// Retorna todas as regras que emitem token.
    pub fn emitting_rules(&self) -> impl Iterator<Item = &AnnotatedRuleRegex> {
        self.rules
            .iter()
            .filter(|entry| matches!(entry.rule.kind, RuleKind::Emit(_)))
    }

    /// Retorna todas as regras que devem ser ignoradas (whitespace/comentarios).
    pub fn skipped_rules(&self) -> impl Iterator<Item = &AnnotatedRuleRegex> {
        self.rules
            .iter()
            .filter(|entry| matches!(entry.rule.kind, RuleKind::Skip))
    }
}

/// Le as regras de `scanner/spec`, parseia e constrói a RE unificada.
pub fn build_unified_regex_spec() -> Result<UnifiedRegexSpec, String> {
    let mut parsed = parsed_regex_rules()?
        .into_iter()
        .enumerate()
        .map(|(declaration_order, (rule, regex))| {
            (
                declaration_order,
                AnnotatedRuleRegex {
                    rule,
                    regex,
                },
            )
        })
        .collect::<Vec<_>>();

    parsed.sort_by_key(|(declaration_order, entry)| (entry.rule.priority, *declaration_order));

    let rules = parsed
        .into_iter()
        .map(|(_, entry)| entry)
        .collect::<Vec<_>>();

    let unified_regex = merge_with_alternation(&rules);

    Ok(UnifiedRegexSpec {
        rules,
        unified_regex,
    })
}

fn merge_with_alternation(rules: &[AnnotatedRuleRegex]) -> Regex {
    let branches = rules
        .iter()
        .map(|entry| entry.regex.clone())
        .collect::<Vec<_>>();

    match branches.as_slice() {
        [] => Regex::EmptySet,
        [single] => single.clone(),
        _ => Regex::Alternation(branches),
    }
}

/// Executa o pipeline completo e escreve os artefatos do scanner gerado.
///
/// Saidas esperadas:
/// - `scanner_impl.rs` com `match state { ... }`
/// - `dfa_table.rs` com metadados do automato minimizado
pub fn generate_scanner_artifacts(
    scanner_impl_path: impl AsRef<Path>,
    dfa_table_path: impl AsRef<Path>,
) -> Result<(), String> {
    let spec = build_unified_regex_spec()?;
    let nfa = build_nfa_from_unified_spec(&spec);
    let dfa = build_dfa_from_nfa(&nfa);
    let minimized = minimize_dfa_hopcroft(&dfa);

    write_generated_scanner_to(scanner_impl_path, &minimized, &spec)?;
    let dfa_table_src = generate_dfa_table_module(&minimized, &spec)?;
    std::fs::write(dfa_table_path, dfa_table_src)
        .map_err(|err| format!("failed writing generated dfa table: {err}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_unified_regex_spec, generate_scanner_artifacts};
    use crate::lexer_gen::regex::Regex;

    fn unique_path(filename: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        p.push(format!("c_compiler_in_rust_{nanos}_{filename}"));
        p
    }

    #[test]
    fn builds_unified_regex_from_all_rules() {
        let spec = build_unified_regex_spec().expect("pipeline should build");
        assert!(!spec.rules.is_empty());

        match &spec.unified_regex {
            Regex::Alternation(branches) => assert_eq!(branches.len(), spec.rules.len()),
            other => panic!("expected alternation, got {other:?}"),
        }
    }

    #[test]
    fn keeps_priority_order() {
        let spec = build_unified_regex_spec().expect("pipeline should build");
        for pair in spec.rules.windows(2) {
            assert!(pair[0].rule.priority <= pair[1].rule.priority);
        }
    }

    #[test]
    fn writes_generated_files() {
        let scanner_path = unique_path("scanner_impl.rs");
        let table_path = unique_path("dfa_table.rs");

        generate_scanner_artifacts(&scanner_path, &table_path).expect("codegen should work");

        let scanner_src = std::fs::read_to_string(&scanner_path).expect("scanner file exists");
        let table_src = std::fs::read_to_string(&table_path).expect("table file exists");

        assert!(scanner_src.contains("pub fn dfa_next_state"));
        assert!(table_src.contains("DFA_STATE_COUNT"));

        let _ = std::fs::remove_file(scanner_path);
        let _ = std::fs::remove_file(table_path);
    }
}
