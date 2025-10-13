use crate::utils::http::HttpResponse;

// headers
pub fn has_header(resp: &HttpResponse, key: &str) -> bool {
    resp.headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(key))
}

pub fn header_contains(resp: &HttpResponse, key: &str, value: &str) -> bool {
    resp.headers.iter().any(|(k, v)| {
        k.eq_ignore_ascii_case(key) && v.to_lowercase().contains(&value.to_lowercase())
    })
}

pub fn header_matches_regex(resp: &HttpResponse, key: &str, pattern: &regex::Regex) -> bool {
    resp.headers.iter().any(|(k, v)| {
        k.eq_ignore_ascii_case(key) && pattern.is_match(v)
    })
}

// body
pub fn body_contains(resp: &HttpResponse, text: &str) -> bool {
    resp.body.to_lowercase().contains(&text.to_lowercase())
}

pub fn body_matches_regex(resp: &HttpResponse, pattern: &regex::Regex) -> bool {
    pattern.is_match(&resp.body)
}

// response
pub fn is_forbidden(resp: &HttpResponse) -> bool {
    resp.status == 403
}

pub fn is_not_found(resp: &HttpResponse) -> bool {
    resp.status == 404
}

pub fn is_success(resp: &HttpResponse) -> bool {
    (200..300).contains(&resp.status)
}

pub fn is_error(resp: &HttpResponse) -> bool {
    (400..600).contains(&resp.status)
}

pub fn is_redirect(resp: &HttpResponse) -> bool {
    (300..400).contains(&resp.status)
}

pub fn is_empty(resp: &HttpResponse) -> bool {
    resp.body.trim().is_empty() && resp.content_length.unwrap_or(0) == 0
}
