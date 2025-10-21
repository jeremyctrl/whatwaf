use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Kona;

impl Detector for Kona {
    fn name(&self) -> &'static str {
        "Kona Site Defender"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["edgesuite"], MatchMode::Any) && resp.is_forbidden()
            || resp.has_header(&["x-reference-error"], MatchMode::Any)
            || resp.header_has("server", &["AkamiGHost"], MatchMode::Any)
    }
}

inventory::submit! {
    &Kona as &dyn Detector
}
