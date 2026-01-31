use once_cell::sync::Lazy;
use regex::Regex;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Asset {
    pub mime: String,
    pub hash: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub content_length: Option<u64>,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub assets: Vec<Asset>,
}

static SVG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?is)<svg.*?</svg>").unwrap());

pub fn fetch(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<HttpResponse, reqwest::Error> {
    let res = client.get(url).send()?;

    let status = res.status().as_u16();
    let content_length = res.content_length();
    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = res.text().unwrap_or_default();
    let assets = extract_svg_assets(&body);

    Ok(HttpResponse {
        status,
        content_length,
        headers,
        body,
        assets,
    })
}

pub fn add_param(original: &str, key: &str, value: &str) -> String {
    if let Ok(mut url) = reqwest::Url::parse(original) {
        url.query_pairs_mut().append_pair(key, value);
        return url.to_string();
    }

    original.to_string()
}

pub fn extract_svg_assets(body: &str) -> Vec<Asset> {
    SVG_RE
        .find_iter(body)
        .map(|m| {
            let bytes = m.as_str().as_bytes();

            Asset {
                mime: "image/svg+xml".into(),
                hash: sha256_hex(bytes),
                size: bytes.len(),
            }
        })
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
