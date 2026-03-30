use once_cell::sync::Lazy;
use regex::Regex;

use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct DDoSGuard;

static HEADERS: Lazy<Vec<Regex>> = Lazy::new(|| vec![Regex::new(r"__ddg\d+_").unwrap()]);

impl Detector for DDoSGuard {
    fn name(&self) -> &'static str {
        "DDoS-Guard"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("server", &["ddos-guard"], MatchMode::Any)
            || resp.header_matches("set-cookie", &HEADERS, MatchMode::Any)
    }
}

inventory::submit! {
    &DDoSGuard as &dyn Detector
}
