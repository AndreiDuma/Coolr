use std::collections::BTreeSet;

use crate::lexer::nfa::{PatternID, State, NFA};
use crate::lexer::regex::{self, Ast};
use crate::lexer::util::StateID;

#[derive(Clone, Copy)]
struct StartEndIDs {
    start: StateID,
    end: StateID,
}

#[derive(Clone, Default)]
pub(super) struct Builder {
    pattern: PatternID,
    states: Vec<State>,
    start_states: Vec<StateID>,
    alphabet: BTreeSet<char>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_ast(&mut self, ast: &regex::Ast, pattern: PatternID) {
        // Save the pattern to be used later, when creating empty and
        // character states.
        self.pattern = pattern;

        // Traverse the AST in postorder, joining sub-NFAs into
        // "composite" NFAs until the root is reached. The start state
        // of the resulting NFA is recorded.
        let sub = ast
            .iter()
            .fold(Vec::new(), |mut stack, ast| {
                let sub = match ast {
                    Ast::Empty => self.add_empty(),
                    Ast::Character(chr) => self.add_character(*chr),
                    Ast::Concatenation(children) => {
                        self.add_concatenation(stack.iter().rev().copied().take(children.len()))
                    }
                    Ast::Alternation(children) => {
                        self.add_alternation(stack.iter().rev().copied().take(children.len()))
                    }
                    Ast::Repetition(_) => self.add_repetition(stack.pop().unwrap()),
                };
                stack.push(sub);
                stack
            })
            .pop()
            .unwrap();
        self.start_states.push(sub.start);

        // Another pass through the AST updates the NFA's alphabet.
        ast.iter().for_each(|ast| {
            if let Ast::Character(chr) = ast {
                self.alphabet.insert(*chr);
            }
        });
    }

    pub fn build(mut self) -> NFA {
        let start = self.add_state(State::Alternation(self.start_states.clone()));

        NFA::from_parts(self.states, start, self.alphabet)
    }

    fn add_empty(&mut self) -> StartEndIDs {
        let end = self.add_match_state();
        let start = self.add_state(State::Empty(end));
        StartEndIDs { start, end }
    }

    fn add_character(&mut self, chr: char) -> StartEndIDs {
        let end = self.add_match_state();
        let start = self.add_state(State::Character(end, chr));
        StartEndIDs { start, end }
    }

    fn add_concatenation<I>(&mut self, it: I) -> StartEndIDs
    where
        I: DoubleEndedIterator<Item = StartEndIDs>,
    {
        let mut it_rev = it.rev();
        let last = it_rev
            .next()
            .expect("concatenations cannot have zero sub-expressions");

        let end = last.end;
        let start = it_rev.fold(last.start, |next, sub| {
            self.patch(sub.end, State::Empty(next));
            sub.start
        });
        StartEndIDs { start, end }
    }

    fn add_alternation<I>(&mut self, it: I) -> StartEndIDs
    where
        I: Iterator<Item = StartEndIDs>,
    {
        let end = self.add_match_state();
        let alternatives = it
            .map(|sub| {
                self.patch(sub.end, State::Empty(end));
                sub.start
            })
            .collect();
        let start = self.add_state(State::Alternation(alternatives));
        StartEndIDs { start, end }
    }

    fn add_repetition(&mut self, sub: StartEndIDs) -> StartEndIDs {
        let end = self.add_match_state();
        let start = self.add_state(State::Alternation(vec![sub.start, end]));
        self.patch(sub.end, State::Alternation(vec![sub.start, end]));
        StartEndIDs { start, end }
    }

    fn add_match_state(&mut self) -> StateID {
        self.add_state(State::Match(self.pattern))
    }

    fn add_state(&mut self, state: State) -> StateID {
        let new = StateID::new(self.states.len());
        self.states.push(state);
        new
    }

    fn patch(&mut self, id: StateID, state: State) {
        self.states[id] = state;
    }
}