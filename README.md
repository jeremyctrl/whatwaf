<div align="center">

# whatwaf

<a href="https://crates.io/crates/whatwaf"><img src="https://img.shields.io/crates/v/whatwaf.svg" alt="Crates.io"></a>
<a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>

Heuristic web application firewall (WAF) detector.

</div>

`whatwaf` sends a series of crafted HTTP probe requests to a target site and analyzes the responses for indicators of WAF blocking behavior.  

It detects common commercial and open-source firewalls by matching characteristic response headers, patterns, and bodies.

## How It Works

`whatwaf` performs multiple probes, such as SQL injection, XSS, and local file inclusion (LFI) payloads, and compares the target's HTTP responses against known WAF fingerprints.

Detection is based on:

- HTTP status codes
- Response headers containing WAF vendor signatures
- Response bodies containing diagnostic strings or challenge pages
- Regular-expression matching for vendor-specific phrases

## Installation

Install via [Cargo](https://doc.rust-lang.org/stable/cargo/):

```bash
cargo install whatwaf
```

## Usage

```bash
whatwaf https://example.com
```

### Example

```bash
[*] checking https://example.com
[*] running 4 probes
[*] plain request probe: payload=None
        [-] no detection
[*] xss probe: payload='<script>alert(1)</script>'
        [+] waf=cloudflare status=403
[~] the site https://example.com is behind Cloudflare waf
```

## Detections

| WAF | Vendor | Country of Origin |
| :--- | :------ | :----------------- |
| **ArvanCloud** | Abr Arvan | 🇮🇷 |
| **Astra** | Astra Security | 🇮🇳 |
| **ASPA** | Aspa Engineering Co. | 🇮🇷 |
| **Barracuda** | Barracuda Networks, Inc. | 🇺🇸 |
| **Cloudflare** WAF | Cloudflare, Inc. | 🇺🇸 |
| **Cloudfront** WAF | Amazon Web Services | 🇺🇸 |
| **Datadome** | Datadome | 🇫🇷 |
| **DotDefender** | Applicure Technologies | 🇮🇱 |
| **FortiWeb** | Fortinet, Inc. | 🇺🇸 |
| **Incapsula** | Imperva, Inc. | 🇺🇸 |
| **Janusec** Application Gateway | JANUSEC | 🇺🇳* |
| **Kona Site Defender** | Akamai Technologies | 🇺🇸 |
| **NexusGuard** | NexusGuard, Inc. | 🇸🇬 |
| **SafeLine** | Chaitin Tech | 🇨🇳 |
| **Sucuri** | Sucuri, Inc | 🇺🇸 |
| **Wordfence** | Defiant Inc. | 🇺🇸 |

*Country of Origin is not clearly documented.