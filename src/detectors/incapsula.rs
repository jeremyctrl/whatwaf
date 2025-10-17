use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Incapsula;

impl Detector for Incapsula {
    fn name(&self) -> &'static str {
        "Incapsula"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::header_contains(resp, "set-cookie", "incap_ses") {
            return true;
        }

        if checks::header_contains(resp, "set-cookie", "visid_incap") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Incapsula as &dyn Detector
}
