use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Sucuri;

impl Detector for Sucuri {
    fn name(&self) -> &'static str {
        "Sucuri"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        (resp.has_header(&["x-sucuri-block"], MatchMode::Any)
            || resp.body_has(&["Sucuri Website Firewall"], MatchMode::Any))
            && resp.is_forbidden()
    }
}

inventory::submit! {
    &Sucuri as &dyn Detector
}
