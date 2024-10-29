use std::collections::{btree_set, BTreeMap, BTreeSet, HashMap, VecDeque};

use crate::thompson::{State, NFA};

struct DFA {
    initial_state: State,
    accepting_states: BTreeSet<State>,
    transitions: BTreeMap<(State, char), State>,
}

impl DFA {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Configuration {
    states: BTreeSet<State>,
}

impl Configuration {
    fn new() -> Self {
        Self::default()
    }

    fn with_state(state: State) -> Self {
        let mut config = Self::new();
        config.states.insert(state);
        config
    }

    fn with_states(states: BTreeSet<State>) -> Configuration {
        Configuration { states }
    }

    fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = State> + '_ {
        self.states.iter().copied()
    }
}

// impl IntoIterator for Configuration {
//     type Item = State;

//     type IntoIter = btree_set::IntoIter<State>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.states.into_iter()
//     }
// }

/// TODO: convert to Configuration -> Configuration
impl NFA {
    fn follow_epsilon(&self, config: &Configuration) -> Configuration {
        let mut states = BTreeSet::new();

        let mut queue = VecDeque::new();
        for state in config.iter() {
            queue.push_back(state)
        }

        while let Some(current) = queue.pop_front() {
            if states.contains(&current) {
                continue;
            }
            states.insert(current);

            for next in self.on_epsilon(current) {
                queue.push_back(next);
            }
        }
        Configuration { states }
    }

    fn follow_character(&self, config: &Configuration, chr: char) -> Configuration {
        let mut states = BTreeSet::new();

        for s in config.iter() {
            if let Some(state) = self.on_character(s, chr) {
                states.insert(state);
            }
        }
        Configuration::with_states(states)
    }
}

pub fn subset_construction(nfa: &NFA) -> DFA {
    let initial_config = nfa.follow_epsilon(&Configuration::with_state(nfa.initial_state()));

    let mut configs = BTreeSet::new();
    configs.insert(initial_config.clone()); // TODO: can we not clone here?

    let mut queue = VecDeque::new();
    queue.push_back(initial_config);

    let mut transitions: HashMap<(Configuration, char), Configuration> = HashMap::new();

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
