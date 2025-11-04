use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Zenedge;

impl Detector for Zenedge {
    fn name(&self) -> &'static str {
        "Zenedge"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.has_header(&["X-Zen-Fury"], MatchMode::Any)
            && resp.header_has("Server", &["ZENEDGE"], MatchMode::Any)
    }
}

inventory::submit! {
    &Zenedge as &dyn Detector
}
