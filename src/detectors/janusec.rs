use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Janusec;

static BODY: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Policy ID:\s*\d+,\s*by Janusec Application Gateway").unwrap()
});

impl Detector for Janusec {
    fn name(&self) -> &'static str {
        "Janusec"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_matches_regex(resp, &BODY) {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Janusec as &dyn Detector
}
