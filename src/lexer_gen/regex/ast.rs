#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum Regex {
    EmptySet,

    Epsilon,

    Literal(char),

    AnyChar,

    CharClass(CharClass),

    Concat(Vec<Regex>),

    Alternation(Vec<Regex>),

    Star(Box<Regex>),

    Plus(Box<Regex>),

    Optional(Box<Regex>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharClass {
    pub negated: bool,

    pub items: Vec<CharClassItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharClassItem {
    Char(char),

    Range(char, char),
}

impl CharClass {
    pub fn matches(&self, ch: char) -> bool {
        let hit = self.items.iter().any(|item| match item {
            CharClassItem::Char(c) => *c == ch,

            CharClassItem::Range(lo, hi) => *lo <= ch && ch <= *hi,
        });
        if self.negated { !hit } else { hit }
    }
}
