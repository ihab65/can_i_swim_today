use structopt::StructOpt;

// API_KEY = 5e205416dfa82a16871aa55675e6aab6

#[derive(StructOpt, Debug)]
struct Cli {
    city: String,
    country_code: String
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args);
}
