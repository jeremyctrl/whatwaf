use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Cloudfront;

impl Detector for Cloudfront {
    fn name(&self) -> &'static str {
        "Cloudfront"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("x-cache", &["Error from cloudfront"], MatchMode::Any)
    }
}

inventory::submit! {
    &Cloudfront as &dyn Detector
}
