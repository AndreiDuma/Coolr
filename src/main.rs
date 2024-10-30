use lexer::{
    nfa::{self, Automaton, PatternID},
    regex,
};

mod lexer;

fn main() {
    println!("One day I will be a Cool compiler!");

    let ast = regex::Ast::Repetition(Box::new(regex::Ast::Alternation(vec![
        regex::Ast::Concatenation(vec![regex::Ast::Character('a'), regex::Ast::Character('b')]),
        regex::Ast::Character('c'),
    ])));
    dbg!(&ast);

    for c in ast.iter() {
        println!("{c:?}");
    }

    let nfa = nfa::NFA::new(&ast, PatternID::new(0));
    dbg!(&nfa);

    dbg!(nfa.execute("ab"));

    // let dfa = dfa::build(&nfa);
    // dbg!(&dfa);

    // let re = "(ab|c)*";
    // regex::tokenize(re.chars()).for_each(|t| println!("{t:?}"));
}
