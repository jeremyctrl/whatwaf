use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Sucuri;

impl Detector for Sucuri {
    fn name(&self) -> &'static str {
        "Sucuri"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::has_header(resp, "x-sucuri-block") && checks::is_forbidden(resp) {
            return true;
        }

        if checks::body_contains(resp, "Sucuri Website Firewall") && checks::is_forbidden(resp) {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Sucuri as &dyn Detector
}
