use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct SafeLine;

impl Detector for SafeLine {
    fn name(&self) -> &'static str {
        "SafeLine"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(&["slg-caption", "slg-text", "slg-desc"], MatchMode::All) ||
            resp.header_has("set-cookie", &["sl-session"], MatchMode::Any)
    }
}

inventory::submit! {
    &SafeLine as &dyn Detector
}
