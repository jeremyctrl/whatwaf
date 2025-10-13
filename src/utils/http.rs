#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub content_length: Option<u64>,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

pub fn fetch(url: &str) -> HttpResponse {
    let res = reqwest::blocking::get(url).unwrap();
    let status = res.status().as_u16();
    let content_length = res.content_length();
    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = res.text().unwrap_or_default();

    HttpResponse { 
        status: status, 
        content_length: content_length, 
        headers: headers, 
        body: body 
    }
}