use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct ArvanCloud;

impl Detector for ArvanCloud {
    fn name(&self) -> &'static str {
        "ArvanCloud"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::header_contains(resp, "server", "ArvanCloud") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &ArvanCloud as &dyn Detector
}