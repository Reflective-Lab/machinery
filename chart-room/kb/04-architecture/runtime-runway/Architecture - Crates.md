---
type: architecture-module
source-path: runtime-runway/crates/
last-scanned: 2026-06-07
scanned-version: 3.4.2
tags: [architecture, runtime-runway]
---

# runtime-runway — Crates

<!-- @generated:start -->

Part of [[Architecture - Overview|runtime-runway]]. Workspace version `3.4.2` at commit `012b81b`. All 11 crates live at `runtime-runway/crates/`.

Crate descriptions are quoted from each `Cargo.toml` `description` field.

## Crate inventory (11)

| Crate | Path | Role |
|---|---|---|
| `api-server` | `crates/api-server` | "Runtime Runway reference Cloud Run server — wires all five runway-* crates" |
| `application` | `crates/application` | "Converge distribution - packages domain packs, providers, and runtime into a deployable product" |
| `llm` | `crates/llm` | "LLM inference and training for Converge agents using Burn and llama.cpp" |
| `runway-accounts` | `crates/runway-accounts` | "Account, organisation, and billing management for the Reflective platform" |
| `runway-app-host` | `crates/runway-app-host` | "Runtime Runway app execution container packet and host bootstrap" |
| `runway-auth` | `crates/runway-auth` | "Firebase Auth token validation and Axum middleware for Runtime Runway backends" |
| `runway-middleware` | `crates/runway-middleware` | "Axum tower middleware stack for all Runtime Runway backends: request-id, health, errors, shutdown" |
| `runway-secrets` | `crates/runway-secrets` | "GCP Secret Manager client with typed fetch and zeroize-on-drop for Runtime Runway services" |
| `runway-storage` | `crates/runway-storage` | "Shared storage abstraction for Runtime Runway apps — local (redb + local FS + fastembed) and remote (Firestore + GCS + Vertex AI)" |
| `runway-storage-contract` | `crates/runway-storage-contract` | "Contract test suite for runway-storage. Asserts equivalence and shape parity across backends." |
| `runway-telemetry` | `crates/runway-telemetry` | "OpenTelemetry + Sentry + structured logging bootstrap for all Runtime Runway services" |

## Composition

- **`api-server`** is the reference wiring binary. It loads `RunwayConfig` from env, projects it into per-crate config structs, and assembles the middleware stack + storage + auth + telemetry + accounts. Other Cloud Run services are expected to wire the same way.
- **`application`** is the CLI/TUI distribution binary (`converge`) and library that packages domain packs and providers from [[../bedrock-platform/Architecture - Converge|Converge]] into a deployable product.
- **`llm`** is the inference/training surface for Converge agents (Burn + llama.cpp). Includes an optional `converge-llm-server` binary gated by the `"server"` feature.
- **`runway-storage`** is the **only** crate hosting both local (`redb` + local FS + `fastembed`) and remote (Firestore + GCS + Vertex AI) backends behind one trait. **`runway-storage-contract`** asserts parity between backends via shared contract tests.
- The four small `runway-*` services (`accounts`, `auth`, `middleware`, `secrets`, `telemetry`) are designed as composable plug-ins for any Runtime Runway HTTP service.

## Config types (constructor-injected)

The crates expose typed config structs that the binary builds and passes in. None of these libraries read env directly:

| Crate | Config struct | Location |
|---|---|---|
| `api-server` | `RunwayConfig` | `crates/api-server/src/config.rs:12` |
| `runway-accounts` | `AccountsConfig` | `crates/runway-accounts/src/config.rs` |
| `runway-telemetry` | `TelemetryConfig` | `crates/runway-telemetry/src/lib.rs` |
| `runway-app-host` | `HostConfig` | `crates/runway-app-host/src/config.rs` |
| `application` | `AppConfig`, `ProviderConfig`, `AuthConfig` | `crates/application/src/config.rs` |
| `llm` | `LlmConfig`, `TokenizerConfig` | `crates/llm/src/config.rs` |

See [[../decisions/2026-05-23-runway-config-injection|ADR — Runway Config Injection]] for the decision and rationale.

## Cross-references

- [[Architecture - Overview|runtime-runway overview]]
- [[Architecture - Ops|runtime-runway ops]]
- [[../decisions/2026-05-23-runway-config-injection|ADR — Runway Config Injection]]

<!-- @generated:end -->
