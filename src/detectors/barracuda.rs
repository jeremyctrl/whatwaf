use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Barracuda;

impl Detector for Barracuda {
    fn name(&self) -> &'static str {
        "Barracuda"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["Barracuda Networks"], MatchMode::Any) && resp.is_not_found()
    }
}

inventory::submit! {
    &Barracuda as &dyn Detector
}
