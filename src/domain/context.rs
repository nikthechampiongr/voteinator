use std::collections::HashMap;

use crate::serialize::VoteEntry;

use super::{
    Candidate,
    vote::{Vote, VotePreference},
};

pub struct ContextBuilder {
    candidates: HashMap<u32, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<u32, Vote>,
    seats: u32,
}

impl ContextBuilder {
    pub fn new(seats: u32) -> Self {
        Self {
            candidates: HashMap::new(),
            candidate_names: Vec::new(),
            votes: HashMap::new(),
            seats,
        }
    }

    pub fn insert_vote(&mut self, vote: VoteEntry) {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.candidates.entry(vote.vote_option)
        {
            self.candidate_names.push(vote.vote_option_full);
            e.insert(Candidate::new(self.candidate_names.len() - 1));
        }

        let curr_vote = self.votes.entry(vote.user_id).or_insert(Vote::new());
        if vote.vote_rank != 0 {
            curr_vote.push(VotePreference::new(vote.vote_option, vote.vote_rank));
        }
        if vote.vote_option == 1810 {}
    }

    pub fn finish(self) -> Result<Context, String> {
        if self.seats < 2 {
            return Err("Available seats are less than 2".to_string());
        }

        for (voter_id, vote) in self.votes.iter() {
            if let Err(e) = vote.validate() {
                return Err(format!("Error found in vote {voter_id}: {e}"));
            }
        }
        let quota = (self.votes.len() as f64 / self.seats as f64).ceil() as u32;

        Ok(Context {
            candidates: self.candidates,
            candidate_names: self.candidate_names,
            votes: self.votes,
            seats_remaining: self.seats,
            quota,
        })
    }
}

pub struct Context {
    candidates: HashMap<u32, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<u32, Vote>,
    seats_remaining: u32,
    quota: u32,
}

impl Context {
    pub fn quota(&self) -> u32 {
        self.quota
    }

    pub fn seats_remaining(&self) -> u32 {
        self.seats_remaining
    }

    pub fn get_name(&self, id: u32) -> Option<String> {
        if let Some(id) = self.candidates.get(&id) {
            Some(self.candidate_names[id.interned_id()].clone())
        } else {
            None
        }
    }
}

pub enum RoundResult {
    CandidateSucceeded(String, u32, HashMap<String, usize>),
    CandidateEliminated(String, HashMap<String, usize>),
}

impl Iterator for Context {
    type Item = RoundResult;

    fn next(&mut self) -> Option<Self::Item> {
        // No more seats available. Elections over
        if self.seats_remaining == 0 {
            return None;
        }

        let mut votes: HashMap<u32, Vec<u32>> = generate_vote_tally_map(&self.candidates);

        // No eligible candidates. Election concluded.
        if votes.is_empty() {
            return None;
        }

        for (id, vote) in self.votes.iter_mut() {
            while vote.peek().is_some() && !votes.contains_key(&vote.peek().unwrap()) {
                vote.pop();
            }
            let vote_pref = vote.peek();

            if vote_pref.is_none() {
                continue;
            }
            votes.entry(vote_pref.unwrap()).and_modify(|v| v.push(*id));
        }

        let mut biggest_winner = None;
        let mut biggest_winner_votes: u32 = 0;
        let mut biggest_loser = None;
        let mut biggest_loser_votes: u32 = u32::MAX;

        for (candidate, votes) in &votes {
            if votes.len() >= self.quota.try_into().unwrap()
                && votes.len() > biggest_winner_votes.try_into().unwrap()
            {
                biggest_winner_votes = votes.len() as u32;
                biggest_winner = Some(candidate);
            }

            if votes.len() < biggest_loser_votes.try_into().unwrap() {
                biggest_loser_votes = votes.len().try_into().unwrap();
                biggest_loser = Some(candidate);
            }
        }

        match (biggest_winner, biggest_loser) {
            (Some(winner), _) => {
                let curr_votes = votes.get(winner).unwrap();

                for vote in curr_votes {
                    let vote = self.votes.get_mut(vote).unwrap();
                    vote.pop();
                    vote.multiply_strength(1.0 - (self.quota / biggest_winner_votes) as f64);
                }
                let candidate = self.candidates.get_mut(winner).unwrap();
                candidate.eliminate();
                self.seats_remaining -= 1;
                Some(RoundResult::CandidateSucceeded(
                    self.candidate_names[candidate.interned_id()].clone(),
                    biggest_winner_votes,
                    HashMap::from_iter(
                        votes
                            .iter()
                            .map(|(k, v)| (self.get_name(*k).unwrap(), v.len())),
                    ),
                ))
            }
            (None, Some(loser)) => {
                let candidate = self.candidates.get_mut(loser).unwrap();
                candidate.eliminate();

                let curr_votes = votes.get(loser).unwrap();
                for vote in curr_votes {
                    let vote = self.votes.get_mut(vote).unwrap();
                    vote.pop();
                }

                Some(RoundResult::CandidateEliminated(
                    self.candidate_names[candidate.interned_id()].clone(),
                    HashMap::from_iter(
                        votes
                            .iter()
                            .map(|(k, v)| (self.get_name(*k).unwrap(), v.len())),
                    ),
                ))
            }
            (None, None) => None,
        }
    }
}

fn generate_vote_tally_map(candidates: &HashMap<u32, Candidate>) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();

    for (key, value) in candidates {
        if value.is_eliminated() {
            continue;
        }

        map.insert(*key, Vec::new());
    }

    map
}
