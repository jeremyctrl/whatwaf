use crate::utils::http::HttpResponse;

pub trait Detector: Sync + Send {
    fn name(&self) -> &'static str;
    fn detect(&self, resp: &HttpResponse) -> bool;
}

pub mod aspa;
pub mod astra;
pub mod barracuda;
pub mod cloudflare;
pub mod cloudfront;
pub mod datadome;
pub mod dotdefender;
pub mod incapsula;
pub mod janusec;
pub mod kona;
pub mod nexusguard;
pub mod sucuri;
pub mod wordfence;
