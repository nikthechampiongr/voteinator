use voteinator::{domain::context::RoundResult, serialize::create_context};

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);
    if args.len() != 2 {
        eprintln!("Received an incorrect amount of arguments");
        eprintln!("Usage: voteinator vote-csv num_of_seats");
        std::process::exit(1);
    }

    let ctx = create_context(args.next().unwrap(), args.next().unwrap().parse().unwrap());

    for (i, res) in ctx.enumerate() {
        let i = i + 1;
        match res {
            RoundResult::CandidateSucceeded(winner, votes) => {
                println!("Round {i} Candidate {winner} has won a seat with {votes} votes")
            }
            RoundResult::CandidateEliminated(loser) => {
                println!("Round {i} Candidate {loser} is eliminated");
            }
        }
        let mut _thingy = String::new();
        std::io::stdin().read_line(&mut _thingy).unwrap();
    }
    println!("Election concluded");
}
