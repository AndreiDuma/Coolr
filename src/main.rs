mod dfa;
mod nfa;
mod regex;

fn main() {
    println!("One day I will be a Cool compiler!");

    let expr = regex::Expr::Repetition(Box::new(regex::Expr::Alternation(vec![
        regex::Expr::Concatenation(vec![
            regex::Expr::Character('a'),
            regex::Expr::Character('b'),
        ]),
        regex::Expr::Character('c'),
    ])));
    dbg!(&expr);

    let nfa = nfa::build(expr);
    dbg!(&nfa);

    let dfa = dfa::build(&nfa);
    dbg!(&dfa);

    let re = "(ab|c)*";
    regex::tokenize(re.chars()).for_each(|t| println!("{t:?}"));
}
