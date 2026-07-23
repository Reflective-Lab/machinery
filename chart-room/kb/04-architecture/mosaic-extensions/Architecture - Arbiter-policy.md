---
type: architecture-module
source-path: mosaic-extensions/arbiter-policy/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, arbiter-policy]
---

# Arbiter Policy — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Cedar-backed authorization gates for Converge formations."*
> — `mosaic-extensions/arbiter-policy/README.md:14-19`

## Member crates (1)

Single crate `crates/arbiter`. Path: `mosaic-extensions/arbiter-policy/crates/arbiter/`.

## Key dependencies

From `arbiter-policy/crates/arbiter/Cargo.toml:29-50`: `converge-core`, `converge-pack`, `cedar-policy` 4.10, `cedar-policy-symcc` 0.5 (symbolic compiler), `axum`, `tokio`, `ed25519-dalek` (signed delegation tokens).

Cedar is the policy engine; `cedar-policy-symcc` is its symbolic-compilation surface (compiles Cedar policies symbolically for analysis). The `axum` + `ed25519-dalek` pair signals an HTTP service that issues and verifies signed delegation tokens.

## Role

Authorization gate **and** delegation token issuer. Arbiter answers "is this allowed under these signed credentials and this Cedar policy?" for a formation — and produces audit-shaped signed tokens that downstream actors can verify offline.

## Boundary

Owns: authorization decisions + delegation token issuance via Cedar.
Does NOT own: admission/promotion (→ [[../bedrock-platform/Architecture - Converge|Converge]]), SMT-level invariants (→ [[Architecture - Soter-smt|Soter]]).

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Soter-smt|Soter SMT]] — sibling assurance, SMT-style

<!-- @generated:end -->
