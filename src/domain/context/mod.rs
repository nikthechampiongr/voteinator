use std::collections::HashMap;

use super::{Candidate, Restriction, vote::Vote};

pub mod builder;
mod iter_helpers;
pub use builder::ContextBuilder;

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
}

struct WinnerLoserStruct {
    biggest_winner: Option<usize>,
    biggest_winner_votes: usize,
    biggest_loser: Option<usize>,
    #[allow(dead_code)]
    biggest_loser_votes: usize,
}

impl WinnerLoserStruct {
    fn calculate(ctx: &Context, votes: &HashMap<usize, Vec<usize>>, quota: usize) -> Self {
        let mut biggest_winner = None;
        let mut biggest_winner_votes: usize = 0;
        let mut biggest_loser = None;
        let mut biggest_loser_votes: usize = usize::MAX;

        for (candidate, votes) in votes {
            if ctx.sum_votes(votes) >= quota && ctx.sum_votes(votes) > biggest_winner_votes {
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

        if let Some((candidate_id, group_id)) = self.handle_restrictions() {
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
        } = WinnerLoserStruct::calculate(self, &votes, self.quota());

        match (biggest_winner, biggest_loser) {
            (Some(winner), _) => {
                let curr_votes = votes.get(&winner).unwrap();

                for vote in curr_votes {
                    let vote = self.votes.get_mut(vote).unwrap();
                    vote.pop();
                    vote.multiply_strength(1.0 - (self.quota as f64 / biggest_winner_votes as f64));
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
                            .map(|(k, v)| (self.get_name(*k).unwrap(), self.sum_votes(v))),
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
                            .map(|(k, v)| (self.get_name(*k).unwrap(), self.sum_votes(v))),
                    ),
                ))
            }
            (None, None) => None,
        }
    }
}
