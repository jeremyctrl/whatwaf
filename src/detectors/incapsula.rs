use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Incapsula;

impl Detector for Incapsula {
    fn name(&self) -> &'static str {
        "Incapsula"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("set-cookie", &["incap_ses", "visid_incap"], MatchMode::Any)
    }
}

inventory::submit! {
    &Incapsula as &dyn Detector
}
