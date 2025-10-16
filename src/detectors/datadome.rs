use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Datadome;

impl Detector for Datadome {
    fn name(&self) -> &'static str {
        "Datadome"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::has_header(resp, "x-datadome") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Datadome as &dyn Detector
}
