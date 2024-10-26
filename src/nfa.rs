use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State(usize);

enum Transition {
    Empty,
    Character(char),
}

pub struct ThompsonNFA {
    // alphabet: Alphabet,
    states: BTreeSet<State>,
    initial_state: State,
    accepting_states: BTreeSet<State>,
    transitions: BTreeMap<(State, char), BTreeSet<State>>,
}

// fn subset_construction(nfa: &NFA) -> DFA {
//     todo!()
// }
