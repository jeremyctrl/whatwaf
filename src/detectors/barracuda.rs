use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Barracuda;

impl Detector for Barracuda {
    fn name(&self) -> &'static str {
        "Barracuda"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "Barracuda Networks") && checks::is_not_found(resp) {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Barracuda as &dyn Detector
}