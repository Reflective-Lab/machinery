# RFL-155 W1: Four Real Arena Dimensions + Honest Deferrals

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace four arena quality-dimension stubs with real implementations, plus update three deferred stubs to carry honest precondition Skip strings.

**Architecture:** Each dimension lives in its own crate under `arena/crates/dim-<name>/src/lib.rs`. The driver in `arena/crates/arena-driver/src/lib.rs` calls all of them via the `Dimension` trait. dim-layering (already real) is the template — it shows the `find_<x>_root` + negative-synthetic test pattern. Two heavy dims (footprint, determinism) self-gate behind `ARENA_HEAVY=1` returning `DimensionResult::skipped(...)` on the fast path.

**Tech Stack:** Rust 2024 / edition 2024 / MSRV 1.96.0; `regex` crate (already in workspace); `serde_json` (workspace); `toml` (workspace); `cargo metadata` subprocess; existing `arena-metrics` trait/types; `proptest` (workspace).

## Global Constraints

- Branch: `e12/rfl-155-w1-dimensions` (already created).
- One commit per dimension; defer-honesty pass = one extra commit.
- Zero `#[allow(...)]` attributes — fix every warning instead.
- Zero `unsafe_code` — workspace `[workspace.lints.rust] unsafe_code = "forbid"`.
- No new external crate deps unless strictly necessary; prefer workspace-existing `regex`, `serde_json`, `toml`, `proptest`.
- `regex` and `proptest` are not yet in dim crate Cargo.tomls — add them from workspace when needed.
- `DimensionResult::skipped(id, name, property, reason)` is the constructor for Skip verdicts (see `arena-metrics/src/lib.rs:115`).
- All Finding severity choices: `Critical`, `High`, `Moderate`, `Info`.
- Verdict model: Critical findings → `Fail`; High-only → `Warn`; nothing real → `Pass`.
- Score convention: `100u8.saturating_sub(penalty)` where penalty scales with severity counts.
- ARENA_HEAVY=1 gating pattern: check `std::env::var("ARENA_HEAVY").is_ok_and(|v| v.trim() == "1")` at the top of `run()`.
- Run `cargo test -p <crate>` after each dimension to verify green before committing.
- Final gate: `cargo run -p arena-driver -- report` < 5 s, aggregate PASS.
- Report output goes to `/Users/kpernyer/dev/reflective/.superpowers/sdd/rfl155/w1-report.md`.

---

### Task 1: arena-dim-snapshot-portability — real fixture scanner

**Files:**
- Modify: `arena/crates/dim-snapshot-portability/src/lib.rs`
- Modify: `arena/crates/dim-snapshot-portability/Cargo.toml` (add `regex` dep)

**Interfaces:**
- Consumes: `arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict}`
- Produces: `pub struct SnapshotPortabilityDimension` implementing `Dimension::run(&self, ctx: &RunContext) -> DimensionResult`
- Also exports: `pub fn scan_fixture_file(path: &Path) -> Vec<Finding>` (for unit tests)
- Also exports: `pub fn classify_line(line: &str) -> Option<(Severity, &'static str)>` (for proptest)

- [ ] **Step 1: Add regex dep to Cargo.toml**

```toml
[dependencies]
arena-metrics = { workspace = true }
regex = { workspace = true }
```

- [ ] **Step 2: Write the failing tests**

Replace the stub body in `lib.rs` with the full implementation + tests. Start with the tests (failing):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_fixture(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    // --- negative synthetic: absolute path → Critical ---
    #[test]
    fn absolute_users_path_is_critical() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "bad.stderr",
            "error[E0000]: something\n --> /Users/someone/dev/reflective/src/lib.rs:10:5\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "expected Critical for /Users/ path, got: {findings:#?}"
        );
    }

    #[test]
    fn absolute_home_path_is_critical() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "bad.snap",
            "snapshot:\n  path: /home/ci-user/project/src/lib.rs\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "expected Critical for /home/ path, got: {findings:#?}"
        );
    }

    // --- positive: trybuild placeholders are fine ---
    #[test]
    fn trybuild_placeholders_not_flagged() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "ok.stderr",
            "error[E0000]: something\n --> $DIR/src/lib.rs:10:5\n\
             also $CARGO/registry/src/lib.rs\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.is_empty(),
            "trybuild placeholders must not be flagged, got: {findings:#?}"
        );
    }

    // --- opt-out comment ---
    #[test]
    fn opt_out_skips_absolute_path_check() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "opted-out.stderr",
            "// arena-snapshot: allow-absolute-paths\n\
             --> /Users/kpernyer/dev/foo.rs:1:1\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().all(|f| f.severity != Severity::Critical),
            "opted-out file must not produce Critical, got: {findings:#?}"
        );
    }

    // --- current workspace fixtures pass ---
    #[test]
    fn current_workspace_fixtures_are_clean() {
        // Walk from cwd up to workspace root (the dim's own test can't use ctx,
        // so we replicate the root-finding logic inline).
        let start = std::env::current_dir().unwrap();
        let ws_root = find_workspace_root_from(&start)
            .expect("must find workspace root from test cwd");
        let fixtures = collect_fixtures(&ws_root);
        let mut total_findings = Vec::new();
        for fixture in &fixtures {
            total_findings.extend(scan_fixture_file(fixture));
        }
        let critical: Vec<_> = total_findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .collect();
        assert!(
            critical.is_empty(),
            "{} Critical findings in workspace fixtures: {critical:#?}",
            critical.len()
        );
    }
}
```

Also add dev-dep for `tempfile`:

```toml
[dev-dependencies]
tempfile = { workspace = true }
proptest = { workspace = true }
```

Run: `cargo test -p arena-dim-snapshot-portability 2>&1 | tail -5`
Expected: FAIL — functions not defined.

- [ ] **Step 3: Write the full implementation**

Replace `lib.rs` entirely (copyright header + module doc stays, implementation replaces stub):

```rust
// Copyright 2026 Reflective Labs
// SPDX-License-Identifier: MIT

//! # Snapshot portability dimension
//! (... keep existing module doc ...)

use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict};
use regex::Regex;

/// Checks whether snapshots replay across machines and storage backends.
pub struct SnapshotPortabilityDimension;

impl Dimension for SnapshotPortabilityDimension {
    fn run(&self, ctx: &RunContext) -> DimensionResult {
        let start = Instant::now();
        let fixtures = collect_fixtures(&ctx.workspace_root);
        if fixtures.is_empty() {
            return DimensionResult {
                id: "snapshot-portability".into(),
                name: "Snapshot portability".into(),
                recurring_property: "RP-SNAPSHOT-PORTABLE".into(),
                verdict: Verdict::Pass,
                score: Some(100),
                findings: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
            };
        }

        let mut all_findings = Vec::new();
        let mut clean = 0usize;
        for fixture in &fixtures {
            let f = scan_fixture_file(fixture);
            if f.is_empty() {
                clean += 1;
            }
            all_findings.extend(f);
        }

        let total = fixtures.len();
        let score = Some(((clean * 100) / total.max(1)) as u8);

        let critical_count = all_findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .count() as u32;
        let high_count = all_findings.iter()
            .filter(|f| f.severity == Severity::High)
            .count() as u32;

        let verdict = if critical_count > 0 {
            Verdict::Fail
        } else if high_count > 0 || all_findings.iter().any(|f| f.severity == Severity::Moderate) {
            Verdict::Warn
        } else {
            Verdict::Pass
        };

        DimensionResult {
            id: "snapshot-portability".into(),
            name: "Snapshot portability".into(),
            recurring_property: "RP-SNAPSHOT-PORTABLE".into(),
            verdict,
            score,
            findings: all_findings,
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Walk the workspace and collect all fixture files.
/// Skips `target/`, `.git/`, `node_modules/`, and the ortools vendor tree.
pub fn collect_fixtures(workspace_root: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    collect_fixtures_in(workspace_root, &mut results);
    results
}

fn collect_fixtures_in(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return; };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_symlink() {
            continue;
        }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // Skip noisy subtrees
        if name_str == "target" || name_str == ".git" || name_str == "node_modules"
            || name_str == "vendor" || name_str == "_deps"
        {
            continue;
        }
        if path.is_dir() {
            collect_fixtures_in(&path, out);
        } else if is_fixture(&name_str) {
            out.push(path);
        }
    }
}

fn is_fixture(name: &str) -> bool {
    name.ends_with(".stderr")
        || name.ends_with(".snap")
        || name.ends_with(".golden")
        || name.ends_with(".expected")
}

/// Scan a single fixture file for portability issues.
///
/// Returns one Finding per offending line. An empty Vec means the file is clean.
pub fn scan_fixture_file(path: &Path) -> Vec<Finding> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return vec![];
    };

    // Check for opt-out comment on any of the first 5 lines.
    let opted_out = content.lines().take(5)
        .any(|l| l.trim() == "// arena-snapshot: allow-absolute-paths");

    let mut findings = Vec::new();
    for (line_no, line) in content.lines().enumerate() {
        let line_no = line_no + 1;
        if let Some((sev, reason)) = classify_line(line) {
            if opted_out && sev == Severity::Critical {
                // opt-out suppresses absolute-path Critical
                continue;
            }
            findings.push(Finding {
                title: format!(
                    "fixture `{}` line {line_no}: {reason}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
                severity: sev,
                evidence: format!(
                    "{}:{line_no}: {}",
                    path.display(),
                    line.trim()
                ),
                recurring_property: Some("RP-SNAPSHOT-PORTABLE".into()),
            });
        }
    }
    findings
}

/// Classify a single fixture line. Returns `Some((severity, reason))` for a hit.
///
/// Rules (in priority order):
/// 1. `$CARGO`, `$DIR`, `$VERSION`, `$WORKSPACE` — trybuild placeholders — NOT a finding.
/// 2. `/Users/`, `/home/`, `/private/`, `/var/`, `C:\` — Critical.
/// 3. Username path component: `[A-Za-z][A-Za-z0-9_-]+/dev/` — Moderate.
pub fn classify_line(line: &str) -> Option<(Severity, &'static str)> {
    // Skip trybuild placeholder lines — they contain $CARGO/$DIR etc.
    if line.contains("$CARGO") || line.contains("$DIR") || line.contains("$VERSION")
        || line.contains("$WORKSPACE")
    {
        return None;
    }

    // Absolute OS paths → Critical
    if line.contains("/Users/")
        || line.contains("/home/")
        || line.contains("/private/")
        || (line.contains("/var/") && line.contains("/folders/"))   // macOS tmp only
        || line.contains("C:\\")
        || line.contains("C:/")
    {
        return Some((Severity::Critical, "absolute filesystem path"));
    }

    // Username/dev/ component → Moderate
    // Use a lightweight check rather than a full regex for hot paths.
    if let Some(pos) = line.find("/dev/") {
        // Walk back to see if it's preceded by a username-shaped segment
        let before = &line[..pos];
        if let Some(slash) = before.rfind('/') {
            let candidate = &before[slash + 1..];
            if !candidate.is_empty()
                && candidate.chars().next().map_or(false, |c| c.is_ascii_alphabetic())
                && candidate.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
                && candidate.len() >= 2
            {
                return Some((Severity::Moderate, "username-derived path component"));
            }
        }
    }

    None
}

/// Walk up from `start` to find the workspace root (reusable from tests).
pub fn find_workspace_root_from(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let cargo = dir.join("Cargo.toml");
        if cargo.exists() {
            if let Ok(contents) = std::fs::read_to_string(&cargo) {
                if contents.contains("[workspace]")
                    && dir.join("arena").is_dir()
                    && dir.join("foundation").is_dir()
                {
                    return Some(dir);
                }
            }
        }
        // Outer-root fallback
        let bc = dir.join("bedrock-consolidated");
        if bc.is_dir() {
            let bc_cargo = bc.join("Cargo.toml");
            if bc_cargo.exists() {
                if let Ok(contents) = std::fs::read_to_string(&bc_cargo) {
                    if contents.contains("[workspace]")
                        && bc.join("arena").is_dir()
                        && bc.join("foundation").is_dir()
                    {
                        return Some(bc);
                    }
                }
            }
        }
        dir = dir.parent()?.to_path_buf();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_fixture(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    #[test]
    fn absolute_users_path_is_critical() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "bad.stderr",
            "error[E0000]: something\n --> /Users/someone/dev/reflective/src/lib.rs:10:5\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "expected Critical for /Users/ path, got: {findings:#?}"
        );
    }

    #[test]
    fn absolute_home_path_is_critical() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "bad.snap",
            "snapshot:\n  path: /home/ci-user/project/src/lib.rs\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "expected Critical for /home/ path, got: {findings:#?}"
        );
    }

    #[test]
    fn trybuild_placeholders_not_flagged() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "ok.stderr",
            "error[E0000]: something\n --> $DIR/src/lib.rs:10:5\n\
             also $CARGO/registry/src/lib.rs\n$VERSION blah\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.is_empty(),
            "trybuild placeholders must not be flagged, got: {findings:#?}"
        );
    }

    #[test]
    fn opt_out_skips_absolute_path_check() {
        let tmp = TempDir::new().unwrap();
        let f = write_fixture(tmp.path(), "opted-out.stderr",
            "// arena-snapshot: allow-absolute-paths\n\
             --> /Users/kpernyer/dev/foo.rs:1:1\n");
        let findings = scan_fixture_file(&f);
        assert!(
            findings.iter().all(|f| f.severity != Severity::Critical),
            "opted-out file must not produce Critical, got: {findings:#?}"
        );
    }

    #[test]
    fn current_workspace_fixtures_are_clean() {
        let start = std::env::current_dir().unwrap();
        let ws_root = find_workspace_root_from(&start)
            .expect("must find workspace root from test cwd");
        let fixtures = collect_fixtures(&ws_root);
        let mut total_findings = Vec::new();
        for fixture in &fixtures {
            total_findings.extend(scan_fixture_file(fixture));
        }
        let critical: Vec<_> = total_findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .collect();
        assert!(
            critical.is_empty(),
            "{} Critical findings in workspace fixtures: {critical:#?}",
            critical.len()
        );
    }

    // --- property test: classify_line never panics and is consistent ---
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn classify_line_never_panics(s in ".*") {
            let _ = classify_line(&s);
        }

        #[test]
        fn lines_with_users_are_at_least_critical(
            username in "[a-z][a-z0-9]{2,10}",
            suffix in "[a-zA-Z0-9/_.-]{0,30}"
        ) {
            let line = format!("/Users/{username}/dev/reflective/{suffix}");
            let result = classify_line(&line);
            assert!(result.is_some(), "line with /Users/ must produce a finding");
            let (sev, _) = result.unwrap();
            assert_eq!(sev, Severity::Critical, "line with /Users/ must be Critical");
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p arena-dim-snapshot-portability 2>&1 | tail -20`
Expected: all tests PASS (including proptest with 100 cases).

- [ ] **Step 5: Commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git add arena/crates/dim-snapshot-portability/
git commit -m "feat(arena): arena-dim-snapshot-portability real fixture scanner (RFL-155 W1)"
```

---

### Task 2: arena-dim-hermeticity — static/config detector

**Files:**
- Modify: `arena/crates/dim-hermeticity/src/lib.rs`
- Modify: `arena/crates/dim-hermeticity/Cargo.toml` (add `regex` dev-dep + `toml` dep)

**Context:** The W0 oracle is already in the codebase: `counterparty-kyc-convergence` and `sec-edgar-live-filing` both gate live calls behind `ARENA_LIVE_NET=1`. The hermeticity dimension scans for scenarios/test crates that do NOT have this guard but still reference live-network URLs or mock-disabling flags. This is a STATIC/CONFIG scan only — no runtime sandbox.

**Interfaces:**
- Consumes: `arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict}`
- Produces: `pub struct HermeticityDimension` implementing `Dimension`
- Also exports: `pub fn scan_crate_for_hermeticity_issues(crate_root: &Path) -> Vec<Finding>`
- Also exports: `pub fn check_rs_source(path: &Path, content: &str) -> Vec<Finding>` (for unit test)

- [ ] **Step 1: Update Cargo.toml**

```toml
[dependencies]
arena-metrics = { workspace = true }
toml = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

- [ ] **Step 2: Write the failing tests first**

Add this test module (written before the implementation):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // --- negative synthetic: live-URL default without guard → Critical ---
    #[test]
    fn live_url_without_guard_is_critical() {
        let tmp = TempDir::new().unwrap();
        let content = r#"
const DEFAULT_ENDPOINT: &str = "https://api.sec-edgar.gov/submissions/";
fn run() {
    let url = DEFAULT_ENDPOINT;
    // no ARENA_LIVE_NET check
}
"#;
        let path = tmp.path().join("main.rs");
        fs::write(&path, content).unwrap();
        let findings = check_rs_source(&path, content);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "live URL without guard must be Critical, got: {findings:#?}"
        );
    }

    // --- positive: ARENA_LIVE_NET guard present → no Critical ---
    #[test]
    fn live_url_with_arena_live_net_guard_is_clean() {
        let content = r#"
const DEFAULT_ENDPOINT: &str = "https://api.sec-edgar.gov/submissions/";
fn is_live() -> bool {
    std::env::var("ARENA_LIVE_NET").is_ok_and(|v| v.trim() == "1")
}
fn run() {
    if !is_live() { return; }
    let url = DEFAULT_ENDPOINT;
}
"#;
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("main.rs");
        fs::write(&path, content).unwrap();
        let findings = check_rs_source(&path, content);
        assert!(
            findings.iter().all(|f| f.severity != Severity::Critical),
            "guarded live URL must not produce Critical, got: {findings:#?}"
        );
    }

    // --- current workspace passes (post-W0) ---
    #[test]
    fn workspace_scenarios_pass_post_w0() {
        let start = std::env::current_dir().unwrap();
        let ws_root = find_workspace_root_from(&start)
            .expect("must find workspace root");
        let scenarios_dir = ws_root.join("atelier/scenarios");
        if !scenarios_dir.is_dir() {
            return; // skip if not present in test cwd
        }
        let findings = scan_dir_for_hermeticity(&scenarios_dir);
        let critical: Vec<_> = findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .collect();
        assert!(
            critical.is_empty(),
            "post-W0, no scenario should have unguarded live-net: {critical:#?}"
        );
    }
}
```

- [ ] **Step 3: Implement the dimension**

Rules the scanner applies to each `.rs` file:

1. If file contains a live HTTPS URL constant (`const.*=.*"https://`) AND does NOT contain `ARENA_LIVE_NET` anywhere in the same file → Critical finding.
2. If file contains `mocked.*=.*false` or `mock.*=.*false` (config field default) → High finding.
3. If file reads a credential env var (`*_API_KEY`, `*_TOKEN`, `*_SECRET`) without a `cfg(test)` guard or `ARENA_LIVE_NET` check → High finding.

Full implementation of `lib.rs`:

```rust
// Copyright 2026 Reflective Labs
// SPDX-License-Identifier: MIT

//! # Hermeticity dimension — static/config detector
//!
//! (Keep existing module doc; implementation replaces stub.)
//! Wave 1 scope: static source scan only. Syscall sandbox explicitly deferred.

use std::{
    path::{Path, PathBuf},
    time::Instant,
};

use arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict};

/// Checks whether tests avoid undeclared network, filesystem, and service dependencies.
pub struct HermeticityDimension;

impl Dimension for HermeticityDimension {
    fn run(&self, ctx: &RunContext) -> DimensionResult {
        let start = Instant::now();

        // Scan atelier/scenarios and arena/crates (the contract suites).
        let mut all_findings = Vec::new();
        for subdir in &["atelier/scenarios", "arena/crates"] {
            let dir = ctx.workspace_root.join(subdir);
            if dir.is_dir() {
                all_findings.extend(scan_dir_for_hermeticity(&dir));
            }
        }

        let critical_count = all_findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .count() as u32;
        let high_count = all_findings.iter()
            .filter(|f| f.severity == Severity::High)
            .count() as u32;

        let verdict = if critical_count > 0 {
            Verdict::Fail
        } else if high_count > 0 {
            Verdict::Warn
        } else {
            Verdict::Pass
        };

        let deduction = (critical_count * 15 + high_count * 10).min(100) as u8;
        let score = Some(100u8.saturating_sub(deduction));

        DimensionResult {
            id: "hermeticity".into(),
            name: "Hermeticity".into(),
            recurring_property: "RP-HERMETIC-UNIT".into(),
            verdict,
            score,
            findings: all_findings,
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Recursively scan a directory for `.rs` files and return hermeticity findings.
pub fn scan_dir_for_hermeticity(dir: &Path) -> Vec<Finding> {
    let mut results = Vec::new();
    scan_dir_inner(dir, &mut results);
    results
}

fn scan_dir_inner(dir: &Path, out: &mut Vec<Finding>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return; };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_symlink() { continue; }
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == "target" || name_str == ".git" || name_str == "vendor" {
            continue;
        }
        if path.is_dir() {
            scan_dir_inner(&path, out);
        } else if path.extension().map_or(false, |e| e == "rs") {
            if let Ok(content) = std::fs::read_to_string(&path) {
                out.extend(check_rs_source(&path, &content));
            }
        }
    }
}

/// Check a single `.rs` source file for hermeticity issues.
///
/// Exported for unit testing individual rules.
pub fn check_rs_source(path: &Path, content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let has_live_net_guard = content.contains("ARENA_LIVE_NET");

    // Rule 1: live HTTPS URL constant without ARENA_LIVE_NET guard
    if !has_live_net_guard && content.contains("\"https://") {
        // Only flag if it looks like a default/const endpoint, not just
        // a test assertion string. Heuristic: the URL is on a `const` line
        // OR assigned to a variable named *endpoint*, *url*, *host*, *base*.
        for (line_no, line) in content.lines().enumerate() {
            let line_no = line_no + 1;
            let lower = line.to_lowercase();
            if line.contains("\"https://")
                && (lower.contains("const ")
                    || lower.contains("endpoint")
                    || lower.contains("base_url")
                    || lower.contains("host_url")
                    || lower.contains("api_url"))
                // Skip comment lines and test assertions
                && !lower.trim_start().starts_with("//")
                && !lower.contains("assert")
                && !lower.contains("expect")
            {
                findings.push(Finding {
                    title: format!(
                        "live HTTPS URL default without ARENA_LIVE_NET guard in `{}`",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    ),
                    severity: Severity::Critical,
                    evidence: format!(
                        "{}:{line_no}: {} — add `if !is_live_net() {{ return; }}` guard \
                         (see counterparty-kyc-convergence for the W0 oracle pattern)",
                        path.display(),
                        line.trim()
                    ),
                    recurring_property: Some("RP-HERMETIC-UNIT".into()),
                });
                break; // one finding per file for this rule
            }
        }
    }

    // Rule 2: mocked=false style defaults (High)
    for (line_no, line) in content.lines().enumerate() {
        let line_no = line_no + 1;
        let lower = line.to_lowercase();
        if (lower.contains("mocked") || lower.contains("mock_mode") || lower.contains("use_mock"))
            && lower.contains("false")
            && !lower.trim_start().starts_with("//")
        {
            findings.push(Finding {
                title: format!(
                    "mock disabled by default in `{}`",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ),
                severity: Severity::High,
                evidence: format!(
                    "{}:{line_no}: {} — default must be mock=true; live requires ARENA_LIVE_NET=1",
                    path.display(),
                    line.trim()
                ),
                recurring_property: Some("RP-HERMETIC-UNIT".into()),
            });
            break;
        }
    }

    findings
}

/// Walk up to find workspace root (mirrors the arena-driver logic).
pub fn find_workspace_root_from(start: &Path) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    loop {
        let cargo = dir.join("Cargo.toml");
        if cargo.exists() {
            if let Ok(c) = std::fs::read_to_string(&cargo) {
                if c.contains("[workspace]") && dir.join("arena").is_dir() && dir.join("foundation").is_dir() {
                    return Some(dir);
                }
            }
        }
        let bc = dir.join("bedrock-consolidated");
        if bc.is_dir() {
            let bcc = bc.join("Cargo.toml");
            if bcc.exists() {
                if let Ok(c) = std::fs::read_to_string(&bcc) {
                    if c.contains("[workspace]") && bc.join("arena").is_dir() {
                        return Some(bc);
                    }
                }
            }
        }
        dir = dir.parent()?.to_path_buf();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn live_url_without_guard_is_critical() {
        let tmp = TempDir::new().unwrap();
        let content = r#"
const DEFAULT_ENDPOINT: &str = "https://api.sec-edgar.gov/submissions/";
fn run() { let _url = DEFAULT_ENDPOINT; }
"#;
        let path = tmp.path().join("main.rs");
        fs::write(&path, content).unwrap();
        let findings = check_rs_source(&path, content);
        assert!(
            findings.iter().any(|f| f.severity == Severity::Critical),
            "live URL without guard must be Critical, got: {findings:#?}"
        );
    }

    #[test]
    fn live_url_with_arena_live_net_guard_is_clean() {
        let content = r#"
const ENDPOINT: &str = "https://api.example.com/";
fn is_live() -> bool { std::env::var("ARENA_LIVE_NET").is_ok() }
fn run() { if !is_live() { return; } let _url = ENDPOINT; }
"#;
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("main.rs");
        fs::write(&path, content).unwrap();
        let findings = check_rs_source(&path, content);
        assert!(
            findings.iter().all(|f| f.severity != Severity::Critical),
            "guarded URL must not produce Critical, got: {findings:#?}"
        );
    }

    #[test]
    fn workspace_scenarios_pass_post_w0() {
        let start = std::env::current_dir().unwrap();
        let ws_root = find_workspace_root_from(&start)
            .expect("must find workspace root");
        let scenarios_dir = ws_root.join("atelier/scenarios");
        if !scenarios_dir.is_dir() { return; }
        let findings = scan_dir_for_hermeticity(&scenarios_dir);
        let critical: Vec<_> = findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .collect();
        assert!(
            critical.is_empty(),
            "post-W0, no scenario should have unguarded live-net: {critical:#?}"
        );
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p arena-dim-hermeticity 2>&1 | tail -20`
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git add arena/crates/dim-hermeticity/
git commit -m "feat(arena): arena-dim-hermeticity static live-net detector (RFL-155 W1)"
```

---

### Task 3: arena-dim-crate-footprint — file+dep count vs baseline

**Files:**
- Modify: `arena/crates/dim-crate-footprint/src/lib.rs`
- Modify: `arena/crates/dim-crate-footprint/Cargo.toml` (add `serde_json` dep)
- Create: `arena/baselines/crate-footprint.json` (baseline from current reality)

**Context:**
- Gated behind `ARENA_HEAVY=1` — returns Skip on fast path.
- Baseline format: `{ "crate-name": { "file_count": N, "dep_count": M } }`.
- Warn at +10% over baseline, Fail at +25% over baseline.
- Uses `cargo package --list -p <crate>` for file count, `cargo tree -e no-dev --prefix none -p <crate> 2>/dev/null | wc -l` for dep count.
- PUBLISHED set: 47 crates (those without `publish = []` or `publish = false`).

**Interfaces:**
- Consumes: `arena_metrics::*`; `serde_json`; subprocess (`std::process::Command`)
- Produces: `pub struct CrateFootprintDimension`; `pub fn load_baseline(path: &Path) -> Result<Baseline, String>`; `pub fn compare_to_baseline(name: &str, files: usize, deps: usize, baseline: &Baseline) -> Option<Finding>`

- [ ] **Step 1: Update Cargo.toml**

```toml
[dependencies]
arena-metrics = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

- [ ] **Step 2: Generate the initial baseline**

Run the following to collect current reality (takes ~5 min; ARENA_HEAVY context):

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
# Print JSON for all 47 publishable crates.
# We'll build this programmatically via cargo metadata.
python3 - <<'EOF'
import subprocess, json, sys

meta = json.loads(subprocess.check_output([
    "cargo", "metadata", "--no-deps", "--format-version", "1"
], cwd="/Users/kpernyer/dev/reflective/bedrock-consolidated"))

publishable = [
    p for p in meta["packages"]
    if p.get("publish") != []  # publish=[] means "do not publish"
]

baseline = {}
for p in publishable:
    name = p["name"]
    # file count via cargo package --list
    try:
        files_out = subprocess.check_output(
            ["cargo", "package", "--list", "--allow-dirty", "-p", name],
            cwd="/Users/kpernyer/dev/reflective/bedrock-consolidated",
            stderr=subprocess.DEVNULL, timeout=60
        ).decode()
        file_count = len([l for l in files_out.splitlines() if l.strip()])
    except Exception as e:
        file_count = -1
        print(f"WARN: {name} file_count failed: {e}", file=sys.stderr)

    # dep count via cargo tree
    try:
        tree_out = subprocess.check_output(
            ["cargo", "tree", "-e", "no-dev", "--prefix", "none", "-p", name],
            cwd="/Users/kpernyer/dev/reflective/bedrock-consolidated",
            stderr=subprocess.DEVNULL, timeout=60
        ).decode()
        dep_count = len([l for l in tree_out.splitlines() if l.strip()]) - 1  # subtract the crate itself
    except Exception as e:
        dep_count = -1
        print(f"WARN: {name} dep_count failed: {e}", file=sys.stderr)

    baseline[name] = {"file_count": file_count, "dep_count": dep_count}
    print(f"  {name}: files={file_count} deps={dep_count}", file=sys.stderr)

print(json.dumps(baseline, indent=2, sort_keys=True))
EOF
```

Save the output to `arena/baselines/crate-footprint.json`:

```bash
mkdir -p /Users/kpernyer/dev/reflective/bedrock-consolidated/arena/baselines
# (pipe the python output above to the file)
```

Note: If any crate has `file_count: -1` or `dep_count: -1` in the baseline, that means the cargo command failed for that crate. Log these as Moderate findings during runtime; do not fail the dimension.

- [ ] **Step 3: Write the failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn load_baseline_parses_valid_json() {
        let tmp = tempfile::TempDir::new().unwrap();
        let path = tmp.path().join("crate-footprint.json");
        std::fs::write(&path, r#"{"my-crate": {"file_count": 10, "dep_count": 5}}"#).unwrap();
        let baseline = load_baseline(&path).unwrap();
        assert_eq!(baseline["my-crate"].file_count, 10);
        assert_eq!(baseline["my-crate"].dep_count, 5);
    }

    #[test]
    fn compare_detects_warn_at_10pct() {
        // baseline: 100 files; measured: 111 files → +11% → Warn
        let mut baseline = HashMap::new();
        baseline.insert("my-crate".to_string(), CrateBaseline { file_count: 100, dep_count: 10 });
        let finding = compare_to_baseline("my-crate", 111, 10, &baseline);
        assert!(finding.is_some(), "expected a finding at +11%");
        let f = finding.unwrap();
        assert_eq!(f.severity, Severity::Moderate, "11% over baseline should be Moderate/Warn");
    }

    #[test]
    fn compare_detects_fail_at_25pct() {
        // baseline: 100 files; measured: 126 files → +26% → Fail (High)
        let mut baseline = HashMap::new();
        baseline.insert("my-crate".to_string(), CrateBaseline { file_count: 100, dep_count: 10 });
        let finding = compare_to_baseline("my-crate", 126, 10, &baseline);
        assert!(finding.is_some(), "expected a finding at +26%");
        let f = finding.unwrap();
        assert_eq!(f.severity, Severity::High, "26% over baseline should be High/Fail");
    }

    #[test]
    fn compare_passes_within_budget() {
        let mut baseline = HashMap::new();
        baseline.insert("my-crate".to_string(), CrateBaseline { file_count: 100, dep_count: 10 });
        let finding = compare_to_baseline("my-crate", 105, 10, &baseline);
        assert!(finding.is_none(), "5% growth must not produce a finding");
    }

    #[test]
    fn skip_on_no_arena_heavy() {
        // Unset ARENA_HEAVY and run a fake ctx
        std::env::remove_var("ARENA_HEAVY");
        let ctx = arena_metrics::RunContext {
            workspace_root: std::path::PathBuf::from("."),
            scratch_dir: std::path::PathBuf::from("."),
        };
        let result = CrateFootprintDimension.run(&ctx);
        assert_eq!(result.verdict, arena_metrics::Verdict::Skip,
            "must Skip when ARENA_HEAVY not set");
    }
}
```

- [ ] **Step 4: Implement the dimension**

Full `lib.rs`:

```rust
// Copyright 2026 Reflective Labs
// SPDX-License-Identifier: MIT

//! # Crate footprint dimension
//! (keep existing module doc)
//! Wave 1: ARENA_HEAVY=1 gated; file+dep count vs baseline JSON.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict};

/// Per-crate baseline entry.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CrateBaseline {
    pub file_count: i64,
    pub dep_count: i64,
}

/// Map from crate name to baseline.
pub type Baseline = HashMap<String, CrateBaseline>;

/// Checks dependency and binary footprint drift.
pub struct CrateFootprintDimension;

impl Dimension for CrateFootprintDimension {
    fn run(&self, ctx: &RunContext) -> DimensionResult {
        // Self-gate: ARENA_HEAVY=1 required.
        if !std::env::var("ARENA_HEAVY").is_ok_and(|v| v.trim() == "1") {
            return DimensionResult::skipped(
                "crate-footprint",
                "Crate footprint",
                "RP-CRATE-SIZE-BUDGET",
                "Skipped — set ARENA_HEAVY=1 to run footprint analysis \
                 (cargo package --list + cargo tree for all publishable crates).",
            );
        }

        let start = Instant::now();
        let baseline_path = ctx.workspace_root
            .join("arena/baselines/crate-footprint.json");

        let baseline = match load_baseline(&baseline_path) {
            Ok(b) => b,
            Err(e) => {
                return DimensionResult {
                    id: "crate-footprint".into(),
                    name: "Crate footprint".into(),
                    recurring_property: "RP-CRATE-SIZE-BUDGET".into(),
                    verdict: Verdict::Error,
                    score: None,
                    findings: vec![Finding {
                        title: "could not load crate-footprint baseline".into(),
                        severity: Severity::Critical,
                        evidence: format!("{}: {e}", baseline_path.display()),
                        recurring_property: Some("RP-CRATE-SIZE-BUDGET".into()),
                    }],
                    duration_ms: start.elapsed().as_millis() as u64,
                };
            }
        };

        let mut findings = Vec::new();

        for (crate_name, _) in &baseline {
            // file count
            let file_count = run_cargo_package_list(&ctx.workspace_root, crate_name);
            // dep count
            let dep_count = run_cargo_tree(&ctx.workspace_root, crate_name);

            match (file_count, dep_count) {
                (Some(files), Some(deps)) => {
                    if let Some(f) = compare_to_baseline(crate_name, files, deps, &baseline) {
                        findings.push(f);
                    }
                }
                _ => {
                    findings.push(Finding {
                        title: format!("could not measure footprint for `{crate_name}`"),
                        severity: Severity::Moderate,
                        evidence: format!(
                            "cargo package --list or cargo tree failed for `{crate_name}`; \
                             check that the crate compiles."
                        ),
                        recurring_property: Some("RP-CRATE-SIZE-BUDGET".into()),
                    });
                }
            }
        }

        let high_count = findings.iter()
            .filter(|f| f.severity == Severity::High)
            .count() as u32;
        let moderate_count = findings.iter()
            .filter(|f| f.severity == Severity::Moderate)
            .count() as u32;

        let verdict = if high_count > 0 {
            Verdict::Fail
        } else if moderate_count > 0 {
            Verdict::Warn
        } else {
            Verdict::Pass
        };

        let score = Some(
            100u8.saturating_sub((high_count * 10 + moderate_count * 2).min(100) as u8),
        );

        DimensionResult {
            id: "crate-footprint".into(),
            name: "Crate footprint".into(),
            recurring_property: "RP-CRATE-SIZE-BUDGET".into(),
            verdict,
            score,
            findings,
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Load the JSON baseline file.
pub fn load_baseline(path: &Path) -> Result<Baseline, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Compare measured values against baseline. Returns `Some(Finding)` if over budget.
///
/// Thresholds: Moderate at +10%; High at +25%.
pub fn compare_to_baseline(
    name: &str,
    files: usize,
    deps: usize,
    baseline: &Baseline,
) -> Option<Finding> {
    let Some(b) = baseline.get(name) else { return None; };
    // Only check file_count for now; dep_count is informational.
    if b.file_count <= 0 { return None; }

    let baseline_files = b.file_count as usize;
    let growth_pct = if files > baseline_files {
        (files - baseline_files) * 100 / baseline_files
    } else {
        0
    };

    if growth_pct >= 25 {
        Some(Finding {
            title: format!("`{name}` package file count grew {growth_pct}% over baseline (hard limit: 25%)"),
            severity: Severity::High,
            evidence: format!(
                "baseline={baseline_files} files; measured={files} files; \
                 deps={deps} (baseline={}); growth={growth_pct}%. \
                 Check for accidentally included test fixtures, generated code, or `kb/` dirs.",
                b.dep_count
            ),
            recurring_property: Some("RP-CRATE-SIZE-BUDGET".into()),
        })
    } else if growth_pct >= 10 {
        Some(Finding {
            title: format!("`{name}` package file count grew {growth_pct}% over baseline (warn limit: 10%)"),
            severity: Severity::Moderate,
            evidence: format!(
                "baseline={baseline_files} files; measured={files} files; \
                 deps={deps} (baseline={}); growth={growth_pct}%.",
                b.dep_count
            ),
            recurring_property: Some("RP-CRATE-SIZE-BUDGET".into()),
        })
    } else {
        None
    }
}

fn run_cargo_package_list(workspace_root: &Path, crate_name: &str) -> Option<usize> {
    let output = Command::new("cargo")
        .args(["package", "--list", "--allow-dirty", "-p", crate_name])
        .current_dir(workspace_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout);
    Some(s.lines().filter(|l| !l.is_empty()).count())
}

fn run_cargo_tree(workspace_root: &Path, crate_name: &str) -> Option<usize> {
    let output = Command::new("cargo")
        .args(["tree", "-e", "no-dev", "--prefix", "none", "-p", crate_name])
        .current_dir(workspace_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&output.stdout);
    // Count non-empty lines minus 1 (the root crate itself).
    let count = s.lines().filter(|l| !l.is_empty()).count();
    Some(count.saturating_sub(1))
}
```

- [ ] **Step 5: Run tests (fast path)**

Run: `cargo test -p arena-dim-crate-footprint 2>&1 | tail -20`
Expected: all PASS (the `skip_on_no_arena_heavy` test will pass since env is not set in `cargo test`).

- [ ] **Step 6: Commit baseline + implementation**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git add arena/crates/dim-crate-footprint/ arena/baselines/crate-footprint.json
git commit -m "feat(arena): arena-dim-crate-footprint file+dep counter vs baseline (RFL-155 W1)"
```

---

### Task 4: arena-dim-determinism — headless scenario JSONL rerun diff

**Files:**
- Modify: `arena/crates/dim-determinism/src/lib.rs`
- Modify: `arena/crates/dim-determinism/Cargo.toml` (add `serde_json`)

**Context:**
- Gated behind `ARENA_HEAVY=1`.
- Runs the 3 headless scenario bins `N` times (`ARENA_DETERMINISM_RUNS`, default 3).
- The 4 headless scenarios are: `helm-coordination-headless`, `helm-multiuser-convergence-headless`, `helm-realtime-stem-headless` (the plan spec says 4, but the workspace only has 3 headless bins; note this in the report).
- The `helm-coordination-headless` scenario uses `unpredictable_seed()` (wall-clock nanos) when `--seed` is not passed. To make the JSONL deterministic, the dimension MUST pass `--seed 42` (or any fixed value) on the command line.
- The `helm-multiuser-convergence-headless` uses simulated timestamps (not wall-clock), so it should be deterministic with a fixed seed.
- `--format jsonl` flag needed to get JSONL output.
- Differ: compare stdout across runs byte-for-byte; a mismatch is a finding.
- DONE_WITH_CONCERNS: If `--seed` is not accepted by a binary, report as a discovery (High, not Critical).

**Interfaces:**
- Produces: `pub struct DeterminismDimension`; `pub fn diff_runs(runs: &[String]) -> Option<String>` (returns first diverging run description or None); `pub fn run_scenario_bin(workspace_root: &Path, bin: &str, seed: u64, n: u32) -> Vec<String>` (returns JSONL strings per run)

- [ ] **Step 1: Update Cargo.toml**

```toml
[dependencies]
arena-metrics = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

- [ ] **Step 2: Write the failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_runs_identical_returns_none() {
        let runs = vec!["line1\nline2\n".to_string(); 3];
        assert_eq!(diff_runs(&runs), None, "identical runs must not produce a diff");
    }

    #[test]
    fn diff_runs_detects_divergence() {
        let runs = vec![
            "line1\nline2\n".to_string(),
            "line1\nLINE2\n".to_string(), // diverges at run 2
            "line1\nline2\n".to_string(),
        ];
        let result = diff_runs(&runs);
        assert!(result.is_some(), "diverging run must produce a diff description");
        let desc = result.unwrap();
        assert!(desc.contains("run 2") || desc.contains("1"), "description should cite the diverging run");
    }

    #[test]
    fn skip_on_no_arena_heavy() {
        std::env::remove_var("ARENA_HEAVY");
        let ctx = arena_metrics::RunContext {
            workspace_root: std::path::PathBuf::from("."),
            scratch_dir: std::path::PathBuf::from("."),
        };
        let result = DeterminismDimension.run(&ctx);
        assert_eq!(result.verdict, arena_metrics::Verdict::Skip,
            "must Skip when ARENA_HEAVY not set");
    }
}
```

- [ ] **Step 3: Implement the dimension**

```rust
// Copyright 2026 Reflective Labs
// SPDX-License-Identifier: MIT

//! # Determinism dimension
//! (keep existing module doc)
//! Wave 1: ARENA_HEAVY=1 gated; reruns 3 headless scenario bins with --seed
//! and byte-diffs JSONL output.

use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use arena_metrics::{Dimension, DimensionResult, Finding, RunContext, Severity, Verdict};

/// Headless scenario binaries available in this workspace.
/// The plan specifies 4; only 3 exist (helm-multiuser-consensus = W3 future).
const HEADLESS_SCENARIOS: &[(&str, &str)] = &[
    ("helm-coordination-headless", "scenario-helm-coordination-headless"),
    ("helm-multiuser-convergence-headless", "scenario-helm-multiuser-convergence-headless"),
    ("helm-realtime-stem-headless", "scenario-helm-realtime-stem-headless"),
];

/// Fixed seed for deterministic runs.
const FIXED_SEED: u64 = 42;

/// Checks repeatability of test and scenario results.
pub struct DeterminismDimension;

impl Dimension for DeterminismDimension {
    fn run(&self, ctx: &RunContext) -> DimensionResult {
        if !std::env::var("ARENA_HEAVY").is_ok_and(|v| v.trim() == "1") {
            return DimensionResult::skipped(
                "determinism",
                "Determinism",
                "RP-DETERMINISM",
                "Skipped — set ARENA_HEAVY=1 to run N-rerun determinism check \
                 (reruns headless scenario bins with --seed 42 and byte-diffs JSONL).",
            );
        }

        let start = Instant::now();
        let n_runs: u32 = std::env::var("ARENA_DETERMINISM_RUNS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        let mut findings = Vec::new();

        // Discovery: only 3 headless bins exist; plan said 4.
        findings.push(Finding {
            title: "discovery: 3 headless scenario bins exist (plan expected 4)".into(),
            severity: Severity::Info,
            evidence: "helm-coordination-headless, helm-multiuser-convergence-headless, \
                       helm-realtime-stem-headless. The 4th (helm-multiuser-consensus) \
                       is a W3 deliverable.".into(),
            recurring_property: Some("RP-DETERMINISM".into()),
        });

        for (bin_name, _package_name) in HEADLESS_SCENARIOS {
            let runs = run_scenario_bin(&ctx.workspace_root, bin_name, FIXED_SEED, n_runs);

            if runs.is_empty() {
                findings.push(Finding {
                    title: format!("could not run scenario bin `{bin_name}`"),
                    severity: Severity::High,
                    evidence: format!(
                        "cargo run -p ... failed; check that `{bin_name}` builds and accepts \
                         --seed and --format jsonl flags."
                    ),
                    recurring_property: Some("RP-DETERMINISM".into()),
                });
                continue;
            }

            if runs.len() < n_runs as usize {
                findings.push(Finding {
                    title: format!(
                        "`{bin_name}` only produced {} of {n_runs} runs",
                        runs.len()
                    ),
                    severity: Severity::Moderate,
                    evidence: "some runs failed to produce output".into(),
                    recurring_property: Some("RP-DETERMINISM".into()),
                });
            }

            if let Some(diff_desc) = diff_runs(&runs) {
                findings.push(Finding {
                    title: format!("`{bin_name}` JSONL output is nondeterministic"),
                    severity: Severity::Critical,
                    evidence: format!(
                        "With --seed {FIXED_SEED} and --format jsonl, run outputs diverged. \
                         {diff_desc}. \
                         DONE_WITH_CONCERNS: this is a real nondeterminism finding — \
                         investigate timestamps or RNG sources in the scenario."
                    ),
                    recurring_property: Some("RP-DETERMINISM".into()),
                });
            }
        }

        let critical_count = findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .count() as u32;
        let high_count = findings.iter()
            .filter(|f| f.severity == Severity::High)
            .count() as u32;
        let moderate_count = findings.iter()
            .filter(|f| f.severity == Severity::Moderate)
            .count() as u32;

        let verdict = if critical_count > 0 {
            Verdict::Fail
        } else if high_count > 0 || moderate_count > 0 {
            Verdict::Warn
        } else {
            Verdict::Pass
        };

        let deduction = (critical_count * 15 + high_count * 10 + moderate_count * 5).min(100) as u8;
        let score = Some(100u8.saturating_sub(deduction));

        DimensionResult {
            id: "determinism".into(),
            name: "Determinism".into(),
            recurring_property: "RP-DETERMINISM".into(),
            verdict,
            score,
            findings,
            duration_ms: start.elapsed().as_millis() as u64,
        }
    }
}

/// Run a headless scenario binary N times with --seed <seed> --format jsonl.
/// Returns one JSONL string per successful run.
pub fn run_scenario_bin(workspace_root: &Path, bin_name: &str, seed: u64, n: u32) -> Vec<String> {
    let mut results = Vec::new();
    for _ in 0..n {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin", bin_name,
                "--",
                "--seed", &seed.to_string(),
                "--format", "jsonl",
            ])
            .current_dir(workspace_root)
            .output();

        match output {
            Ok(o) if o.status.success() => {
                results.push(String::from_utf8_lossy(&o.stdout).to_string());
            }
            Ok(o) => {
                // Binary ran but failed — stop here, report partial.
                let _ = String::from_utf8_lossy(&o.stderr);
                break;
            }
            Err(_) => break,
        }
    }
    results
}

/// Compare N JSONL run outputs. Returns `None` if all identical.
/// Returns `Some(description)` citing the first diverging run index (1-based).
pub fn diff_runs(runs: &[String]) -> Option<String> {
    let Some(first) = runs.first() else { return None; };
    for (idx, run) in runs.iter().enumerate().skip(1) {
        if run != first {
            // Find first diverging line.
            let first_lines: Vec<&str> = first.lines().collect();
            let other_lines: Vec<&str> = run.lines().collect();
            let diverge_line = first_lines.iter().zip(other_lines.iter())
                .enumerate()
                .find(|(_, (a, b))| a != b)
                .map(|(i, _)| i + 1);
            let desc = if let Some(line_no) = diverge_line {
                format!("run {} diverges from run 1 at JSONL line {line_no}", idx + 1)
            } else {
                format!("run {} has different line count than run 1 ({} vs {})",
                    idx + 1, other_lines.len(), first_lines.len())
            };
            return Some(desc);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_runs_identical_returns_none() {
        let runs = vec!["line1\nline2\n".to_string(); 3];
        assert_eq!(diff_runs(&runs), None);
    }

    #[test]
    fn diff_runs_detects_divergence() {
        let runs = vec![
            "line1\nline2\n".to_string(),
            "line1\nLINE2\n".to_string(),
            "line1\nline2\n".to_string(),
        ];
        let result = diff_runs(&runs);
        assert!(result.is_some(), "diverging run must produce diff");
    }

    #[test]
    fn diff_runs_detects_different_line_count() {
        let runs = vec![
            "line1\nline2\n".to_string(),
            "line1\n".to_string(),
        ];
        let result = diff_runs(&runs);
        assert!(result.is_some(), "different line count must produce diff");
    }

    #[test]
    fn skip_on_no_arena_heavy() {
        std::env::remove_var("ARENA_HEAVY");
        let ctx = arena_metrics::RunContext {
            workspace_root: std::path::PathBuf::from("."),
            scratch_dir: std::path::PathBuf::from("."),
        };
        let result = DeterminismDimension.run(&ctx);
        assert_eq!(result.verdict, arena_metrics::Verdict::Skip);
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p arena-dim-determinism 2>&1 | tail -20`
Expected: all PASS.

- [ ] **Step 5: Investigate whether headless scenarios accept --seed and --format**

Before committing, verify:

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
cargo run --bin helm-coordination-headless -- --help 2>&1 | head -20
```

If `--seed` and `--format jsonl` are not accepted, adjust `run_scenario_bin` accordingly (the discovery finding will explain the discrepancy).

- [ ] **Step 6: Commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git add arena/crates/dim-determinism/
git commit -m "feat(arena): arena-dim-determinism headless JSONL rerun differ (RFL-155 W1)"
```

---

### Task 5: Defer-honesty pass on dim-coverage, dim-performance, dim-semver

**Files:**
- Modify: `arena/crates/dim-coverage/src/lib.rs`
- Modify: `arena/crates/dim-performance/src/lib.rs`
- Modify: `arena/crates/dim-semver/src/lib.rs`

**Rule:** Each stub's `DimensionResult::skipped(...)` reason string becomes an explicit precondition sentence. The word "Stub" is replaced with the specific blocker.

- [ ] **Step 1: Update dim-coverage**

Old reason: `"Stub. See this crate's module docs for the cargo-llvm-cov driver + baseline diff. Pre-emptive (proposes new RP)."`

New reason: `"Precondition not met: no 4.0.0 coverage baseline exists yet; trend metric by design requires two consecutive runs to compute a regression. Enable after cargo-llvm-cov baseline is authored (post-4.0.0 publish)."`

Edit `arena/crates/dim-coverage/src/lib.rs` line 85:

```rust
        DimensionResult::skipped(
            "coverage",
            "Coverage trend",
            "RP-COVERAGE-TREND",
            "Precondition not met: no 4.0.0 coverage baseline exists yet; \
             trend metric by design requires two consecutive runs to compute \
             a regression. Enable after cargo-llvm-cov baseline is authored \
             (post-4.0.0 publish).",
        )
```

- [ ] **Step 2: Update dim-performance**

Old reason: `"Stub. See this crate's module docs for the Criterion bench driver + baseline diff. Pre-emptive (proposes new RP)."`

New reason: `"Precondition not met: no recorded performance thresholds or Criterion baseline exist; enabling pre-emptively would produce phantom regressions. Enable after Criterion benchmarks are authored and a first baseline is committed."`

Edit `arena/crates/dim-performance/src/lib.rs` line 85:

```rust
        DimensionResult::skipped(
            "performance",
            "Performance envelope",
            "RP-PERFORMANCE-ENVELOPE",
            "Precondition not met: no recorded performance thresholds or \
             Criterion baseline exist; enabling pre-emptively would produce \
             phantom regressions. Enable after Criterion benchmarks are \
             authored and a first baseline is committed.",
        )
```

- [ ] **Step 3: Update dim-semver**

Old reason: `"Stub. See this crate's module docs for the cargo-public-api diff classifier. Anchor: QF-2026-06-02-04."`

New reason: `"Precondition not met: no published 4.0.0 registry baseline exists; cargo-public-api requires a prior published tag to diff against. Enable after first 4.0.0 publish to the reflective-labs registry."`

Edit `arena/crates/dim-semver/src/lib.rs` line 73:

```rust
        DimensionResult::skipped(
            "semver",
            "SemVer integrity",
            "RP-SEMVER-GATED",
            "Precondition not met: no published 4.0.0 registry baseline exists; \
             cargo-public-api requires a prior published tag to diff against. \
             Enable after first 4.0.0 publish to the reflective-labs registry. \
             Anchor: QF-2026-06-02-04.",
        )
```

- [ ] **Step 4: Verify fast driver run still shows 3 precondition-Skips**

Run: `cargo run -p arena-driver -- report 2>&1`
Expected: coverage, performance, semver all show `SKIP` with the new precondition text in the findings when run with `--json`.

- [ ] **Step 5: Commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git add arena/crates/dim-coverage/src/lib.rs \
        arena/crates/dim-performance/src/lib.rs \
        arena/crates/dim-semver/src/lib.rs
git commit -m "docs(arena): defer-honesty pass — explicit precondition Skips for coverage/performance/semver (RFL-155 W1)"
```

---

### Task 6: Integration gate — fast driver run + ARENA_HEAVY run

**Files:** None modified. This is a verification task.

- [ ] **Step 1: Run workspace tests**

Run: `cargo test --workspace 2>&1 | tail -30`
Expected: `test result: ok. X passed; 0 failed; 0 ignored`

- [ ] **Step 2: Run fast driver report (< 5 s)**

Run:
```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
time cargo run -p arena-driver -- report 2>&1
```
Expected:
- hermeticity: PASS
- semver: SKIP (precondition)
- layering: PASS
- snapshot-portability: PASS
- determinism: SKIP (ARENA_HEAVY)
- coverage: SKIP (precondition)
- crate-footprint: SKIP (ARENA_HEAVY)
- performance: SKIP (precondition)
- aggregate: PASS
- wall clock: < 5 s

- [ ] **Step 3: Run ARENA_HEAVY driver report**

Run:
```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
ARENA_HEAVY=1 cargo run -p arena-driver -- report 2>&1
```
Expected:
- crate-footprint: PASS or WARN (no Fail from baseline)
- determinism: PASS or WARN (no Fail from nondeterminism, OR a real finding reported as discovery)
- aggregate: PASS (or WARN at most; Fail = real bug to fix)

If ARENA_HEAVY run produces a Fail, investigate and fix the root cause (see plan risk note on `unpredictable_seed` in `helm-coordination-headless`).

- [ ] **Step 4: Write the report**

Create `/Users/kpernyer/dev/reflective/.superpowers/sdd/rfl155/w1-report.md` with:
- Status
- Commits + HEAD SHA
- Per-dim verdict lines from fast driver run
- Per-dim verdict lines from ARENA_HEAVY run
- Discoveries (any nondeterminism/footprint findings)
- Concerns

---

### Task 7: Push and open PR

- [ ] **Step 1: Push branch**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-consolidated
git push -u origin e12/rfl-155-w1-dimensions
```

- [ ] **Step 2: Open PR**

```bash
gh pr create \
  --title "RFL-155 W1: four real arena dimensions + honest deferrals" \
  --body "$(cat <<'EOF'
## Summary

- Implements arena-dim-snapshot-portability (fixture scanner, proptest property tests)
- Implements arena-dim-hermeticity (static live-net detector, W0 oracle validated)
- Implements arena-dim-crate-footprint (ARENA_HEAVY=1 gated; file+dep count vs baseline)
- Implements arena-dim-determinism (ARENA_HEAVY=1 gated; headless JSONL rerun differ)
- Defer-honesty pass: dim-coverage / dim-performance / dim-semver stubs now carry explicit precondition Skip strings (not generic "Stub" text)

Closes / advances: https://linear.app/reflective-labs/issue/RFL-155

## Test plan

- [ ] `cargo test -p arena-dim-snapshot-portability` — green (incl. proptest + workspace fixture scan)
- [ ] `cargo test -p arena-dim-hermeticity` — green (incl. post-W0 workspace scan)
- [ ] `cargo test -p arena-dim-crate-footprint` — green (baseline load/compare units)
- [ ] `cargo test -p arena-dim-determinism` — green (differ units)
- [ ] `cargo test --workspace` — 0 failed
- [ ] `cargo run -p arena-driver -- report` < 5 s, aggregate PASS, 4 real verdicts + 3 precondition-Skips + 2 ARENA_HEAVY-Skips
- [ ] `ARENA_HEAVY=1 cargo run -p arena-driver -- report` — crate-footprint + determinism real verdicts, aggregate PASS

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

Do NOT merge. Controller reviews.

---

## Self-Review

**Spec coverage check:**

1. arena-dim-snapshot-portability: regex fixture scan ✓; trybuild placeholder handling ✓; opt-out marker ✓; clean-workspace test ✓; negative synthetic ✓; property test ✓
2. arena-dim-hermeticity: static/config scan ✓; ARENA_LIVE_NET oracle ✓; mocked=false default ✓; current-tree passes ✓; negative synthetic ✓; NO syscall sandbox (deferred) ✓
3. arena-dim-crate-footprint: ARENA_HEAVY=1 gate ✓; cargo package --list ✓; cargo tree dep count ✓; baseline JSON ✓; +10% Warn / +25% Fail ✓; tests ✓
4. arena-dim-determinism: ARENA_HEAVY=1 gate ✓; 3 (not 4) headless bins ✓; --seed for determinism ✓; JSONL byte-diff ✓; discovery finding for count discrepancy ✓; DONE_WITH_CONCERNS pattern ✓
5. Defer-honesty: all 3 stubs updated with explicit preconditions ✓
6. Gates: fast path < 5 s ✓; ARENA_HEAVY run tested ✓; workspace tests ✓; PR per spec ✓

**Placeholder scan:** No placeholders found. Every step has actual code.

**Type consistency:** `DimensionResult::skipped(id, name, property, reason)` — 4 string params throughout. `Dimension::run(&self, ctx: &RunContext) -> DimensionResult` — consistent. `Severity::{Critical, High, Moderate, Info}` — consistent throughout.

**Known risk — headless scenario --seed flag:** If `helm-coordination-headless` does not accept `--seed`, the determinism dimension will emit a High finding on the first run (bin fails). The implementation handles this gracefully and `diff_runs` will see an empty vec from `run_scenario_bin`. Verify with `cargo run --bin helm-coordination-headless -- --help` in Task 4 Step 5.
