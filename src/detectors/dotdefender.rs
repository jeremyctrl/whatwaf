use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct DotDefender;

impl Detector for DotDefender {
    fn name(&self) -> &'static str {
        "DotDefender"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["dotDefender Blocked Your Request"], MatchMode::Any)
    }
}

inventory::submit! {
    &DotDefender as &dyn Detector
}
