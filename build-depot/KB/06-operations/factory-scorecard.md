---
tags: [operations, factory-health, scorecard, trend]
source: llm
date: 2026-06-07
---

# Factory Health Scorecard

One row per recurring health review cycle. The point is the **trend**, not the
snapshot. If the factory is degrading, this table must show it before the next
incident does.

Build-Depot owns the canonical scorecard schema in
`build-depot/docs/operations/factory-scorecard.md`. This file remains the
append-only historical Markdown view until RFL-162 replaces manual counting
with graph-backed exports.

Cycle entries are append-only; never delete a row. If a number was reported
wrong, add a corrective row in the next cycle and explain in `Notes`.

## Scorecard

| Cycle | Date       | Open (A/B/C/D) | Closed Δ | RP Enforced | Risks Open / Due | Standards / ADRs (cum.) | CI Green | Notes |
|------:|------------|---------------:|---------:|------------:|-----------------:|------------------------:|----------|-------|
| 1     | 2026-06-02 | 21 (4/10/6/2)  | +0       | 0 / 14      | 0 / 0            | 0 / 0                   | 0 / 0    | Baseline; 14-property RP table + ledger instantiated. No drift checks yet. |
| 2     | 2026-06-07 | 15 (1/5/7/2)   | +21      | 8 / 14      | 1 / 0            | 3 / 4                   | 1 / 1    | First real cycle since baseline. `just doctor` (16 checks) green in CI. Mechanized YANK + FRESH-CLONE-GREEN. RR-2026-06-07-01 accepted (Tauri GTK3 glib). Promoted 3 standards from QF-28/29/30 closures. |
| 3     | 2026-07-02 | 18 (0/9/7/2)   | +26      | 15 / 17     | 1 / 0            | 10 / 7                  | 2 / 5    | Linear migration + cross-agent alignment cycle. Closed -08-09 (commerce-rails CI green) + -02-14 (publishable, ADR backfilled); demoted -26-01 D→C (decision executed). Opened -04..-07 (this review: milestone-done check missing, hermetic-audit red, branch-hygiene debt, audit-ignore expiry) + -02/-03 (parallel session). CI red: doctor (stale-review — clears this commit), fresh-clone (-07-02), hermetic-audit (-02-05). |

### Column definitions

- **Open (A/B/C/D)** — open findings, derived from `QUALITY_BACKLOG.md` open
  bucket counts at end of cycle.
- **Closed Δ** — closures during this cycle (delta vs prior). Cycle 1 is
  baseline (no prior, Δ=0).
- **RP Enforced** — count of `RP-*` rows with `Status` cell containing
  `Enforced`. The other rows are `Aspired` (recipe pending) or
  convention-only.
- **Risks Open / Due** — `RR-*` entries open at end of cycle / entries whose
  revisit date fell within this cycle.
- **Standards / ADRs (cum.)** — cumulative count of files under
  `KB/05-engineering/standards/*.md` and
  `KB/04-architecture/decisions/*.md`. Not delta; total. Trend = monotone
  increase.
- **CI Green** — number of GitHub Actions workflows where the most recent
  scheduled or manual run on `main` is green / total workflows defined.
  Today: 1 (doctor.yml) / 1.

### How to update

At the end of each cycle (per `AGENTS.md > Recurring Software-Factory Health
Review`), run:

```bash
just snapshot         # derives most cells of this row
ls KB/05-engineering/standards/ | wc -l   # Standards (cum.)
ls KB/04-architecture/decisions/*.md | wc -l   # ADRs (cum.)
gh run list -R Reflective-Lab/reflective --workflow=doctor.yml --limit 1
```

As Build-Depot coverage grows, prefer graph queries and Build-Depot exports
over manual counts, while keeping the append-only scorecard row as the durable
human-readable record.

Append the new row beneath the previous one. Keep `Notes` to one line — the
detailed log lives in `QUALITY_BACKLOG.md > Review Cycles`.

## Trend lens

Two questions to ask at every review:

1. **Are we closing faster than we open?** If `Closed Δ` consistently lags
   the count of new B/C/D openings, the backlog is silently growing and
   `Bucket A SLA breaches` will start ticking up. Cycle 1 → Cycle 2 = +21
   closed against ≈6 net new opens = healthy.
2. **Is `RP Enforced` monotone increasing?** Properties that *flip back* from
   `Enforced` to `Aspired` are a red flag — the gate broke, or the user
   added a new property without yet mechanizing it. Cycle 2's flip from
   0 to 8 properties enforced is exceptional (first real flywheel turn).

Watch for the slope, not the absolute number. A flat scorecard for two
cycles in a row means the cycle is performative — fix the input, not the
output.
