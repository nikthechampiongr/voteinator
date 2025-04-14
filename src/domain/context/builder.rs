use std::collections::HashMap;

use crate::{
    deserialize::VoteEntry,
    domain::{
        Candidate, Restriction,
        vote::{Vote, VotePreference},
    },
};

use super::Context;

pub struct ContextBuilder {
    candidates: HashMap<usize, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<usize, Vote>,
    restrictions: Vec<Restriction>,
    seats: usize,
}

impl ContextBuilder {
    pub fn new(seats: usize) -> Self {
        Self {
            candidates: HashMap::new(),
            candidate_names: Vec::new(),
            votes: HashMap::new(),
            restrictions: Vec::new(),
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

    pub fn insert_restriction(&mut self, restriction: Restriction) {
        self.restrictions.push(restriction);
    }

    pub fn finish(mut self) -> Result<Context, String> {
        if self.seats < 2 {
            return Err("Available seats are less than 2".to_string());
        }

        for (voter_id, vote) in self.votes.iter() {
            if let Err(e) = vote.validate() {
                return Err(format!("Error found in vote {voter_id}: {e}"));
            }
        }

        let quota = (self.votes.len() as f64 / self.seats as f64).ceil() as usize;

        for i in 0..self.restrictions.len() {
            for j in 0..self.restrictions[i].members().len() {
                let member = self.restrictions[i].members()[j];
                let member = self.candidates.get_mut(&member).ok_or(format!(
                    "{} member {} is not a candidate",
                    self.restrictions[i].group_name(),
                    &member
                ))?;
                member.insert_group(i)?;
            }
        }

        Ok(Context {
            candidates: self.candidates,
            candidate_names: self.candidate_names,
            votes: self.votes,
            seats_remaining: self.seats,
            quota,
            restrictions: self.restrictions,
            active_group_elimination: None,
        })
    }
}
