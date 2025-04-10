use std::collections::HashMap;

use crate::serialize::VoteEntry;

use super::{
    Candidate,
    vote::{Vote, VotePreference},
};

pub enum Round {
    WinnerFound(u32),
    CandidateEliminated(u32),
    ElectionConcluded,
}

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

        self.votes
            .entry(vote.user_id)
            .and_modify(|v| {
                if vote.vote_rank == 0 {
                    return;
                }

                v.push(super::vote::VotePreference::new(
                    vote.vote_option,
                    vote.vote_rank,
                ));
            })
            .or_insert({
                let mut v = Vote::new();

                if vote.vote_rank != 0 {
                    v.push(VotePreference::new(vote.vote_option, vote.vote_rank));
                }
                v
            });
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
        let quota = self.seats
            / std::convert::TryInto::<u32>::try_into(self.votes.len())
                .expect("We have more votes than u32::MAX. This program has reached its limit");

        Ok(Context {
            candidates: self.candidates,
            candidate_names: self.candidate_names,
            votes: self.votes,
            seats: self.seats,
            quota,
        })
    }
}

pub struct Context {
    candidates: HashMap<u32, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<u32, Vote>,
    seats: u32,
    quota: u32,
}

impl Context {}
