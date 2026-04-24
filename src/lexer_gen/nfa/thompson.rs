//! Construcao de Thompson para converter RE em NFA.
//!
//! O algoritmo de Thompson é elegante por ser puramente composicional:
//! Para cada tipo de Regex, existe um "template" ou fragmento com exatos
//! 1 estado de entrada e 1 estado de saída. Nós colamos esses fragmentos
//! usando epsilon-transições para formar o autômato inteiro.

use crate::lexer_gen::nfa::state::{Nfa, TransitionSymbol};
use crate::lexer_gen::regex::Regex;
use crate::pipeline::UnifiedRegexSpec;

/// Fragmento intermediario da construcao de Thompson.
///
/// Durante a recursão, cada pedaço da AST retorna um `Fragment`.
/// Ele contém os IDs do estado de início e do estado de fim desse bloco.
#[derive(Debug, Clone, Copy)]
struct Fragment {
    start: usize,
    accept: usize, // Neste contexto local, `accept` é apenas a saída do fragmento, não um estado final global.
}

/// Constrói NFA para o conjunto de regras unificado.
///
/// Cada regra ganha um estado de aceitacao proprio, anotado com `rule_index`,
/// que corresponde ao indice em `UnifiedRegexSpec.rules`.
pub fn build_nfa_from_unified_spec(spec: &UnifiedRegexSpec) -> Nfa {
    let mut nfa = Nfa::new();
    
    // Cria um super estado inicial. Ele vai ramificar (via epsilon) para
    // o início do autômato de CADA REGRA individualmente.
    let global_start = nfa.add_state();
    nfa.start_state = global_start;

    for (rule_index, entry) in spec.rules.iter().enumerate() {
        // Constrói o NFA só para esta regra (recursivamente).
        let fragment = build_fragment(&mut nfa, &entry.regex);
        
        // Liga o super estado inicial ao início do NFA desta regra.
        nfa.add_epsilon(global_start, fragment.start);
        
        // Marca o estado de saída *deste fragmento* como um verdadeiro estado final de aceitação.
        // Se a string parar aqui, achamos o token referente à regra `rule_index`.
        nfa.states[fragment.accept].accept_rule_index = Some(rule_index);
    }

    nfa
}

/// Constrói os sub-autômatos de forma recursiva baseada no tipo de nó da AST.
fn build_fragment(nfa: &mut Nfa, regex: &Regex) -> Fragment {
    match regex {
        Regex::EmptySet => {
            // Regra morta. Cria início e fim mas NÃO OS LIGA. Nenhuma string passa.
            let start = nfa.add_state();
            let accept = nfa.add_state();
            Fragment { start, accept }
        }
        Regex::Epsilon => {
            // Regra epsilon. Liga o início ao fim sem consumir nada.
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_epsilon(start, accept);
            Fragment { start, accept }
        }
        Regex::Literal(ch) => {
            // Consome 1 caractere exato.
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::Literal(*ch));
            Fragment { start, accept }
        }
        Regex::AnyChar => {
            // Consome o coringa (ponto).
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::AnyChar);
            Fragment { start, accept }
        }
        Regex::CharClass(class) => {
            // Consome uma classe (ex: [a-zA-Z]).
            let start = nfa.add_state();
            let accept = nfa.add_state();
            nfa.add_symbol_transition(start, accept, TransitionSymbol::CharClass(class.clone()));
            Fragment { start, accept }
        }
        Regex::Concat(parts) => {
            // Concatenar blocos: A B C...
            if parts.is_empty() {
                return build_fragment(nfa, &Regex::Epsilon);
            }

            let mut iter = parts.iter();
            
            // Constrói o primeiro fragmento (ex: A).
            let mut current = build_fragment(nfa, iter.next().expect("non-empty concat"));

            for part in iter {
                // Constrói o próximo fragmento (ex: B).
                let next = build_fragment(nfa, part);
                
                // Cola o fim do atual no início do próximo (fim do A aponta pro início do B).
                nfa.add_epsilon(current.accept, next.start);
                
                // Atualiza o ponteiro de "atual", pegando o start do primeirão e o accept do último lido.
                current = Fragment {
                    start: current.start,
                    accept: next.accept,
                };
            }

            current
        }
        Regex::Alternation(arms) => {
            // A | B | C
            let start = nfa.add_state();
            let accept = nfa.add_state();

            if arms.is_empty() {
                return Fragment { start, accept };
            }

            for arm in arms {
                // Para cada sub-expressão, constrói seu fragmento.
                let branch = build_fragment(nfa, arm);
                
                // Liga o start geral aos starts de todas as ramificações.
                nfa.add_epsilon(start, branch.start);
                
                // Liga o accept de todas as ramificações ao accept geral.
                nfa.add_epsilon(branch.accept, accept);
            }

            Fragment { start, accept }
        }
        Regex::Star(inner) => {
            // Kleene Star (A*) - zero ou mais.
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner); // Constrói o miolo (A).

            // Pula direto pro fim (faz o "zero").
            nfa.add_epsilon(start, accept);
            // Entra no miolo (faz o "ou mais").
            nfa.add_epsilon(start, child.start);
            // Ao terminar o miolo, volta pro início dele pra repetir o loop.
            nfa.add_epsilon(child.accept, child.start);
            // Ao terminar o miolo, pode sair pro aceitador.
            nfa.add_epsilon(child.accept, accept);

            Fragment { start, accept }
        }
        Regex::Plus(inner) => {
            // A+ (um ou mais). Mesma coisa do Star, mas SEM a transição de start direto pro accept.
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner);

            // Obrigatório entrar no miolo pelo menos uma vez.
            nfa.add_epsilon(start, child.start);
            // Volta do fim do miolo pro início do miolo pra repetir.
            nfa.add_epsilon(child.accept, child.start);
            // Sai do miolo.
            nfa.add_epsilon(child.accept, accept);

            Fragment { start, accept }
        }
        Regex::Optional(inner) => {
            // A? (zero ou um).
            let start = nfa.add_state();
            let accept = nfa.add_state();
            let child = build_fragment(nfa, inner);

            // Pula direto pro fim (o caso "zero").
            nfa.add_epsilon(start, accept);
            // Ou entra no miolo (o caso "um").
            nfa.add_epsilon(start, child.start);
            // Sai do miolo sem loop.
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

        // Verifica se cada regra produziu pelo menos um estado final no NFA.
        let expected = (0..spec.rules.len()).collect::<Vec<_>>();
        assert_eq!(accepted, expected);
    }
}
