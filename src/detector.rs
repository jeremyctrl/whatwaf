use crate::detectors::Detector;
use crate::utils::http::HttpResponse;

inventory::collect!(&'static dyn Detector);

pub fn run_detectors(resp: &HttpResponse) -> Option<Vec<&'static str>> {
    let mut matches = Vec::new();
    for d in inventory::iter::<&'static dyn Detector> {
        if d.detect(resp) {
            matches.push(d.name());
        }
    }

    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}
