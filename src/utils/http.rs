#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub content_length: Option<u64>,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub fn fetch(client: &reqwest::blocking::Client, url: &str) -> Result<HttpResponse, reqwest::Error> {
    let res = client.get(url).send()?;

    let status = res.status().as_u16();
    let content_length = res.content_length();
    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = res.text().unwrap_or_default();

    Ok(HttpResponse {
        status: status,
        content_length: content_length,
        headers: headers,
        body: body,
    })
}

pub fn add_param(original: &str, key: &str, value: &str) -> String {
    if let Ok(mut url) = reqwest::Url::parse(original) {
        url.query_pairs_mut().append_pair(key, value);
        return url.to_string();
    }

    original.to_string()
}
