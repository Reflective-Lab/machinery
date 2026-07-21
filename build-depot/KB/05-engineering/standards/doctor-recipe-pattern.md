---
tags: [standards, ci, quality-factory, doctor]
source: llm
date: 2026-06-07
promoted-from: QF-2026-06-02-28
property: RP-POLICY-FRESH, RP-RELEASE-TRAIN-INTEGRITY, RP-LAYERING, RP-CRATE-SIZE-BUDGET, RP-SNAPSHOT-PORTABLE, RP-RUSTC-DRIFT-CONTAINED
---

# Doctor Recipe Pattern

How a `RP-*` standing property graduates from "Aspired" to "Enforced" in this
workspace: every property gets a **numbered accumulating check** inside the
appropriate doctor recipe, wired into `.github/workflows/doctor.yml`.

Two doctors live at the root:

- `just quality-doctor` — meta-policy drift (the ledger, RP table, snapshot,
  cross-refs, root agent-pointer files). Catches the factory drifting away
  from its own rules.
- `just project-doctor` — train invariants (release order, layering, crate
  size, snapshot portability, rustc pinning). Catches the train drifting
  away from its publish contract.

Both compose into `just doctor`.

## Rule

A new property is mechanized by adding a numbered check inside the matching
doctor recipe. The check obeys five invariants:

1. **No `set -e`.** Recipes use `set -uo pipefail` (not `-euo`) because they
   must run *every* check and accumulate failures, not abort on the first
   `✗`. The exit code reports the *count* of failed checks, so the operator
   sees the full picture in one run.
2. **Numbered comment header.** Each check opens with `# N. RP-NAME — short
   sentence.` so a reader can skim recipe → properties in seconds.
3. **`✓` / `✗` per outcome, increment a `fails` counter on `✗`.** No silent
   skips. If a check cannot run (missing tool, missing directory), that is
   an `✗` with a clear cause.
4. **End with `exit "$fails"`.** Exit code 0 = all checks pass. Exit code N
   = N checks failed. CI gates on this naturally.
5. **Side-effect-free.** Doctor reads state and reports drift. It never
   mutates files. If you want a fix, run the named remediation recipe
   instead.

## Recipe skeleton

```just
# project-doctor — drift checks across the publishable release train.
project-doctor:
    #!/usr/bin/env bash
    set -uo pipefail
    fails=0
    echo "── project-doctor ──"

    # 1. RP-NAME-A — one-line statement of the invariant.
    if [[ <invariant-holds> ]]; then
        echo "✓ <human-readable pass message>"
    else
        echo "✗ <human-readable fail message>"
        fails=$((fails+1))
    fi

    # 2. RP-NAME-B — ...
    ...

    if [[ "$fails" -gt 0 ]]; then
        echo "── ✗ project-doctor: $fails check(s) failed ──"
    else
        echo "── ✓ project-doctor: all checks passed ──"
    fi
    exit "$fails"
```

## How to apply

When closing a finding that promises a standard:

1. Pick the right doctor. Meta-policy → `quality-doctor`. Train invariant
   → `project-doctor`. Cross-train hygiene (security, deps) → consider a
   new doctor (`deps-doctor`, `security-doctor`) rather than overloading
   the existing ones.
2. Add the numbered check at the next available slot, with the
   `# N. RP-NAME — ...` header.
3. Run the recipe locally and verify both states: green when invariant
   holds, ✗ counted when you intentionally break it.
4. Run the matching CI job (`gh workflow run doctor.yml`) and verify it
   exercises the new check on a clean runner.
5. Flip the `Status` column in the `QUALITY_BACKLOG.md` `RP-*` table from
   `Aspired (...)` to `Enforced (via .github/workflows/doctor.yml, check
   N)`. `quality-doctor` check 4 will validate the `Tracked by` references
   if any open findings remain.

## Exceptions

- A property that is purely a *convention* (commit-message discipline, AI
  shortcut declaration, evidence citation) cannot fully mechanize. Document
  the convention in `AGENTS.md`, mark `Status: Aspired (convention-only)`
  in the RP table, and add the partial-mechanization (lint, pre-commit
  hook) as a separate closure-eligible finding.
- A property that needs *cross-repo cargo state* (semver public-API diff,
  cargo package size) belongs in `project-doctor` but may need a new CI
  workflow rather than `doctor.yml`, because it requires installed
  toolchain components beyond what `doctor.yml` ships.

## Why this pattern

The accumulator + non-`-e` shape is load-bearing for trust in the gate.
A `set -e` recipe that exits on first `✗` reports only the first drift
each cycle; the operator fixes it, runs again, sees the next one, ad
infinitum. The accumulator surfaces the *whole* drift surface every run,
so the operator estimates and prioritizes correctly.

The "no side effects" rule is what makes the gate boring enough to live
in CI. A doctor that fixes things on its own becomes a script the team
fears; a doctor that only reports is one the team trusts.

## Provenance

Closing finding: `QF-2026-06-02-28` (project-doctor recipe + 5 RPs
mechanized). Same-day closures `QF-2026-06-02-29` (publish-status drift)
and `QF-2026-06-02-30` (rustc pinning drift) demonstrate the
finding-→-fix-→-standard loop. Closing commits in
`Reflective-Lab/reflective`: `d85b107` (recipes), `4416e85` (CI wiring),
`ad25379` (SHA backfill), `ed44065` (PAT removal), `756668a` (subextension
checkouts), `463616b` (Node 24 actions).
