//! Minimizacao de DFA com o algoritmo de Hopcroft.
//!
//! O DFA gerado pelo `subset.rs` geralmente tem muitos estados redundantes.
//! O algoritmo de Hopcroft encontra e mescla estados equivalentes para que
//! a nossa tabela e o código gerado final fiquem o menor possível.

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

    // 1. Precisamos do alfabeto do DFA.
    let alphabet = collect_alphabet(dfa);
    
    // 2. Partição inicial. Dividimos os estados do DFA em blocos com base na regra de aceitação.
    // Ex: Todos os estados que aceitam `KwIf` ficam num bloco, os que não aceitam nada (None) em outro.
    // Estados que aceitam coisas diferentes NUNCA podem ser mesclados.
    let mut partitions = initial_partitions(dfa);
    
    // 3. A Worklist (Fila de trabalho) começa com todos os blocos iniciais.
    let mut worklist = partitions.clone();

    // 4. O loop principal de Hopcroft.
    while let Some(splitter) = worklist.pop() {
        // `splitter` é um bloco de estados que sabemos que é "diferente" dos demais.
        
        for symbol in &alphabet {
            // A) Computa o predecessor (Preimage): quem consegue chegar no `splitter` lendo `symbol`?
            let preimage = predecessor_set(dfa, symbol, &splitter);
            if preimage.is_empty() {
                continue;
            }

            let mut next_partitions = Vec::<BTreeSet<usize>>::new();

            // B) Para cada bloco existente, vamos tentar quebrá-lo ao meio usando o preimage.
            for part in partitions {
                let intersection = set_intersection(&part, &preimage);
                
                // Se o bloco inteiro vai pro splitter (intersection == part)
                // ou ninguém vai (intersection vazia), então esse símbolo NÃO quebra esse bloco.
                if intersection.is_empty() || intersection.len() == part.len() {
                    next_partitions.push(part);
                    continue;
                }

                // O bloco rachou! Uma parte vai pro splitter, outra não.
                let difference = set_difference(&part, &preimage);
                
                // Salva as duas metades no lugar do bloco original.
                next_partitions.push(intersection.clone());
                next_partitions.push(difference.clone());

                // C) Atualiza a worklist pra que as novas metades possam atuar como splitters no futuro.
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

    // 5. Após estabilizar (a worklist vazia), reconstruímos o DFA fundindo os estados dos blocos restantes.
    rebuild_minimized_dfa(dfa, partitions)
}

/// Cria a partição 0: agrupa estados pela ação de aceitação deles.
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

/// Coleta todos os símbolos que engatilham transições no DFA.
fn collect_alphabet(dfa: &Dfa) -> Vec<TransitionSymbol> {
    // Como Symbol é meio chato de usar como chave limpa, criamos um SymbolKey.
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

/// Predecessor (Preimage): Descobre quais estados do DFA vão parar dentro do `target_set` ao ler `symbol`.
fn predecessor_set(
    dfa: &Dfa,
    symbol: &TransitionSymbol,
    target_set: &BTreeSet<usize>,
) -> BTreeSet<usize> {
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

/// Utilitário matemático: Interseção
fn set_intersection(left: &BTreeSet<usize>, right: &BTreeSet<usize>) -> BTreeSet<usize> {
    left.intersection(right).copied().collect::<BTreeSet<_>>()
}

/// Utilitário matemático: Diferença (left - right)
fn set_difference(left: &BTreeSet<usize>, right: &BTreeSet<usize>) -> BTreeSet<usize> {
    left.difference(right).copied().collect::<BTreeSet<_>>()
}

/// Constroi um DFA novinho mesclando os estados que ficaram no mesmo bloco da partição final.
fn rebuild_minimized_dfa(dfa: &Dfa, mut partitions: Vec<BTreeSet<usize>>) -> Dfa {
    // 1. Deixa o bloco que contém o estado inicial (0) na posição 0 da lista pra facilitar.
    partitions.sort_by_key(|block| block.iter().next().copied().unwrap_or(usize::MAX));

    if let Some(start_block_idx) = partitions
        .iter()
        .position(|block| block.contains(&dfa.start_state))
    {
        partitions.swap(0, start_block_idx);
    }

    // 2. Mapa `Velho_ID` -> `Novo_ID` (O Novo ID é só o índice do bloco na lista `partitions`).
    let mut old_to_new = HashMap::<usize, usize>::new();
    for (new_idx, block) in partitions.iter().enumerate() {
        for old_idx in block {
            old_to_new.insert(*old_idx, new_idx);
        }
    }

    // 3. Monta os novos estados.
    let mut minimized_states = Vec::<DfaState>::with_capacity(partitions.len());
    for block in &partitions {
        // Escolhe o primeiro estado do bloco como "representante" pra copiar os dados.
        // Já que o algoritmo não quebrou o bloco, significa que todos os estados ali
        // têm EXATAMENTE O MESMO comportamento.
        let representative = *block
            .iter()
            .next()
            .expect("partition block should never be empty");
        let rep_state = &dfa.states[representative];

        // Faz um merge da lista de "nfa_states" que originou cada estado (só pra debug/logs ficarem fiéis).
        let mut merged_nfa_states = block
            .iter()
            .flat_map(|old_idx| dfa.states[*old_idx].nfa_states.iter().copied())
            .collect::<Vec<_>>();
        merged_nfa_states.sort_unstable();
        merged_nfa_states.dedup();

        // 4. Reescreve os destinos das transições usando o mapa `old_to_new`.
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

    // O start_state sempre será 0 graças ao `swap` que fizemos ali no começo.
    Dfa {
        start_state: 0,
        states: minimized_states,
    }
}

/// Um "hash" simplificado do TransitionSymbol para usarmos no collect_alphabet().
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
