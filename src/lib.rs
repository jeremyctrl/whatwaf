pub mod detector;
pub mod detectors;
pub mod utils;

use std::time::Duration;

use reqwest::{Proxy, blocking::Client};

use crate::detector::run_detectors;
use crate::utils::http::{add_param, fetch};

pub struct ScanConfig {
    pub timeout: u64,
    pub follow_redirects: bool,
    pub proxy: Option<String>,
}

pub struct DetectionResult {
    pub waf_name: String,
    pub status: u16,
    pub probe: String,
}

pub fn list_detectors() -> Vec<&'static str> {
    inventory::iter::<&'static dyn detectors::Detector>
        .into_iter()
        .map(|d| d.name())
        .collect()
}

pub fn scan_url(
    url: &str,
    config: ScanConfig,
) -> Result<Option<DetectionResult>, Box<dyn std::error::Error>> {
    let probes = vec![
        ("plain request", None),
        ("xss", Some(("q", "<script>alert(1)</script>"))),
        ("sql injection", Some(("id", "' OR '1'=1'"))),
        ("lfi", Some(("file", "../../../../etc/passwd"))),
    ];

    let mut builder = Client::builder().timeout(Duration::from_secs(config.timeout));

    if !config.follow_redirects {
        builder = builder.redirect(reqwest::redirect::Policy::none());
    }

    if let Some(proxy) = config.proxy {
        builder = builder.proxy(Proxy::all(proxy)?);
    }

    let client = builder.build()?;

    for (probe_name, probe) in probes {
        let probe_url = match probe {
            Some((k, v)) => add_param(url, k, v),
            None => url.to_string(),
        };

        let resp = fetch(&client, &probe_url)?;

        if let Some(name) = run_detectors(&resp) {
            return Ok(Some(DetectionResult {
                waf_name: name.to_string(),
                status: resp.status,
                probe: probe_name.to_string(),
            }));
        }
    }

    Ok(None)
}
