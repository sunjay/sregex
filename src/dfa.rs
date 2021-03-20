mod cursor;

use std::convert::TryInto;

use fxhash::FxHashMap as HashMap;

pub use cursor::*;

type InputChar = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StateId(u16);

#[derive(Debug, Default, Clone)]
struct State {
    /// if true, a string will be accepted if it ends at this state
    accept: bool,
    /// The next states that can be visted from this state and the input that leads to them
    next_states: HashMap<InputChar, StateId>,
}

/// Deterministic finite automata
#[derive(Debug, Clone)]
pub struct Dfa {
    /// The list of states (indexed by ID), must be non-empty
    ///
    /// The first state is always the start state
    states: Vec<State>,
}

impl Default for Dfa {
    fn default() -> Self {
        Self {
            states: vec![State::default()],
        }
    }
}

impl Dfa {
    /// Returns a cursor to the start state of the DFA
    pub fn start(&self) -> DfaState {
        assert!(!self.states.is_empty());
        DfaState::new(self, self.start_id())
    }

    /// Returns a mutable cursor to the start state of the DFA
    pub fn start_mut(&mut self) -> DfaStateMut {
        assert!(!self.states.is_empty());
        DfaStateMut::new(self, self.start_id())
    }

    /// Returns the ID of the start state
    fn start_id(&self) -> StateId {
        // Start state is always the first state
        StateId(0)
    }

    /// Returns true if this DFA accepts the given bytes or false if the bytes are not accepted
    pub fn match_bytes(&self, bytes: &[u8]) -> bool {
        let mut curr = self.start();
        for &byte in bytes {
            curr = match curr.transition(byte) {
                Some(next) => next,
                None => return false,
            };
        }

        curr.is_accepted()
    }

    fn push_state(&mut self) -> StateId {
        let id = self.states.len();
        self.states.push(State::default());
        StateId(id.try_into().expect("DFAs with more than u16::MAX states are not supported"))
    }

    fn state(&self, id: StateId) -> &State {
        let StateId(index) = id;
        // Safety: StateId is guaranteed to be a valid index by construction
        unsafe { self.states.get_unchecked(index as usize) }
    }

    fn state_mut(&mut self, id: StateId) -> &mut State {
        let StateId(index) = id;
        // Safety: StateId is guaranteed to be a valid index by construction
        unsafe { self.states.get_unchecked_mut(index as usize) }
    }
}
