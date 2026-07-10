# Standard: Repository-boundary layering

**Property:** complement to `RP-LAYERING` in `QUALITY_BACKLOG.md`. Different
axis: this rule is about *repository boundaries* in the workspace
architecture, where `RP-LAYERING` is about the *publish status* of
individual crates.

**Enforcement:** `just project-doctor` checks 7 and 9 (root `Justfile`),
wired into `.github/workflows/doctor.yml`'s `project-doctor` job.
Check 7: Foundation/extension/showcase must not path-dep into product workspaces.
Check 9: Foundation crates must not path-dep `runtime-runway` without a
`# RP-HELMS-SUBSTRATE-SEAM` exemption comment (added 2026-07-04, RFL-128).

## The rule

**Foundation, extension, showcase, and test repos must not path-dep into
product repos.** Products consume the platform — never the other way.

| Layer | Repos | Can consume |
|---|---|---|
| Foundation | `bedrock-platform/{converge, axiom, organism, helms}` | only foundation |
| Extension | `mosaic-extensions/*` | foundation |
| Showcase | `atelier-showcase` | foundation + extension |
| Test | `arena-tests` | foundation + extension + showcase |
| Runtime substrate | `runtime-runway` | foundation + extension |
| Commerce | `commerce-rails` | foundation |
| **Product** | `marquee-apps/*`, `studio-apps`, `beacon-sites`, `mobile-apps` | **everything above** |

The check rejects `[dependencies]` entries in any upstream-layer repo
whose resolved `path` field points into a product directory.

## What the check catches

Concrete example from 2026-06-07: `atelier-showcase/Cargo.toml` declared
path-deps:

```toml
quorum-truths = { path = "../marquee-apps/quorum-sense/crates/quorum-truths" }
quorum-app    = { path = "../marquee-apps/quorum-sense/crates/quorum-app" }
quorum-domain = { path = "../marquee-apps/quorum-sense/crates/quorum-domain" }
```

These violated the rule (showcase reaching into product). The check
would have flagged each with:

```
atelier-showcase: <consuming-crate> → quorum-truths (path-dep into marquee-apps/)
```

The fix is in the consuming repo: either move the shared types to
foundation/extension (so both showcase and product can depend on
them), or invert the dependency (product depends on showcase, not the
other way).

## How to extend the rule

The list of upstream repos and product directories is hard-coded in
`project-doctor` check 7. If the architecture grows a new layer:

1. Update the `upstream_only` list in `project-doctor` check 7.
2. Update the table above.
3. Update `release-train.yaml` if the new repo belongs in the
   publish train.

If the rule generalises further (e.g. extension-repo subdirectories
get their own layer), promote the hard-coded lists to declarative
fields in `release-train.yaml` (`upstream_only: bool`,
`forbidden_ancestors: [str]`) and have the check read from there.

## Publish-boundary seams (RP-LAYERING forks)

When a publishable crate needs behavior that lives in an unpublishable
substrate crate, the dependency **must not** point at the substrate — not
even as a dev-dependency (project-doctor check 2 walks every dep kind).
The sanctioned pattern is a **seam**: the publishable crate declares a
minimal trait (the seam), and the substrate implements it downstream.
Concrete implementations are created and injected downstream; interfaces
flow upstream. Every seam must be explicit and documented at both ends:

1. A doc comment on the trait naming RP-LAYERING and the implementing
   crate (the upstream end).
2. A manifest comment in the implementing crate's module explaining why
   the glue lives on the unpublishable side (the downstream end).
3. An entry in the table below.

| Seam | Publishable side (declares) | Unpublishable side (implements) | Since |
|---|---|---|---|
| `JobsRuntime` | `runway-app-host` (`builder.rs`) | `runway-ambient` (`AmbientJobs`, `jobs_runtime.rs`) | 2026-07-02 (v3.5.0) |
| EventHub / SSE | `helm-coordination` | `runway-app-host` (`EventHub`, `EventHubHandle`, `EventCursor`) | 2026-07-04 (RFL-128) |
| EventHub / SSE | `helm-governed-jobs` | `runway-app-host` (`EventHub`, `EventHubHandle`, `EventCursor`) | 2026-07-04 (RFL-128) |
| EventHub / SSE + `SessionOwnershipLayer` | `helm-session-host` | `runway-app-host` (`SessionOwnershipLayer`, `EventHub`, `EventHubHandle`, `EventCursor`) | 2026-07-04 (RFL-128) |

Registered while resolving the first fork of this kind: `runway-app-host`
normal-depped `runway-ambient` for the ambient jobs surface, which made the
host unpublishable and blocked five helms crates from crates.io.

The three helms→runway seams above were legitimized during RFL-128
(2026-07-04). `HelmModule` / `ModuleState` were extracted to `helm-module-contracts`
(violation 1 resolved). `helm-operator-control` and `helm-truth-execution`
dropped their `runway-app-host` dep entirely (violation 2 partially resolved);
the remaining three crates retain it only for EventHub/SSE/SessionOwnershipLayer
and carry `# RP-HELMS-SUBSTRATE-SEAM` comments in their `Cargo.toml` files.
Project-doctor check 9 mechanically enforces that no new unannotated
Foundation→substrate edges are introduced.

## Cross-references

- `QUALITY_BACKLOG.md` — `RP-LAYERING` row (publish-status axis;
  this standard is the repository-boundary axis).
- `bedrock-platform/converge/CLAUDE.md` — "Do not depend on
  converge-core, converge-runtime, or other internal crates" — the
  Converge-internal version of the layering rule.
- `KB/04-architecture/runtime-injection-boundaries.md` — the
  "consequence lanes" diagram showing which layer owns which authority.
