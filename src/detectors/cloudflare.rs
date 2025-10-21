use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Cloudflare;

impl Detector for Cloudflare {
    fn name(&self) -> &'static str {
        "Cloudflare"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(
            &["Sorry, you have been blocked", "Cloudflare Ray ID"],
            MatchMode::All,
        ) && resp.is_forbidden()
    }
}

inventory::submit! {
    &Cloudflare as &dyn Detector
}
