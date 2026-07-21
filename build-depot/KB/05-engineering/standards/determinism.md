# Standard: Test determinism

**Property:** `RP-DETERMINISM` in `QUALITY_BACKLOG.md`.

**Status:** Scoping. The property is `Aspired`. This document enumerates
the residual determinism axes, names what is already enforced elsewhere,
and frames the enforcement design choice for the next paired session.
Tracked by `QF-2026-06-07-04`.

## What "deterministic" means here

> *Test outputs do not depend on dev-machine env, network, wall clock,
> or absolute filesystem location.*

A determinism violation is anything that lets the same test, same code,
same Rust toolchain, produce different verdicts on different machines or
at different times. The pattern is the same one `RP-HERMETIC-UNIT`
closed for network: "passes on my machine, fails on yours."

Determinism is not the same as hermeticity. A test can be hermetic
(no I/O outside the sandbox) but non-deterministic (uses unseeded RNG).
A test can be deterministic but non-hermetic (reads a file from disk
that happens to never change). Both properties matter; `RP-HERMETIC-UNIT`
covers the network axis only.

## What is already enforced

Three of the four axes named in the property already have partial
enforcement under sibling properties. The table below names what is
covered today and what remains.

| Axis | Covered by | Residual |
|---|---|---|
| Network | `RP-HERMETIC-UNIT` — clippy `disallowed_methods = "deny"` on `reqwest::Client::new` / `::blocking::Client::new` / `::builder` across all 6 train workspaces | Belt-and-suspenders runtime socket-open assertion — `QF-2026-06-07-03` |
| Env vars | Partial — `RP-HERMETIC-UNIT` catches API-key-reading clients (which read env at construction); broader env reads are uncaught | Anything calling `std::env::vars()` or reading non-API-key env vars at test time |
| Absolute filesystem location | `RP-SNAPSHOT-PORTABLE` — `project-doctor` check 4 greps `.stderr` fixtures for absolute paths; `$CARGO`/`$VERSION` placeholders in trybuild | Runtime assertions on `tempfile::TempDir` paths; `*.snap` insta fixtures not yet scanned |
| Wall clock | **Nothing today.** | All `chrono`/`std::time::SystemTime`/`Instant::now()` callsites exercised by unit tests without a clock-injection seam |
| Iteration ordering | **Nothing today.** | `HashMap`/`HashSet` iteration in assertions; `cargo test` itself uses ordered output but the assertions are at the mercy of the type chosen |
| Unseeded RNG | **Nothing today.** | `rand::random()`, `rand::thread_rng()`, any `getrandom`-backed source in test paths |

The first three rows are existing properties' jurisdiction. This
standard scopes the remaining three.

## Enumerated residual axes

### Wall clock

Production code that reads `chrono::Utc::now()`, `chrono::Local::now()`,
`std::time::SystemTime::now()`, or `std::time::Instant::now()` and is
exercised by unit tests without a clock-injection seam will silently
change behaviour at:

- midnight UTC (date rollover in assertions);
- daylight-saving transitions (Local clocks);
- year boundaries (test data ageing past the boundary);
- the moment a test data fixture is older than a threshold the code
  computes from "now."

The pattern is "passes for a window, then starts failing." Catching it
requires either making the clock injectable (constructor accepts a
`fn() -> DateTime<Utc>` or `Clock` trait object) or making the lint
catch the call.

### Iteration ordering

`std::collections::HashMap` and `HashSet` are intentionally
randomly-seeded per process to defend against algorithmic complexity
attacks. Their iteration order is **deliberately unpredictable**. Test
assertions like:

```rust
let expected = vec!["a", "b", "c"];
let got: Vec<_> = my_map.into_iter().map(|(k, _)| k).collect();
assert_eq!(expected, got);
```

…pass on the developer's machine more often than not, and fail on CI
once in a while. The fix is structural: use `BTreeMap` / `BTreeSet`
when the assertion cares about order, or sort the collected `Vec`
before asserting. The lint surface candidate: flag
`HashMap`/`HashSet`-typed values directly compared (or `into_iter()`'d
into a `Vec` and compared) inside `#[test]` paths.

### Unseeded RNG

`rand::random()` and `rand::thread_rng()` are seeded from the OS at
test time. A test that asserts on the output of an RNG-driven function
without injecting a deterministic seed is non-deterministic by
construction. Production code rarely needs unseeded RNG; tests need it
even less.

Two acceptable shapes:

```rust
// Inject a seedable RNG at the boundary.
fn shuffle_with<R: Rng>(items: &mut Vec<T>, rng: &mut R) { ... }

// Or use a deterministic seed in tests.
let mut rng = StdRng::seed_from_u64(42);
shuffle_with(&mut items, &mut rng);
```

The lint surface candidate: deny `rand::random` and `rand::thread_rng`
in test paths via clippy `disallowed_methods`. Production code that
legitimately needs OS entropy keeps an `#[allow]` + comment, same
shape as `RP-HERMETIC-UNIT`.

## Enforcement design — the trade-off

Three plausible enforcement strategies, each with characteristic
strengths and costs:

| Strategy | Catches | Cost |
|---|---|---|
| Clippy `disallowed_methods` | Static calls to specific functions: `SystemTime::now`, `chrono::Utc::now`, `rand::random`, `rand::thread_rng`, `std::env::vars` | Noisy in production code that legitimately reads the wall clock or env; requires `#[allow]` + justification at every callsite. Same playbook as `RP-HERMETIC-UNIT`. |
| AST-walking `xtask` | Patterns the lint can't express: `HashMap`/`HashSet`-typed values in assertions, `into_iter()` over unordered collections, `tempfile::TempDir.path()` interpolated into assertion strings | Bespoke maintenance burden; one `xtask` per pattern; works only at workspace build time. |
| Convention only | Philosophical violations: test independence (no shared mutable state between tests), test order non-reliance, etc. | Reviewer-enforced; no mechanical check; relies on the standard being read at PR time. |

A pragmatic close would mix all three: clippy for the easy axes (wall
clock, RNG, broad env reads), `xtask` for one or two high-value AST
patterns (HashMap iteration in `#[test]`), convention for the rest.
But the specific combination — and which axes are worth the operator
friction of `#[allow]` annotations across the platform — is the
design call this scoping doc defers.

## Enforcement decisions (2026-06-08, QF-2026-06-08-01)

The paired-call decisions on the design questions framed above:

| Axis | Mechanism | Where | Cost |
|---|---|---|---|
| Wall clock | Clippy `disallowed_methods = "deny"` on `std::time::SystemTime::now`, `std::time::Instant::now`, `chrono::Utc::now`, `chrono::Local::now` | All 6 train workspaces eventually | M — expect ~20-50 `#[allow]` rollouts across the platform |
| RNG | Clippy `disallowed_methods = "deny"` on `rand::random`, `rand::thread_rng` | All 6 train workspaces eventually | S — expect ~5-10 `#[allow]` rollouts |
| Broad env | Clippy `disallowed_methods = "deny"` on `std::env::vars` | All 6 train workspaces eventually | S — few callsites; reading specific keys via `std::env::var("FOO")` stays allowed |
| `HashMap`/`HashSet` iteration in assertions | **Convention only** | Reviewer-enforced at PR time | Free; AST detection in `#[test]` context is high-false-positive |
| Test independence (no shared mutable state, no order reliance) | **Convention only** | Reviewer-enforced at PR time | Free; mechanical detection is genuinely hard |
| `TempDir` paths in assertion strings | Deferred (overlaps with `RP-SNAPSHOT-PORTABLE`; revisit if observed in the wild) | — | — |

### Production code patterns

When the clippy deny lands, production code that legitimately needs a
disallowed call annotates `#[allow(clippy::disallowed_methods)]` plus a
justification comment, same playbook as `RP-HERMETIC-UNIT`. The
expected callsite shapes:

- **Wall clock** — log timestamping (every tracing/log integration),
  cache TTLs, rate-limiter state. These are not test-exercised paths
  in a way that affects assertions; `#[allow]` is the right call.
- **OS-entropy RNG** — key/secret generation, opaque ID minting. These
  intentionally need OS entropy; `#[allow]` with a "this is OS
  entropy, not test-noise" justification.
- **Broad env** — diagnostics like `--show-env` CLI flags. Rare;
  `#[allow]` is fine.

Test code that needs a clock or randomness uses dependency injection,
NOT `#[allow]`:

```rust
// Production
pub struct Service<C: Clock = SystemClock> {
    clock: C,
}

impl Service {
    pub fn new() -> Self { Self { clock: SystemClock } }
    pub fn with_clock<C: Clock>(clock: C) -> Service<C> { Service { clock } }
}

// Test
let fixed_clock = FixedClock::at("2026-06-08T12:00:00Z");
let service = Service::with_clock(fixed_clock);
assert_eq!(service.next_due(), ...);  // deterministic
```

### Pilot — prism-analytics

`mosaic-extensions/prism-analytics/clippy.toml` extended with the
deny list 2026-06-08. Five existing wall-clock callsites surfaced in
test code (in `engine.rs`): one tempdir-uniqueness `SystemTime::now()`,
four `Instant::now()` benchmark timing instrumentations inside
`#[ignore]`'d benchmarks. All five annotated with
`#[allow(clippy::disallowed_methods)]` + per-function justification
comments. `cargo clippy --workspace --all-targets` green.

Choice of pilot workspace: prism-analytics had **zero** baseline for
the prior `RP-HERMETIC-UNIT` rollout (no `reqwest::Client::new`
callsites in src/), making it the cleanest place to validate the
determinism deny list shape without the workspace's own
`RP-HERMETIC-UNIT` rollout cost biasing the signal.

### Cross-train rollout

The remaining 5 train workspaces — converge, axiom, organism, helms,
runtime-runway, commerce-rails — get the deny list as a follow-up.
Tracked by `QF-2026-06-08-06`. Per-workspace migration calls mirror
the `RP-HERMETIC-UNIT` rollout (`QF-2026-06-02-05`).

## What this scoping is NOT

- It is not the enforcement implementation. That is the next paired
  session.
- It is not a commitment to enforce all six axes. Some may stay
  convention-only forever (test independence) if the mechanical cost
  is high.
- It is not a critique of any specific existing test. Today's tests
  may be entirely deterministic by accident; the property is about
  the surface, not the current state.

## Cross-references

- `QUALITY_BACKLOG.md` — `RP-DETERMINISM`, `QF-2026-06-07-04`.
- `KB/05-engineering/standards/hermetic-unit-tests.md` — the
  `RP-HERMETIC-UNIT` close that proved the playbook (lint + DI seam +
  per-workspace migration).
- `KB/06-operations/factory-health.md` — the `just doctor` /
  `quality-doctor` / `project-doctor` recipes that would gain a check
  if the lint route is chosen.
- `AGENTS.md > Test/code attribution` — relevant when a determinism
  fix is mis-classified as "the test was wrong" rather than "the
  surface needed a clock seam."
