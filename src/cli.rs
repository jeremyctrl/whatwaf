use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "whatwaf",
    version
)]
pub struct Args {
    // Target URL to check
    pub url: Option<String>,
}