use super::{Dfa, State, StateId, InputChar};

/// A cursor into a DFA
#[derive(Debug)]
pub struct DfaState<'a> {
    dfa: &'a Dfa,
    current: StateId,
}

impl<'a> DfaState<'a> {
    pub(super) fn new(dfa: &'a Dfa, current: StateId) -> Self {
        Self {dfa, current}
    }

    /// Initiate a transition from the curren state to the next state based on the given input
    ///
    /// Returns a cursor to the next state or `None` if no transition for the given input exists.
    pub fn transition(&self, input: InputChar) -> Option<Self> {
        let next_state = self.state().next_states.get(&input).copied()?;
        Some(Self::new(self.dfa, next_state))
    }

    /// Returns true if this is an accept state
    pub fn is_accepted(&self) -> bool {
        self.state().accept
    }

    fn state(&self) -> &State {
        self.dfa.state(self.current)
    }
}

/// A mutable cursor into a DFA
#[derive(Debug)]
pub struct DfaStateMut<'a> {
    dfa: &'a mut Dfa,
    current: StateId,
}

impl<'a> DfaStateMut<'a> {
    pub(super) fn new(dfa: &'a mut Dfa, current: StateId) -> Self {
        Self {dfa, current}
    }

    /// Mark this state as an accept state
    pub fn mark_accept(&mut self) {
        self.state_mut().accept = true;
    }

    fn state_mut(&mut self) -> &mut State {
        self.dfa.state_mut(self.current)
    }
}
