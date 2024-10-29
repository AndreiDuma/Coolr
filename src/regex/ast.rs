#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Ast {
    Empty,
    Character(char),
    // Bracket(Vec<Range<char>>),
    Concatenation(Vec<Ast>),
    Alternation(Vec<Ast>),
    Repetition(Box<Ast>),
}
