mod client;
mod logic;

use std::env;
use getopts::Options;
use client::SCClient;
use logic::OwnGameLogic;

fn print_usage(program: &str, options: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", options.usage(&brief));
}

fn main() {
    // Parse command line arguments
    let args = env::args().collect::<Vec<_>>();
    let mut options = Options::new();
    options.optopt("h", "host", "The game server's host address", "HOST");
    options.optopt("p", "port", "The game server's port", "PORT");
    options.optopt("r", "reservation", "A game reservation", "RESERVATION");
    options.optflag("H", "help", "Prints usage info");
    
    let parsed_args = options.parse(&args[1..]).expect("Could not parse arguments!");
    if parsed_args.opt_present("help") {
        print_usage(&args[0], options);
        return;
    }

    let host = parsed_args.opt_str("host").unwrap_or("localhost".to_owned());
    let port = parsed_args.opt_str("port").unwrap_or("13050".to_owned());
    let reservation = parsed_args.opt_str("reservation").unwrap_or("".to_owned());
    
    // Setup the client and the delegate
    let client = SCClient::new(OwnGameLogic);
}
