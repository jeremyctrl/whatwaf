use crate::utils::http::HttpResponse;

pub trait Detector: Sync + Send {
    fn name(&self) -> &'static str;
    fn detect(&self, resp: &HttpResponse) -> bool;
}