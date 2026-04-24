//! Construcao de subconjuntos (NFA -> DFA).
//!
//! O algoritmo central que elimina a ambiguidade (epsilon-transições e múltiplas transições pro mesmo char)
//! do NFA, agrupando conjuntos de estados NFA em um único estado DFA.

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

    // 1. Pega todas as "classes de equivalência" do alfabeto original.
    // Isso é vital para não testar todos os infinitos possíveis caracteres na hora
    // de verificar transições.
    let alphabet = collect_alphabet(nfa);

    // 2. Cria o estado inicial do DFA. Ele é o Epsilon-Closure do estado inicial do NFA.
    // Ou seja: tudo o que é alcançável antes mesmo de lermos a primeira letra do código-fonte.
    let start_closure = epsilon_closure(nfa, std::iter::once(nfa.start_state));
    // A chave (Key) de um estado DFA é o conjunto ordenado de IDs dos estados NFA.
    let start_key = key_from_set(&start_closure);

    let mut dfa_states = Vec::<DfaState>::new();
    
    // Mapa auxiliar: converte o conjunto de estados NFA (a "chave") no ID do estado DFA correspondente.
    // Evita criarmos estados DFA duplicados se chegarmos no mesmo conjunto de estados de formas diferentes.
    let mut state_ids = HashMap::<Vec<usize>, usize>::new();
    
    // Fila de trabalho (Worklist): guarda as "chaves" dos estados DFA que acabamos de criar,
    // mas que ainda precisamos computar as transições de saída.
    let mut queue = VecDeque::<Vec<usize>>::new();

    let start_id = 0;
    state_ids.insert(start_key.clone(), start_id);
    dfa_states.push(make_dfa_state(nfa, &start_closure));
    queue.push_back(start_key);

    // 3. Processa a fila de trabalho até não haver mais estados não explorados.
    while let Some(current_key) = queue.pop_front() {
        // Pega qual é o ID numérico desse conjunto no nosso vetor de estados.
        let current_id = *state_ids
            .get(&current_key)
            .expect("DFA state key should exist");

        // Para CADA símbolo do alfabeto "abstrato"...
        for symbol in &alphabet {
            // A) Computa os estados do NFA diretamente alcançáveis lendo esse símbolo.
            let moved = move_on_symbol(nfa, &current_key, symbol);
            if moved.is_empty() {
                continue; // Se não for pra lugar nenhum, pula esse símbolo (DFA morre).
            }

            // B) Para onde quer que a gente tenha ido, propaga pelo Epsilon-Closure.
            let closure = epsilon_closure(nfa, moved.into_iter());
            let next_key = key_from_set(&closure);
            if next_key.is_empty() {
                continue;
            }

            // C) Verifica se esse conjunto de destino já virou um estado no DFA.
            let next_id = if let Some(existing) = state_ids.get(&next_key) {
                // Já existia! Apenas reaproveita o ID.
                *existing
            } else {
                // É um conjunto novo. Cria um ID, registra no dicionário e joga na fila de trabalho.
                let new_id = dfa_states.len();
                state_ids.insert(next_key.clone(), new_id);
                dfa_states.push(make_dfa_state(nfa, &closure));
                queue.push_back(next_key);
                new_id
            };

            // D) Adiciona a transição determinística.
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

/// Cria o objeto `DfaState` avaliando se ele é um estado final.
fn make_dfa_state(nfa: &Nfa, nfa_set: &BTreeSet<usize>) -> DfaState {
    DfaState {
        nfa_states: key_from_set(nfa_set),
        transitions: HashMap::new(),
        // Se pelo menos um dos estados do NFA era um estado de aceitação, o DFA também é.
        accept_rule_index: best_accept_rule_index(nfa, nfa_set),
    }
}

/// Acha a regra de maior prioridade (menor número de index) neste conjunto de estados NFA.
fn best_accept_rule_index(nfa: &Nfa, nfa_set: &BTreeSet<usize>) -> Option<usize> {
    nfa_set
        .iter()
        // Pega todos os índices de aceitação que não são None.
        .filter_map(|idx| nfa.states[*idx].accept_rule_index)
        // Como a regra de prioridade mais alta tem o menor número (ex: 0 vence de 10), pegamos o `.min()`.
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
            // Um jeito sujo mas prático de gerar "hash" pra evitar duplicata na lista.
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
    // Ex: a letra 'a' pode casar com Literal('a') e CharClass([a-z]), gerando assinatura [0, 1].
    // A letra 'b' casa só com CharClass([a-z]), gerando assinatura [1].
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
                // Otimização: se a classe só tem 1 letra, vira um Literal simples.
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

/// Epsilon-Closure: Dado um conjunto inicial de estados NFA, acha todos os outros estados NFA
/// que podemos chegar SOMENTE cruzando pontes de Epsilon-transições (sem ler nada da entrada).
fn epsilon_closure<I>(nfa: &Nfa, initial: I) -> BTreeSet<usize>
where
    I: IntoIterator<Item = usize>,
{
    let mut closure = BTreeSet::<usize>::new(); // O resultado final.
    let mut stack = Vec::<usize>::new(); // Pilha de DFS para exploração de grafo.

    // 1. Começamos a explorar a partir de cada estado da semente inicial.
    for state in initial {
        if closure.insert(state) {
            stack.push(state);
        }
    }

    // 2. Faz uma Busca em Profundidade (DFS) pra achar todo mundo que tá conectado por Epsilon.
    while let Some(state) = stack.pop() {
        for next in &nfa.states[state].epsilon {
            // Se conseguimos adicionar no BTreeSet, é porque ainda não visitamos!
            if closure.insert(*next) {
                stack.push(*next);
            }
        }
    }

    closure
}

/// Calcula os estados NFA alcancaveis por um simbolo de particao a partir de um conjunto de origens.
///
/// Usa um caractere representativo da classe de particao para verificar quais
/// transicoes NFA o aceitam semanticamente, em vez de igualdade estrutural.
/// Isso e' correto porque a particao do `collect_alphabet` garante que todos os chars de um grupo
/// tem comportamento identico em todos os estados NFA.
fn move_on_symbol(nfa: &Nfa, from_set: &[usize], query: &TransitionSymbol) -> BTreeSet<usize> {
    let Some(rep) = query.representative() else {
        return BTreeSet::new();
    };

    let mut dest = BTreeSet::<usize>::new();
    for state_idx in from_set {
        let state = &nfa.states[*state_idx];
        for transition in &state.transitions {
            // Usa o caracter genérico/representativo da classe abstrata para ver
            // se o estado atual consegue atravessar a ponte baseada em símbolo.
            if transition.symbol.matches(rep) {
                dest.insert(transition.to);
            }
        }
    }
    dest
}

/// Helper pra converter Set -> Vec pra poder usar como chave do HashMap de estados visitados.
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
