//! Construcao de subconjuntos (NFA -> DFA).

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::lexer_gen::dfa::state::{Dfa, DfaState};
use crate::lexer_gen::nfa::{Nfa, TransitionSymbol};

/// Converte um NFA em DFA usando construcao de subconjuntos.
///
/// Observacao: a determinizacao aqui usa `TransitionSymbol` como alfabeto
/// abstrato. O refinamento de classes de caracteres para um alfabeto disjunto
/// pode ser aplicado em um passo posterior.
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

/// Computa o alfabeto do DFA como uma particao de caracteres.
///
/// Agrupa todos os codepoints ASCII (0..=127) por sua "assinatura": o conjunto
/// de indices de simbolos NFA que os aceitam. Cada grupo vira um elemento disjunto
/// do alfabeto, garantindo que a construcao de subconjuntos seja deterministica
/// mesmo quando simbolos NFA se sobrepoem (ex: `Literal('i')` e `CharClass([A-Za-z_])`).
fn collect_alphabet(nfa: &Nfa) -> Vec<TransitionSymbol> {
    use crate::lexer_gen::regex::{CharClass, CharClassItem};

    // Coleta todos os simbolos distintos do NFA (sem duplicatas por chave de debug).
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

    // Para cada codepoint ASCII 0..=127, calcula sua assinatura:
    // lista ordenada dos indices dos simbolos brutos que o aceitam.
    // Agrupa codepoints com assinatura identica na mesma classe de equivalencia.
    let mut groups: BTreeMap<Vec<usize>, Vec<char>> = BTreeMap::new();
    for code in 0u32..=127 {
        if let Some(ch) = char::from_u32(code) {
            let sig: Vec<usize> = raw_symbols
                .iter()
                .enumerate()
                .filter(|(_, sym)| sym.matches(ch))
                .map(|(i, _)| i)
                .collect();
            // Ignora codepoints que nao correspondem a nenhum simbolo NFA.
            if !sig.is_empty() {
                groups.entry(sig).or_default().push(ch);
            }
        }
    }

    // Converte cada grupo em um TransitionSymbol::CharClass disjunto.
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

/// Calcula os estados NFA alcancaveis por um simbolo de particao a partir de um conjunto.
///
/// Usa um caractere representativo da classe de particao para verificar quais
/// transicoes NFA o aceitam semanticamente, em vez de igualdade estrutural.
/// Isso e' correto porque a particao garante que todos os chars de um grupo
/// tem comportamento identico em todos os estados NFA.
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
    use crate::lexer_gen::dfa::build_dfa_from_nfa;
    use crate::lexer_gen::nfa::build_nfa_from_unified_spec;
    use crate::pipeline::build_unified_regex_spec;

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
