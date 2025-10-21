use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct NexusGuard;

static BODY: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![Regex::new(r"(?i)(?:https?://)?(?:speresources\.)?nexusguard\.com\.wafpage").unwrap()]
});

impl Detector for NexusGuard {
    fn name(&self) -> &'static str {
        "NexusGuard"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.has_header(&["x-nxg", "x-nxg-waf"], MatchMode::Any)
            || resp.body_matches(&BODY, MatchMode::Any)
    }
}

inventory::submit! {
    &NexusGuard as &dyn Detector
}
