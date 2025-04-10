use voteinator::serialize::create_context;

fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);
    if args.len() != 2 {
        eprintln!("Received an incorrect amount of arguments");
        eprintln!("Usage: voteinator vote-csv num_of_seats");
        std::process::exit(1);
    }

    let ctx = create_context(args.next().unwrap(), args.next().unwrap().parse().unwrap());
}
