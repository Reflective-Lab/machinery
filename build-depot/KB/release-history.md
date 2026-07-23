# Release History

Canonical record of yanked-and-replaced crate versions across the Reflective
workspace. Every yank on `crates.io` for a Reflective-published crate gets a
row here **at the time of yank**, per `RP-YANK-DISCOVERABLE`.

This file is the single source of truth for the yank trail. Per-repo
`CHANGELOG.md` may mirror the relevant rows but does not replace them.

## Format

Each yanked version is its own section. The header is the exact crate name
and the yanked semver, prefixed with `###`. Each section MUST include the
five required fields (others are optional).

```markdown
### <crate-name> v<yanked-version>

- Yanked: YYYY-MM-DD
- Yanked by: <name or `ci`>
- Reason: <one-line, externally-readable reason>
- Successor: v<successor-version> (published YYYY-MM-DD)
- Migration: <one-line note for consumers, or `n/a` if same API surface>
```

`just project-doctor` check 6 fails if any `### <crate> v<ver>` block is
missing one of `Yanked:`, `Reason:`, `Successor:`, or `Migration:`. Required
fields are validated by structural lint; their *content* is reviewed at PR
time.

## Yank-and-replace runbook

When a Reflective-published crate version needs to be yanked:

1. **Decide.** Yank is an externally-visible event. Open a finding in
   `QUALITY_BACKLOG.md` before yanking unless the cause is already tracked
   (e.g. an open `QF-*` with `Status: Open`).
2. **Identify the successor.** A yank without a published successor leaves
   `cargo` users stranded mid-resolve. Publish the successor first, then
   yank the predecessor.
3. **Record here first.** Add the entry to this file with all five required
   fields BEFORE running `cargo yank`. The entry is the durable trail; the
   yank command is the side-effect.
4. **Execute the yank.**
   ```bash
   cargo yank --vers <yanked-version> <crate-name>
   ```
5. **Mirror to per-repo CHANGELOG** if the repo maintains one. Include the
   row under the matching version's `## Yanked` section with a link back
   here.
6. **Cite the entry** in any release-note, Slack post, or GitHub Discussion
   that announces the yank. The header `### <crate> v<ver>` is the
   stable anchor.

The `RR-*` risk register format does NOT apply to yanks. A yank is a
completed corrective event with public visibility; a risk register entry is
an accepted ongoing exposure with a revisit date. Don't conflate them.

## Entries

### converge-atelier-domain v1.0.1

- Yanked: 2026-06-02
- Yanked by: Codex
- Reason: *Pre-runbook yank; reason not recorded at time of action. Reconstructed
  retrospectively as part of `QF-2026-06-02-10` closure: the v1.0.1 surface
  did not match the converge v3.9.x Suggestor contract the train was
  publishing in the same window.*
- Successor: v1.0.2 (published 2026-06-02)
- Migration: API surface unchanged from a consumer perspective. Consumers
  taking `^1.0` resolve to v1.0.2 automatically on next `cargo update`.

### axiom-truth v0.15.1

- Yanked: 2026-06-02
- Yanked by: Codex
- Reason: *Pre-runbook yank; reason not recorded at time of action. Reconstructed
  retrospectively as part of `QF-2026-06-02-10` closure: v0.15.1 shipped a
  guidance-backend call path that bypassed the dependency-injected selector,
  silently calling the live LLM in unit tests. v0.15.2 dependency-injected
  the backend selector (commit `3fbe4fe feat(guidance)!: dependency-inject the
  backend selector`) and is the replacement.*
- Successor: v0.15.2 (published 2026-06-02)
- Migration: Constructors gained a `Backend` parameter. Consumers passing
  `Backend::default()` get the prior behavior; tests should wire stub
  variants explicitly per `RP-HERMETIC-UNIT`.

## Cross-references

- `RP-YANK-DISCOVERABLE` and `RP-AI-EVIDENCE-CITED` in `QUALITY_BACKLOG.md`
- `QF-2026-06-02-10` (yank-and-replace runbook closure)
- `QF-2026-06-02-24` (yank trail closure)
- `KB/05-engineering/standards/doctor-recipe-pattern.md` (how check 6 is structured)
- [[release-naming]] (named train releases — alphabetical city scheme)
