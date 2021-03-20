mod cursor;

use std::convert::TryInto;

use fxhash::FxHashMap as HashMap;

pub use cursor::*;

type InputChar = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateId(u16);

#[derive(Debug, Clone)]
pub struct State {
    id: StateId,
    /// if true, a string will be accepted if it ends at this state
    accept: bool,
    /// The next states that can be visted from this state and the input that leads to them
    next_states: HashMap<InputChar, StateId>,
}

impl State {
    pub fn new(id: StateId) -> Self {
        Self {
            id,
            accept: false,
            next_states: HashMap::default(),
        }
    }

    pub fn add_transition(&mut self, input: InputChar, next_state: StateId) {
        assert!(
            !self.next_states.contains_key(&input),
            "bug: transition to state `{:?}` from state `{:?}` via input `{}` would overwite an existing transition",
            self.id,
            next_state,
            input as char,
        );

        self.next_states.insert(input, next_state);
    }

    /// Mark this state as an accept state
    pub fn mark_accept(&mut self) {
        self.accept = true;
    }
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
        let mut dfa = Self {states: Vec::new()};
        // Must have at least one state. First state is the start state.
        dfa.push_state();
        dfa
    }
}

impl Dfa {
    /// Returns a cursor to the start state of the DFA
    pub fn start(&self) -> DfaState {
        assert!(!self.states.is_empty());
        DfaState::new(self, self.start_id())
    }

    /// Returns the ID of the start state
    pub fn start_id(&self) -> StateId {
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

    /// Pushes a new empty state into the DFA and returns its ID
    pub fn push_state(&mut self) -> StateId {
        let next_index = self.states.len();
        let id = StateId(next_index.try_into()
            .expect("DFAs with more than u16::MAX states are not supported"));

        self.states.push(State::new(id));

        id
    }

    /// Get a state based on its ID
    pub fn state(&self, id: StateId) -> &State {
        let StateId(index) = id;
        // Safety: StateId is guaranteed to be a valid index by construction
        unsafe { self.states.get_unchecked(index as usize) }
    }

    /// Gets a mutable state based on its ID
    pub fn state_mut(&mut self, id: StateId) -> &mut State {
        let StateId(index) = id;
        // Safety: StateId is guaranteed to be a valid index by construction
        unsafe { self.states.get_unchecked_mut(index as usize) }
    }
}
