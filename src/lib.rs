mod dfa;

#[derive(Debug, Clone)]
pub struct Regex {
    dfa: dfa::Dfa,
}

impl Regex {
    pub fn parse(pattern: &[u8]) -> Self {
        let mut dfa = dfa::Dfa::default();

        let mut curr = dfa.start_id();
        for &ch in pattern {
            let next_state = dfa.push_state();
            dfa.state_mut(curr).add_transition(ch, next_state);
            curr = next_state;
        }
        // Mark the last state we get to as the acceptance state
        dfa.state_mut(curr).mark_accept();

        Self {dfa}
    }

    /// Returns true if this regex matches the given bytes
    pub fn match_bytes(&self, bytes: &[u8]) -> bool {
        self.dfa.match_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_regex() {
        let re = Regex::parse(b"");
        assert!(re.match_bytes(b""));
        assert!(!re.match_bytes(b"a"));
        assert!(!re.match_bytes(b"abc"));
    }

    #[test]
    fn single_byte_regex() {
        let re = Regex::parse(b"a");
        assert!(!re.match_bytes(b""));
        assert!(re.match_bytes(b"a"));
        assert!(!re.match_bytes(b"abc"));
    }

    #[test]
    fn multi_byte_regex() {
        let re = Regex::parse(b"abc");
        assert!(!re.match_bytes(b""));
        assert!(!re.match_bytes(b"a"));
        assert!(re.match_bytes(b"abc"));
    }
}
