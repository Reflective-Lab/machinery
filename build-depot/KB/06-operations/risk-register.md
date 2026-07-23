# Risk Register

Accepted and deferred risks are tracked here with owners and revisit dates.
Entries are not closure records; each risk remains visible until re-accepted,
promoted back to the active backlog, or resolved.

## RR-2026-06-07-01

- Date accepted: 2026-06-07
- Originating finding: `QF-2026-06-06-01`
- ADR: `KB/04-architecture/decisions/2026-06-07-tauri-gtk3-glib-risk.md`
- Risk: Fathom and Scout desktop lockfiles retain `glib 0.18.5` through the
  current Tauri/Wry Linux GTK3 stack, leaving GitHub Dependabot advisory
  `GHSA-wrw7-89jp-8q8g` open.
- Acceptance: Keep the Dependabot alerts open and visible, but do not publish
  Linux desktop release artifacts for Fathom or Scout while this risk is open.
  Continue macOS and Windows desktop work.
- Owner: Reflective platform owner
- Monitoring owner: Codex during dependency remediation sessions
- Revisit date: 2026-07-07
- Revisit trigger: Any Tauri/Wry release that moves Linux desktop dependencies
  to `glib >= 0.20`, any new Fathom or Scout Linux desktop release plan, or any
  escalation in advisory severity.
- Check: `cargo tree -i glib@0.18.5 --target all` in both desktop crates;
  inspect release workflows and release assets for Linux desktop artifacts.
- Status: Accepted risk
