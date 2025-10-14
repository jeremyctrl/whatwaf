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