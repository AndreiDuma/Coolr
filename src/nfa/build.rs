use crate::nfa::NFA;
use crate::regex;

pub fn build(expr: regex::Ast) -> NFA {
    match expr {
        regex::Ast::Empty => NFA::empty(),
        regex::Ast::Character(c) => NFA::character(c),
        regex::Ast::Concatenation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::concatenation)
            .expect("a concatenation expression must not be empty"),
        regex::Ast::Alternation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::union)
            .expect("an alternation expression must not be empty"),
        regex::Ast::Repetition(expr) => NFA::kleene_star(build(*expr)),
    }
}
