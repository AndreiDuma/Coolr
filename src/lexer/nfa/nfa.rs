use crate::lexer::regex;
use crate::lexer::util::{StateID, StateSet};

use crate::lexer::nfa::build::Builder;
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug)]
pub(super) enum State {
    Empty(StateID),
    Alternation(Vec<StateID>),
    Character(StateID, char),
    Match(PatternID),
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct PatternID(usize);

#[derive(Debug)]
pub struct NFA {
    states: Vec<State>,
    start_state: StateID,
    alphabet: BTreeSet<char>,
}

impl NFA {
    pub fn new(ast: &regex::Ast, pattern: PatternID) -> Self {
        let mut builder = Builder::new();
        builder.add_ast(ast, pattern);
        builder.build()
    }

    pub(super) fn from_parts(
        states: Vec<State>,
        start_state: StateID,
        alphabet: BTreeSet<char>,
    ) -> Self {
        Self {
            states,
            start_state,
            alphabet,
        }
    }

    pub fn alphabet(&self) -> impl Iterator<Item = char> + '_ {
        self.alphabet.iter().copied()
    }

    fn on_epsilon(&self, state: StateID) -> Vec<StateID> {
        match self.states[state] {
            State::Empty(s) => vec![s],
            State::Alternation(ref alts) => alts.clone(),
            _ => vec![],
        }
    }

    fn on_character(&self, state: StateID, chr: char) -> Option<StateID> {
        match self.states[state] {
            State::Character(s, c) if c == chr => Some(s),
            _ => None,
        }
    }

    fn follow_epsilon(&self, start: StateSet) -> StateSet {
        let mut next = StateSet::new();

        let mut queue = VecDeque::new();
        for state in start.iter() {
            queue.push_back(state)
        }

        while let Some(current) = queue.pop_front() {
            if next.contains(current) {
                continue;
            }
            next.insert(current);

            for next in self.on_epsilon(current) {
                queue.push_back(next);
            }
        }
        next
    }

    fn follow_character(&self, start: StateSet, chr: char) -> StateSet {
        let mut next = StateSet::new();

        for s in start.iter() {
            if let Some(state) = self.on_character(s, chr) {
                next.insert(state);
            }
        }
        next
    }
}

pub trait Automaton<T> {
    fn start(&self) -> T;

    fn next(&self, current: T, chr: char) -> T;

    fn execute(&self, haystack: &str) -> T {
        haystack
            .chars()
            .fold(self.start(), |current, chr| self.next(current, chr))
    }
}

impl Automaton<StateSet> for NFA {
    fn start(&self) -> StateSet {
        self.follow_epsilon(StateSet::with_state(self.start_state))
    }

    fn next(&self, current: StateSet, chr: char) -> StateSet {
        self.follow_epsilon(self.follow_character(current, chr))
    }
}
