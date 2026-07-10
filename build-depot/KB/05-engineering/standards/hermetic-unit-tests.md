# Standard: Hermetic unit tests

**Property:** `RP-HERMETIC-UNIT` — *Unit tests issue zero outbound network
requests.* (Reflective `QUALITY_BACKLOG.md`)

**Motivating finding:** `QF-2026-06-02-05` — `axiom-truth` v0.15.1 shipped a
`guide_heading` that read `ChatBackendSelectionConfig::from_env()` inside
`select_backend`, which then probed live LLM providers via real
`reqwest` calls during `cargo test`. The unit test asserted local-only
fallback but actually returned the live result whenever the developer
had `OPENAI_API_KEY` etc. set. Cost leak, non-determinism, and
credential exposure all at once.

**Pilot:** `bedrock-platform/organism/crates/intelligence` (2026-06-07).
`MistralOcrProvider` migrated; clippy `disallowed_methods` lint
configured at organism workspace level. DeepSeek, LightOn, vision/, and
web.rs are the next per-provider migrations.

## The pattern

For every constructor that builds an HTTP client (or any I/O resource)
internally, expose **two** entry points:

```rust
impl Provider {
    /// Convenience constructor — builds the default HTTP client. Use
    /// this in production paths where the caller doesn't need DI.
    #[must_use]
    #[allow(clippy::disallowed_methods)]
    // `clippy::disallowed_methods` (RP-HERMETIC-UNIT / QF-2026-06-02-05)
    // intentionally allowed here: production callers who don't need DI
    // use this path. Tests use `with_http_client` instead.
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self::with_http_client(reqwest::blocking::Client::new(), api_key, model)
    }

    /// Dependency-injection constructor — hermetic-test entry point.
    /// Production code that needs custom timeouts / proxies / TLS roots
    /// also uses this.
    #[must_use]
    pub fn with_http_client(
        client: reqwest::blocking::Client,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self { /* … */ client, /* … */ }
    }

    /// Reads env vars + calls `Self::new(...)`. Convenience for binaries
    /// that load configuration from the environment.
    pub fn from_env() -> Result<Self, Error> {
        let api_key = std::env::var("PROVIDER_API_KEY")?;
        Ok(Self::new(api_key, "default-model"))
    }
}
```

### Hermetic test shape

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_constructs_hermetically_via_di() {
        // A bare reqwest::Client doesn't open a socket until a request
        // is sent. The point is the *constructor* doesn't bake in
        // environment lookup or implicit defaults the test author
        // can't see.
        let stub_client = reqwest::blocking::Client::builder()
            .no_proxy()
            .build()
            .expect("stub client builds without env or network");
        let provider = Provider::with_http_client(stub_client, "fake-key", "model");
        assert_eq!(provider.name(), "provider");
    }

    // For tests that exercise the actual request path, wire `stub_client`
    // to a `wiremock` server and assert against recorded requests —
    // never against the live API.
}
```

## Enforcement layers

Two complementary checks:

### 1. Dependency injection (structural, primary)

Every `from_env()` or default constructor that builds a network client
must delegate to a `with_http_client(...)` (or `with_<resource>_*`) DI
constructor. Production callers that don't care use `new`/`from_env`;
test authors must explicitly thread a client.

Rejected alternatives:
- **Per-test sandbox** (`#[hermetic_test]` macro running each test in
  `unshare -n`) — Linux-only; macOS dev experience would be
  inconsistent.
- **Capability-based stdlib** (`cap-std`) — too invasive; touches every
  I/O site in production code.
- **Convention only** — too loose; QF-2026-06-02-05 happened with the
  convention in place.

### 2. Clippy `disallowed_methods` lint (mechanical, belt-and-suspenders)

`clippy.toml` at the workspace root lists the network constructors that
must not be called outside DI-aware contexts:

```toml
disallowed-methods = [
    { path = "reqwest::Client::new", reason = "use with_http_client(...) — RP-HERMETIC-UNIT / QF-2026-06-02-05" },
    { path = "reqwest::blocking::Client::new", reason = "use with_http_client(...) — RP-HERMETIC-UNIT / QF-2026-06-02-05" },
]
```

`[workspace.lints.clippy]` activates the lint at `warn` during
migration. Each unmigrated callsite is a visible warning in
`cargo clippy` — that's the per-workspace TODO list. Promote to `deny`
once every callsite in the workspace is either migrated or annotated
with `#[allow(clippy::disallowed_methods)]` + a justification comment.

## Migration order across the train

Per the QF-2026-06-02-05 next-action:

1. **axiom** — already migrated (v0.15.2; `guide_heading`).
2. **organism/intelligence** — in progress as of 2026-06-07:
   - `MistralOcrProvider`: migrated (pilot).
   - `DeepSeekOcrProvider`, `LightOnOcrProvider`: visible warnings; same
     pattern as Mistral.
   - `vision/backends.rs` (Anthropic / GPT-4o / Gemini / Pixtral): same
     pattern.
   - `web.rs` and `receipt.rs`: same pattern.
3. **helms** — ingest paths.
4. **runtime-runway** — the largest surface (10 reqwest callsites; 10
   from_env callsites). Production code legitimately does I/O in many
   places (Stripe, GCP, Firebase); the migration is *about test
   hermeticity*, not removing the I/O.
5. **prism-analytics** (mosaic-extensions) — LLM hooks.
6. **commerce-rails** — Stripe webhook ingress (one callsite).

`converge` has 5 `from_env` + 1 reqwest callsite — those are kernel
infrastructure (`runtime`, `kernel`); migration should be reviewed
against Converge's authority-boundary constraints before changes land.

## When the lint goes `deny`

Promote `disallowed_methods = "warn"` → `"deny"` in
`[workspace.lints.clippy]` after:

- Every constructor that touches `reqwest`/`tokio::net` has a
  `with_*_client` DI entry point.
- Every test that exercises a network-touching code path constructs
  via DI with a stub.
- The number of `#[allow(clippy::disallowed_methods)]` annotations is
  bounded and audited (each one cites the convenience-default
  justification per the pattern above).

At that point, `RP-HERMETIC-UNIT` can flip to `Enforced` in
`QUALITY_BACKLOG.md`.

## Runtime audit layer (defence in depth)

The clippy `disallowed_methods` lint covers HTTP-client *construction*.
A sufficiently determined regression — code that opens a raw TCP socket
directly via `std::net::TcpStream` or `tokio::net::TcpStream`, or some
transitive dependency that does the same from inside a "stub" client —
would bypass the lint. The class is rare (the finding's confidence
estimate was "~once a year"), but real.

`.github/workflows/hermetic-audit.yml` (shipped 2026-06-08 under
`QF-2026-06-07-03`) is the runtime defence:

- Scheduled weekly + `workflow_dispatch`.
- Checks out all train-relevant repos (same set as `fresh-clone.yml`).
- Pre-fetches dependencies (network required for the cache warmup).
- Runs `unshare -rn cargo test --workspace --all-targets --offline`
  per workspace: the kernel-level network namespace has no
  connectivity beyond loopback, so any test that opens a TCP socket
  gets `EADDRNOTAVAIL` / `ENETUNREACH` and fails.

Why audit-only and not a `#[hermetic_test]` proc-macro, `cap-std`
adoption, or `LD_PRELOAD` socket interceptor: those paths cost orders
of magnitude more engineering (proc-macro maintenance, full
capability-passing migration, or custom-binary integration). The
audit-only path is one workflow file, weekly cost is one CI run,
failures surface in the Actions UI. When (if) the lint-bypass class
becomes observable, the heavier paths get a real case behind them.

The audit covers the same set as the lint — converge, axiom, organism,
helms, prism-analytics, runtime-runway, commerce-rails. macOS dev
environment is intentionally unprotected at runtime; the clippy lint
runs there at edit time, which is the same coverage real developers
have for the construction layer.

## Cross-references

- `QUALITY_BACKLOG.md`:
  - `RP-HERMETIC-UNIT` (the property)
  - `QF-2026-06-02-05` (the open finding — In progress)
- Pilot:
  - `bedrock-platform/organism/clippy.toml`
  - `bedrock-platform/organism/Cargo.toml` (`[workspace.lints.clippy]`)
  - `bedrock-platform/organism/crates/intelligence/src/ocr/cloud.rs`
    (`MistralOcrProvider::with_http_client`)
- Precedent: `axiom` v0.15.2 (`guide_heading` migration), see
  `KB/release-history.md` → `axiom-truth v0.15.1` entry.
