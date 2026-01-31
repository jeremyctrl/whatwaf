mod cli;

use clap::Parser;

use whatwaf::{ProbeResult, ScanConfig, list_detectors, scan_url};

fn main() {
    let args = cli::Args::parse();

    if args.list {
        println!("[~] whatwaf can currently recognize:");
        for d in list_detectors() {
            println!("\t{}", d);
        }

        return;
    }

    let url = match args.url {
        Some(u) => u,
        None => {
            eprintln!("[!] no URL provided. usage: whatwaf [OPTIONS] <URL>");
            return;
        }
    };

    let config = ScanConfig {
        timeout: args.timeout,
        follow_redirects: args.location,
        proxy: args.proxy,
    };

    println!("[*] scanning {}", url);

    let mut detected = false;

    let res = scan_url(
        &url,
        config,
        Some(|r: &ProbeResult| {
            println!("[*] {} probe: url={}", r.probe_name, r.url);

            match &r.detected_wafs {
                Some(wafs) => {
                    println!(
                        "\t[+] waf=({}) status={}",
                        wafs.join(", ").to_lowercase(),
                        r.status
                    );

                    if wafs.len() == 1 {
                        println!("[~] detected waf: {}", wafs[0]);
                    } else {
                        println!("[~] detected wafs:");
                        for w in wafs {
                            println!("\t- {}", w.to_lowercase());
                        }
                    }

                    detected = true;

                    false
                }
                None => {
                    println!("\t[-] no detection (status={})", r.status);
                    true
                }
            }
        }),
    );

    if let Err(e) = res {
        eprintln!("[!] scan failed: {}", e);
        return;
    }

    if !detected {
        println!("[~] no waf detected for {}", url);
    }
}
