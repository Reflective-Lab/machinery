# Mobile Dev Setup Script — Design

**Date:** 2026-06-23
**Script location:** `reflective/scripts/setup-mobile-dev.sh`

## Purpose

A standalone shell script that clones the exact nested repo structure needed for mobile development onto a new machine. Run once on a fresh machine; SSH key access to `Reflective-Lab` and `kpernyer` GitHub orgs is a prerequisite.

## Repo Structure Cloned

```
~/dev/reflective/                          ← Reflective-Lab/reflective (skipped if already exists)
  marquee-apps/                            ← Reflective-Lab/marquee-apps
    marquee-apps/quorum-sense/             ← Reflective-Lab/quorum-sense
    marquee-apps/atlas-integration/        ← Reflective-Lab/atlas-integration
    marquee-apps/vouch-lending/            ← Reflective-Lab/vouch-lending
  bedrock-platform/                        ← Reflective-Lab/bedrock-platform
    bedrock-platform/axiom/               ← Reflective-Lab/axiom
    bedrock-platform/converge/            ← Reflective-Lab/converge
    bedrock-platform/organism/            ← Reflective-Lab/organism
    bedrock-platform/helms/               ← Reflective-Lab/helms
  runtime-runway/                          ← Reflective-Lab/runtime-runway
  commerce-rails/                          ← Reflective-Lab/commerce-rails
  mosaic-extensions/                       ← Reflective-Lab/mosaic-extensions
```

All repos cloned on `main`.

## Behaviour

- `set -euo pipefail` — fails immediately and loudly on any error
- Checks if `~/dev/reflective` exists; clones root only if missing
- Clones sub-repos sequentially, printing a status line before each
- Does not install toolchains, configure SSH, or set up direnv — git structure only
- Safe to inspect which step failed because output is sequential and labelled

## Non-Goals

- Parallel cloning
- Idempotent re-runs (one-time setup script)
- Toolchain installation (Rust, Node, etc.)
- SSH key setup
