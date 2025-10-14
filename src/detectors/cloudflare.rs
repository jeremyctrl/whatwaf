use crate::detectors::Detector;
use crate::utils::{checks, http::HttpResponse};

pub struct Cloudflare;

impl Detector for Cloudflare {
    fn name(&self) -> &'static str {
        "Cloudflare"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        if checks::body_contains(resp, "Sorry, you have been blocked")
            && checks::body_contains(resp, "Cloudflare Ray ID")
            && checks::is_forbidden(resp)
        {
            return true;
        }

        false
    }
}

inventory::submit! {
    &Cloudflare as &dyn Detector
}
