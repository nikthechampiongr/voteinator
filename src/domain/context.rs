use std::collections::HashMap;

use crate::deserialize::{Restriction, VoteEntry};

use super::{
    Candidate,
    vote::{Vote, VotePreference},
};

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

pub struct Context {
    candidates: HashMap<usize, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<usize, Vote>,
    seats_remaining: usize,
    quota: usize,
    restrictions: Vec<Restriction>,
    active_group_elimination: Option<usize>,
}

impl Context {
    pub fn quota(&self) -> usize {
        self.quota
    }

    pub fn seats_remaining(&self) -> usize {
        self.seats_remaining
    }

    pub fn get_name(&self, id: usize) -> Option<String> {
        self.candidates
            .get(&id)
            .map(|id| self.candidate_names[id.interned_id()].clone())
    }

    fn calculate_votes(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut votes: HashMap<usize, Vec<usize>> = generate_vote_tally_map(&self.candidates);

        if votes.is_empty() {
            return votes;
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
        votes
    }
}

struct WinnerLoserStruct {
    biggest_winner: Option<usize>,
    biggest_winner_votes: usize,
    biggest_loser: Option<usize>,
    #[allow(dead_code)]
    biggest_loser_votes: usize,
}

impl WinnerLoserStruct {
    fn calculate(votes: &HashMap<usize, Vec<usize>>, quota: usize) -> Self {
        let mut biggest_winner = None;
        let mut biggest_winner_votes: usize = 0;
        let mut biggest_loser = None;
        let mut biggest_loser_votes: usize = usize::MAX;

        for (candidate, votes) in votes {
            if votes.len() >= quota && votes.len() > biggest_winner_votes {
                biggest_winner_votes = votes.len();
                biggest_winner = Some(*candidate);
            }

            if votes.len() < biggest_loser_votes {
                biggest_loser_votes = votes.len();
                biggest_loser = Some(*candidate);
            }
        }

        Self {
            biggest_winner,
            biggest_loser,
            biggest_loser_votes,
            biggest_winner_votes,
        }
    }
}

pub enum RoundResult {
    CandidateSucceeded(String, usize, HashMap<String, usize>),
    CandidateEliminated(String, HashMap<String, usize>),
    CandidateEliminatedByRestriction(String, String),
}

impl Iterator for Context {
    type Item = RoundResult;

    fn next(&mut self) -> Option<Self::Item> {
        // No more seats available. Elections over
        if self.seats_remaining == 0 {
            return None;
        }

        if let Some((candidate_id, group_id)) = handle_restrictions(self) {
            return Some(RoundResult::CandidateEliminatedByRestriction(
                self.get_name(candidate_id).unwrap(),
                self.restrictions[group_id].group_name().to_string(),
            ));
        }

        let votes = self.calculate_votes();

        // No eligible candidates. Election concluded.
        if votes.is_empty() {
            return None;
        }

        let WinnerLoserStruct {
            biggest_winner,
            biggest_loser,
            biggest_winner_votes,
            biggest_loser_votes: _,
        } = WinnerLoserStruct::calculate(&votes, self.quota());

        match (biggest_winner, biggest_loser) {
            (Some(winner), _) => {
                let curr_votes = votes.get(&winner).unwrap();

                for vote in curr_votes {
                    let vote = self.votes.get_mut(vote).unwrap();
                    vote.pop();
                    vote.multiply_strength(1.0 - (self.quota / biggest_winner_votes) as f64);
                }

                let candidate = self.candidates.get_mut(&winner).unwrap();
                candidate.eliminate();
                self.seats_remaining -= 1;

                self.active_group_elimination = candidate.group();

                if let Some(group_id) = candidate.group() {
                    self.restrictions[group_id].decrement();
                }

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
                let candidate = self.candidates.get_mut(&loser).unwrap();
                candidate.eliminate();

                let curr_votes = votes.get(&loser).unwrap();
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

fn handle_restrictions(ctx: &mut Context) -> Option<(usize, usize)> {
    if let Some(group_id) = ctx.active_group_elimination {
        let group = ctx.restrictions.get_mut(group_id).unwrap();

        if group.limit() != 0 {
            return None;
        }

        for candidate_id in ctx.restrictions[group_id].members() {
            let member = ctx.candidates.get_mut(candidate_id).unwrap();

            if member.is_eliminated() {
                continue;
            }
            member.eliminate();

            for vote in ctx.votes.values_mut() {
                if let Some(id) = vote.peek() {
                    if id == *candidate_id {
                        vote.pop();
                    }
                }
            }

            return Some((*candidate_id, group_id));
        }
    }
    None
}

fn generate_vote_tally_map(candidates: &HashMap<usize, Candidate>) -> HashMap<usize, Vec<usize>> {
    let mut map = HashMap::new();

    for (key, value) in candidates {
        if value.is_eliminated() {
            continue;
        }

        map.insert(*key, Vec::new());
    }

    map
}
