use std::io::BufRead;

use crate::domain::{
    Restriction,
    context::{Context, ContextBuilder},
};

#[derive(serde::Deserialize, Debug)]
pub struct VoteEntry {
    #[serde(alias = "vote_time", skip)]
    _vote_time: String,
    pub vote_option: usize,
    pub vote_rank: usize,
    pub user_id: usize,
    #[serde(alias = "username", skip)]
    _username: String,
    #[serde(alias = "name", skip)]
    _name: String,
    #[serde(alias = "trust_level", skip)]
    _trust_level: u16,
    pub vote_option_full: String,
}

pub fn create_context(file_name: String, seats: usize, restrictions: Option<String>) -> Context {
    let mut ctx = ContextBuilder::new(seats);

    let votes = std::io::BufReader::new(std::fs::File::open(file_name).unwrap());
    let mut votes = csv::Reader::from_reader(votes);

    for vote in votes.deserialize() {
        ctx.insert_vote(vote.unwrap());
    }

    deserialize_restrictions(restrictions, &mut ctx);

    ctx.finish().unwrap()
}

pub fn deserialize_restrictions(restrictions: Option<String>, ctx: &mut ContextBuilder) {
    if let Some(restrictions) = restrictions {
        // Need to deserialize manually
        let mut restrictions =
            std::io::BufReader::new(std::fs::File::open(restrictions).unwrap()).lines();

        let headers = restrictions.next().unwrap().unwrap();
        let mut headers = headers.split(',');

        assert_eq!(
            headers.next(),
            Some("group_name"),
            "First header was not `group_name`"
        );

        assert_eq!(
            headers.next(),
            Some("limit"),
            "First header was not `limit`"
        );

        assert_eq!(
            headers.next(),
            Some("members"),
            "First header was not `members`"
        );

        for line in restrictions {
            let line = line.unwrap();
            let mut line = line.split(',');

            let group_name = line.next().unwrap();
            let limit = line.next().unwrap().parse().unwrap();
            let mut members = Vec::new();

            for member in line {
                members.push(member.parse().unwrap());
            }
            ctx.insert_restriction(Restriction::new(group_name.to_string(), limit, members));
        }
    }
}
