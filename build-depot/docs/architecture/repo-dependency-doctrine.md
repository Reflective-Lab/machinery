# Repo Dependency Doctrine

Who may depend on whom, through what, and what is merely "better when
available". Decided 2026-07-10.

## The rule in one line

**Dependency arrows point downward through released artifacts — never
sideways through sibling checkouts.**

## The layers

```
┌─────────────────────────────────────────────────────────────┐
│ ~/dev/reflective  (fleet checkout — OPTIONAL governance)     │
│   root Justfile doctor gates, QUALITY_BACKLOG, fleet KB      │
│      │ invokes                                               │
│      ▼                                                       │
│ machinery/build-depot  (factory control plane)               │
│   gate scripts, adoption contract, scorecard, publisher      │
├─────────────────────────────────────────────────────────────┤
│ machinery products (runtime-runway, commerce-rails, …)       │
│   depend on bedrock ONLY via released artifacts              │
│   (registry versions after Kellnr; git-tag deps today)       │
├─────────────────────────────────────────────────────────────┤
│ applications                                                 │
│   depend on bedrock AND machinery products — again only via  │
│   released artifacts, never via ../sibling paths             │
├─────────────────────────────────────────────────────────────┤
│ framework/bedrock  (foundation — SELF-CONTAINED)             │
│   depends on nothing else in the fleet. Hard deps: pinned    │
│   rustc (rust-toolchain.toml), protoc/cmake, vendored        │
│   native solvers built in-repo (`just deps`).                │
└─────────────────────────────────────────────────────────────┘
```

## What each layer may assume

| Repo | May hard-depend on | Better when available (must degrade gracefully) |
| --- | --- | --- |
| `framework/bedrock` | Toolchain + in-repo vendor builds only. Nothing else in the fleet. | Fleet checkout enables the root doctor gate and cross-repo kb links. Absence changes nothing about build/test/release. |
| `machinery` products | Released bedrock artifacts (registry / git tags). | Sibling bedrock checkout enables local `[patch]` overrides during development — convenience, never committed. |
| applications | Released bedrock + machinery artifacts. | Same as above. |
| `~/dev/reflective` root | The fleet being checked out (it IS the fleet layout). | n/a — the root layer is itself the optional one. |

## How self-containment is enforced (bedrock)

1. **GitHub CI is the continuous proof.** Every push to `bedrock-platform`
   builds and tests on a runner that has ONLY the bedrock checkout — no
   machinery, no outer workspace, no `~/dev/reflective` layout. A green CI
   run is a fresh-clone self-containment certificate.
2. **`cargo metadata` closure**: all workspace members resolve inside the
   repo; no path dependency escapes it (verified 2026-07-10, zero
   escapes).
3. **Root `project-doctor`** (when the fleet IS checked out) guards the
   arrows: publishable crates must not path-dep unpublishable ones;
   upstream workspaces must not path-dep into product workspaces;
   Foundation→runtime-runway path deps need an explicit
   `RP-HELMS-SUBSTRATE-SEAM` exemption.
4. **Root `check-all-fresh` / `test-all-fresh`** re-verify fresh-clone
   builds across the fleet on demand.

## Cross-repo references that are allowed

- **Docs pointers** (e.g. bedrock kb pointing at a Build-Depot doc) are
  fine: they degrade to a dead link, not a build failure. Mark them with
  the repo name (`machinery/build-depot/...`) so the reader knows a
  different checkout is being referenced.
- **Governance invocation** (root Justfile → build-depot gate scripts) is
  fine: the root layer is by definition the fleet checkout, so it may
  assume the fleet exists. Consequence: `machinery` is a Tier-0-critical
  checkout for the fleet gate — but never for any repo's own build.

## Cross-repo references that are forbidden

- Cargo `path = "../../…"` escaping a repo boundary (except the
  documented exemption seam above, and never committed in a publishable
  crate).
- Build scripts, Justfiles, or CI workflows inside a product/foundation
  repo referencing `~/dev/reflective`, `MASTERPLAN.md`, or sibling repo
  paths.
- Tests that only pass when a sibling checkout exists.

## Known transition debt

- `runtime-runway` consumes bedrock via git-tag deps until the Kellnr
  registry is live (August 2026); then registry versions.
- `arena-driver` in bedrock retains a legacy outer-root discovery
  fallback (looks for `MASTERPLAN.md` + `KB/`); harmless (repo-internal
  detection is primary) but a candidate for removal.
