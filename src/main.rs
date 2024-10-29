mod dfa;
mod nfa;
mod regex;
mod util;

fn main() {
    println!("One day I will be a Cool compiler!");

    let ast = regex::Ast::Repetition(Box::new(regex::Ast::Alternation(vec![
        regex::Ast::Concatenation(vec![regex::Ast::Character('a'), regex::Ast::Character('b')]),
        regex::Ast::Character('c'),
    ])));
    dbg!(&ast);

    let nfa = nfa::NFA::new(&ast);
    dbg!(&nfa);

    let dfa = dfa::build(&nfa);
    dbg!(&dfa);

    let re = "(ab|c)*";
    regex::tokenize(re.chars()).for_each(|t| println!("{t:?}"));
}
