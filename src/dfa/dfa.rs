use std::collections::{BTreeMap, BTreeSet};

use crate::util::StateID;

#[derive(Debug)]
pub struct DFA {
    initial_state: StateID,
    accepting_states: BTreeSet<StateID>,
    transitions: BTreeMap<(StateID, char), StateID>,
}

impl DFA {}
