# Factory Scorecard

Build-Depot owns the scorecard schema for the Reflective software factory.
Historical hand-written rows may remain in the parent workspace until graph
exports fully replace them, but new scorecard semantics belong here.

## Purpose

The scorecard is a trend surface. Its job is to show degradation before the
next incident exposes it.

## Minimum Metrics

- open findings by bucket and area
- mean time to close by bucket
- Bucket A SLA breach count
- drift findings opened vs. closed
- CI green rate per active repo
- standards added or revised
- ADRs added
- accepted risks opened, re-accepted, promoted, or materialized
- incidents open by severity and repository
- aggregate runtime/security/dependency/test signal volume
- affected users or event counts for grouped telemetry signals
- repos with stale release or unknown CI state
- repo adoption state by tier
- repo adoption state by cohort
- repo adoption gaps without linked issues or accepted risks

## Local Export

While Omnigraph ingest is deferred, use:

```bash
just scorecard
```

The export is JSON with these top-level slices:

- `graph_records` - total node and edge counts plus edge counts by type.
- `findings` - totals, open count, bucket counts, and status counts.
- `repositories` - total, layer counts, adoption counts, and blocking failures.
- `incidents` - totals, open count, and severity counts.
- `signals` - aggregate signal totals, event counts, affected users, category
  counts, source counts, and status counts.
- `pull_requests`, `check_runs`, and `deployments` - lifecycle evidence counts.
- `risks`, `adrs`, and `owners` - governance and accountability counts.
- `recurring_properties` - totals by enforcement status.
- `standards` - standard count.

This shape should remain compatible with the future Omnigraph-backed export.

## Data Sources

- `QUALITY_BACKLOG.md`
- Linear issues and projects
- GitHub workflow/check status
- Sentry issues
- `release-train.yaml`
- Omnigraph records defined by `build-depot.pg`
- repository adoption scans defined in [Repository Adoption](repository-adoption.md)
- signal semantics defined in
  [Signal Capture And Improvement](signal-capture.md)

## Direction

RFL-162 should make Build-Depot produce a machine-readable scorecard export.
Until that lands, parent workspace Markdown can keep append-only human-readable
history, but it should refer to Build-Depot for schema and interpretation.

Omnigraph runtime is deferred for one week while the deployment target is
reassessed. During that window, scorecard work should still define metrics and
record shapes so the export can be enabled once ingest is live.
