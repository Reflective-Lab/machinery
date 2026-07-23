## 2026-07-10

- **A local git tag silently diverged from the remote's tag of the same name.** During the RFL-195 wave, API mappings were derived by reading `git show v4.0.0:<path>` in the local framework/bedrock checkout — but its `v4.0.0` pointed at fa2c6d1 while cargo resolved the remote tag to 8fe9ff9 (upstream had retagged; `git fetch --tags` does not move an existing local tag without `--force`). Several "verified" API facts were stale; the port agents caught it because `cargo check` compiles against the remote tag. Rule: when a tag is the contract, treat cargo's resolved checkout (`~/.cargo/git/checkouts`) as ground truth, or `git fetch --force --tags` first.
- **Scout verdict "Cargo.toml surgery, zero source edits" failed on contact for runway.** converge 3.4→4.0 carried real API drift (ContextView→Context trait, typed ProposedFact/UnitInterval, select_chat_backend moved to manifold, storage `local` feature deleted) that cost a two-crate source port (~60 errors). The scouts had verified dep-surface existence, not consumer compilation. A scout report that clears a migration should include a cheap compile probe (`cargo check` with the new deps on a throwaway branch); name-level dep auditing systematically under-detects breaking API drift.
- **Renaming a CI secret reference in workflow YAML is a two-actor operation with a silent gap in the middle.** When Shipyard was dropped mid-session, `secrets.SHIPYARD_SSH_KEY` was renamed to `secrets.BEDROCK_SSH_KEY` across 5 runway workflows — a clean, reviewable text change. But an agent can edit the *reference* and cannot create the *actual GitHub secret* (that's an org-admin action). The result: CI silently breaks the next time those workflows run, with no error until then, until a human creates the new secret. Whenever a commit renames a `secrets.<NAME>` reference, say so explicitly and treat it as a blocking action item for the human — don't let it read as "done" alongside the code change it rode in on.
- **A Linear issue's status can say Done while its own description checklist still has unchecked, unresolved items.** RFL-195 showed `status: Done` at session close, but Phase 0 (RFL-194 gate) and Phase 5 (gap-table sign-off, repo retirement) were still open — nothing in this session closed them, and nothing indicated who or what flipped the issue to Done. Status and checklist state can drift independently; don't infer "nothing left to do" from `status: Done` alone when the description carries its own tracked checklist — read both, and comment out the mismatch rather than resolve it silently.

- **`git subtree add --squash` breaks intra-workspace relative `Cargo.toml` path deps.** Consolidating build-depot, runtime-runway, commerce-rails, and chart-room into one `machinery` repo moved runtime-runway from `reflective/runtime-runway/` to `reflective/machinery/runtime-runway/` — one directory level deeper. Two crates (`runway-storage`, `runway-app-host`) had hardcoded relative paths to `../../../bedrock-platform/helms/contracts/crates/helm-*` that no longer resolved, and the `helm-event-substrate`/`helm-module-contracts` version pins (`0.1.0`/`0.3.0`) were stale against bedrock's actual `4.0.0`. `cargo check` failed with an opaque "No such file or directory" several directories removed from the real cause. After any repo consolidation that changes nesting depth, grep every `Cargo.toml` for relative `path = "../.."` deps before trusting a green build.
- **`cargo test` output truncation nearly produced a false "2 tests" report for runtime-runway.** The harness truncated a large `just test` run to a 2KB preview plus a saved file; naively reading the tail suggested only "2 passed" when the real total (grepping every `test result:` line and summing) was 405. Always sum all `test result:` lines from the full saved output — never trust a truncated preview's tail as the complete picture for multi-crate `cargo test --workspace` runs.

## 2026-07-06

- Svelte 5 runes mode (`runes: true` in svelte.config.js) silently breaks reactivity for plain `let` declarations — they are no longer reactive. Reactive state requires `$state()`. Event handlers require `onclick=` not `on:click=`. The build succeeds and the page renders correctly; the only symptom is that interactions do nothing. Check the config before debugging event handlers in any Zone site.

- "Governed" as an adjective has become a LinkedIn buzzword with no signal — overused to the point of meaninglessness. During the beacon-sites repositioning session, replacing it with vocabulary from the article series ("traceable commitment," "shared model," "organizational intelligence," "coordinated execution") produced copy that is more specific, harder to copy, and internally consistent with the theory. Same verdict for "agentic AI." Enforce this as a vocabulary standard: if an adjective could appear on any AI vendor's site without changing meaning, it should not appear on Reflective's.

## 2026-07-05

- **GitHub ubuntu-latest is a single 72 GB root partition.** `/mnt` is mounted from the same `/dev/root` device — not a separate disk. Redirecting `CARGO_TARGET_DIR`/`CARGO_HOME` to `/mnt` is a no-op for disk space purposes. The actual fix for large Rust dep trees (polars + ort + burn) is `CARGO_PROFILE_DEV_DEBUG=0` + `CARGO_PROFILE_TEST_DEBUG=0`, which strips debug symbols and cuts artifact size ~3-5×.
- **bedrock-platform does not exist as a GitHub repo.** It is a local umbrella directory only. Each component (axiom, converge, organism, helms) is its own `Reflective-Lab/<name>` repo. CI scripts must clone them individually; any script that references `Reflective-Lab/bedrock-platform` will 403 immediately. *(Superseded 2026-07-07: `Reflective-Lab/bedrock-platform` now exists on GitHub and fetches fine — true when written.)*
- **Org-level `REFLECTIVE_SIBLINGS_TOKEN` needs `admin:org` scope to set.** `gh secret set --org` fails silently on 403 until you run `gh auth refresh -h github.com -s admin:org` first.

## 2026-07-03

- Two independent agent sessions writing QUALITY_BACKLOG.md in the same hour
  collided on finding IDs (both minted QF-2026-07-02-02/-03). Date-sequence
  IDs have no concurrency story; renumbered ours to -06/-07. If it recurs,
  worth a finding: ID reservation or per-agent suffix.
- The hermetic-audit gate's only real catch was a loopback request —
  `unshare -n` leaves `lo` DOWN, so an in-process server test failed. The
  seal's boundary is external network; loopback-to-self is inside it.
- OR-Tools v9.15 + vendor HiGHS v1.14.0 disagree on setLocalOptionValue's
  signature; the --copy-dt-needed-entries rustflag never worked (positional,
  lands after the -l list). First SHIM marker in the fleet now tracks it.
- The `done` playbook itself still pointed at MILESTONES.md and ~/dev/work —
  playbooks migrated 2026-07-02 carried pre-Linear content. Rewritten.
