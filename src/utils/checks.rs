use crate::utils::http::HttpResponse;
use regex::Regex;

pub enum MatchMode {
    Any,
    All,
}

impl HttpResponse {
    // headers
    pub fn has_header(&self, keys: &[&str], mode: MatchMode) -> bool {
        match mode {
            MatchMode::Any => self
                .headers
                .iter()
                .any(|(k, _)| keys.iter().any(|key| k.eq_ignore_ascii_case(key))),
            MatchMode::All => keys.iter().all(|key| {
                self.headers
                    .iter()
                    .any(|(k, _)| k.eq_ignore_ascii_case(key))
            }),
        }
    }

    pub fn header_has(&self, key: &str, texts: &[&str], mode: MatchMode) -> bool {
        self.headers.iter().any(|(k, v)| {
            if !k.eq_ignore_ascii_case(key) {
                return false;
            }

            let val = v.to_lowercase();
            match mode {
                MatchMode::Any => texts.iter().any(|t| val.contains(&t.to_lowercase())),
                MatchMode::All => texts.iter().all(|t| val.contains(&t.to_lowercase())),
            }
        })
    }

    pub fn header_matches(&self, key: &str, patterns: &[Regex], mode: MatchMode) -> bool {
        self.headers.iter().any(|(k, v)| {
            if !k.eq_ignore_ascii_case(key) {
                return false;
            }

            match mode {
                MatchMode::Any => patterns.iter().any(|re| re.is_match(v)),
                MatchMode::All => patterns.iter().all(|re| re.is_match(v)),
            }
        })
    }

    pub fn headers_match(&self, patterns: &[Regex]) -> bool {
        self.headers
            .iter()
            .any(|(k, v)| patterns.iter().any(|re| re.is_match(k) || re.is_match(v)))
    }

    // body
    pub fn body_has(&self, texts: &[&str], mode: MatchMode) -> bool {
        let body_lc = self.body.to_lowercase();
        match mode {
            MatchMode::Any => texts.iter().any(|t| body_lc.contains(&t.to_lowercase())),
            MatchMode::All => texts.iter().all(|t| body_lc.contains(&t.to_lowercase())),
        }
    }

    pub fn body_matches(&self, patterns: &[Regex], mode: MatchMode) -> bool {
        match mode {
            MatchMode::Any => patterns.iter().any(|re| re.is_match(&self.body)),
            MatchMode::All => patterns.iter().all(|re| re.is_match(&self.body)),
        }
    }

    // response
    pub fn is_forbidden(&self) -> bool {
        self.status == 403
    }

    pub fn is_not_found(&self) -> bool {
        self.status == 404
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_error(&self) -> bool {
        (400..600).contains(&self.status)
    }

    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status)
    }

    pub fn is_empty(&self) -> bool {
        self.body.trim().is_empty() && self.content_length.unwrap_or(0) == 0
    }

    // assets
    pub fn has_asset(&self) -> bool {
        !self.assets.is_empty()
    }

    pub fn asset_hash_has(&self, texts: &[&str], mode: MatchMode) -> bool {
        self.assets.iter().any(|a| match mode {
            MatchMode::Any => texts.iter().any(|t| a.hash.contains(t)),
            MatchMode::All => texts
                .iter()
                .all(|t| self.assets.iter().any(|a| a.hash.contains(t))),
        })
    }

    pub fn asset_hash_is(&self, hashes: &[&str], mode: MatchMode) -> bool {
        match mode {
            MatchMode::Any => self
                .assets
                .iter()
                .any(|a| hashes.iter().any(|h| a.hash.eq_ignore_ascii_case(h))),

            MatchMode::All => hashes
                .iter()
                .all(|h| self.assets.iter().any(|a| a.hash.eq_ignore_ascii_case(h))),
        }
    }
}
