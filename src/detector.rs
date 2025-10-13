use crate::detectors::Detector;
use crate::utils::http::HttpResponse;

inventory::collect!(&'static dyn Detector);

pub fn run_detectors(resp: &HttpResponse) -> Option<&'static str> {
    for d in inventory::iter::<&'static dyn Detector> {
        if d.detect(resp) {
            return Some(d.name());
        }
    }

    None
}
