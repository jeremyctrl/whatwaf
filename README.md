# whatwaf

Simple heuristic web application firewall (WAF) detector.

`whatwaf` sends a small set of probe requests to a target site and analyzes the responses for signs of WAF blocking behavior. 

# Usage

```bash
cargo install whatwaf
```

```bash
whatwaf https://example.com
```

# Detections

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
| **Incapsula** | Imperva, Inc. | 🇺🇸 |
| **Janusec** Application Gateway | JANUSEC | 🇺🇳* |
| **Kona Site Defender** | Akamai Technologies | 🇺🇸 |
| **NexusGuard** | NexusGuard Inc. | 🇸🇬 |
| **Sucuri** | Sucuri, Inc | 🇺🇸 |

*Country of Origin is not clearly documented.