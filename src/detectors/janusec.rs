use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Janusec;

static BODY: Lazy<Vec<Regex>> =
    Lazy::new(|| vec![Regex::new(r"Policy ID:\s*\d+,\s*by Janusec Application Gateway").unwrap()]);

impl Detector for Janusec {
    fn name(&self) -> &'static str {
        "Janusec"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_matches(&BODY, MatchMode::Any)
    }
}

inventory::submit! {
    &Janusec as &dyn Detector
}
