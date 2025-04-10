use crate::domain::context::{Context, ContextBuilder};

#[derive(serde::Deserialize, Debug)]
pub struct VoteEntry {
    #[serde(alias = "vote_time", skip)]
    _vote_time: String,
    pub vote_option: u32,
    pub vote_rank: u32,
    pub user_id: u32,
    #[serde(alias = "username", skip)]
    _username: String,
    #[serde(alias = "name", skip)]
    _name: String,
    #[serde(alias = "trust_level", skip)]
    _trust_level: u16,
    pub vote_option_full: String,
}

pub fn create_context(file_name: String, seats: u32) -> Context {
    let mut ctx = ContextBuilder::new(seats);

    let votes = std::io::BufReader::new(std::fs::File::open(file_name).unwrap());
    let mut votes = csv::Reader::from_reader(votes);

    for vote in votes.deserialize() {
        ctx.insert_vote(vote.unwrap());
    }

    ctx.finish().unwrap()
}
