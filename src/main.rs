mod cli;
mod utils;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    if let Some(url) = args.url {
        let result = utils::http::fetch(&url);
        println!("{:?}", result);
    } else {
        println!("no url provided")
    }
}
