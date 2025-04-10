use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(serde::Deserialize)]
struct VoteEntry {
    #[serde(alias = "vote_time", skip)]
    _vote_time: String,
    vote_option: u32,
    vote_rank: u32,
    user_id: u32,
    #[serde(alias = "username", skip)]
    _username: String,
    #[serde(alias = "name", skip)]
    _name: String,
    #[serde(alias = "trust_level", skip)]
    _trust_level: u16,
    vote_option_full: String,
}

#[derive(Debug)]
struct VotePreference {
    candidate_id: u32,
    preference: u32,
}

impl VotePreference {
    fn new(candidate_id: u32, preference: u32) -> Self {
        VotePreference {
            candidate_id,
            preference,
        }
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
struct Vote {
    strength: f64,
    preferences: BinaryHeap<VotePreference>,
}

impl Vote {
    fn new() -> Self {
        Vote {
            strength: 1.0,
            preferences: BinaryHeap::new(),
        }
    }

    fn push(&mut self, v: VotePreference) {
        self.preferences.push(v);
    }

    fn pop(&mut self) -> Result<Option<u32>, String> {
        let c = self.preferences.pop();

        Ok(c.map(|c| c.preference))
    }

    fn validate(&self) -> Result<(), String> {
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

enum Round {
    WinnerFound(u32),
    CandidateEliminated(u32),
    ElectionConcluded,
}

struct ContextBuilder {
    candidates: HashMap<u32, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<u32, Vote>,
    seats: u32,
}

impl ContextBuilder {
    fn new(seats: u32) -> Self {
        Self {
            candidates: HashMap::new(),
            candidate_names: Vec::new(),
            votes: HashMap::new(),
            seats,
        }
    }

    fn insert_vote(&mut self, vote: VoteEntry) {
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

                v.push(VotePreference::new(vote.vote_option, vote.vote_rank));
            })
            .or_insert({
                let mut v = Vote::new();

                if vote.vote_rank != 0 {
                    v.push(VotePreference::new(vote.vote_option, vote.vote_rank));
                }
                v
            });
    }

    fn finish(self) -> Result<Context, String> {
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

struct Context {
    candidates: HashMap<u32, Candidate>,
    candidate_names: Vec<String>,
    votes: HashMap<u32, Vote>,
    seats: u32,
    quota: u32,
}

impl Context {}

struct Candidate {
    interned_index: usize,
    eliminated: bool,
}

impl Candidate {
    fn new(interned_index: usize) -> Self {
        Candidate {
            interned_index,
            eliminated: false,
        }
    }
}

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);
    if args.len() != 2 {
        eprintln!("Received an incorrect amount of arguments");
        eprintln!("Usage: voteinator vote-csv num_of_seats");
        std::process::exit(1);
    }

    let votes = std::io::BufReader::new(std::fs::File::open(args.next().unwrap()).unwrap());
    let seats: u32 = args.next().unwrap().parse().unwrap();

    let mut ctx = ContextBuilder::new(seats);

    let mut votes = csv::Reader::from_reader(votes);

    for vote in votes.deserialize() {
        ctx.insert_vote(vote.unwrap());
    }

    let mut ctx = ctx.finish().unwrap();

    for (i, candidate) in ctx.candidates.iter().enumerate() {
        println!(
            "Id {} Candidate {i}: {}",
            candidate.0, ctx.candidate_names[candidate.1.interned_index]
        );
    }

    for vote in ctx.votes.values() {
        dbg!(vote);
    }
}
