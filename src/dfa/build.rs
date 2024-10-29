use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::dfa::DFA;
use crate::nfa::NFA;
use crate::util::StateSet;

pub fn build(nfa: &NFA) -> DFA {
    let initial_config = nfa.follow_epsilon(&StateSet::with_state(nfa.start_state()));

    let mut configs = BTreeSet::new();
    configs.insert(initial_config.clone()); // TODO: can we not clone here?

    let mut queue = VecDeque::new();
    queue.push_back(initial_config);

    let mut transitions: HashMap<(StateSet, char), StateSet> = HashMap::new();

    while let Some(config) = queue.pop_front() {
        for chr in nfa.alphabet() {
            let temp = nfa.follow_character(&config, chr);
            let temp = nfa.follow_epsilon(&temp);
            if !temp.is_empty() {
                if configs.insert(temp.clone()) {
                    queue.push_back(temp.clone());
                }
                transitions.insert((config.clone(), chr), temp);
            }
        }
    }

    todo!()
}
