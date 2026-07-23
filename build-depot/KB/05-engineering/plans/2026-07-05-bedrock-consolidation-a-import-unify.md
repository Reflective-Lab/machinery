# Bedrock Consolidation Plan A: Import & Unify (Phases 0–3) — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Collapse 13 repos (axiom, converge, organism, 8 mosaic extensions + mosaic shell, arena-tests, atelier-showcase) into the `bedrock-platform` repo as one Cargo workspace at lockstep 4.0.0, history preserved, tests green.

**Architecture:** Per the approved spec `KB/05-engineering/specs/2026-07-05-bedrock-platform-consolidation-design.md`. Each source repo is filter-repo'd into a target subdirectory and merged with `--allow-unrelated-histories`; then one root workspace replaces the ~13 old workspace roots; then names and publish flags are normalized. Helms is NOT touched by this plan (Plan B, gated on RFL-128). **Dual-track (spec decision 13):** everything is built in a fresh clone at `~/dev/reflective/bedrock-consolidated` and merges to branch `consolidation/main` — the current multi-repo setup keeps building unchanged until an explicit cutover, and source repos are only ever cloned, never modified.

**Tech Stack:** git-filter-repo (installed at `/opt/homebrew/bin/git-filter-repo`), cargo (edition 2024, rust 1.96.0), just, python3 for audit scripts.

## Global Constraints

- Lockstep version: **4.0.0** for every crate; `version.workspace = true` everywhere.
- `edition = "2024"`, `rust-version = "1.96.0"` in root `workspace.package` (already uniform).
- Strict package naming `{subsystem}-{crate}`; renames land BEFORE any publish (nothing publishes in Plan A).
- Every internal dep in root `workspace.dependencies` carries `registry = "reflective-labs"` alongside `path` + `version`.
- `publish = false` for: arena crates, atelier crates, the 15 `embassy-*` ports.
- **The current setup is never modified**: source repos are cloned read-only; no move-asides, no `.gitignore` edits in the live shell checkout, no root-Justfile repoints. All work lands in the fresh clone; the only writes outside it are the additive helms adoption branch (Task 5) and pushes to `consolidation/main`.
- Helms content stays out of the consolidated tree (Plan B); the nested helms repo stays where it is.
- Helm-dependent arena/atelier members are imported but **excluded from the workspace** (tracked in the Linear issue; re-included by Plan B).
- There is no freeze: the manifest is a **snapshot** of the mains. Later upstream commits (e.g. RFL-128 when it lands) sync in via the recipe in Task 10.
- No shims, no disabled tests without a same-day Linear issue (RP-BRANCH-HYGIENE).
- Work happens on issue branch `e{N}/rfl-{ID}-consolidation-import-unify` in the fresh clone; PR targets `consolidation/main` (created in Task 10), NOT `main`.

**Execution home:** all commands run with `cd ~/dev/reflective` unless the step says otherwise. `SCRATCH=~/dev/reflective/.attic-consolidation` holds import clones; `CONSOLIDATED=~/dev/reflective/bedrock-consolidated` is the fresh working clone (Task 2).

---

### Task 1: Linear issue, snapshot manifest, prerequisites

**Files:**
- Create: `~/dev/reflective/.attic-consolidation/snapshot-manifest.json` (committed into the clone by Task 2)
- Create: `~/dev/reflective/.attic-consolidation/` (directory, outside any repo)

**Interfaces:**
- Produces: `snapshot-manifest.json` — `{ "<repo-path>": { "sha": "<40-hex>", "branch": "main" } }` for all 13 source repos. Tasks 3–5 verify every clone against it.
- Produces: the Linear issue ID used in the branch name by Task 2.

- [ ] **Step 1: Create the Linear issue**

Use the Linear MCP (`mcp__linear-server__save_issue`), team RFL, title
`Consolidation Plan A: import 13 repos into bedrock-platform and unify workspace`,
description linking the spec path and this plan path. Record the issue ID
(e.g. RFL-140) — it names the branch in Task 2 and receives the exclusion
list in Task 5.

- [ ] **Step 2: Verify every source repo is clean, on main, and pushed**

```bash
cd ~/dev/reflective
REPOS="bedrock-platform/axiom bedrock-platform/converge bedrock-platform/organism \
mosaic-extensions/arbiter-policy mosaic-extensions/crucible-models mosaic-extensions/embassy-ports \
mosaic-extensions/ferrox-solvers mosaic-extensions/manifold-adapters mosaic-extensions/mnemos-knowledge \
mosaic-extensions/prism-analytics mosaic-extensions/soter-smt arena-tests atelier-showcase mosaic-extensions"
for r in $REPOS; do
  dirty=$(git -C $r status --porcelain | wc -l | tr -d ' ')
  git -C $r fetch origin main --quiet
  ahead=$(git -C $r rev-list origin/main..main --count)
  behind=$(git -C $r rev-list main..origin/main --count)
  echo "$r dirty=$dirty ahead=$ahead behind=$behind"
done
```

Expected: every line `dirty=0 ahead=0 behind=0` (the check is against each
repo's `main`; arena/atelier sitting on RFL-128 branches is fine — those
branch deltas are NOT imported; they sync in via the Task 10 recipe after
RFL-128 lands, and Plan B consumes them). A dirty or unpushed `main` is the
only STOP condition.

- [ ] **Step 3: Write the snapshot manifest**

```bash
mkdir -p ~/dev/reflective/.attic-consolidation
python3 - <<'EOF'
import json, subprocess
repos = ["bedrock-platform/axiom","bedrock-platform/converge","bedrock-platform/organism",
"mosaic-extensions/arbiter-policy","mosaic-extensions/crucible-models","mosaic-extensions/embassy-ports",
"mosaic-extensions/ferrox-solvers","mosaic-extensions/manifold-adapters","mosaic-extensions/mnemos-knowledge",
"mosaic-extensions/prism-analytics","mosaic-extensions/soter-smt","arena-tests","atelier-showcase","mosaic-extensions"]
import os; os.chdir(os.path.expanduser("~/dev/reflective"))
m = {r: {"sha": subprocess.check_output(["git","-C",r,"rev-parse","main"]).decode().strip(),
         "branch": "main"} for r in repos}
out = os.path.expanduser("~/dev/reflective/.attic-consolidation/snapshot-manifest.json")
open(out,"w").write(json.dumps(m, indent=2) + "\n")
EOF
cat ~/dev/reflective/.attic-consolidation/snapshot-manifest.json
```

Expected: JSON with 14 entries, each a 40-hex SHA. (Copied into the fresh
clone and committed in Task 2.)

- [ ] **Step 4: Verify git-filter-repo works**

```bash
git filter-repo --version
```

Expected: a version string (e.g. `2.4x.x`). If missing: `brew install git-filter-repo`.

---

### Task 2: Create the consolidated working clone

**Files:**
- Create: `~/dev/reflective/bedrock-consolidated/` (fresh clone of bedrock-platform)
- Modify (in the clone only): `.gitignore` (remove `/axiom/`, `/converge/`, `/organism/` lines; keep `/helms/`)
- Create (in the clone): `kb/consolidation/snapshot-manifest.json` (copied from Task 1)

**Interfaces:**
- Produces: work branch `e12/rfl-{ID}-consolidation-import-unify` in `$CONSOLIDATED`, a clone with NO nested checkouts (fresh clones don't have them — they're separate repos), so import merges cannot collide with anything. The live `~/dev/reflective/bedrock-platform` checkout is not touched by this or any later task.

- [ ] **Step 1: Fresh clone + work branch**

```bash
git clone git@github.com:Reflective-Lab/bedrock-platform.git ~/dev/reflective/bedrock-consolidated
cd ~/dev/reflective/bedrock-consolidated
git checkout -b e12/rfl-{ID}-consolidation-import-unify   # {ID} from Task 1
ls   # note: no axiom/ converge/ organism/ helms/ — nested repos never clone
```

Expected: clean clone of the shell content (docs, AGENTS.md, Justfile, kb),
no nested subsystem directories.

- [ ] **Step 2: Un-ignore the import paths (in the clone), commit manifest + gitignore**

Edit `.gitignore` in the clone: delete the `/axiom/`, `/converge/`,
`/organism/` lines. Keep `/helms/` (Plan B). Then:

```bash
cd ~/dev/reflective/bedrock-consolidated
mkdir -p kb/consolidation
cp ~/dev/reflective/.attic-consolidation/snapshot-manifest.json kb/consolidation/
git add .gitignore kb/consolidation/snapshot-manifest.json
git commit -m "chore(consolidation): snapshot manifest + open tree for foundation imports (RFL-{ID})"
```

Expected: commit created; `git status` clean.

---

### Task 3: Import axiom, converge, organism → `foundation/`

**Files:**
- Create: `~/dev/reflective/.attic-consolidation/import-one.sh` (reusable import script)
- Create (by merges): `bedrock-platform/foundation/{axiom,converge,organism}/**`

**Interfaces:**
- Consumes: `snapshot-manifest.json` (Task 1), open shell repo (Task 2).
- Produces: `import-one.sh <source-path> <target-subdir> <repo-key>` — used again by Tasks 4–5. History-preserving merge per subsystem.

- [ ] **Step 1: Write the reusable import script**

```bash
cat > ~/dev/reflective/.attic-consolidation/import-one.sh <<'EOF'
#!/bin/bash
# import-one.sh <source-repo-path> <target-subdir> <manifest-key>
# Fresh-clones the source (READ-ONLY on the original), verifies against the
# snapshot manifest, rewrites history into <target-subdir>, merges into the
# current branch of the consolidated clone.
set -euo pipefail
SRC="$1"; TARGET="$2"; KEY="$3"
ROOT=~/dev/reflective
SCRATCH=~/dev/reflective/.attic-consolidation
BEDROCK="$ROOT/bedrock-consolidated"
WANT=$(python3 -c "import json;print(json.load(open('$BEDROCK/kb/consolidation/snapshot-manifest.json'))['$KEY']['sha'])")
CLONE="$SCRATCH/import-$(basename $TARGET)"
rm -rf "$CLONE"
git clone --quiet "$ROOT/$SRC" "$CLONE"
GOT=$(git -C "$CLONE" rev-parse main)
[ "$GOT" = "$WANT" ] || { echo "SHA MISMATCH for $KEY: want $WANT got $GOT"; exit 1; }
git -C "$CLONE" filter-repo --to-subdirectory-filter "$TARGET" --force
cd "$BEDROCK"
git remote add "imp-$(basename $TARGET)" "$CLONE"
git fetch --quiet "imp-$(basename $TARGET)"
git merge --allow-unrelated-histories --no-edit \
  -m "feat(consolidation): import $KEY -> $TARGET (history preserved, $WANT)" \
  "imp-$(basename $TARGET)/main"
git remote remove "imp-$(basename $TARGET)"
echo "OK: $KEY -> $TARGET"
EOF
chmod +x ~/dev/reflective/.attic-consolidation/import-one.sh
```

Expected: script exists, executable.

- [ ] **Step 2: Import the three foundation subsystems**

Source paths are the live nested checkouts — read-only (import-one.sh only
`git clone`s them):

```bash
cd ~/dev/reflective/bedrock-consolidated
IMP=~/dev/reflective/.attic-consolidation/import-one.sh
$IMP bedrock-platform/axiom    foundation/axiom    bedrock-platform/axiom
$IMP bedrock-platform/converge foundation/converge bedrock-platform/converge
$IMP bedrock-platform/organism foundation/organism bedrock-platform/organism
```

Expected: three `OK:` lines, three merge commits.

- [ ] **Step 3: Verify history survived**

```bash
git log --oneline --follow foundation/converge/crates/kernel/Cargo.toml | tail -3
git log --oneline foundation/axiom | head -3
```

Expected: pre-import commits visible (authors/dates from the converge repo,
not just today's merge).

- [ ] **Step 4: Verify each subsystem still builds standalone**

The organism → converge relative paths (`../converge/crates/*`) still
resolve inside `foundation/`. Their own workspace roots are still present
until Task 7, so:

```bash
(cd foundation/converge && cargo test --quiet 2>&1 | tail -3)
(cd foundation/organism && cargo test --quiet 2>&1 | tail -3)
(cd foundation/axiom    && cargo test --quiet 2>&1 | tail -3)
```

Expected: each ends with `test result: ok` lines (axiom pulls
`converge-manifold-adapters` 1.1.2 from crates.io for now — flipped in
Task 8). If a failure appears, STOP: it existed pre-import (repos were
frozen green) or the move broke a path — diagnose before continuing.
No commit needed (Step 2's merges are the commits).

---

### Task 4: Import the 8 extension repos + mosaic shell → `extensions/`

**Files:**
- Create (by merges): `bedrock-platform/extensions/{arbiter,crucible,embassy,ferrox,manifold,mnemos,prism,soter}/**`
- Create (by merge): `bedrock-platform/extensions/integration-harness/**`, `bedrock-platform/kb/mosaic/**`

**Interfaces:**
- Consumes: `import-one.sh` (Task 3).
- Produces: all extension sources in-tree. Their `../../bedrock-platform/...` path deps are BROKEN until Task 7 — expected and accepted; the gate for this task is tree + history correctness, not compilation.

- [ ] **Step 1: Import the 8 nested extension repos (suffixes dropped)**

```bash
cd ~/dev/reflective/bedrock-consolidated
IMP=~/dev/reflective/.attic-consolidation/import-one.sh
$IMP mosaic-extensions/arbiter-policy     extensions/arbiter   mosaic-extensions/arbiter-policy
$IMP mosaic-extensions/crucible-models    extensions/crucible  mosaic-extensions/crucible-models
$IMP mosaic-extensions/embassy-ports      extensions/embassy   mosaic-extensions/embassy-ports
$IMP mosaic-extensions/ferrox-solvers     extensions/ferrox    mosaic-extensions/ferrox-solvers
$IMP mosaic-extensions/manifold-adapters  extensions/manifold  mosaic-extensions/manifold-adapters
$IMP mosaic-extensions/mnemos-knowledge   extensions/mnemos    mosaic-extensions/mnemos-knowledge
$IMP mosaic-extensions/prism-analytics    extensions/prism     mosaic-extensions/prism-analytics
$IMP mosaic-extensions/soter-smt          extensions/soter     mosaic-extensions/soter-smt
```

Expected: eight `OK:` lines.

- [ ] **Step 2: Import the mosaic shell (integration-harness, docs, kb)**

```bash
$IMP mosaic-extensions extensions/mosaic-shell mosaic-extensions
git mv extensions/mosaic-shell/integration-harness extensions/integration-harness
mkdir -p kb/mosaic
git mv extensions/mosaic-shell/kb   kb/mosaic
git mv extensions/mosaic-shell/docs kb/mosaic/docs
git mv extensions/mosaic-shell/REVIEW-GUIDE.md kb/mosaic/REVIEW-GUIDE.md
# Shell AGENTS.md/README/Justfile are superseded by the consolidated repo's own:
git rm -q extensions/mosaic-shell/AGENTS.md extensions/mosaic-shell/README.md \
  extensions/mosaic-shell/Justfile extensions/mosaic-shell/.gitignore
rmdir extensions/mosaic-shell 2>/dev/null || git status --short extensions/mosaic-shell
git commit -m "feat(consolidation): mosaic shell content -> extensions/integration-harness + kb/mosaic (RFL-{ID})"
```

Expected: `extensions/mosaic-shell` empty/gone; commit created. If
`git status` shows leftovers under `.mosaic-shell`, move each to
`kb/mosaic/` before committing — nothing gets silently dropped.

- [ ] **Step 3: Verify tree and history**

```bash
ls extensions/
git log --oneline --follow extensions/ferrox/Cargo.toml | tail -2
```

Expected: 9 directories (8 extensions + integration-harness); ferrox history
shows pre-import commits.

---

### Task 5: Import arena → `arena/`, atelier → `atelier/`; extract crm-helm; record exclusions

**Files:**
- Create (by merges): `bedrock-platform/arena/**`, `bedrock-platform/atelier/**`
- Create: `showcase/crm-helm/**` on a new branch of the helms GitHub repo (via scratch clone — the live helms checkout is not touched)
- Create: `bedrock-platform/kb/consolidation/workspace-exclusions.md`

**Interfaces:**
- Consumes: `import-one.sh`.
- Produces: `workspace-exclusions.md` — the exact member paths Task 7 must NOT include; Plan B re-includes them.

- [ ] **Step 1: Import both repos**

```bash
cd ~/dev/reflective/bedrock-consolidated
IMP=~/dev/reflective/.attic-consolidation/import-one.sh
$IMP arena-tests       arena   arena-tests
$IMP atelier-showcase  atelier atelier-showcase
```

Expected: two `OK:` lines.

- [ ] **Step 2: Move the crm-helm scenario to the helms repo (it is app-platform material)**

Use a scratch clone — the live helms checkout is mid-RFL-128 and must not
be touched:

```bash
git clone git@github.com:Reflective-Lab/helms.git ~/dev/reflective/.attic-consolidation/helms-adoption 2>/dev/null \
  || git clone ~/dev/reflective/bedrock-platform/helms ~/dev/reflective/.attic-consolidation/helms-adoption
cd ~/dev/reflective/.attic-consolidation/helms-adoption
git checkout main
git checkout -b e12/rfl-{ID}-adopt-crm-helm-scenario
mkdir -p showcase
cp -R ~/dev/reflective/bedrock-consolidated/atelier/scenarios/crm-helm showcase/crm-helm
git add showcase/crm-helm
git commit -m "feat(showcase): adopt crm-helm scenario from atelier (consolidation RFL-{ID}; depends on runway-app-host)"
git push -u origin e12/rfl-{ID}-adopt-crm-helm-scenario   # pushes to the helms origin
cd ~/dev/reflective/bedrock-consolidated
git rm -r -q atelier/scenarios/crm-helm
git commit -m "feat(consolidation): crm-helm scenario -> helms repo (runway dep, app-platform per spec decision 8)"
```

(If the scratch clone came from the local nested repo, its `origin` is the
local checkout — repoint before pushing:
`git remote set-url origin $(git -C ~/dev/reflective/bedrock-platform/helms remote get-url origin)`.)

Expected: helms branch pushed to the helms GitHub remote; scenario removed
from the consolidated tree. (Its history stays in the imported atelier
history; the helms copy is a plain adoption.) Open a PR on helms
referencing RFL-{ID} — it merges on the helms cadence, not this plan's.

- [ ] **Step 3: Compute and record the helm-dependent exclusion list**

```bash
cd ~/dev/reflective/bedrock-consolidated
{
  echo "# Workspace exclusions (Plan A) — re-included by Plan B (helm split)"
  echo "# A member is excluded iff its Cargo.toml references helm/prio/application/workbench/truth-catalog/capability/director crates."
  grep -rlE '^(helm-|prio-|application-|workbench-|truth-catalog|capability-|director-)[a-z-]* *=' \
    arena atelier --include=Cargo.toml | sort
} > kb/consolidation/workspace-exclusions.md
cat kb/consolidation/workspace-exclusions.md
git add kb/consolidation/workspace-exclusions.md
git commit -m "chore(consolidation): record helm-dependent workspace exclusions (RFL-{ID})"
```

Expected (based on 2026-07-05 audit): `arena/Cargo.toml` (workspace root —
its helm `workspace.dependencies` entries get commented in Task 7, root
itself is not excluded), `arena/crates/cross-extension-smoke/Cargo.toml`,
and the three `atelier/scenarios/helm-*-headless/Cargo.toml`. Paste the
final list into the RFL-{ID} Linear issue so Plan B has it.

---

### Task 6: External-dependency skew audit

**Files:**
- Create: `bedrock-platform/kb/consolidation/dep-skew-report.md`

**Interfaces:**
- Produces: per-external-crate version resolution table that Task 7 applies in root `workspace.dependencies`.

- [ ] **Step 1: Generate the skew report**

```bash
cd ~/dev/reflective/bedrock-consolidated
python3 - <<'EOF'
import re, collections
from pathlib import Path
roots = [p for p in Path('.').glob('*/Cargo.toml')] + [p for p in Path('.').glob('*/*/Cargo.toml')]
deps = collections.defaultdict(set)
for t in roots:
    txt = t.read_text()
    m = re.search(r'^\[workspace\.dependencies\]\s*$(.*?)(?=^\[(?!workspace\.dependencies)|\Z)', txt, re.M | re.S)
    if not m: continue
    for line in m.group(1).splitlines():
        d = re.match(r'^([a-zA-Z0-9_-]+)\s*=\s*(.+)$', line.strip())
        if not d: continue
        name, spec = d.groups()
        v = re.search(r'version\s*=\s*"([^"]+)"', spec) or re.match(r'"([^"]+)"', spec)
        if v and 'path =' not in spec:          # externals only
            deps[name].add((v.group(1), str(t)))
out = ["# External dependency skew report", ""]
for name, pairs in sorted(deps.items()):
    if len({v for v, _ in pairs}) > 1:
        out.append(f"## {name}")
        out += [f"- `{v}` in `{t}`" for v, t in sorted(pairs)]
        out.append(f"- RESOLUTION: (fill in — default: highest listed)")
        out.append("")
Path('kb/consolidation/dep-skew-report.md').write_text("\n".join(out) + "\n")
print(f"conflicting externals: {sum(1 for p in deps.values() if len({v for v,_ in p})>1)} of {len(deps)}")
EOF
```

Expected: a count line; report written.

- [ ] **Step 2: Resolve every conflict in the report**

For each `## <crate>` section, replace `RESOLUTION: (fill in ...)` with the
chosen version — default rule: the highest version listed, unless a major
bump would change semantics (if a MAJOR-version conflict appears, e.g.
`0.x` vs `1.x`, flag it to Karl before choosing). Then:

```bash
git add kb/consolidation/dep-skew-report.md
git commit -m "chore(consolidation): external dep skew resolutions (RFL-{ID})"
```

Expected: no `(fill in` left: `grep -c '(fill in' kb/consolidation/dep-skew-report.md` → `0`.

---

### Task 7: Root workspace unification

**Files:**
- Create: `bedrock-platform/Cargo.toml` (the single root workspace)
- Create: `bedrock-platform/.cargo/config.toml` (registry definition — required before any `registry = "reflective-labs"` dep parses)
- Modify: every old workspace root (`foundation/converge/Cargo.toml`, `foundation/organism/Cargo.toml`, `foundation/axiom/Cargo.toml`, all `extensions/*/Cargo.toml` roots, `arena/Cargo.toml`, `atelier/Cargo.toml`) — `[workspace]` tables removed, `workspace.dependencies` migrated to root
- Delete: all per-subsystem `Cargo.lock` files
- Create: `bedrock-platform/Cargo.lock` (generated)

**Interfaces:**
- Consumes: exclusion list (Task 5), skew resolutions (Task 6).
- Produces: `cargo metadata` resolves one workspace; every internal dep is `{ path, version = "4.0.0", registry = "reflective-labs" }` in root `workspace.dependencies`, consumed as `workspace = true`.

This is the big-bang commit series. Sub-commits are fine; the gate is at the end.

- [ ] **Step 0: Define the registry (before any dep references it)**

Create `bedrock-platform/.cargo/config.toml` exactly:

```toml
[registries.reflective-labs]
index = "ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git"

[net]
git-fetch-with-cli = true
```

NO token in this file — download/publish auth is per-user
(`~/.cargo/config.toml` `[http] user-agent = "shipyard <token>"` on stable,
`[unstable] registry-auth = true` on nightly; token from Keychain per the
secrets pattern; SSH key `~/.ssh/id_ed25519_shipyard`). Verify no secret
committed: `grep -i 'user-agent\|token' .cargo/config.toml` → no matches.
Plan A never fetches from the registry (all internal deps also carry
`path`), so no token is needed to build — the entry only makes the
`registry = "reflective-labs"` field on workspace deps parseable.

- [ ] **Step 1: Write the root Cargo.toml skeleton**

```toml
[workspace]
resolver = "2"
members = [
    "foundation/axiom",
    "foundation/converge/crates/*",
    "foundation/organism/crates/*",
    "extensions/arbiter/crates/*",
    "extensions/crucible/crates/*",
    "extensions/embassy/crates/*",
    "extensions/ferrox/crates/*",
    "extensions/ferrox/examples/*",
    "extensions/manifold/crates/*",
    "extensions/mnemos/crates/*",
    "extensions/prism/crates/*",
    "extensions/soter/crates/*",
    "extensions/integration-harness",
    "arena/crates/*",
    "atelier/crates/*",
    "atelier/scenarios/*",
    "atelier/tutorials/*",
]
exclude = [
    # Helm-dependent members, re-included by Plan B (kb/consolidation/workspace-exclusions.md):
    "arena/crates/cross-extension-smoke",
    "atelier/scenarios/helm-coordination-headless",
    "atelier/scenarios/helm-realtime-stem-headless",
    "atelier/scenarios/helm-multiuser-convergence-headless",
]
default-members = [
    "foundation/axiom",
    "foundation/converge/crates/*",
    "foundation/organism/crates/*",
]

[workspace.package]
version = "4.0.0"
edition = "2024"
rust-version = "1.96.0"
license = "MIT"
repository = "https://github.com/Reflective-Lab/bedrock-platform"

[workspace.dependencies]
# Internal crates (filled in Step 2), pattern:
# converge-kernel = { path = "foundation/converge/crates/kernel", version = "4.0.0", registry = "reflective-labs" }
# Externals (filled in Step 3 from dep-skew-report resolutions)
```

Adjust member globs to reality (`cargo metadata` in Step 5 is the referee —
if a subsystem keeps crates outside `crates/`, add that path explicitly;
the exclusions list from Task 5 is authoritative for `exclude`).

- [ ] **Step 2: Migrate internal deps to root workspace.dependencies**

For every `[package]` crate (`grep -rl '^\[package\]' --include=Cargo.toml foundation extensions arena atelier`), add one root entry:
`<name> = { path = "<dir>", version = "4.0.0", registry = "reflective-labs" }`.
Generate mechanically:

```bash
cd ~/dev/reflective/bedrock-consolidated
python3 - <<'EOF'
import re
from pathlib import Path
lines = []
for t in sorted(Path('.').rglob('Cargo.toml')):
    if 'target' in t.parts or t.parent == Path('.'): continue
    txt = t.read_text()
    if not re.search(r'^\[package\]', txt, re.M): continue
    name = re.search(r'^name\s*=\s*"([^"]+)"', txt, re.M).group(1)
    lines.append(f'{name} = {{ path = "{t.parent}", version = "4.0.0", registry = "reflective-labs" }}')
print("\n".join(lines))
EOF
```

Paste the output under `[workspace.dependencies]` in the root Cargo.toml
(internal section). Binary-only leaves (scenarios/tutorials/examples) can
be skipped — nothing depends on them.

- [ ] **Step 3: Migrate external deps using the skew resolutions**

Copy every external entry from the old `[workspace.dependencies]` tables
into the root, applying each RESOLUTION from
`kb/consolidation/dep-skew-report.md`. Then delete the old
`[workspace]`, `[workspace.package]`, and `[workspace.dependencies]`
tables from the old roots (for `foundation/axiom/Cargo.toml` keep the
`[package]` — it is a real crate; the others were virtual roots whose
files then hold nothing and are `git rm`'d).

- [ ] **Step 4: Point member crates at the workspace**

In every member `Cargo.toml`: `version.workspace = true` (replacing any
literal version), and every dep that names an internal crate or a root
external becomes `<name> = { workspace = true }` (preserve per-crate
`features = [...]` additions — `workspace = true` composes with features).
Rewrite the old relative-path deps (`../converge/crates/...`,
`../../bedrock-platform/...`) the same way. Mechanical sweep + compiler as
referee. Also flip axiom's crates.io dep:
`converge-manifold-adapters = { version = "1.1.2", features = ["llm-all"] }`
→ `{ workspace = true, features = ["llm-all"] }` (renamed in Task 8).

- [ ] **Step 5: Delete old lockfiles, resolve, gate**

```bash
cd ~/dev/reflective/bedrock-consolidated
find foundation extensions arena atelier -maxdepth 2 -name Cargo.lock -exec git rm -q {} \;
cargo metadata --format-version 1 > /dev/null && echo METADATA-OK
cargo build --workspace 2>&1 | tail -3
cargo test  --workspace 2>&1 | tail -5
```

Expected: `METADATA-OK`; build and tests end green. Iterate on Steps 1–4
until they do — every failure is a missed rewrite, a member-glob gap, or a
skew resolution that needs revisiting (update the report if so).

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(consolidation): one root workspace, lockstep 4.0.0, workspace deps (RFL-{ID})

All sub-workspace roots and lockfiles removed; internal deps are
{ path, version, registry=shipyard }; helm-dependent members excluded
pending Plan B."
```

---

### Task 8: Naming normalization

**Files:**
- Modify: `[package] name` in renamed crates' Cargo.tomls, all dep references, all `use`/`extern` sites
- Create: `bedrock-platform/kb/consolidation/rename-table.md`

**Interfaces:**
- Produces: final published names; Plan C's publish job and all docs use these.

- [ ] **Step 1: Build the full rename table**

Start from the spec's table (spec section "Naming normalization") minus the
helms rows (Plan B applies those):

```
converge-arbiter-policy      -> arbiter-policy
converge-mnemos-knowledge    -> mnemos-knowledge
converge-fuzzy-inference     -> prism-fuzzy
converge-embassy-arxiv       -> embassy-arxiv          (and the other 14 embassy-*)
converge-manifold-adapters   -> manifold-adapters
converge-crucible-models     -> crucible-models
converge-soter-smt           -> soter-smt
```

Then audit for stragglers — any package whose name's prefix doesn't match
its subsystem directory:

```bash
cd ~/dev/reflective/bedrock-consolidated
python3 - <<'EOF'
import re
from pathlib import Path
for t in sorted(Path('.').rglob('Cargo.toml')):
    if 'target' in t.parts: continue
    txt = t.read_text()
    if not re.search(r'^\[package\]', txt, re.M): continue
    name = re.search(r'^name\s*=\s*"([^"]+)"', txt, re.M).group(1)
    sub = t.parts[1] if t.parts[0] in ('foundation','extensions') else t.parts[0]
    if not (name == sub or name.startswith(sub + '-') or name.startswith('example-') or name.startswith('scenario-')):
        print(f"{name:45s} {t.parent}  (subsystem: {sub})")
EOF
```

Record every rename pair (old, new) in `kb/consolidation/rename-table.md`.
Leaf binaries (`example-*`, `scenario-*`, solver example bins) keep their
names — they never publish. Anything ambiguous: ask Karl, don't guess.

- [ ] **Step 2: Apply renames mechanically**

For each pair, three sweeps (kebab in TOML, snake in Rust):

```bash
cd ~/dev/reflective/bedrock-consolidated
apply_rename() {  # apply_rename old-name new-name
  old="$1"; new="$2"
  old_id="${old//-/_}"; new_id="${new//-/_}"
  grep -rl --include=Cargo.toml -F "\"$old\"" . | grep -v target | xargs -I{} sed -i '' "s/\"$old\"/\"$new\"/g" {}
  grep -rl --include=Cargo.toml -E "^$old *=" . | grep -v target | xargs -I{} sed -i '' "s/^$old *=/$new =/" {}
  grep -rl --include='*.rs' -E "\b$old_id\b" . | grep -v target | xargs -I{} sed -i '' "s/\b$old_id\b/$new_id/g" {}
}
# one call per row of rename-table.md, e.g.:
apply_rename converge-arbiter-policy arbiter-policy
# ... (every pair in the table)
```

Note: crates consumed under a rename alias
(`arbiter = { package = "converge-arbiter-policy", ... }`) are covered by
the first sweep (quoted-name match); their Rust idents (`arbiter::`) don't
change.

- [ ] **Step 3: Gate and commit**

```bash
cargo build --workspace 2>&1 | tail -3
cargo test  --workspace 2>&1 | tail -5
grep -rn --include=Cargo.toml -E 'converge-(arbiter|mnemos|fuzzy|embassy|manifold|crucible|soter)' . | grep -v target | wc -l
```

Expected: green build/tests; final grep `0`. Commit:

```bash
git add -A
git commit -m "refactor(consolidation): strict {subsystem}-{crate} names before first publish (RFL-{ID})

Rename table: kb/consolidation/rename-table.md"
```

---

### Task 9: Publish gating flags

**Files:**
- Modify: `Cargo.toml` of every non-published crate (`publish = false`)
- Create: `bedrock-platform/kb/consolidation/publish-gating.md`

**Interfaces:**
- Produces: the day-one catalog. Plan C's publish job publishes exactly the crates without `publish = false`.

- [ ] **Step 1: Apply the gating rule**

`publish = false` in `[package]` for: every crate under `arena/` and
`atelier/`, all 15 `embassy-*` ports, all binary leaves
(ferrox examples, servers), and any lib crate failing the rule
"≥1 consumer AND has tests". Compute the lib-crate check:

```bash
cd ~/dev/reflective/bedrock-consolidated
python3 - <<'EOF'
import re, json, subprocess
meta = json.loads(subprocess.check_output(["cargo","metadata","--format-version","1","--no-deps"]))
by_name = {p["name"]: p for p in meta["packages"]}
consumed = {d["name"] for p in meta["packages"] for d in p["dependencies"]}
for p in meta["packages"]:
    is_lib = any("lib" in t["kind"] for t in p["targets"])
    has_tests = any("test" in t["kind"] for t in p["targets"]) or "tests" in str(p["manifest_path"])
    if is_lib and (p["name"] not in consumed):
        print(f"UNCONSUMED LIB: {p['name']:40s} publish={'true' if p['publish'] is None else p['publish']}")
EOF
```

Every `UNCONSUMED LIB` line must end up `publish = false` (or get a
consumer — but that's Plan-B/backlog work, not this plan's).

- [ ] **Step 2: Record the catalog and the promotion criteria**

Write `kb/consolidation/publish-gating.md`: the list of `publish = false`
lib crates, each with the promotion rule from the spec ("test suite + a
consuming atelier scenario → publish"). Create one Linear issue titled
`Publish-gated crates: promotion backlog` containing the same list.

- [ ] **Step 3: Gate and commit**

```bash
cargo metadata --format-version 1 --no-deps > /dev/null && echo OK
git add -A
git commit -m "chore(consolidation): publish gating — earned publish=true per spec decision 9 (RFL-{ID})"
```

---

### Task 9b: Boundary & footprint audit (spec decision 14)

**Files:**
- Create: `bedrock-consolidated/kb/consolidation/boundary-audit.md`

**Interfaces:**
- Consumes: the unified workspace (Task 7) with final names (Task 8).
- Produces: the layer map + violating-edge list that arena's dim-layering consumes go-forward, and the Linear issues Plan B / backlog work executes. Plan A does NOT fix edges — move-as-is holds.

- [ ] **Step 1: Generate the audit**

```bash
cd ~/dev/reflective/bedrock-consolidated
python3 - <<'EOF'
import json, subprocess, re
from pathlib import Path
meta = json.loads(subprocess.check_output(["cargo","metadata","--format-version","1","--no-deps"]))
TRANSPORT = {"tonic","prost","prost-types","tonic-build","axum","hyper","tower","tower-http","warp","actix-web"}

def layer(p):
    path = p["manifest_path"]
    name = p["name"]
    if "/foundation/converge/crates/" in path:
        return 5 if name in ("converge-protocol","converge-client","converge-runtime") else 1
    if "/foundation/axiom" in path or "/foundation/organism/" in path: return 2
    if "/foundation/helm/" in path: return 3
    if "/extensions/" in path: return 4
    if "/arena/" in path or "/atelier/" in path: return 6
    return 0  # unclassified — must be resolved by hand

pkgs = {p["name"]: p for p in meta["packages"]}
out = ["# Boundary audit (decision 14) — layer map + violations", "",
       "Layers: 1 engine-core, 2 reasoning, 3 helm-spine, 4 extensions, 5 service-surface, 6 proof", ""]
out.append("## Violating edges (consumer layer < producer layer = depending downstream)")
viol = 0
for p in meta["packages"]:
    for d in p["dependencies"]:
        if d["name"] in pkgs and layer(pkgs[d["name"]]) > layer(p) and layer(p) != 6 and layer(p) > 0:
            out.append(f"- `{p['name']}` (L{layer(p)}) -> `{d['name']}` (L{layer(pkgs[d['name']])})  VERDICT: (fill in)")
            viol += 1
out += ["", "## Transport deps in layers 1-4 (automatic failures)"]
tviol = 0
for p in meta["packages"]:
    if 1 <= layer(p) <= 4:
        hits = sorted({d["name"] for d in p["dependencies"]} & TRANSPORT)
        if hits:
            out.append(f"- `{p['name']}` (L{layer(p)}): {', '.join(hits)}  VERDICT: (fill in)")
            tviol += 1
out += ["", "## Unclassified crates (layer 0 — classify by hand)"]
for p in meta["packages"]:
    if layer(p) == 0: out.append(f"- `{p['name']}` at {p['manifest_path']}")
Path("kb/consolidation/boundary-audit.md").write_text("\n".join(out) + "\n")
print(f"edge violations: {viol}, transport violations: {tviol}")
EOF
```

Expected: counts printed; report written. Known-at-spec-time entries that
MUST appear (sanity check the script found them): `axiom-truth -> manifold-adapters`,
the `organism*/axiom* -> converge-runtime` edges. If they're absent, the
layer classifier is wrong — fix it before trusting the rest.

- [ ] **Step 2: Verdicts and Linear issues**

Every `VERDICT: (fill in)` gets a verdict from Karl (present the report,
don't guess): typically `contract-extraction` (axiom→manifold), `invert`
(reasoning→runtime), `accept-as-layer-5`, or `move-to-app-platform`.
Create one Linear issue per violation family with the verdict and the
affected crates. Check completeness:

```bash
grep -c '(fill in' kb/consolidation/boundary-audit.md
```

Expected: `0`.

- [ ] **Step 3: Commit**

```bash
git add kb/consolidation/boundary-audit.md
git commit -m "docs(consolidation): boundary audit — layer map, violations, verdicts (RFL-{ID}, decision 14)"
```

---

### Task 10: Final gates, dual-track doc, PR to consolidation/main

**Files:**
- Create: `bedrock-consolidated/kb/consolidation/dual-track.md` (sync recipe, cutover criteria, expiry)
- Modify: `bedrock-consolidated/README.md` (top section: product map for the new layout)

**Interfaces:**
- Produces: merged PR on `consolidation/main`; documented sync recipe. Plans B and C start from this state. The live setup and root-repo Justfile are untouched (cutover concern, spec decision 13).

- [ ] **Step 1: Full verification sweep**

```bash
cd ~/dev/reflective/bedrock-consolidated
cargo fmt --all --check
cargo clippy --workspace --all-targets 2>&1 | tail -3
cargo test --workspace 2>&1 | tail -5
cargo build 2>&1 | tail -2   # default-members = foundation only, the fast loop
(cd arena && cargo run -p arena-driver 2>&1 | tail -5) || true  # arena quality dims, if runnable pre-Plan-B
```

Expected: fmt clean, clippy clean (or pre-existing-warning parity with the
frozen repos — new warnings are regressions), tests green. Record arena
output in the PR description (some dimensions may Skip pending Plan B —
list them, don't hide them).

- [ ] **Step 2: Write the dual-track doc (sync recipe, cutover criteria, expiry)**

Create `kb/consolidation/dual-track.md` in the clone with exactly this
operational content:

```markdown
# Dual-track: syncing and cutover (spec decision 13)

The live multi-repo setup keeps building; this tree tracks it until cutover.

## Syncing upstream work (run whenever an old main moves)

git filter-repo is deterministic: re-importing an updated upstream produces
the same rewritten commits plus the new ones, so this is an incremental merge:

    cd ~/dev/reflective/bedrock-consolidated
    ~/dev/reflective/.attic-consolidation/import-one.sh <source> <target> <key>
    # e.g. after RFL-128 lands on arena-tests main:
    #   1. update kb/consolidation/snapshot-manifest.json: new SHA for arena-tests
    #   2. import-one.sh arena-tests arena arena-tests
    # then re-run Task 7 Step 4-style dep rewrites on any NEW crates only,
    # and the full gate: cargo test --workspace

After each sync, update the manifest SHA and note the sync in the Linear
dual-track issue.

## Cutover criteria (all must hold)

1. 4.0.0 published to Shipyard (Plan C).
2. Consuming apps build from registry deps, not sibling paths.
3. Factory CI green on consolidation/main (Plan C).
4. Plan B (helm split) merged.

## Expiry

The dual period is tracked debt (frontload-over-shims): the Linear
dual-track issue carries an explicit expiry date; if the date passes
without cutover, escalate — do not silently extend.
```

```bash
cd ~/dev/reflective/bedrock-consolidated
git add kb/consolidation/dual-track.md
git commit -m "docs(consolidation): dual-track sync recipe, cutover criteria, expiry (RFL-{ID})"
```

Also create the Linear dual-track issue (team RFL, title
`Dual-track window: bedrock-consolidated vs live setup — sync log + expiry`)
with an expiry date from Karl (ask; suggest 4–6 weeks out) and the cutover
criteria above.

- [ ] **Step 3: Update the consolidated README product map**

Replace the layout section of `README.md` (in the clone) with the spec's
target-layout tree (foundation/extensions/arena/atelier, one workspace,
4.0.0, consumption line: `cargo add converge-kernel --registry reflective-labs`)
plus a banner: "This is the consolidated tree on `consolidation/main`; the
live layout remains on `main` until cutover (see kb/consolidation/dual-track.md)."
Commit.

- [ ] **Step 4: Create consolidation/main and open the PR against it**

```bash
cd ~/dev/reflective/bedrock-consolidated
git push origin origin/main:refs/heads/consolidation/main   # branch off current remote main
git push -u origin e12/rfl-{ID}-consolidation-import-unify
gh pr create --base consolidation/main \
  --title "Consolidation Plan A: import 13 repos, unify workspace at 4.0.0 (RFL-{ID})" \
  --body "Spec: KB/05-engineering/specs/2026-07-05-bedrock-platform-consolidation-design.md (root repo)
Linear: <RFL-{ID} URL>

Targets consolidation/main (dual-track, spec decision 13) — bedrock-platform
main and the live multi-repo setup are untouched.

- 13 repos imported with history (filter-repo + merge; snapshot manifest in kb/consolidation/)
- One root workspace, lockstep 4.0.0, workspace deps with registry=reflective-labs
- Naming normalized per rename-table.md; publish gating per publish-gating.md
- Helm-dependent members excluded pending Plan B (workspace-exclusions.md)
- Sync recipe + cutover criteria in kb/consolidation/dual-track.md

🤖 Generated with [Claude Code](https://claude.com/claude-code)"
```

Expected: `consolidation/main` exists on the remote at the same SHA as
`main`; PR opens against it. The PR gate is the Step-1 sweep (bedrock CI
arrives in Plan C), stated in the PR body.

- [ ] **Step 5: Post-merge follow-ups (after Karl merges)**

- Nothing in the live setup changes at merge time — that is the point.
  Archiving the 12 absorbed GitHub repos, repointing the root Justfile, and
  removing old checkouts all happen at CUTOVER (dual-track.md criteria),
  each with Karl's authorization.
- The `$SCRATCH/import-*` clones may be removed after the PR merges (they
  are derived artifacts, reproducible from the manifest).
- Run the merge-cleanup skill for the work branch (remote branch only —
  `consolidation/main` stays).

---

## Out of scope (Plans B & C)

- **Plan B (phase 4, gated on RFL-128):** helm headless spine →
  `foundation/helm/`; `application-kernel`→`helm-kernel` etc. renames;
  re-include the exclusion list; helms repo keeps the app remainder.
- **Plan C (phases 5–6):** software factory port (Justfile recipes, doctor
  gates, CI workflows, skills/AGENTS.md), self-contained KB, Shipyard
  `publish.yml`, tag v4.0.0 — all on `consolidation/main`.
- **Cutover (own event, after Plans B & C and app migration to registry
  deps):** `consolidation/main` → `main`, archive absorbed repos, repoint
  root Justfile/factory-alert, remove old checkouts (user-authorized),
  boundary-registry update ("Bedrock owns the headless Helm spine").
  Criteria and expiry in `kb/consolidation/dual-track.md`.
