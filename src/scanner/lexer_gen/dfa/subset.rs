use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::scanner::lexer_gen::dfa::state::{Dfa, DfaState};
use crate::scanner::lexer_gen::nfa::{Nfa, TransitionSymbol};

pub fn build_dfa_from_nfa(nfa: &Nfa) -> Dfa {
    if nfa.states.is_empty() {
        return Dfa::default();
    }

    let alphabet = collect_alphabet(nfa);

    let start_closure = epsilon_closure(nfa, std::iter::once(nfa.start_state));

    let start_key = key_from_set(&start_closure);

    let mut dfa_states = Vec::<DfaState>::new();

    let mut state_ids = HashMap::<Vec<usize>, usize>::new();

    let mut queue = VecDeque::<Vec<usize>>::new();

    let start_id = 0;
    state_ids.insert(start_key.clone(), start_id);
    dfa_states.push(make_dfa_state(nfa, &start_closure));
    queue.push_back(start_key);

    while let Some(current_key) = queue.pop_front() {
        let current_id = *state_ids
            .get(&current_key)
            .expect("DFA state key should exist");

        for symbol in &alphabet {
            let moved = move_on_symbol(nfa, &current_key, symbol);
            if moved.is_empty() {
                continue;
            }

            let closure = epsilon_closure(nfa, moved.into_iter());
            let next_key = key_from_set(&closure);
            if next_key.is_empty() {
                continue;
            }

            let next_id = if let Some(existing) = state_ids.get(&next_key) {
                *existing
            } else {
                let new_id = dfa_states.len();
                state_ids.insert(next_key.clone(), new_id);
                dfa_states.push(make_dfa_state(nfa, &closure));
                queue.push_back(next_key);
                new_id
            };

            dfa_states[current_id]
                .transitions
                .insert(symbol.clone(), next_id);
        }
    }

    Dfa {
        start_state: start_id,
        states: dfa_states,
    }
}

fn make_dfa_state(nfa: &Nfa, nfa_set: &BTreeSet<usize>) -> DfaState {
    DfaState {
        nfa_states: key_from_set(nfa_set),
        transitions: HashMap::new(),

        accept_rule_index: best_accept_rule_index(nfa, nfa_set),
    }
}

fn best_accept_rule_index(nfa: &Nfa, nfa_set: &BTreeSet<usize>) -> Option<usize> {
    nfa_set
        .iter()
        .filter_map(|idx| nfa.states[*idx].accept_rule_index)
        .min()
}

fn collect_alphabet(nfa: &Nfa) -> Vec<TransitionSymbol> {
    use crate::scanner::lexer_gen::regex::{CharClass, CharClassItem};

    let mut raw_symbols: Vec<TransitionSymbol> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for state in &nfa.states {
        for t in &state.transitions {
            let key = format!("{:?}", t.symbol);
            if seen.insert(key) {
                raw_symbols.push(t.symbol.clone());
            }
        }
    }

    if raw_symbols.is_empty() {
        return Vec::new();
    }

    let mut groups: BTreeMap<Vec<usize>, Vec<char>> = BTreeMap::new();
    for code in 0u32..=127 {
        if let Some(ch) = char::from_u32(code) {
            let sig: Vec<usize> = raw_symbols
                .iter()
                .enumerate()
                .filter(|(_, sym)| sym.matches(ch))
                .map(|(i, _)| i)
                .collect();

            if !sig.is_empty() {
                groups.entry(sig).or_default().push(ch);
            }
        }
    }

    groups
        .into_values()
        .filter(|chars| !chars.is_empty())
        .map(|chars| {
            if chars.len() == 1 {
                TransitionSymbol::Literal(chars[0])
            } else {
                TransitionSymbol::CharClass(CharClass {
                    negated: false,
                    items: chars.into_iter().map(CharClassItem::Char).collect(),
                })
            }
        })
        .collect()
}

fn epsilon_closure<I>(nfa: &Nfa, initial: I) -> BTreeSet<usize>
where
    I: IntoIterator<Item = usize>,
{
    let mut closure = BTreeSet::<usize>::new();
    let mut stack = Vec::<usize>::new();

    for state in initial {
        if closure.insert(state) {
            stack.push(state);
        }
    }

    while let Some(state) = stack.pop() {
        for next in &nfa.states[state].epsilon {
            if closure.insert(*next) {
                stack.push(*next);
            }
        }
    }

    closure
}

fn move_on_symbol(nfa: &Nfa, from_set: &[usize], query: &TransitionSymbol) -> BTreeSet<usize> {
    let Some(rep) = query.representative() else {
        return BTreeSet::new();
    };

    let mut dest = BTreeSet::<usize>::new();
    for state_idx in from_set {
        let state = &nfa.states[*state_idx];
        for transition in &state.transitions {
            if transition.symbol.matches(rep) {
                dest.insert(transition.to);
            }
        }
    }
    dest
}

fn key_from_set(set: &BTreeSet<usize>) -> Vec<usize> {
    set.iter().copied().collect()
}

#[cfg(test)]
mod tests {
    use crate::scanner::lexer_gen::dfa::build_dfa_from_nfa;
    use crate::scanner::lexer_gen::nfa::build_nfa_from_unified_spec;
    use crate::scanner::pipeline::build_unified_regex_spec;

    #[test]
    fn subset_construction_creates_start_state() {
        let spec = build_unified_regex_spec().expect("valid spec");
        let nfa = build_nfa_from_unified_spec(&spec);
        let dfa = build_dfa_from_nfa(&nfa);

        assert_eq!(dfa.start_state, 0);
        assert!(!dfa.states.is_empty());
    }

    #[test]
    fn chooses_lowest_rule_index_for_accept_state() {
        let spec = build_unified_regex_spec().expect("valid spec");
        let nfa = build_nfa_from_unified_spec(&spec);
        let dfa = build_dfa_from_nfa(&nfa);

        for state in &dfa.states {
            if let Some(selected) = state.accept_rule_index {
                let expected = state
                    .nfa_states
                    .iter()
                    .filter_map(|idx| nfa.states[*idx].accept_rule_index)
                    .min();
                assert_eq!(Some(selected), expected);
            }
        }
    }
}
