use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct FortiGuard;

static BODY: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![Regex::new(r"Attack ID:\s*2(?:0*\d{2})").unwrap()] // https://docs.fortinet.com/document/fortiweb/8.0.1/log-message-reference/445549/attack
});

impl Detector for FortiGuard {
    fn name(&self) -> &'static str {
        "FortiGuard"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["<h2 class=\"fgd_icon\">block</h2>"], MatchMode::Any)
            && resp.body_matches(&BODY, MatchMode::Any)
    }
}

inventory::submit! {
    &FortiGuard as &dyn Detector
}
