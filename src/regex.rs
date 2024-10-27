use std::{collections::BTreeSet, ops::Range};

use crate::thompson::NFA;

#[derive(Clone, PartialEq, Eq)]
struct Alphabet {
    characters: BTreeSet<char>,
    ranges: BTreeSet<Range<char>>, // TODO: should this be a Vec?
}

#[derive(Clone, PartialEq, Eq)]
enum Ast {
    Empty,
    Literal(char),
    // Bracket(Vec<Range<char>>),
    Concatenation(Vec<Ast>),
    Alternation(Vec<Ast>),
    Repetition(Box<Ast>),
}

struct Regex {
    pattern: String,
    ast: Ast,
}

fn thompsons_construction(regex: Regex) -> NFA {
    match regex.ast {
        Ast::Empty => NFA::empty(),
        Ast::Literal(c) => NFA::character(c),
        // Ast::Bracket(_) => todo!(),
        Ast::Concatenation(_) => todo!(),
        Ast::Alternation(_) => todo!(),
        Ast::Repetition(_) => todo!(),
    }
}
