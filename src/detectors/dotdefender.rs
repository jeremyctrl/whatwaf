use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct DotDefender;

impl Detector for DotDefender {
    fn name(&self) -> &'static str {
        "DotDefender"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "dotDefender Blocked Your Request") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &DotDefender as &dyn Detector
}
