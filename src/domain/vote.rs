use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
pub struct VotePreference {
    candidate_id: u32,
    preference: u32,
}

impl VotePreference {
    pub fn new(candidate_id: u32, preference: u32) -> Self {
        VotePreference {
            candidate_id,
            preference,
        }
    }
    pub fn candidate_id(&self) -> u32 {
        self.candidate_id
    }

    #[allow(dead_code)]
    pub fn preference(&self) -> u32 {
        self.preference
    }
}

impl std::cmp::PartialOrd for VotePreference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for VotePreference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.preference
            .cmp(&other.preference)
            .reverse()
            .then(self.candidate_id.cmp(&other.candidate_id))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::vote::VotePreference;

    #[test]
    fn ensure_ordering_of_preferences_is_reverse_of_number_ordering() {
        let a = VotePreference::new(1, 1);
        let b = VotePreference::new(1, 2);
        assert!(a > b);
    }
}

impl std::cmp::Eq for VotePreference {}

impl std::cmp::PartialEq for VotePreference {
    fn eq(&self, other: &Self) -> bool {
        self.preference == other.preference
    }
}

#[derive(Debug)]
pub(super) struct Vote {
    strength: f64,
    preferences: BinaryHeap<VotePreference>,
}

impl Vote {
    pub(super) fn new() -> Self {
        Self {
            strength: 1.0,
            preferences: BinaryHeap::new(),
        }
    }

    #[allow(dead_code)]
    pub(super) fn strength(&self) -> f64 {
        self.strength
    }

    pub(super) fn multiply_strength(&mut self, multiplier: f64) {
        self.strength *= multiplier;
    }

    pub(super) fn push(&mut self, v: VotePreference) {
        self.preferences.push(v);
    }

    pub(super) fn pop(&mut self) -> Option<u32> {
        let c = self.preferences.pop();

        c.map(|c| c.preference)
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        let mut candidates = HashSet::new();
        let mut preferences = HashSet::new();

        for vote in self.preferences.iter() {
            if !candidates.insert(vote.candidate_id) {
                return Err(format!(
                    "Vote contained candidate {} twice",
                    vote.candidate_id
                ));
            }

            if !preferences.insert(vote.preference) {
                return Err(format!(
                    "Vote contained preference {} twice",
                    vote.preference
                ));
            }
        }
        Ok(())
    }

    pub(super) fn peek(&self) -> Option<u32> {
        self.preferences.peek().map(|f| f.candidate_id())
    }
}
