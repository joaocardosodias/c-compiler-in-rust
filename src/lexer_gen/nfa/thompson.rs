//! Construcao de Thompson para converter RE em NFA.

use crate::lexer_gen::nfa::state::{Nfa, TransitionSymbol};
use crate::lexer_gen::regex::Regex;
use crate::pipeline::UnifiedRegexSpec;

/// Fragmento intermediario da construcao de Thompson.
#[derive(Debug, Clone, Copy)]
struct Fragment {
    start: usize,
    accept: usize,
}

/// Constrói NFA para o conjunto de regras unificado.
///
/// Cada regra ganha um estado de aceitacao proprio, anotado com `rule_index`,
/// que corresponde ao indice em `UnifiedRegexSpec.rules`.
pub fn build_nfa_from_unified_spec(spec: &UnifiedRegexSpec) -> Nfa {
    let mut nfa = Nfa::new();
    let global_start = nfa.add_state();
    nfa.start_state = global_start;

    for (rule_index, entry) in spec.rules.iter().enumerate() {
        let fragment = build_fragment(&mut nfa, &entry.regex);
        nfa.add_epsilon(global_start, fragment.start);
        nfa.states[fragment.accept].accept_rule_index = Some(rule_index);
    }

    nfa
}

fn build_fragment(nfa: &mut Nfa, regex: &Regex) -> Fragment {
    match regex {
        Regex::EmptySet => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            Fragment { start, accept }
        }
        Regex::Epsilon => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_epsilon(start, accept);
            Fragment { start, accept }
        }
        Regex::Literal(ch) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::Literal(*ch));
            Fragment { start, accept }
        }
        Regex::AnyChar => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::AnyChar);
            Fragment { start, accept }
        }
        Regex::CharClass(class) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::CharClass(class.clone()));
            Fragment { start, accept }
        }
        Regex::Concat(parts) => {
            if parts.is_empty() {
                return build_fragment(nfa, &Regex::Epsilon);
            }

            let mut iter = parts.iter();
            let mut current = build_fragment(nfa, iter.next().expect("non-empty concat"));

            for part in iter {
                let next = build_fragment(nfa, part);
                nfa.add_epsilon(current.accept, next.start);
                current = Fragment {
                    start: current.start,
                    accept: next.accept,
                };
            }

            current
        }
        Regex::Alternation(arms) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();

            if arms.is_empty() {
                return Fragment { start, accept };
            }

            for arm in arms {
                let branch = build_fragment(nfa, arm);
                nfa.add_epsilon(start, branch.start);
                nfa.add_epsilon(branch.accept, accept);
            }

            Fragment { start, accept }
        }
        Regex::Star(inner) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner);

            nfa.add_epsilon(start, accept);
            nfa.add_epsilon(start, child.start);
            nfa.add_epsilon(child.accept, child.start);
            nfa.add_epsilon(child.accept, accept);

            Fragment { start, accept }
        }
        Regex::Plus(inner) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner);

            nfa.add_epsilon(start, child.start);
            nfa.add_epsilon(child.accept, child.start);
            nfa.add_epsilon(child.accept, accept);

            Fragment { start, accept }
        }
        Regex::Optional(inner) => {
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner);

            nfa.add_epsilon(start, accept);
            nfa.add_epsilon(start, child.start);
            nfa.add_epsilon(child.accept, accept);

            Fragment { start, accept }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::build_unified_regex_spec;

    use super::build_nfa_from_unified_spec;

    #[test]
    fn creates_global_start_state() {
        let spec = build_unified_regex_spec().expect("valid rules");
        let nfa = build_nfa_from_unified_spec(&spec);

        assert_eq!(nfa.start_state, 0);
        assert!(!nfa.states.is_empty());
    }

    #[test]
    fn annotates_accept_states_with_rule_index() {
        let spec = build_unified_regex_spec().expect("valid rules");
        let nfa = build_nfa_from_unified_spec(&spec);

        let mut accepted = nfa
            .states
            .iter()
            .filter_map(|state| state.accept_rule_index)
            .collect::<Vec<_>>();
        accepted.sort_unstable();
        accepted.dedup();

        let expected = (0..spec.rules.len()).collect::<Vec<_>>();
        assert_eq!(accepted, expected);
    }
}
