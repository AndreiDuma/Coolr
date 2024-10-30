# Coolr – A Cool Compiler Written in Rust – WIP

**This is a work in progress!** Follow this repository over the next
couple of months to see a Cool compiler come to life.

## Plan

I'm planning to implement, "from scratch", the basic components of a
classical compiler. The rough plan is as follows:

- [ ] `cmd`: CLI program to run the compiler an a set of Cool files;
- [ ] `lexer`: turn a stream of characters into a stream of tokens:
  - [x] `regex::Ast`: structured representation for regular
        expressions;
  - [ ] `regex::parser`: parse regex strings into `regex::Ast`s;
  - [ ] `nfa::NFA`: represent Thompson NFAs;
  - [ ] `nfa::build`: turn a list of `regex::Ast`s into an `nfa::NFA`
        via [Thompson's construction](https://en.wikipedia.org/wiki/Thompson%27s_construction);
  - [ ] `nfa::search`: execute an NFA on a given string haystack;
  - [ ] `dfa::DFA`: represent a deterministic automata;
  - [ ] `dfa::build`: construct a `dfa::DFA` from a Thompson
        `nfa::NFA` using the [powerset construction](https://en.wikipedia.org/wiki/Powerset_construction);
  - [ ] `dfa::minimize`: perform DFA minimization on a `dfa::DFA`
        using [Hopcroft's algorithm](https://en.wikipedia.org/wiki/DFA_minimization#Hopcroft's_algorithm);
  - [ ] `dfa::search`: execute a DFA on a given string haystack;
  - [ ] `gen::Config`: define a language's lexical categories using
        regular expressions;
  - [ ] `gen::Lexer`: iterates over a character stream and produces
        tokens;
  - [ ] `gen::generate`: generates a `gen::Lexer` from a
        `lex::Config`;
- [ ] `parser`: turn a stream of tokens into an `ast::Ast`;
- [ ] `ast::Ast`: abstract syntax tree type;
- [ ] `interpreter`: directly interpret an AST;
- [ ] ... to be added later.

## Useful references

- [The Cool Reference Manual](https://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf)
- [The Cool Runtime System](https://web.stanford.edu/class/cs143/materials/cool-runtime.pdf)
