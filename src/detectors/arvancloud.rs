use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct ArvanCloud;

impl Detector for ArvanCloud {
    fn name(&self) -> &'static str {
        "ArvanCloud"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("server", &["ArvanCloud"], MatchMode::Any)
    }
}

inventory::submit! {
    &ArvanCloud as &dyn Detector
}
