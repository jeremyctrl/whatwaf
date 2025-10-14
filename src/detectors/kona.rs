use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Kona;

impl Detector for Kona {
    fn name(&self) -> &'static str {
        "Kona Site Defender"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "edgesuite") && checks::is_forbidden(resp) {
            return true;
        }

        if checks::has_header(resp, "x-reference-error") {
            return true;
        }

        if checks::header_contains(resp, "server", "AkamiGHost") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Kona as &dyn Detector
}