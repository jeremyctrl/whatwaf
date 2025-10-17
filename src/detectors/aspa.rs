use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct ASPA;

impl Detector for ASPA {
    fn name(&self) -> &'static str {
        "ASPA"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::header_contains(resp, "server", "ASPA-WAF") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &ASPA as &dyn Detector
}