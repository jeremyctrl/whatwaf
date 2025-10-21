use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct ASPA;

impl Detector for ASPA {
    fn name(&self) -> &'static str {
        "ASPA"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("server", &["ASPA-WAF"], MatchMode::Any)
    }
}

inventory::submit! {
    &ASPA as &dyn Detector
}
