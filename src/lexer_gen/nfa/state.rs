use crate::lexer_gen::regex::CharClass;

/// NFA completo usado como entrada da construcao de subconjuntos (DFA).
#[derive(Debug, Clone)]
pub struct Nfa {
    /// Estado inicial global.
    // Todo NFA tem exatamente um estado inicial. A execução do autômato começa aqui.
    pub start_state: usize,
    
    /// Todos os estados do automato.
    // Armazenamos os estados em um `Vec`. O índice no `Vec` funciona como a "identidade" (ID) do estado.
    // Isso evita problemas complexos de propriedade/lifetimes (como usar ponteiros ou Rc<RefCell<>>),
    // tornando a criação e iteração do grafo extremamente rápida e cache-friendly.
    pub states: Vec<NfaState>,
}

impl Nfa {
    /// Cria NFA vazio (sem estados).
    pub fn new() -> Self {
        Self {
            start_state: 0,
            states: Vec::new(),
        }
    }

    /// Adiciona um novo estado ao NFA e retorna seu indice (seu ID).
    pub fn add_state(&mut self) -> usize {
        let idx = self.states.len(); // O ID é a posição em que será inserido.
        self.states.push(NfaState::default()); // Cria um estado vazio (sem transições).
        idx
    }

    /// Adiciona transicao epsilon (ε) entre dois estados.
    // Epsilon-transições permitem pular de `from` para `to` SEM consumir caracteres da string.
    pub fn add_epsilon(&mut self, from: usize, to: usize) {
        self.states[from].epsilon.push(to);
    }

    /// Adiciona transicao consumindo simbolo entre dois estados.
    // Transições por símbolo OBRIGAM o consumo de exatamente 1 caractere da string.
    pub fn add_symbol_transition(&mut self, from: usize, to: usize, symbol: TransitionSymbol) {
        self.states[from]
            .transitions
            .push(SymbolTransition { symbol, to });
    }
}

impl Default for Nfa {
    fn default() -> Self {
        Self::new()
    }
}

/// Estado do NFA.
#[derive(Debug, Clone, Default)]
// `Default` facilita a criação de estados vazios lá no `add_state()`.
pub struct NfaState {
    /// Fechos epsilon.
    // Lista de estados de destino alcançáveis SEM consumir entrada.
    // Uma das diferenças fundamentais entre NFA e DFA é que no NFA um estado
    // pode ter múltiplas transições epsilon saindo dele.
    pub epsilon: Vec<usize>,
    
    /// Transicoes com consumo de simbolo.
    // Em um NFA, pode haver múltiplas transições para o *mesmo* símbolo.
    pub transitions: Vec<SymbolTransition>,
    
    /// Regra aceita por este estado (indice em `UnifiedRegexSpec.rules`).
    // Se não for `None`, este estado é um ESTADO FINAL (de aceitação).
    // O `usize` indica QUAL regra foi casada. Se batermos aqui, significa
    // que reconhecemos um token (por ex, o token na posição 5 das regras).
    pub accept_rule_index: Option<usize>,
}

/// Transicao com consumo de um simbolo (ou classe de simbolos).
#[derive(Debug, Clone)]
pub struct SymbolTransition {
    // O que precisa ser lido no input para pegar esta transição.
    pub symbol: TransitionSymbol,
    // O ID do estado de destino.
    pub to: usize,
}

/// Alfabeto abstrato usado no NFA.
// Não usamos apenas `char` porque nossas regras têm [a-z], `.`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransitionSymbol {
    /// Um caractere literal (ex: 'a').
    Literal(char),
    /// Coringa `.` (qualquer char exceto \n).
    AnyChar,
    /// Classe de caracteres `[ ... ]` (ex: [0-9]).
    CharClass(CharClass),
}

impl TransitionSymbol {
    /// Verifica se um caractere real (do código-fonte) e' aceito por este simbolo da transição.
    pub fn matches(&self, ch: char) -> bool {
        match self {
            TransitionSymbol::Literal(c) => *c == ch,
            TransitionSymbol::AnyChar => true,
            TransitionSymbol::CharClass(class) => class.matches(ch),
        }
    }

    /// Retorna um caractere representativo deste simbolo.
    // Isso é um truque necessário depois, na Minimização de Hopcroft ou Construção de Subconjuntos,
    // quando precisamos particionar/agrupar estados, precisamos de um caractere concreto que
    // ative a transição, para não precisarmos testar todos os 1 milhão de unicodes.
    pub fn representative(&self) -> Option<char> {
        match self {
            TransitionSymbol::Literal(ch) => Some(*ch),
            // '\x01' é um caractere genérico que usamos para testar o AnyChar.
            TransitionSymbol::AnyChar => Some('\x01'),
            // Pega o primeiro char ascii (0 a 127) que dá match na classe.
            TransitionSymbol::CharClass(class) => (0u32..=127)
                .filter_map(char::from_u32)
                .find(|&ch| class.matches(ch)),
        }
    }
}
