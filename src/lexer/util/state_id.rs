use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateID(usize);

impl StateID {
    pub fn new(value: usize) -> Self {
        StateID(value)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl<T> Index<StateID> for Vec<T> {
    type Output = T;

    fn index(&self, index: StateID) -> &Self::Output {
        &self[index.0]
    }
}

impl<T> IndexMut<StateID> for Vec<T> {
    fn index_mut(&mut self, index: StateID) -> &mut Self::Output {
        &mut self[index.0]
    }
}
