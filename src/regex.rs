use std::{collections::BTreeSet, ops::Range};

use crate::nfa::ThompsonNFA;

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

fn thompsons_construction(regex: Regex) -> ThompsonNFA {
    match regex.ast {
        Ast::Empty => todo!(),
        Ast::Literal(_) => todo!(),
        // Ast::Bracket(_) => todo!(),
        Ast::Concatenation(_) => todo!(),
        Ast::Alternation(_) => todo!(),
        Ast::Repetition(_) => todo!(),
    }
}

fn f() {
    let re = Ast::Concatenation(vec![Ast::Literal('i'), Ast::Literal('f')]);
}