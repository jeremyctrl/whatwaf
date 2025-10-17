use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Astra;

impl Detector for Astra {
    fn name(&self) -> &'static str {
        "Astra"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "This website is protected by getastra.com") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Astra as &dyn Detector
}
