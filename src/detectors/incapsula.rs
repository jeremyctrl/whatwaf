use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Incapsula;

impl Detector for Incapsula {
    fn name(&self) -> &'static str {
        "Incapsula"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "incapsula incident id:") && checks::is_forbidden(resp) {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Incapsula as &dyn Detector
}