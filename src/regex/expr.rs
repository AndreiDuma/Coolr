use crate::nfa::NFA;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    Empty,
    Character(char),
    // Bracket(Vec<Range<char>>),
    Concatenation(Vec<Expr>),
    Alternation(Vec<Expr>),
    Repetition(Box<Expr>),
}
