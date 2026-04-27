use std::fmt::Write as _;
use std::path::Path;

use crate::scanner::lexer_gen::dfa::Dfa;
use crate::scanner::lexer_gen::nfa::TransitionSymbol;
use crate::scanner::pipeline::UnifiedRegexSpec;
use crate::scanner::RuleKind;

pub fn generate_rust_match_scanner(dfa: &Dfa, spec: &UnifiedRegexSpec) -> Result<String, String> {
    validate_accept_indices(dfa, spec)?;

    let mut out = String::new();
    write_header(&mut out);
    write_accept_table(&mut out, dfa, spec);
    write_next_state_fn(&mut out, dfa);

    Ok(out)
}

pub fn write_generated_scanner_to(
    output_path: impl AsRef<Path>,
    dfa: &Dfa,
    spec: &UnifiedRegexSpec,
) -> Result<(), String> {
    let source = generate_rust_match_scanner(dfa, spec)?;
    std::fs::write(output_path, source)
        .map_err(|err| format!("failed writing generated scanner: {err}"))
}

pub fn generate_dfa_table_module(dfa: &Dfa, spec: &UnifiedRegexSpec) -> Result<String, String> {
    validate_accept_indices(dfa, spec)?;

    let mut out = String::new();
    out.push_str("// This file is generated. Do not edit by hand.\n\n");
    out.push_str("pub const DFA_STATE_COUNT: usize = ");
    let _ = writeln!(out, "{};", dfa.states.len());
    out.push_str("pub const DFA_START_STATE: usize = ");
    let _ = writeln!(out, "{};\n", dfa.start_state);

    out.push_str("pub const DFA_ACCEPT_RULE_INDEX: [Option<usize>; DFA_STATE_COUNT] = [\n");
    for state in &dfa.states {
        match state.accept_rule_index {
            Some(rule_idx) => {
                let _ = writeln!(out, "    Some({rule_idx}),");
            }
            None => out.push_str("    None,\n"),
        }
    }
    out.push_str("];\n");

    Ok(out)
}

fn validate_accept_indices(dfa: &Dfa, spec: &UnifiedRegexSpec) -> Result<(), String> {
    let max = spec.rules.len();
    for (idx, state) in dfa.states.iter().enumerate() {
        if let Some(rule_idx) = state.accept_rule_index {
            if rule_idx >= max {
                return Err(format!(
                    "dfa state {idx} refers to missing rule index {rule_idx} (rules: {max})"
                ));
            }
        }
    }
    Ok(())
}

fn write_header(out: &mut String) {
    out.push_str("// This file is generated. Do not edit by hand.\n");
    out.push_str("use crate::scanner::tokens::TokenKind;\n\n");

    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    out.push_str("pub enum GeneratedAcceptAction {\n");
    out.push_str("    Emit(TokenKind),\n");
    out.push_str("    Skip,\n");
    out.push_str("}\n\n");
}

fn write_accept_table(out: &mut String, dfa: &Dfa, spec: &UnifiedRegexSpec) {
    out.push_str("pub fn dfa_accept_action(state: usize) -> Option<GeneratedAcceptAction> {\n");
    out.push_str("    match state {\n");

    for (state_idx, state) in dfa.states.iter().enumerate() {
        if let Some(rule_idx) = state.accept_rule_index {
            let rule = &spec.rules[rule_idx].rule;
            match rule.kind {
                RuleKind::Emit(token) => {
                    let _ = writeln!(
                        out,
                        "        {state_idx} => Some(GeneratedAcceptAction::Emit(TokenKind::{token:?})),"
                    );
                }
                RuleKind::Skip => {
                    let _ = writeln!(
                        out,
                        "        {state_idx} => Some(GeneratedAcceptAction::Skip),"
                    );
                }
            }
        }
    }

    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn write_next_state_fn(out: &mut String, dfa: &Dfa) {
    out.push_str("pub fn dfa_next_state(state: usize, ch: char) -> Option<usize> {\n");
    out.push_str("    match state {\n");

    for (state_idx, state) in dfa.states.iter().enumerate() {
        let _ = writeln!(out, "        {state_idx} => {{");

        let mut transitions = state
            .transitions
            .iter()
            .map(|(symbol, target)| (symbol_key(symbol), symbol, *target))
            .collect::<Vec<_>>();
        transitions.sort_by(|left, right| left.0.cmp(&right.0));

        if transitions.is_empty() {
            out.push_str("            None\n");
        } else {
            out.push_str("            ");
            for (i, (_, symbol, target)) in transitions.iter().enumerate() {
                if i == 0 {
                    let _ = write!(out, "if {} {{ Some({target}) }}", symbol_predicate(symbol));
                } else {
                    let _ = write!(
                        out,
                        " else if {} {{ Some({target}) }}",
                        symbol_predicate(symbol)
                    );
                }
            }

            out.push_str(" else { None }\n");
        }

        out.push_str("        }\n");
    }

    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n");
}

fn symbol_key(symbol: &TransitionSymbol) -> String {
    match symbol {
        TransitionSymbol::Literal(ch) => format!("lit:{:08x}", *ch as u32),
        TransitionSymbol::AnyChar => "any".to_string(),
        TransitionSymbol::CharClass(class) => format!("class:{class:?}"),
    }
}

fn symbol_predicate(symbol: &TransitionSymbol) -> String {
    match symbol {
        TransitionSymbol::Literal(ch) => format!("ch == {}", char_literal(*ch)),
        TransitionSymbol::AnyChar => "true".to_string(),
        TransitionSymbol::CharClass(class) => {
            let mut item_preds = Vec::<String>::new();
            for item in &class.items {
                match item {
                    crate::scanner::lexer_gen::regex::CharClassItem::Char(ch) => {
                        item_preds.push(format!("ch == {}", char_literal(*ch)));
                    }
                    crate::scanner::lexer_gen::regex::CharClassItem::Range(start, end) => {
                        item_preds.push(format!(
                            "({}..={}).contains(&ch)",
                            char_literal(*start),
                            char_literal(*end)
                        ));
                    }
                }
            }

            let base = if item_preds.is_empty() {
                "false".to_string()
            } else {
                format!("({})", item_preds.join(" || "))
            };

            if class.negated {
                format!("!{base}")
            } else {
                base
            }
        }
    }
}

fn char_literal(ch: char) -> String {
    match ch {
        '\\' => "'\\\\'".to_string(),
        '\'' => "'\\\''".to_string(),
        '\n' => "'\\n'".to_string(),
        '\r' => "'\\r'".to_string(),
        '\t' => "'\\t'".to_string(),
        '\0' => "'\\0'".to_string(),
        c if c.is_control() => format!("'\\u{{{:x}}}'", c as u32),
        c => format!("'{c}'"),
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::lexer_gen::codegen::generate_rust_match_scanner;
    use crate::scanner::lexer_gen::dfa::build_dfa_from_nfa;
    use crate::scanner::lexer_gen::minimize::minimize_dfa_hopcroft;
    use crate::scanner::lexer_gen::nfa::build_nfa_from_unified_spec;
    use crate::scanner::pipeline::build_unified_regex_spec;

    #[test]
    fn generates_match_based_scanner_source() {
        let spec = build_unified_regex_spec().expect("valid spec");
        let nfa = build_nfa_from_unified_spec(&spec);
        let dfa = build_dfa_from_nfa(&nfa);
        let minimized = minimize_dfa_hopcroft(&dfa);

        let src = generate_rust_match_scanner(&minimized, &spec).expect("codegen should succeed");
        assert!(src.contains("pub fn dfa_next_state"));
        assert!(src.contains("pub fn dfa_accept_action"));
        assert!(src.contains("GeneratedAcceptAction"));
    }
}
