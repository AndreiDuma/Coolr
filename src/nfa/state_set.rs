use std::collections::BTreeSet;

use crate::nfa::StateID;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StateSet {
    states: BTreeSet<StateID>,
}

impl StateSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(state: StateID) -> Self {
        let mut config = Self::new();
        config.states.insert(state);
        config
    }

    pub fn with_states(states: BTreeSet<StateID>) -> StateSet {
        StateSet { states }
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = StateID> + '_ {
        self.states.iter().copied()
    }
}
