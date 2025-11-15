use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct FrontDoor;

impl Detector for FrontDoor {
    fn name(&self) -> &'static str {
        "Azure Front Door"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["afd_azwaf_tok"], MatchMode::Any)
    }
}

inventory::submit! {
    &FrontDoor as &dyn Detector
}
