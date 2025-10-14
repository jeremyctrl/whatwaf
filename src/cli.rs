use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "whatwaf", version)]
pub struct Args {
    /// Target URL to check
    pub url: Option<String>,

    /// Per-request timeout in seconds
    #[arg(short = 'T', long = "timeout", default_value_t = 10)]
    pub timeout: u64,

    /// Route requests via this proxy
    #[arg(short = 'x', long = "proxy")]
    pub proxy: Option<String>,
}
