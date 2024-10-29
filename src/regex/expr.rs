use crate::nfa::NFA;

// #[derive(Clone, PartialEq, Eq)]
// struct Alphabet {
//     characters: BTreeSet<char>,
//     ranges: BTreeSet<Range<char>>, // TODO: should this be a Vec?
// }

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Empty,
    Character(char),
    // Bracket(Vec<Range<char>>),
    Concatenation(Vec<Expr>),
    Alternation(Vec<Expr>),
    Repetition(Box<Expr>),
}

// struct Regex {
//     pattern: String,
//     ast: Expr,
// }

pub fn thompsons_construction(expr: Expr) -> NFA {
    match expr {
        Expr::Empty => NFA::empty(),
        Expr::Character(c) => NFA::character(c),
        // Ast::Bracket(_) => todo!(),
        Expr::Concatenation(exprs) => exprs
            .into_iter()
            .map(thompsons_construction)
            .reduce(NFA::concatenation)
            .expect("a concatenation expression must not be empty"),
        Expr::Alternation(exprs) => exprs
            .into_iter()
            .map(thompsons_construction)
            .reduce(NFA::union)
            .expect("an alternation expression must not be empty"),
        Expr::Repetition(expr) => NFA::kleene_star(thompsons_construction(*expr)),
    }
}
