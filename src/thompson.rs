#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State(usize);

impl State {
    fn shifted(&self, amount: usize) -> Self {
        State(self.0 + amount)
    }
}

enum Transitions {
    None,
    OneCharacter(char, State),
    OneEpsilon(State),
    TwoEpsilon(State, State),
}

impl Transitions {
    fn shifted(&self, amount: usize) -> Self {
        match self {
            Transitions::None => Transitions::None,
            Transitions::OneCharacter(c, s) => Transitions::OneCharacter(*c, s.shifted(amount)),
            Transitions::OneEpsilon(s) => Transitions::OneEpsilon(s.shifted(amount)),
            Transitions::TwoEpsilon(s, t) => {
                Transitions::TwoEpsilon(s.shifted(amount), t.shifted(amount))
            }
        }
    }
}

pub struct NFA {
    transitions: Vec<Transitions>,
    initial_state: State,
    accepting_state: State,
}

// fn subset_construction(nfa: &NFA) -> DFA {
//     todo!()
// }

impl NFA {
    pub fn empty() -> Self {
        Self {
            initial_state: State(0),
            accepting_state: State(1),
            transitions: vec![Transitions::OneEpsilon(State(1)), Transitions::None],
        }
    }

    pub fn character(c: char) -> Self {
        Self {
            initial_state: State(0),
            accepting_state: State(1),
            transitions: vec![Transitions::OneCharacter(c, State(1)), Transitions::None],
        }
    }

    pub fn concatenation(self, other: Self) -> Self {
        let other = other.shifted(self.size() - 1);
        Self {
            accepting_state: other.accepting_state,
            transitions: {
                let mut transitions = self.transitions;
                transitions.pop();
                transitions.extend(other.transitions);
                transitions
            },
            ..self
        }
    }

    pub fn union(self, other: Self) -> Self {
        let this = self.shifted(1);
        let other = other.shifted(this.size() + 1);

        Self {
            initial_state: State(0),
            accepting_state: State(this.size() + other.size() + 1),
            transitions: {
                let mut transitions = vec![Transitions::TwoEpsilon(
                    this.initial_state,
                    other.initial_state,
                )];
                transitions.extend(this.transitions);
                transitions.extend(other.transitions);
                transitions.push(Transitions::None);
                transitions
            },
        }
    }

    pub fn kleene_star(self) -> Self {
        let this = self.shifted(1);
        let accepting_state = State(this.size() + 1);
        Self {
            initial_state: State(0),
            accepting_state,
            transitions: {
                let mut transitions =
                    vec![Transitions::TwoEpsilon(this.initial_state, accepting_state)];
                transitions.extend(this.transitions);
                transitions.push(Transitions::None);
                transitions
            },
        }
    }

    fn size(&self) -> usize {
        self.transitions.len()
    }

    fn shifted(self, amount: usize) -> Self {
        Self {
            initial_state: self.initial_state.shifted(amount),
            accepting_state: self.accepting_state.shifted(amount),
            transitions: self
                .transitions
                .into_iter()
                .map(|t| t.shifted(amount))
                .collect(),
        }
    }
}
