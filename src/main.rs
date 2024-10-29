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

    let nfa = regex::thompsons_construction(expr);
    dbg!(&nfa);

    let re = "(ab|c)*";
    regex::tokenize(re.chars()).for_each(|t| println!("{t:?}"));
}
