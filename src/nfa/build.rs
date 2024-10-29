use crate::nfa::NFA;
use crate::regex::{self, Ast};
use crate::util::StateID;

pub fn build(expr: Ast) -> NFA {
    match expr {
        Ast::Empty => NFA::empty(),
        Ast::Character(c) => NFA::character(c),
        Ast::Concatenation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::concatenation)
            .expect("a concatenation expression must not be empty"),
        Ast::Alternation(exprs) => exprs
            .into_iter()
            .map(build)
            .reduce(NFA::union)
            .expect("an alternation expression must not be empty"),
        Ast::Repetition(expr) => NFA::kleene_star(build(*expr)),
    }
}

////

#[derive(Clone)]
enum State {
    Empty(StateID),
    Union(Vec<StateID>),
    Character(StateID, char),
    Match,
}

#[derive(Clone, Copy)]
struct StatePair {
    start: StateID,
    end: StateID,
}

#[derive(Clone, Default)]
struct Builder {
    states: Vec<State>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn build(ast: &regex::Ast) -> NFA {
    //     let mut builder = Self::new();
    //     let sub = builder.add_ast(ast);

    // 	NFA::from_parts(builder.states)

    //     todo!()
    // }

    fn add_ast(&mut self, ast: &regex::Ast) -> StatePair {
        ast.iter()
            .fold(Vec::new(), |mut stack, ast| {
                let sub = match ast {
                    Ast::Empty => self.add_empty(),
                    Ast::Character(chr) => self.add_character(*chr),
                    Ast::Concatenation(children) => {
                        self.add_concatenation(stack.iter().rev().copied().take(children.len()))
                    }
                    Ast::Alternation(children) => {
                        self.add_alternation(stack.iter().rev().copied().take(children.len()))
                    }
                    Ast::Repetition(_) => self.add_repetition(stack.pop().unwrap()),
                };
                stack.push(sub);
                stack
            })
            .pop()
            .unwrap()
    }

    fn add_empty(&mut self) -> StatePair {
        let end = self.add_state(State::Match);
        let start = self.add_state(State::Empty(end));
        StatePair { start, end }
    }

    fn add_character(&mut self, chr: char) -> StatePair {
        let end = self.add_state(State::Match);
        let start = self.add_state(State::Character(end, chr));
        StatePair { start, end }
    }

    fn add_concatenation<I>(&mut self, it: I) -> StatePair
    where
        I: DoubleEndedIterator<Item = StatePair>,
    {
        let end = self.add_state(State::Match);
        let start = it.rev().fold(end, |next, sub| {
            self.patch(sub.end, State::Empty(next));
            sub.start
        });
        StatePair { start, end }
    }

    fn add_alternation<I>(&mut self, it: I) -> StatePair
    where
        I: Iterator<Item = StatePair>,
    {
        let end = self.add_state(State::Match);
        let alternatives = it
            .map(|sub| {
                self.patch(sub.end, State::Empty(end));
                sub.start
            })
            .collect();
        let start = self.add_state(State::Union(alternatives));
        StatePair { start, end }
    }

    fn add_repetition(&mut self, sub: StatePair) -> StatePair {
        let end = self.add_state(State::Match);
        let start = self.add_state(State::Union(vec![sub.start, end]));
        self.patch(sub.end, State::Union(vec![sub.start, end]));
        StatePair { start, end }
    }

    fn add_state(&mut self, state: State) -> StateID {
        let new = StateID::new(self.states.len());
        self.states.push(state);
        new
    }

    fn patch(&mut self, id: StateID, state: State) {
        self.states[id] = state;
    }
}
