use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Cloudfront;

impl Detector for Cloudfront {
    fn name(&self) -> &'static str {
        "Cloudfront"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::header_contains(resp, "x-cache", "Error from cloudfront") {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Cloudfront as &dyn Detector
}