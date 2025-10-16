use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct NexusGuard;

static BODY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(?:https?://)?(?:speresources\.)?nexusguard\.com\.wafpage").unwrap()
});

impl Detector for NexusGuard {
    fn name(&self) -> &'static str {
        "NexusGuard"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::has_header(resp, "x-nxg") {
            return true;
        }

        if checks::has_header(resp, "x-nxg-waf") {
            return true;
        }

        if checks::body_matches_regex(resp, &BODY) {
            return true;
        }

        false
    }
}

inventory::submit! {
    &NexusGuard as &dyn Detector
}
