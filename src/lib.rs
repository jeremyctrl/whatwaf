mod detector;
mod detectors;
mod utils;

pub use detectors::Detector;

use std::fmt;
use std::time::Duration;

use reqwest::{Proxy, blocking::Client};

use crate::detector::run_detectors;
use crate::utils::http::{add_param, fetch};

pub struct ScanConfig {
    pub timeout: u64,
    pub follow_redirects: bool,
    pub proxy: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ProbeResult {
    pub probe_name: String,
    pub url: String,
    pub status: u16,
    pub detected_wafs: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ScanError {
    InvalidProxy {
        proxy: String,
        source: reqwest::Error,
    },
    ClientBuild(reqwest::Error),
    Request {
        url: String,
        source: reqwest::Error,
    },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::InvalidProxy { proxy, source } => {
                write!(f, "invalid proxy '{}': {}", proxy, source)
            }
            ScanError::ClientBuild(e) => {
                write!(f, "failed to build HTTP client: {}", e)
            }

            ScanError::Request { url, source } => {
                write!(f, "request failed for {}: {}", url, source)
            }
        }
    }
}

impl std::error::Error for ScanError {}

pub fn list_detectors() -> Vec<&'static str> {
    inventory::iter::<&'static dyn detectors::Detector>
        .into_iter()
        .map(|d| d.name())
        .collect()
}

pub fn scan_url<F>(
    url: &str,
    config: ScanConfig,
    mut on_probe: Option<F>,
) -> Result<Option<ProbeResult>, ScanError>
where
    F: FnMut(&ProbeResult) -> bool,
{
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

    if let Some(proxy) = &config.proxy {
        let px = Proxy::all(proxy).map_err(|e| ScanError::InvalidProxy {
            proxy: proxy.clone(),
            source: e,
        })?;
        builder = builder.proxy(px);
    }

    let client = builder.build().map_err(ScanError::ClientBuild)?;

    let mut last_result = None;

    for (probe_name, probe) in probes {
        let probe_url = match probe {
            Some((k, v)) => add_param(url, k, v),
            None => url.to_string(),
        };

        let resp = fetch(&client, &probe_url).map_err(|e| ScanError::Request {
            url: probe_url.clone(),
            source: e,
        })?;

        let result = ProbeResult {
            probe_name: probe_name.to_string(),
            url: probe_url,
            status: resp.status,
            detected_wafs: run_detectors(&resp)
                .map(|v| v.into_iter().map(|s| s.to_string()).collect()),
        };

        if let Some(cb) = on_probe.as_mut() {
            if !cb(&result) {
                last_result = Some(result);
                break;
            }
        }

        last_result = Some(result);
    }

    Ok(last_result)
}
