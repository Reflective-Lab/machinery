---
tags: [audit]
source: llm
---
# Security Audit Tracker

## What We Check
- Committed secrets (API keys, tokens, passwords) in tracked files and git history
- .env files: tracked vs untracked, real values vs placeholders
- Dependency vulnerabilities (`cargo audit`, `bun audit`)
- Secrets on disk that could leak if repo is copied/shared

## Patterns Scanned
`sk-`, `AKIA`, `ghp_`, `glpat-`, `xoxb-`, `Bearer `, `password=`, `secret=`, `token=`, `api_key=`

## Audit History

### 2026-04-13 — Full Baseline

| Project | Secrets in git | .env tracked | .env on disk | Dep vulnerabilities | Status |
|---------|---------------|-------------|-------------|-------------------|--------|
| converge | Clean | No | Clean | **1 vuln** (rsa, no fix avail) + 13 unmaintained warnings | Pass |
| wolfgang | Clean | No | **WARN** (4 API keys) | N/A (no Cargo.lock at root) | WARN |
| organism | Clean | No | Clean | Clean (0 vulns) | Pass |
| saas-killer | Clean | No | Clean | **1 vuln** (rsa) + 9 warnings | FAIL |
| hackathon | Clean | No | Clean | Clean (0 vulns) | Pass |
| epic-brand | Clean | No | **WARN** (Anthropic key + ES JWT) | **1 vuln** (rand via object_store) | FAIL |
| wykkid-preso | Clean | No | Clean | N/A | Pass |
| moosemen-writer | Clean | No | Clean | N/A | Pass |

**Action items:**
- converge: update wasmtime (5 CVEs), rustls-webpki, object_store, ring, rsa
- saas-killer: update rsa
- epic-brand: update object_store/rand; consider rotating Anthropic key
- wolfgang: .env has 4 real API keys on disk — ensure never committed
- JS dep audit gap: `bun audit` not supported — consider alternative tooling
