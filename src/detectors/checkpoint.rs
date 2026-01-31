use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

const ASSET_HASHES: &[&str] = &[
    "ca7a2a4952254b4240c79f15d0e1cdf728705b9c8f5f2734db72579f3e26c32f", // checkpoint logo
];

pub struct CheckPoint;

impl Detector for CheckPoint {
    fn name(&self) -> &'static str {
        "Check Point Application Security"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.asset_hash_is(ASSET_HASHES, MatchMode::Any)
            || resp.body_has(&["Check Point's", "Application Security"], MatchMode::All)
    }
}

inventory::submit! {
    &CheckPoint as &dyn Detector
}
