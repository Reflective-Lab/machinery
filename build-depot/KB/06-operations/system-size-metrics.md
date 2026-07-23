---
tags: [operations, factory-health, metrics, system-size]
---

# Reflective Labs System Size Numbers

Measured locally on 2026-06-08 from `/Users/kpernyer/dev/reflective`.

## Headline Numbers

| Metric | Count |
|---|---:|
| Local git repos | 44 |
| Full workspace files | 7,287 |
| Full workspace total lines | 1,709,756 |
| Full workspace raw code lines | 1,256,761 |
| Conservative source code lines | ~548,000 |
| Rust code lines | ~407,000 |
| Root coordination repo files | 779 |
| Root coordination repo total lines | 75,993 |
| Root coordination repo code lines | 12,814 |

## LOC By Scope

| Scope | Files | Total lines | Code lines |
|---|---:|---:|---:|
| Full local Reflective workspace, 44 git repos | 7,287 | 1,709,756 | 1,256,761 |
| Root coordination repo only | 779 | 75,993 | 12,814 |

## Structural Counts

| Metric | Count |
|---|---:|
| Rust `Cargo.toml` manifests | 286 |
| JS `package.json` manifests | 31 |
| GitHub workflow files | 82 |
| Justfiles/build recipe files | 41 |
| Test/spec-looking source paths | 240 |
| Markdown docs/KB files | 2,396 files, 266k lines |

## Size Metrics To Track

| Area | Metrics |
|---|---|
| Repos/deployables | repos, apps, sites, services, crates, packages |
| Language mix | Rust LOC, TypeScript/Svelte LOC, static asset LOC, fixture/data LOC, doc lines |
| CI/CD | workflow count, required gates, release paths, deploy targets |
| Tests | test files, test cases, coverage, flaky rate, contract tests, e2e tests |
| Dependencies | direct deps, transitive deps, lockfiles, vulnerable packages, SBOM size |
| Operations | environments, cloud resources, queues, jobs, secrets, dashboards, alerts |
| Change complexity | churn, hot files, open findings, ownership gaps, stale branches |
| Governance | ADRs, standards, risk entries, backlog findings, runbooks |

## How To Re-measure

```sh
tokei .
find . -maxdepth 3 -name .git -type d \
  | while read gitdir; do repo=${gitdir%/.git}; tokei "$repo" | rg '^ Total'; done
```

Structural counts should exclude dependency and build directories:

```sh
rg --files --no-ignore \
  -g '!**/target/**' \
  -g '!**/vendor/**' \
  -g '!**/node_modules/**' \
  -g '!**/dist/**' \
  -g '!**/build/**' .
```

Reporting shorthand: 1.26M raw code lines across 44 local repos; roughly 548k
conservative source lines after removing HTML/JSON-heavy artifacts.
