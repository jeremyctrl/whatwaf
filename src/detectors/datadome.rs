use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Datadome;

impl Detector for Datadome {
    fn name(&self) -> &'static str {
        "Datadome"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.has_header(&["x-datadome"], MatchMode::Any)
    }
}

inventory::submit! {
    &Datadome as &dyn Detector
}
