//! Minimizacao de DFA com o algoritmo de Hopcroft.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::lexer_gen::dfa::{Dfa, DfaState};
use crate::lexer_gen::nfa::TransitionSymbol;

/// Executa minimizacao de Hopcroft sobre um DFA.
///
/// O algoritmo preserva linguagem e retorna um DFA equivalente com menor
/// quantidade de estados (modulo estados inalcançaveis, que normalmente nao
/// existem neste ponto da pipeline).
pub fn minimize_dfa_hopcroft(dfa: &Dfa) -> Dfa {
    if dfa.states.len() <= 1 {
        return dfa.clone();
    }

    let alphabet = collect_alphabet(dfa);
    let mut partitions = initial_partitions(dfa);
    let mut worklist = partitions.clone();

    while let Some(splitter) = worklist.pop() {
        for symbol in &alphabet {
            let preimage = predecessor_set(dfa, symbol, &splitter);
            if preimage.is_empty() {
                continue;
            }

            let mut next_partitions = Vec::<BTreeSet<usize>>::new();

            for part in partitions {
                let intersection = set_intersection(&part, &preimage);
                if intersection.is_empty() || intersection.len() == part.len() {
                    next_partitions.push(part);
                    continue;
                }

                let difference = set_difference(&part, &preimage);
                next_partitions.push(intersection.clone());
                next_partitions.push(difference.clone());

                if let Some(pos) = worklist.iter().position(|w| *w == part) {
                    worklist.swap_remove(pos);
                    worklist.push(intersection);
                    worklist.push(difference);
                } else if intersection.len() <= difference.len() {
                    worklist.push(intersection);
                } else {
                    worklist.push(difference);
                }
            }

            partitions = next_partitions;
        }
    }

    rebuild_minimized_dfa(dfa, partitions)
}

fn initial_partitions(dfa: &Dfa) -> Vec<BTreeSet<usize>> {
    let mut groups = BTreeMap::<Option<usize>, BTreeSet<usize>>::new();

    for (state_idx, state) in dfa.states.iter().enumerate() {
        groups
            .entry(state.accept_rule_index)
            .or_default()
            .insert(state_idx);
    }

    groups
        .into_values()
        .filter(|set| !set.is_empty())
        .collect::<Vec<_>>()
}

fn collect_alphabet(dfa: &Dfa) -> Vec<TransitionSymbol> {
    let mut key_set = BTreeSet::<SymbolKey>::new();
    let mut key_to_symbol = HashMap::<SymbolKey, TransitionSymbol>::new();

    for state in &dfa.states {
        for symbol in state.transitions.keys() {
            let key = SymbolKey::from(symbol);
            key_set.insert(key.clone());
            key_to_symbol.entry(key).or_insert_with(|| symbol.clone());
        }
    }

    key_set
        .into_iter()
        .filter_map(|key| key_to_symbol.remove(&key))
        .collect::<Vec<_>>()
}

fn predecessor_set(dfa: &Dfa, symbol: &TransitionSymbol, target_set: &BTreeSet<usize>) -> BTreeSet<usize> {
    dfa.states
        .iter()
        .enumerate()
        .filter_map(|(state_idx, state)| {
            state
                .transitions
                .get(symbol)
                .and_then(|next| target_set.contains(next).then_some(state_idx))
        })
        .collect::<BTreeSet<_>>()
}

fn set_intersection(left: &BTreeSet<usize>, right: &BTreeSet<usize>) -> BTreeSet<usize> {
    left.intersection(right).copied().collect::<BTreeSet<_>>()
}

fn set_difference(left: &BTreeSet<usize>, right: &BTreeSet<usize>) -> BTreeSet<usize> {
    left.difference(right).copied().collect::<BTreeSet<_>>()
}

fn rebuild_minimized_dfa(dfa: &Dfa, mut partitions: Vec<BTreeSet<usize>>) -> Dfa {
    partitions.sort_by_key(|block| block.iter().next().copied().unwrap_or(usize::MAX));

    if let Some(start_block_idx) = partitions
        .iter()
        .position(|block| block.contains(&dfa.start_state))
    {
        partitions.swap(0, start_block_idx);
    }

    let mut old_to_new = HashMap::<usize, usize>::new();
    for (new_idx, block) in partitions.iter().enumerate() {
        for old_idx in block {
            old_to_new.insert(*old_idx, new_idx);
        }
    }

    let mut minimized_states = Vec::<DfaState>::with_capacity(partitions.len());
    for block in &partitions {
        let representative = *block
            .iter()
            .next()
            .expect("partition block should never be empty");
        let rep_state = &dfa.states[representative];

        let mut merged_nfa_states = block
            .iter()
            .flat_map(|old_idx| dfa.states[*old_idx].nfa_states.iter().copied())
            .collect::<Vec<_>>();
        merged_nfa_states.sort_unstable();
        merged_nfa_states.dedup();

        let mut transitions = HashMap::<TransitionSymbol, usize>::new();
        for (symbol, old_target) in &rep_state.transitions {
            if let Some(new_target) = old_to_new.get(old_target) {
                transitions.insert(symbol.clone(), *new_target);
            }
        }

        minimized_states.push(DfaState {
            nfa_states: merged_nfa_states,
            transitions,
            accept_rule_index: rep_state.accept_rule_index,
        });
    }

    Dfa {
        start_state: 0,
        states: minimized_states,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SymbolKey {
    Literal(char),
    AnyChar,
    CharClass(String),
}

impl SymbolKey {
    fn from(symbol: &TransitionSymbol) -> Self {
        match symbol {
            TransitionSymbol::Literal(ch) => Self::Literal(*ch),
            TransitionSymbol::AnyChar => Self::AnyChar,
            TransitionSymbol::CharClass(class) => Self::CharClass(format!("{class:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer_gen::dfa::{Dfa, DfaState, build_dfa_from_nfa};
    use crate::lexer_gen::minimize::minimize_dfa_hopcroft;
    use crate::lexer_gen::nfa::{TransitionSymbol, build_nfa_from_unified_spec};
    use crate::pipeline::build_unified_regex_spec;

    #[test]
    fn minimization_is_non_increasing() {
        let spec = build_unified_regex_spec().expect("valid spec");
        let nfa = build_nfa_from_unified_spec(&spec);
        let dfa = build_dfa_from_nfa(&nfa);
        let minimized = minimize_dfa_hopcroft(&dfa);

        assert!(minimized.states.len() <= dfa.states.len());
    }

    #[test]
    fn minimization_is_idempotent_for_state_count() {
        let spec = build_unified_regex_spec().expect("valid spec");
        let nfa = build_nfa_from_unified_spec(&spec);
        let dfa = build_dfa_from_nfa(&nfa);
        let minimized_once = minimize_dfa_hopcroft(&dfa);
        let minimized_twice = minimize_dfa_hopcroft(&minimized_once);

        assert_eq!(minimized_once.states.len(), minimized_twice.states.len());
    }

    #[test]
    fn merges_equivalent_states() {
        let mut s0 = DfaState::default();
        s0.transitions.insert(TransitionSymbol::Literal('a'), 1);
        s0.transitions.insert(TransitionSymbol::Literal('b'), 2);

        let mut s1 = DfaState {
            accept_rule_index: Some(0),
            ..DfaState::default()
        };
        s1.transitions.insert(TransitionSymbol::Literal('a'), 1);
        s1.transitions.insert(TransitionSymbol::Literal('b'), 2);

        let mut s2 = DfaState {
            accept_rule_index: Some(0),
            ..DfaState::default()
        };
        s2.transitions.insert(TransitionSymbol::Literal('a'), 1);
        s2.transitions.insert(TransitionSymbol::Literal('b'), 2);

        let dfa = Dfa {
            start_state: 0,
            states: vec![s0, s1, s2],
        };

        let minimized = minimize_dfa_hopcroft(&dfa);
        assert_eq!(minimized.states.len(), 2);
    }
}
