use crate::nfa::NFA;
use crate::regex::Expr;

pub fn build(expr: Expr) -> NFA {
    match expr {
        Expr::Empty => NFA::empty(),
        Expr::Character(c) => NFA::character(c),
        // Ast::Bracket(_) => todo!(),
        Expr::Concatenation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::concatenation)
            .expect("a concatenation expression must not be empty"),
        Expr::Alternation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::union)
            .expect("an alternation expression must not be empty"),
        Expr::Repetition(expr) => NFA::kleene_star(build(*expr)),
    }
}
