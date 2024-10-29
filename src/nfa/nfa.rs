use std::collections::{BTreeSet, VecDeque};

/// TODO: move this to a shared module since it's also used in the
/// DFA.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateID(usize);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Configuration {
    states: BTreeSet<StateID>,
}

impl Configuration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_state(state: StateID) -> Self {
        let mut config = Self::new();
        config.states.insert(state);
        config
    }

    pub fn with_states(states: BTreeSet<StateID>) -> Configuration {
        Configuration { states }
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = StateID> + '_ {
        self.states.iter().copied()
    }
}

#[derive(Copy, Clone, Debug)]
enum StateTransitions {
    None,
    OneCharacter(char, StateID),
    OneEpsilon(StateID),
    TwoEpsilon(StateID, StateID),
}

// pub enum Transition {
//     Character(char, State),
//     Epsilon(State),
// }

#[derive(Clone, Debug)]
pub struct NFA {
    alphabet: BTreeSet<char>,
    initial_state: StateID,
    accepting_state: StateID,
    transitions: Vec<StateTransitions>,
}

impl NFA {
    pub fn empty() -> Self {
        Self {
            alphabet: BTreeSet::new(),
            initial_state: StateID(0),
            accepting_state: StateID(1),
            transitions: vec![
                StateTransitions::OneEpsilon(StateID(1)),
                StateTransitions::None,
            ],
        }
    }

    pub fn character(c: char) -> Self {
        Self {
            alphabet: {
                let mut alphabet = BTreeSet::new();
                alphabet.insert(c);
                alphabet
            },
            initial_state: StateID(0),
            accepting_state: StateID(1),
            transitions: vec![
                StateTransitions::OneCharacter(c, StateID(1)),
                StateTransitions::None,
            ],
        }
    }

    pub fn concatenation(first: NFA, second: Self) -> Self {
        let initial_state = StateID(0);
        let accepting_state = StateID(first.size() + second.size() - 2);

        // The accepting state of the first NFA and the initial state
        // of the second NFA need to be merged. Therefore we drop the
        // last state of the first NFA before appending the states of
        // the second one.
        let mut transitions = first.transitions;
        transitions.pop();
        transitions.extend(shift_transitions(second.transitions, transitions.len()));

        Self {
            alphabet: alphabet_union(first.alphabet, second.alphabet),
            initial_state,
            accepting_state,
            transitions,
        }
    }

    pub fn union(first: NFA, second: Self) -> Self {
        let first_size = first.size();

        let initial_state = StateID(0);
        let accepting_state = StateID(first_size + second.size() + 1);

        // Create new initial state.
        let mut transitions = vec![StateTransitions::TwoEpsilon(
            StateID(1),
            StateID(1 + first_size),
        )];

        // Embed first NFA and patch its accepting state.
        transitions.extend({
            let shifted = shift_transitions(first.transitions, 1);
            patch_accepting_state(shifted, StateTransitions::OneEpsilon(accepting_state))
        });

        // Embed second NFA and patch its accepting state.
        transitions.extend({
            let shifted = shift_transitions(second.transitions, 1 + first_size);
            patch_accepting_state(shifted, StateTransitions::OneEpsilon(accepting_state))
        });

        // Create new accepting state.
        transitions.push(StateTransitions::None);

        Self {
            alphabet: alphabet_union(first.alphabet, second.alphabet),
            initial_state,
            accepting_state,
            transitions,
        }
    }

    pub fn kleene_star(nfa: NFA) -> Self {
        let initial_state = StateID(0);
        let accepting_state = StateID(nfa.size() + 1);

        // Create new initial state.
        let mut transitions = vec![StateTransitions::TwoEpsilon(StateID(1), accepting_state)];

        // Embed given NFA and patch its accepting state.
        transitions.extend({
            let shifted = shift_transitions(nfa.transitions, 1);
            patch_accepting_state(
                shifted,
                StateTransitions::TwoEpsilon(StateID(1), accepting_state),
            )
        });

        // Create new accepting state.
        transitions.push(StateTransitions::None);

        Self {
            alphabet: nfa.alphabet,
            initial_state,
            accepting_state,
            transitions,
        }
    }

    pub fn alphabet(&self) -> impl Iterator<Item = char> + '_ {
        self.alphabet.iter().copied()
    }

    pub fn initial_state(&self) -> StateID {
        StateID(0)
    }

    pub fn accepting_state(&self) -> StateID {
        StateID(self.transitions.len() - 1)
    }

    pub fn is_accepting_state(&self, state: StateID) -> bool {
        state == self.accepting_state()
    }

    pub fn size(&self) -> usize {
        self.transitions.len()
    }

    pub fn on_epsilon(&self, state: StateID) -> Vec<StateID> {
        match self.transitions[state.0] {
            StateTransitions::OneEpsilon(s) => vec![s],
            StateTransitions::TwoEpsilon(s, t) => vec![s, t],
            _ => vec![],
        }
    }

    pub fn on_character(&self, state: StateID, chr: char) -> Option<StateID> {
        match self.transitions[state.0] {
            StateTransitions::OneCharacter(c, s) if c == chr => Some(s),
            _ => None,
        }
    }

    // pub fn on_character(&self, state: State) -> Option<(char, State)> {
    //     match self.transitions[state.0] {
    //         StateTransitions::OneCharacter(c, s) => Some((c, s)),
    //         _ => None,
    //     }
    // }

    /// TODO: convert to Configuration -> Configuration
    pub fn follow_epsilon(&self, config: &Configuration) -> Configuration {
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

    pub fn follow_character(&self, config: &Configuration, chr: char) -> Configuration {
        let mut states = BTreeSet::new();

        for s in config.iter() {
            if let Some(state) = self.on_character(s, chr) {
                states.insert(state);
            }
        }
        Configuration::with_states(states)
    }
}

fn shift_transitions(
    mut transitions: Vec<StateTransitions>,
    amount: usize,
) -> Vec<StateTransitions> {
    let shift_state = |s: StateID| StateID(s.0 + amount);
    let shift_transitions = |t| match t {
        StateTransitions::None => StateTransitions::None,
        StateTransitions::OneCharacter(c, s) => StateTransitions::OneCharacter(c, shift_state(s)),
        StateTransitions::OneEpsilon(s) => StateTransitions::OneEpsilon(shift_state(s)),
        StateTransitions::TwoEpsilon(s, t) => {
            StateTransitions::TwoEpsilon(shift_state(s), shift_state(t))
        }
    };

    transitions
        .iter_mut()
        .for_each(|t| *t = shift_transitions(*t));
    transitions
}

fn patch_accepting_state(
    mut transitions: Vec<StateTransitions>,
    new: StateTransitions,
) -> Vec<StateTransitions> {
    *transitions
        .last_mut()
        .expect("should have at least two states") = new;
    transitions
}

fn alphabet_union(mut first: BTreeSet<char>, mut second: BTreeSet<char>) -> BTreeSet<char> {
    first.append(&mut second);
    first
}
