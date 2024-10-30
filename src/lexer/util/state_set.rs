use std::collections::BTreeSet;

use crate::lexer::util::StateID;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StateSet(BTreeSet<StateID>);

impl StateSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(state: StateID) -> Self {
        let mut config = Self::new();
        config.0.insert(state);
        config
    }

    pub fn insert(&mut self, state: StateID) -> bool {
        self.0.insert(state)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, state: StateID) -> bool {
        self.0.contains(&state)
    }

    pub fn iter(&self) -> StateSetIter<'_> {
        StateSetIter(self.0.iter())
    }
}

/// An iterator over all states in a StatesSet.
///
/// The lifetime `'a` refers to the lifetime of the set being iterator
/// over.
pub struct StateSetIter<'a>(std::collections::btree_set::Iter<'a, StateID>);

impl<'a> Iterator for StateSetIter<'a> {
    type Item = StateID;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }
}
