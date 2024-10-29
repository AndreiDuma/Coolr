/// TODO: move this to a shared module since it's also used in the
/// DFA.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateID(pub usize);
