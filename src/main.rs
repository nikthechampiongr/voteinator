use std::collections::HashMap;

use voteinator::{deserialize::create_context, domain::context::RoundResult};

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);
    if args.len() != 2 && args.len() != 3 {
        eprintln!("Received an incorrect amount of arguments");
        eprintln!("Usage: voteinator vote-csv num_of_seats [restrictions.csv]");
        std::process::exit(1);
    }

    let ctx = create_context(
        args.next().unwrap(),
        args.next().unwrap().parse().unwrap(),
        args.next(),
    );
    let quota = ctx.quota();
    let mut seats = ctx.seats_remaining();

    for (i, res) in ctx.enumerate() {
        let i = i + 1;
        match res {
            RoundResult::CandidateSucceeded(winner, votes, total_votes) => {
                println!("Round {i} Candidate {winner} has won a seat with {votes} votes");
                tally_print(&total_votes, seats, quota);
                seats -= 1;
            }
            RoundResult::CandidateEliminated(loser, total_votes) => {
                println!("Round {i} Candidate {loser} is eliminated");
                tally_print(&total_votes, seats, quota);
            }
            RoundResult::CandidateEliminatedByRestriction(candidate, group) => {
                println!(
                    "{candidate} was eliminated because {group} reached its limit of elected candidates"
                )
            }
        }
        let mut _thingy = String::new();
        std::io::stdin().read_line(&mut _thingy).unwrap();
    }
    println!("Election concluded");
}

fn tally_print(votes: &HashMap<String, usize>, seats: usize, quota: usize) {
    println!("Quota: {quota}");
    println!("Seats remaining: {seats}");
    for (key, value) in votes {
        println!("{} -> {}", key, value);
    }
}
