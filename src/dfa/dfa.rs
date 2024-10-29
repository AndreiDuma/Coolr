use std::collections::{BTreeMap, BTreeSet};

use crate::nfa::StateID;

#[derive(Debug)]
pub struct DFA {
    initial_state: StateID,
    accepting_states: BTreeSet<StateID>,
    transitions: BTreeMap<(StateID, char), StateID>,
}

impl DFA {}

// impl IntoIterator for Configuration {
//     type Item = State;

//     type IntoIter = btree_set::IntoIter<State>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.states.into_iter()
//     }
// }
