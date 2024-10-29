use std::collections::BTreeSet;

/// TODO: move this to a shared module since it's also used in the
/// DFA.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State(usize);

#[derive(Copy, Clone, Debug)]
enum StateTransitions {
    None,
    OneCharacter(char, State),
    OneEpsilon(State),
    TwoEpsilon(State, State),
}

// pub enum Transition {
//     Character(char, State),
//     Epsilon(State),
// }

#[derive(Clone, Debug)]
pub struct NFA {
    alphabet: BTreeSet<char>,
    initial_state: State,
    accepting_state: State,
    transitions: Vec<StateTransitions>,
}

impl NFA {
    pub fn empty() -> Self {
        Self {
            alphabet: BTreeSet::new(),
            initial_state: State(0),
            accepting_state: State(1),
            transitions: vec![
                StateTransitions::OneEpsilon(State(1)),
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
            initial_state: State(0),
            accepting_state: State(1),
            transitions: vec![
                StateTransitions::OneCharacter(c, State(1)),
                StateTransitions::None,
            ],
        }
    }

    pub fn concatenation(first: NFA, second: Self) -> Self {
        let initial_state = State(0);
        let accepting_state = State(first.size() + second.size() - 2);

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

        let initial_state = State(0);
        let accepting_state = State(first_size + second.size() + 1);

        // Create new initial state.
        let mut transitions = vec![StateTransitions::TwoEpsilon(
            State(1),
            State(1 + first_size),
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
        let initial_state = State(0);
        let accepting_state = State(nfa.size() + 1);

        // Create new initial state.
        let mut transitions = vec![StateTransitions::TwoEpsilon(State(1), accepting_state)];

        // Embed given NFA and patch its accepting state.
        transitions.extend({
            let shifted = shift_transitions(nfa.transitions, 1);
            patch_accepting_state(
                shifted,
                StateTransitions::TwoEpsilon(State(1), accepting_state),
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

    pub fn initial_state(&self) -> State {
        State(0)
    }

    pub fn accepting_state(&self) -> State {
        State(self.transitions.len() - 1)
    }

    pub fn is_accepting_state(&self, state: State) -> bool {
        state == self.accepting_state()
    }

    pub fn size(&self) -> usize {
        self.transitions.len()
    }

    pub fn on_epsilon(&self, state: State) -> Vec<State> {
        match self.transitions[state.0] {
            StateTransitions::OneEpsilon(s) => vec![s],
            StateTransitions::TwoEpsilon(s, t) => vec![s, t],
            _ => vec![],
        }
    }

    pub fn on_character(&self, state: State, chr: char) -> Option<State> {
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
}

fn shift_transitions(
    mut transitions: Vec<StateTransitions>,
    amount: usize,
) -> Vec<StateTransitions> {
    let shift_state = |s: State| State(s.0 + amount);
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
