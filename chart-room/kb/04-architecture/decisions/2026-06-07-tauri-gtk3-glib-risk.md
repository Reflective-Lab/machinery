# ADR: Tauri GTK3 glib Advisory Handling

- Date: 2026-06-07
- Status: Accepted
- Decision type: security risk handling
- Originating finding: `QF-2026-06-06-01`
- Risk register: `KB/06-operations/risk-register.md#rr-2026-06-07-01`
- Revisit date: 2026-07-07

## Question

How should Reflective handle the remaining GitHub Dependabot `glib`
advisory `GHSA-wrw7-89jp-8q8g` in the Fathom and Scout Tauri desktop
lockfiles when current Tauri/Wry releases still depend on the GTK3 binding
stack that pulls `glib 0.18.5`?

## Decision

Carry a temporary, bounded risk acceptance for the alert while keeping it
visible in GitHub and in the quality ledger.

- Do not dismiss the GitHub Dependabot alerts until `glib >= 0.20` is actually
  present in both desktop lockfiles or the alerts are otherwise resolved by a
  verified upstream change.
- Do not publish Linux desktop release artifacts for `fathom-narrative` or
  `scout-sourcing` while this risk is open.
- Continue macOS and Windows desktop work because the advisory is in the Linux
  GTK3 path and Scout's documented release plan targets macOS and Windows.
- Re-check Tauri/Wry releases during the next dependency remediation pass.

## Options Considered

1. Fork the GTK3 binding stack and backport the `glib` fix.
   Rejected for now. It would create a maintenance burden around an upstream
   stack that Tauri itself describes as blocked by unmaintained GTK3 bindings.
2. Block all desktop work until Tauri/Wry move off `glib 0.18.5`.
   Rejected because the current product release targets are macOS and Windows,
   and the practical exposure is bounded to Linux desktop packaging.
3. Dismiss the Dependabot alerts as irrelevant.
   Rejected. The vulnerable dependency is still present in the lockfiles, so
   dismissing would reduce supply-chain signal quality.
4. Keep the alerts open, prohibit Linux desktop release artifacts, and revisit
   upstream status on a schedule.
   Chosen because it keeps the risk visible without spending factory effort on
   a large upstream fork before Linux desktop becomes a declared release target.

## Evidence

- Both desktop lockfiles were updated to `tauri 2.11.2`; `cargo tree -i
  glib@0.18.5 --target all` still resolves `glib 0.18.5` through
  `tauri -> gtk 0.18.2 / webkit2gtk 2.0.2`.
- `cargo search wry --limit 5` showed `wry 0.55.1` as the latest available
  crate release during the 2026-06-07 check.
- Tauri issue https://github.com/tauri-apps/tauri/issues/12048 documents this
  advisory class as blocked by unmaintained GTK3 bindings.
- A direct scan for `VariantStrIter` found no usage in the Fathom or Scout app
  code; the symbol appears in `glib 0.18.5` itself.
- Scout's `README.md`, `AGENTS.md`, `MILESTONES.md`, and `RELEASE.md` describe
  macOS and Windows desktop packaging, not Linux release artifacts.

## Consequences

- Dependabot will continue to report one moderate alert in each affected app
  repo until the upstream dependency path changes.
- Any future Linux desktop release plan for Fathom or Scout must first close or
  re-evaluate `QF-2026-06-06-01`.
- The quality ledger and risk register remain the source of truth for why the
  alert is still open.
