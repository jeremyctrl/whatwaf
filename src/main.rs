mod cli;
mod utils;

mod detector;
mod detectors;

use clap::Parser;

use crate::{detector::run_detectors, utils::http::add_param, utils::http::fetch};

fn main() {
    let args = cli::Args::parse();

    let url = match args.url {
        Some(u) => u,
        None => {
            eprintln!("[!] no URL provided. usage: whatwaf [OPTIONS] <URL>");
            return;
        }
    };

    let probes = vec![
        ("plain request", (None, None)),
        ("xss", (Some("q"), Some("<script>alert(1)</script>"))),
        ("sql injection", (Some("id"), Some("' OR '1'=1'"))),
        ("lfi", (Some("file"), Some("../../../../etc/passwd"))),
    ];

    let client_builder = reqwest::blocking::Client::builder();

    let client = match client_builder.build() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[!] failed to build reqwest client: {}", e);
            return;
        }
    };

    println!("[*] checking {}", url);
    println!("[*] running {} probes", probes.len());

    for (probe_name, (param, payload)) in probes.iter() {
        let probe_url = if let (Some(k), Some(v)) = (param, payload) {
            println!("[*] {} probe: payload='{}'", probe_name, v);
            add_param(&url, k, v)
        } else {
            println!("[*] {} probe: payload=None", probe_name);
            url.clone()
        };

        let resp = match fetch(&client, &probe_url) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("\t[-] error {}", e);
                continue;
            }
        };

        if let Some(name) = run_detectors(&resp) {
            println!("\t[+] waf={} status={}", name.to_lowercase(), resp.status);
            println!("[~] the site {} is behind {} waf", url, name);
            return;
        }

        println!("\t[-] no detection");
    }

    println!("[~] no waf detected for {}", url);
}
