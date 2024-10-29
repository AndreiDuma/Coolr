use crate::nfa::Builder;
use crate::regex;
use crate::util::{StateID, StateSet};

use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug)]
pub(super) enum State {
    Empty(StateID),
    Alternation(Vec<StateID>),
    Character(StateID, char),
    Match,
}

#[derive(Debug)]
pub struct NFA {
    states: Vec<State>,
    start_state: StateID,
    alphabet: BTreeSet<char>,
}

impl NFA {
    pub fn new(ast: &regex::Ast) -> Self {
        let mut builder = Builder::new();
        builder.add_ast(ast);
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

    //// TODO: rethink everything below.

    pub fn alphabet(&self) -> impl Iterator<Item = char> + '_ {
        self.alphabet.iter().copied()
    }

    pub fn start_state(&self) -> StateID {
        self.start_state
    }

    pub fn on_epsilon(&self, state: StateID) -> Vec<StateID> {
        match self.states[state] {
            State::Empty(s) => vec![s],
            State::Alternation(ref alts) => alts.clone(),
            _ => vec![],
        }
    }

    pub fn on_character(&self, state: StateID, chr: char) -> Option<StateID> {
        match self.states[state] {
            State::Character(s, c) if c == chr => Some(s),
            _ => None,
        }
    }

    pub fn follow_epsilon(&self, set: &StateSet) -> StateSet {
        let mut states = StateSet::new();

        let mut queue = VecDeque::new();
        for state in set.iter() {
            queue.push_back(state)
        }

        while let Some(current) = queue.pop_front() {
            if states.contains(current) {
                continue;
            }
            states.insert(current);

            for next in self.on_epsilon(current) {
                queue.push_back(next);
            }
        }
        states
    }

    pub fn follow_character(&self, config: &StateSet, chr: char) -> StateSet {
        let mut states = StateSet::new();

        for s in config.iter() {
            if let Some(state) = self.on_character(s, chr) {
                states.insert(state);
            }
        }
        states
    }
}
