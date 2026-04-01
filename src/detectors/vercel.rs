use crate::detectors::Detector;
use crate::utils::checks::MatchMode;
use crate::utils::http::HttpResponse;

pub struct Vercel;

impl Detector for Vercel {
    fn name(&self) -> &'static str {
        "Vercel"
    }

    fn detect(&self, resp: &HttpResponse) -> bool {
        resp.header_has("x-vercel-mitigated", &["deny", "challenge"], MatchMode::Any)
    }
}

inventory::submit! {
    &Vercel as &dyn Detector
}
