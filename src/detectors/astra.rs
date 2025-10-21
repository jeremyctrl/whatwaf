use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Astra;

impl Detector for Astra {
    fn name(&self) -> &'static str {
        "Astra"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.body_has(
            &["This website is protected by getastra.com"],
            MatchMode::Any,
        )
    }
}

inventory::submit! {
    &Astra as &dyn Detector
}
