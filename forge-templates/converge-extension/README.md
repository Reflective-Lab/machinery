# {{extension}} — Converge Extension

> Replace `{{extension}}` with the canonical crate name (e.g. `mnemos`,
> `prism`, `arbiter`) and remove this banner before publishing.

`{{extension}}` is a Converge extension. It depends on stable Converge
contracts (`converge-pack`, `converge-model`, `converge-provider-api`) and
ships an implementation downstream of the foundation.

This repository follows the
[Extension Release Checklist][checklist] — the canonical engineering bar
every extension repo must meet before tagging a release alongside Converge.

[checklist]: https://github.com/Reflective-Lab/converge/blob/main/kb/Standards/Extension%20Release%20Checklist.md

## Release ritual

Every release tag must be preceded by these five gates, all green, in
order:

```bash
just security-audit                    # 1. clean supply chain
just coverage                          # 2. ≥ 80% per crate, no regression
PERF_BASELINE=v$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"(.*)".*/\1/') \
    just performance-profile           # 3. baseline locked
SOAK_DURATION_MIN=5 just soak          # 4. stability proven
just lint && cargo test --workspace    # 5. green
```

Or, in one shot: `just release-check`.

Archive `target/security/`, `target/coverage/`, `target/criterion/`,
`target/soak/`, and `kb/Baselines/` alongside the release tag.

## Floor versions

This extension targets:

- Converge `≥ 3.8.1`
- MSRV `1.96.0`
- Edition `2024`
- `unsafe_code = "forbid"` workspace-wide

Converge platform crates resolve from crates.io. Do not add local `[patch.crates-io]` overrides unless a task explicitly requires testing unpublished foundation changes.

## What lives here

```
.
├── crates/                   # workspace member crates
├── kb/                       # extension's own knowledge base (mirrors foundation)
├── scripts/                  # release helpers (criterion baseline extractor, ...)
├── .github/workflows/        # ci, coverage, security, stability
├── Cargo.toml                # workspace
├── Justfile                  # gates: check, lint, test, security-audit,
│                             #        coverage, performance-profile, soak
└── deny.toml                 # cargo-deny configuration
```

## What lives elsewhere

- The kernel, contracts, and convergence loop live in
  [`Reflective-Lab/converge`](https://github.com/Reflective-Lab/converge).
- The release checklist is owned by foundation; this repo follows it.
- Sibling checkouts: `atelier-showcase`, `arbiter-policy`, `embassy-ports`,
  `ferrox-solvers`, `manifold-adapters`, `mnemos-knowledge`,
  `prism-analytics`. See
  [Extension Topology](https://github.com/Reflective-Lab/converge/blob/main/kb/Architecture/Extension%20Topology.md).

## Documentation

The knowledge base in `kb/` is canonical for this extension.

- [`kb/Home.md`](kb/Home.md) — index
- [`kb/Architecture/`](kb/Architecture/) — surface diagrams, port boundaries
- [`kb/Building/`](kb/Building/) — getting-started, release commands
- [`kb/History/CHANGELOG.md`](kb/History/CHANGELOG.md) — release notes
- [`kb/Planning/MILESTONES.md`](kb/Planning/MILESTONES.md) — scheduled delivery
