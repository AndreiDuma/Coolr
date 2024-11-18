#![allow(dead_code)] // TODO: remove this... when possible.

mod lexer;

use lexer::nfa::{self, Automaton, PatternID};
use lexer::regex::{self};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    println!("One day I will be a Cool compiler!");

    let re = "a|b";
    let ast = regex::parse(re);
    println!("{ast:?}");

    // Build a regex AST by hand. In the future, there will be a
    // parser to build ASTs from strings.
    let ast = {
        use regex::Ast;

        Ast::repetition(&Ast::alternation(&[
            Ast::concatenation(&[Ast::character('a')?, Ast::character('b')?])?,
            Ast::character('c')?,
        ])?)?
    };
    println!("AST: {:?}", &ast);

    // Traverse AST.
    println!("Traversal:");
    for a in ast.iter() {
        println!("- {a:?}");
    }

    // Build an NFA from the AST using Thompson's construction.
    let nfa = nfa::NFA::new(&ast, PatternID::new(0));
    println!("NFA: {:?}", &nfa);

    // Execute the NFA on an input string.
    let input = "abcab";
    println!("Execute NFA on {input:?}: {:?}", nfa.execute(input));

    // Build DFA from NFA using the powerset construction.
    // let dfa = dfa::build(&nfa);
    // dbg!(&dfa);

    Ok(())
}
