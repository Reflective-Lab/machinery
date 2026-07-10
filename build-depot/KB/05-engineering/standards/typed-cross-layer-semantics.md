# Standard: Typed cross-layer semantics

**Property:** `RP-TYPED-CROSS-LAYER-SEMANTICS` in `QUALITY_BACKLOG.md`.

**Status:** Aspired (this declaration). Convention-enforced via the Boundary
Checklist in `KB/04-architecture/runtime-injection-boundaries.md`;
mechanical per-crate lint pilot tracked by `QF-2026-06-08-02`.

## The rule

> *A string that carries semantics — a closed set, a bounded number, a
> typed actor, a typed source, a typed route owner, a typed entitlement,
> or a typed event — must be a Rust type before it crosses a layer
> boundary. Stop and add the type before wiring the boundary.*

Source: question 6 of the Boundary Checklist in
`KB/04-architecture/runtime-injection-boundaries.md`.

Layer boundaries in this architecture are not abstract. They are the
specific interfaces between:

- App ↔ Runway (runtime / capability surface)
- App ↔ Commerce Rails (commercial authority)
- App ↔ Helm (operator trust transfer)
- App ↔ Converge (governed reasoning)
- Helm ↔ Converge / Organism (capabilities, truths, intelligence)
- Converge ↔ extension packages (Mosaic, Atelier, etc.)

At each of these, a `&str` or `String` parameter that names "an actor",
"a route owner", "an entitlement key", or "an event kind" is a slow
boundary erosion. Today's `String` is tomorrow's bug where the caller
passes `"orgnaism"` (note the typo) and the call site happily forwards
it through three layers before the consumer rejects it.

## What this property catches

The pattern is concrete: any boundary signature whose parameter is
`&str` / `String` / `Vec<String>` when the intended domain is actually
a finite, named set.

Examples from this codebase's history:

```rust
// Smell — the route_owner is one of a finite set; this signature accepts
// anything.
fn dispatch(route_owner: &str, payload: Bytes) -> Result<...> { ... }

// Better — the closed set is in the type.
enum RouteOwner { AppDomain, HelmModule, Suggestor }
fn dispatch(route_owner: RouteOwner, payload: Bytes) -> Result<...> { ... }
```

```rust
// Smell — entitlement is named at the boundary as a string.
fn check_entitlement(user: UserId, entitlement: &str) -> bool { ... }

// Better — entitlement is one of a known list.
fn check_entitlement(user: UserId, entitlement: Entitlement) -> bool { ... }
```

```rust
// Smell — event kind is a string at the boundary; downstream switches on it.
fn publish(event_kind: &str, event_body: Value) -> Result<...> { ... }

// Better — event kind is a typed enum or a sealed trait, and event_body's
// shape depends on it.
fn publish(event: TypedEvent) -> Result<...> { ... }
```

## What this property does NOT catch

- Strings carrying genuinely-free-form data: human prose, log messages,
  identifiers from external systems (UUIDs, JWTs, API tokens).
- Strings inside the implementation of a layer — only signatures
  crossing layer boundaries are in scope.
- Strings in test fixtures or example data — these don't cross runtime
  boundaries.

The rule is a boundary-discipline rule, not an "all strings must be
types" rule. Distinguishing the two is the judgement question every
boundary review must answer.

## Where the boundaries are

| Layer A | Layer B | Boundary surface |
|---|---|---|
| Application | Runtime Runway | `runway.app.json` schema, capability constructors |
| Application | Commerce Rails | `commerce_rails::*` ports, `commerce-rails-stripe` adapters |
| Application | Helm | `HelmModule` trait, `helm-truth-execution` |
| Application | Converge | `converge_*` SDK crates' public APIs |
| Helm | Converge / Organism | Capability + truth boundary; `capability_core::Capability` |
| Converge | Extensions | `converge_*` extension trait surfaces (Atelier, Mosaic-* ) |

A new public function or constructor on any of these surfaces is a
boundary signature. Its parameter types are this standard's concern.

## Enforcement

Three reinforcing layers, all convention-based:

1. **Standard doc** — this page. Defines the rule, the boundary
   surfaces, what's in scope, what's not. Reviewers cite it at
   boundary-touching PRs.
2. **Boundary Checklist** —
   `KB/04-architecture/runtime-injection-boundaries.md`, question 6.
   The original source of the rule; this standard expands its
   examples.
3. **PR template** — `.github/PULL_REQUEST_TEMPLATE.md` includes the
   typed-cross-layer-semantics checklist item. Every PR author sees it
   at the moment of opening; the checkbox prompts a deliberate
   read-the-standard moment for boundary-touching changes.

### Why not a mechanical lint

`QF-2026-06-08-02` (closed 2026-06-08) considered a `#[boundary]`
marker macro + grep check, a full `dylint` extension, and a composite
GitHub Action. All three were rejected in favour of the convention-
only path. The reasoning:

- **Boundary signatures are added rarely** — a handful per quarter,
  not per day. Mechanical-lint-at-PR-time value scales with frequency;
  for low-frequency events, reviewer attention dominates.
- **The hard problem isn't mechanical** — identifying "which functions
  are boundaries" requires reviewer judgment regardless. Either every
  boundary needs a human-applied marker (then humans are doing the
  hard part anyway) or the lint uses heuristics like `pub fn` in
  `runway-*` that will misfire on legitimate non-boundary internals.
- **The standard already exists** — adding mechanical enforcement on
  top would have added engineering and false-positive risk without
  much marginal value over the standard doc + PR template.

If a real boundary-typing failure mode is observed in the wild (e.g.
a `String` boundary parameter that smuggled wrong-domain semantics
into production), a finding gets filed at that incident, and the
mechanical-lint conversation reopens with a concrete case behind it.
Today's data does not justify the engineering.

## Case study: the adaptive-inquiry drift (2026-07-02, RFL-129/RFL-130)

The failure mode predicted above happened. `JobRegistration.key` was a
plain `String` at the App ↔ Runway boundary, and job dispatch inside
apps was string matching. The consequences, found in one review pass:

- **quorum-sense** registered `adaptive-inquiry` in the manifest `jobs`
  array, but the ambient handler never claimed it. Any enqueued
  `adaptive-inquiry` job would sit in the queue and die by retry
  exhaustion. Nothing at compile time, boot time, or test time noticed.
- **catalyst-biz** and **warden-compliance** both registered ambient
  jobs while having *no jobs runtime at all* — dead metadata with the
  same shape, waiting to become the same bug.
- **quorum-sense** errors flowed through `AmbientError::Other(String)`,
  so retryability was decided by message inspection, and a malformed
  `recall` payload block was silently `unwrap_or`-defaulted instead of
  rejected.

The fix (runtime-runway 3.6.0, PRs runtime-runway#14, quorum-sense#10,
catalyst-biz#1, warden-compliance#1) did not add a lint. It moved the
enforcement **into the seam itself**, which is stronger:

1. **`JobKey` newtype** — validated kebab-case, parsed at every
   boundary (HTTP enqueue, manifest load, handler registration). Wire
   format unchanged via `#[serde(try_from = "String", into = "String")]`.
2. **Fail-fast coverage check** — `AmbientJobs::mount` refuses to boot
   when manifest-registered keys and `handler.job_keys()` drift in
   either direction. The adaptive-inquiry bug is now a startup error,
   not a silent queue graveyard.
3. **Hard builder error** — a packet that declares `jobs` without a
   registered jobs runtime fails `build()`. Dead registrations
   (catalyst, warden) cannot exist.
4. **Errors as variants** — `Other(String)` deleted;
   `is_retryable()` is a total function on the error type.
5. **Conformance test** — the app test parses the *real*
   `runway.app.json` (`include_str!`), not a fixture copy, and asserts
   handler keys == manifest keys. Manifest/code drift fails CI.

The convention-only decision above stands, with a refinement: when a
concrete failure appears, prefer **seam-owned enforcement** (types,
startup checks, conformance tests against real artifacts) over a
generic lint. The seam knows its own invariants; a lint only guesses.

## Companion practices

These fall out of the case study and apply to every boundary, alongside
the core rule:

- **Parse, don't validate.** Wire structs keep `String`; convert to the
  domain type immediately at the boundary and pass only the typed value
  inward. Malformed input is a 400/`InvalidPayload` at the edge, never
  an `unwrap_or(default)` deep inside.
- **No `unwrap_or` on deserialization.** A present-but-malformed
  payload block is an error, not an absent one. Defaulting is only for
  genuinely absent optional input.
- **Retryability lives on the error type.** No consumer may branch on
  an error message string. If callers need to distinguish outcomes, the
  distinction is a variant.
- **Conformance tests read the real artifact.** A test that asserts
  against a hand-copied manifest snippet re-rots the moment the real
  file changes. `include_str!` the shipping file.
- **Registration surfaces are exclusive.** The manifest `jobs` array
  belongs to the ambient worker runtime and nothing else; helm-governed
  work is declared via `operator_packets`. One concept, one surface —
  dual-purpose arrays are how adaptive-inquiry happened. (Precedent:
  the `OperatorPacketRegistration.packet_key` doc comment.)

## Cross-references

- `KB/04-architecture/runtime-injection-boundaries.md` — the
  Boundary Checklist and the consequence-lanes diagram.
- `QUALITY_BACKLOG.md` — `RP-TYPED-CROSS-LAYER-SEMANTICS`,
  `QF-2026-06-02-19`, `QF-2026-06-08-02`.
- `KB/05-engineering/standards/repo-layering.md` — the
  repository-boundary layering rule (different axis: this standard is
  about *what crosses* the boundary; that one is about *which repo
  can depend on which*).
- `KB/05-engineering/standards/hermetic-unit-tests.md` — the
  `RP-HERMETIC-UNIT` close as the playbook for per-crate lint
  rollout.
