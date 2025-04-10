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

    pub fn preference(&self) -> u32 {
        self.preference
    }
}

impl std::cmp::PartialOrd for VotePreference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.preference.cmp(&other.preference))
    }
}

impl std::cmp::Ord for VotePreference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.preference
            .cmp(&other.preference)
            .then(self.candidate_id.cmp(&other.candidate_id))
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

    pub(super) fn strength(&self) -> f64 {
        self.strength
    }

    pub(super) fn multiply_strength(&mut self, multiplier: f64) {
        self.strength *= multiplier;
    }

    pub(super) fn push(&mut self, v: VotePreference) {
        self.preferences.push(v);
    }

    pub(super) fn pop(&mut self) -> Result<Option<u32>, String> {
        let c = self.preferences.pop();

        Ok(c.map(|c| c.preference))
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
}
