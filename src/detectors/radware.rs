use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Radware;

impl Detector for Radware {
    fn name(&self) -> &'static str {
        "Radware"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.has_header(&["rdwr_response"], MatchMode::Any)
            || resp.header_has("server", &["rdwr"], MatchMode::Any)
    }
}

inventory::submit! {
    &Radware as &dyn Detector
}
