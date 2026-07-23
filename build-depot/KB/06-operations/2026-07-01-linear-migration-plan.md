# Linear Migration Implementation Plan

> **Revised 2026-07-02** — Tasks 4–5 as written below are SUPERSEDED. The
> pre-import review found the sub-issue import would corrupt parents on re-run
> (title-only dedup) and blow the rate limit (~1700 calls > 1500/h). The
> implemented `tools/linear-import/import.py` creates one issue per open
> milestone with the deliverable checklist in the description, prunes projects
> to epics with open work (11, incl. one shared Mobile project), and prefixes
> titles with the module. See the Revision addendum in
> [2026-07-01-linear-migration-design.md](2026-07-01-linear-migration-design.md)
> and the "Status 2026-07-02" section at the end of this file.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace 43 MILESTONES.md and 3 EPIC.md files with a single Linear team, importing all open milestones as issues scoped to epic-projects with two-tier labels.

**Architecture:** Four phases — (0) consolidation audit produces a human-reviewed report before any API calls; (1) Linear team/project/label setup; (2) milestone import; (3) agent integration and file retirement. All phases use a shared Python stdlib toolkit with no external dependencies.

**Tech Stack:** Python 3.10+, stdlib only (`http.client`, `json`, `re`, `pathlib`, `unittest`), Linear GraphQL API (`https://api.linear.app/graphql`).

## Global Constraints

- Python stdlib only — no `pip install`, no third-party packages
- `LINEAR_API_KEY` env var — never hardcoded, never logged
- All Linear mutations are idempotent — check before create
- Plans save to `KB/` not `docs/superpowers/` (project convention)
- Branch naming: `e{N}/{linear-id}-{slug}` — enforced by AGENTS.md after Task 6
- Linear team name: `Reflective` (key: `RFL`)
- 22 projects: E1–E10, E11a, E11b, Mobile-Foundation through Mobile-Release-Privacy-Ops
- Two-tier labels: `module:*` (which crates touched) and `type:*` (work nature)

---

### Task 1: Scaffold and Linear API client

**Files:**
- Create: `tools/linear-import/linear_client.py`
- Create: `tools/linear-import/tests/__init__.py` (empty)
- Create: `tools/linear-import/tests/test_parse.py` (placeholder, extended in Task 2)

**Interfaces:**
- Produces: `call(gql: str, variables: dict = None) -> dict` — raises `RuntimeError` on API errors or missing key

- [ ] **Step 1: Create directory structure**

```bash
mkdir -p tools/linear-import/tests
touch tools/linear-import/__init__.py
touch tools/linear-import/tests/__init__.py
```

- [ ] **Step 2: Write `linear_client.py`**

```python
# tools/linear-import/linear_client.py
import http.client
import json
import os
import ssl

_ENDPOINT = "api.linear.app"
_PATH = "/graphql"


def call(gql: str, variables: dict = None) -> dict:
    api_key = os.environ.get("LINEAR_API_KEY")
    if not api_key:
        raise RuntimeError("LINEAR_API_KEY environment variable not set")
    payload = json.dumps({"query": gql, "variables": variables or {}}).encode()
    ctx = ssl.create_default_context()
    conn = http.client.HTTPSConnection(_ENDPOINT, context=ctx)
    try:
        conn.request("POST", _PATH, body=payload, headers={
            "Authorization": api_key,
            "Content-Type": "application/json",
            "Content-Length": str(len(payload)),
        })
        resp = conn.getresponse()
        data = json.loads(resp.read().decode())
    finally:
        conn.close()
    if "errors" in data:
        raise RuntimeError(f"Linear API error: {data['errors']}")
    return data["data"]
```

- [ ] **Step 3: Smoke-test the client against the Linear API**

```bash
export LINEAR_API_KEY=<your-key>
python3 - <<'EOF'
import sys
sys.path.insert(0, "tools/linear-import")
from linear_client import call
data = call("query { viewer { id name organization { name } } }")
print(data)
EOF
```

Expected: `{'viewer': {'id': '...', 'name': 'Kenneth Pernyer', 'organization': {'name': 'Reflective Labs'}}}`

- [ ] **Step 4: Commit**

```bash
git add tools/linear-import/
git commit -m "feat(linear-import): scaffold + Linear GraphQL client"
```

---

### Task 2: MILESTONES.md parser

**Files:**
- Create: `tools/linear-import/parse.py`
- Modify: `tools/linear-import/tests/test_parse.py`

**Interfaces:**
- Produces:
  - `Deliverable(text: str, done: bool)`
  - `Milestone(title, source_file, epics, deadline, body, deliverables)` with `.open_deliverables`, `.has_open_work`, `.is_in_progress`, `.module_label`
  - `find_milestone_files(root: Path) -> list[Path]`
  - `parse_milestone_file(path: Path) -> list[Milestone]`

- [ ] **Step 1: Write the parser unit tests first**

```python
# tools/linear-import/tests/test_parse.py
import sys, textwrap, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from parse import parse_milestone_file, Milestone

FIXTURE = textwrap.dedent("""\
    # Milestones

    ## Recently Completed
    - [x] Old thing done

    ### converge — v3.7 Audit Trace Gradients
    **Deadline:** 2026-09-30 | **Epic:** E9
    - [ ] Promotion receipt carries activated_rules
    - [x] Already done item
    - [ ] Replay reproduces scores

    ### helms — Fully Done Milestone
    **Epic:** E5
    - [x] All done
    - [x] Also done
""")


class TestParser(unittest.TestCase):
    def setUp(self):
        import tempfile, os
        self.tmp = tempfile.NamedTemporaryFile(
            mode="w", suffix="/bedrock-platform/MILESTONES.md",
            delete=False, dir=tempfile.mkdtemp()
        )
        self.tmp.write(FIXTURE)
        self.tmp.flush()
        self.path = Path(self.tmp.name)

    def test_skips_recently_completed_section(self):
        milestones = parse_milestone_file(self.path)
        titles = [m.title for m in milestones]
        self.assertNotIn("Recently Completed", titles)

    def test_parses_epic(self):
        milestones = parse_milestone_file(self.path)
        converge = next(m for m in milestones if "v3.7" in m.title)
        self.assertEqual(converge.epics, ["E9"])

    def test_parses_deadline(self):
        milestones = parse_milestone_file(self.path)
        converge = next(m for m in milestones if "v3.7" in m.title)
        self.assertEqual(converge.deadline, "2026-09-30")

    def test_open_deliverables_excludes_done(self):
        milestones = parse_milestone_file(self.path)
        converge = next(m for m in milestones if "v3.7" in m.title)
        self.assertEqual(len(converge.open_deliverables), 2)

    def test_is_in_progress_when_some_done(self):
        milestones = parse_milestone_file(self.path)
        converge = next(m for m in milestones if "v3.7" in m.title)
        self.assertTrue(converge.is_in_progress)

    def test_fully_done_milestone_has_no_open_work(self):
        milestones = parse_milestone_file(self.path)
        helms = next(m for m in milestones if "Fully Done" in m.title)
        self.assertFalse(helms.has_open_work)

    def test_module_label_from_parent_dir(self):
        milestones = parse_milestone_file(self.path)
        self.assertTrue(milestones[0].module_label.startswith("module:"))


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Run tests — confirm they fail**

```bash
cd /Users/kpernyer/dev/reflective
python3 -m pytest tools/linear-import/tests/test_parse.py -v 2>&1 | head -20
```

Expected: `ModuleNotFoundError: No module named 'parse'`

- [ ] **Step 3: Write `parse.py`**

```python
# tools/linear-import/parse.py
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

_SKIP_SECTIONS = {
    "recently completed", "archived", "time allocation", "repositories",
    "how to use", "status legend", "standing guardrails", "non-goals",
    "portfolio definition of done", "long-term roadmap",
}

_EPIC_RE = re.compile(r'\*\*Epics?:\*\*\s*([^\n|]+)')
_DEADLINE_RE = re.compile(r'\*\*Deadline:\*\*\s*(\d{4}-\d{2}-\d{2})')
_DELIVERABLE_RE = re.compile(r'^\s*-\s+\[([ xX])\]\s+(.+)$', re.MULTILINE)
_HEADING_RE = re.compile(r'^(#{2,3})\s+(.+)$', re.MULTILINE)


@dataclass
class Deliverable:
    text: str
    done: bool


@dataclass
class Milestone:
    title: str
    source_file: Path
    epics: list
    deadline: Optional[str]
    body: str
    deliverables: list

    @property
    def open_deliverables(self):
        return [d for d in self.deliverables if not d.done]

    @property
    def has_open_work(self):
        return bool(self.open_deliverables)

    @property
    def is_in_progress(self):
        done = sum(1 for d in self.deliverables if d.done)
        return done > 0 and self.has_open_work

    @property
    def module_label(self):
        parent = self.source_file.parent.name
        # Sub-directory within a top-level repo → use that name
        # e.g. bedrock-platform/helms/MILESTONES.md → "module:helms"
        # e.g. bedrock-platform/MILESTONES.md → "module:bedrock-platform"
        return f"module:{parent}" if parent and parent != "." else None


def _parse_epics(text: str) -> list:
    m = _EPIC_RE.search(text)
    if not m:
        return []
    return re.findall(r'E\d+[ab]?', m.group(1))


def _parse_deadline(text: str) -> Optional[str]:
    m = _DEADLINE_RE.search(text)
    return m.group(1) if m else None


def parse_milestone_file(path: Path) -> list:
    content = path.read_text(encoding="utf-8")
    milestones = []

    # Split on level-2 or level-3 headings, keeping the heading in each chunk
    chunks = re.split(r'\n(?=#{2,3} )', '\n' + content)

    for chunk in chunks:
        chunk = chunk.strip()
        if not chunk:
            continue
        m = _HEADING_RE.match(chunk)
        if not m:
            continue
        title = m.group(2).strip()
        if any(kw in title.lower() for kw in _SKIP_SECTIONS):
            continue

        body = chunk[m.end():].strip()
        deliverables = [
            Deliverable(
                text=dm.group(2).strip(),
                done=dm.group(1).strip().lower() == 'x',
            )
            for dm in _DELIVERABLE_RE.finditer(body)
        ]

        milestones.append(Milestone(
            title=title,
            source_file=path,
            epics=_parse_epics(body),
            deadline=_parse_deadline(body),
            body=body,
            deliverables=deliverables,
        ))

    return milestones


def find_milestone_files(root: Path) -> list:
    return sorted(root.rglob("MILESTONES.md"))
```

- [ ] **Step 4: Run tests — confirm they pass**

```bash
python3 -m pytest tools/linear-import/tests/test_parse.py -v
```

Expected:
```
test_skips_recently_completed_section PASSED
test_parses_epic PASSED
test_parses_deadline PASSED
test_open_deliverables_excludes_done PASSED
test_is_in_progress_when_some_done PASSED
test_fully_done_milestone_has_no_open_work PASSED
test_module_label_from_parent_dir PASSED
7 passed
```

- [ ] **Step 5: Smoke-test against the real repo**

```bash
python3 - <<'EOF'
import sys
sys.path.insert(0, "tools/linear-import")
from parse import find_milestone_files, parse_milestone_file
from pathlib import Path

files = find_milestone_files(Path("."))
print(f"Found {len(files)} MILESTONES.md files")
for f in files[:3]:
    ms = parse_milestone_file(f)
    open_ms = [m for m in ms if m.has_open_work]
    print(f"  {f.relative_to('.')} → {len(ms)} sections, {len(open_ms)} open")
EOF
```

Expected: `Found 43 MILESTONES.md files` followed by sample output for first 3 files.

- [ ] **Step 6: Commit**

```bash
git add tools/linear-import/
git commit -m "feat(linear-import): MILESTONES.md parser with unit tests"
```

---

### Task 3: Consolidation audit script

**Files:**
- Create: `tools/linear-import/audit.py`
- Create: `tools/linear-import/tests/test_audit.py`

**Interfaces:**
- Consumes: `parse.find_milestone_files`, `parse.parse_milestone_file`, `parse.Milestone`
- Produces:
  - `Finding(file, milestone_title, flag, detail, suggested)` dataclass
  - `audit(root: Path) -> list[Finding]`
  - `write_report(findings, out_path: Path) -> None` — writes markdown report

- [ ] **Step 1: Write audit unit tests**

```python
# tools/linear-import/tests/test_audit.py
import sys, textwrap, tempfile, unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from audit import audit, Finding

def _write_file(content: str, subdir: str = "some-project") -> Path:
    d = Path(tempfile.mkdtemp()) / subdir
    d.mkdir(parents=True, exist_ok=True)
    p = d / "MILESTONES.md"
    p.write_text(content, encoding="utf-8")
    return d.parent  # return root so audit() can find it


class TestAudit(unittest.TestCase):
    def test_flags_no_epic(self):
        root = _write_file(textwrap.dedent("""\
            ### widget — v1.0 Launch
            - [ ] Ship it
        """))
        findings = audit(root)
        self.assertTrue(any(f.flag == "no-epic" for f in findings))

    def test_flags_orphaned_file(self):
        root = _write_file(textwrap.dedent("""\
            ### old thing
            **Epic:** E1
            - [x] All done
        """))
        findings = audit(root)
        self.assertTrue(any(f.flag == "orphaned" for f in findings))

    def test_flags_multi_epic(self):
        root = _write_file(textwrap.dedent("""\
            ### big thing
            **Epic:** E1 + E2 + E3 + E4
            - [ ] Do stuff
        """))
        findings = audit(root)
        self.assertTrue(any(f.flag == "multi-epic" for f in findings))

    def test_clean_milestone_not_flagged(self):
        root = _write_file(textwrap.dedent("""\
            ### converge — v3.7
            **Deadline:** 2026-09-30 | **Epic:** E9
            - [ ] Ship it
        """))
        findings = audit(root)
        self.assertEqual(findings, [])


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: Run tests — confirm they fail**

```bash
python3 -m pytest tools/linear-import/tests/test_audit.py -v 2>&1 | head -10
```

Expected: `ModuleNotFoundError: No module named 'audit'`

- [ ] **Step 3: Write `audit.py`**

```python
# tools/linear-import/audit.py
from dataclasses import dataclass
from pathlib import Path
from parse import find_milestone_files, parse_milestone_file

_MULTI_EPIC_THRESHOLD = 3


@dataclass
class Finding:
    file: str
    milestone_title: str
    flag: str       # orphaned | no-epic | multi-epic
    detail: str
    suggested: str


def audit(root: Path) -> list:
    findings = []
    for path in find_milestone_files(root):
        rel = str(path.relative_to(root))
        milestones = parse_milestone_file(path)
        open_ms = [m for m in milestones if m.has_open_work]

        # Orphaned — file has milestones but none have open work
        if milestones and not open_ms:
            findings.append(Finding(
                file=rel,
                milestone_title="(whole file)",
                flag="orphaned",
                detail="No open deliverables — all done or file has no tracked work",
                suggested="Archive file; skip import",
            ))
            continue

        for m in open_ms:
            if not m.epics:
                findings.append(Finding(
                    file=rel,
                    milestone_title=m.title,
                    flag="no-epic",
                    detail="No **Epic:** field — cannot assign to a Linear project",
                    suggested="Add **Epic:** field or decide to skip import",
                ))
            elif len(m.epics) >= _MULTI_EPIC_THRESHOLD:
                findings.append(Finding(
                    file=rel,
                    milestone_title=m.title,
                    flag="multi-epic",
                    detail=f"References {len(m.epics)} epics: {', '.join(m.epics)}",
                    suggested=f"Pick one primary epic (first listed: {m.epics[0]}); others become labels",
                ))

    return findings


def write_report(findings: list, out_path: Path) -> None:
    lines = [
        "# Linear Migration — Consolidation Report",
        "",
        "> Generated 2026-07-01. Review and act on each item, then run `import.py`.",
        "",
        f"**Total flagged:** {len(findings)}",
        "",
        "---",
        "",
    ]
    if not findings:
        lines.append("No issues found — safe to proceed with `import.py`.\n")
    for f in findings:
        lines += [
            f"## {f.flag.upper()}: {f.milestone_title}",
            "",
            f"**File:** `{f.file}`  ",
            f"**Flag:** `{f.flag}`  ",
            f"**Detail:** {f.detail}  ",
            f"**Suggested action:** {f.suggested}  ",
            "",
            "- [ ] Action taken: _____",
            "",
            "---",
            "",
        ]
    out_path.write_text('\n'.join(lines), encoding="utf-8")
    print(f"Report written to {out_path}")


if __name__ == "__main__":
    import sys
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent.parent.parent
    findings = audit(root)
    out = root / "KB/06-operations/2026-07-01-linear-consolidation-report.md"
    write_report(findings, out)
    print(f"\n{len(findings)} items need attention before import.")
    if findings:
        print("Resolve them in the report, update the source files, then run import.py")
        sys.exit(1)
```

- [ ] **Step 4: Run tests — confirm they pass**

```bash
python3 -m pytest tools/linear-import/tests/test_audit.py -v
```

Expected: `4 passed`

- [ ] **Step 5: Run audit against the real repo**

```bash
python3 tools/linear-import/audit.py
```

Expected: creates `KB/06-operations/2026-07-01-linear-consolidation-report.md` and prints count of flagged items.

- [ ] **Step 6: Commit**

```bash
git add tools/linear-import/ KB/06-operations/2026-07-01-linear-consolidation-report.md
git commit -m "feat(linear-import): consolidation audit script + report"
```

---

## ⏸ Manual Review Gate

**Before proceeding to Task 4:** open `KB/06-operations/2026-07-01-linear-consolidation-report.md` and resolve every flagged item:

- **orphaned** — add deprecation header to the file now; it will be skipped by import
- **no-epic** — add `**Epic:** E{N}` to the milestone section header, or decide to skip
- **multi-epic** — confirm the first-listed epic is the right primary; adjust if not

Re-run `python3 tools/linear-import/audit.py` until it exits 0 (no findings).

Commit any MILESTONES.md changes made during review:
```bash
git add -p
git commit -m "docs(milestones): consolidation pass — assign epics, mark resolved items"
```

---

### Task 4: Linear setup — team, projects, labels

**Files:**
- Create: `tools/linear-import/import.py`

**Interfaces:**
- Consumes: `linear_client.call`
- Produces:
  - `get_or_create_team(name: str, key: str) -> str` — returns team ID
  - `get_or_create_project(name: str, team_id: str) -> str` — returns project ID
  - `get_or_create_label(name: str, color: str, team_id: str) -> str` — returns label ID
  - `get_workflow_states(team_id: str) -> dict` — `{"Todo": id, "In Progress": id}`
  - `PROJECTS: list[dict]` — 22 project defs with `name` and `epics` keys
  - `MODULE_LABELS: list[str]`, `TYPE_LABELS: list[str]`

- [ ] **Step 1: Write `import.py` setup phase**

```python
# tools/linear-import/import.py
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from linear_client import call

# ── Project definitions ───────────────────────────────────────────────────────

PROJECTS = [
    {"name": "E1 — Converge: Publishable Platform",        "epics": ["E1"]},
    {"name": "E2 — Organism: Reasons Before It Acts",      "epics": ["E2"]},
    {"name": "E3 — Lattice: Execution Mesh",               "epics": ["E3"]},
    {"name": "E4 — Marquee Apps: Helm Consumers",          "epics": ["E4"]},
    {"name": "E5 — Helm: Trust Transfer Surface",          "epics": ["E5"]},
    {"name": "E6 — Commerce Rails",                        "epics": ["E6"]},
    {"name": "E7 — Axiom: JTBD to Governed Contracts",     "epics": ["E7"]},
    {"name": "E8 — Long-running HITL Convergence",         "epics": ["E8"]},
    {"name": "E9 — Shared Fuzzy Substrate",                "epics": ["E9"]},
    {"name": "E10 — Commercial Spikes",                    "epics": ["E10"]},
    {"name": "E11a — AI Director UX",                      "epics": ["E11a"]},
    {"name": "E11b — Helm Coordination",                   "epics": ["E11b"]},
    {"name": "Mobile-Foundation",                          "epics": ["M1"]},
    {"name": "Mobile-Quorum-Capture",                      "epics": ["M2"]},
    {"name": "Mobile-Consent-Core",                        "epics": ["M3"]},
    {"name": "Mobile-Domain-Reuse",                        "epics": ["M4"]},
    {"name": "Mobile-Compute-Placement",                   "epics": ["M5"]},
    {"name": "Mobile-Collaborative-UX",                    "epics": ["M6"]},
    {"name": "Mobile-Android-Parity",                      "epics": ["M7"]},
    {"name": "Mobile-Portfolio-Pattern",                   "epics": ["M8"]},
    {"name": "Mobile-Studio-Local-First",                  "epics": ["M9"]},
    {"name": "Mobile-Release-Privacy-Ops",                 "epics": ["M10"]},
]

# Map epic IDs found in MILESTONES.md to project names
EPIC_TO_PROJECT = {
    epic: p["name"]
    for p in PROJECTS
    for epic in p["epics"]
}

MODULE_LABELS = [
    "module:converge", "module:organism", "module:axiom", "module:helms",
    "module:helm-coordination", "module:commerce-rails", "module:runtime-runway",
    "module:mosaic-extensions", "module:arbiter-policy", "module:marquee-apps",
    "module:studio-apps", "module:mobile-apps", "module:lattice",
    "module:beacon-sites", "module:forge-templates", "module:arena-tests",
    "module:atelier-showcase",
]
TYPE_LABELS = ["type:spike", "type:platform", "type:infra", "type:app", "type:docs"]

# Label colors (Linear hex colors)
LABEL_COLORS = {
    "module:": "#6366f1",   # indigo for module labels
    "type:":   "#f59e0b",   # amber for type labels
}

# ── GraphQL helpers ───────────────────────────────────────────────────────────

_GET_TEAMS = "query { teams { nodes { id name } } }"

_CREATE_TEAM = """
mutation($name: String!, $key: String!) {
  teamCreate(input: { name: $name, key: $key }) {
    success team { id name }
  }
}"""

_GET_PROJECTS = """
query($teamId: ID!) {
  team(id: $teamId) { projects { nodes { id name } } }
}"""

_CREATE_PROJECT = """
mutation($name: String!, $teamId: String!) {
  projectCreate(input: { name: $name, teamIds: [$teamId] }) {
    success project { id name }
  }
}"""

_GET_LABELS = """
query($teamId: ID!) {
  team(id: $teamId) { labels { nodes { id name } } }
}"""

_CREATE_LABEL = """
mutation($name: String!, $color: String!, $teamId: String!) {
  issueLabelCreate(input: { name: $name, color: $color, teamId: $teamId }) {
    success issueLabel { id name }
  }
}"""

_GET_STATES = """
query($teamId: ID!) {
  workflowStates(filter: { team: { id: { eq: $teamId } } }) {
    nodes { id name type }
  }
}"""


def get_or_create_team(name: str, key: str) -> str:
    teams = call(_GET_TEAMS)["teams"]["nodes"]
    for t in teams:
        if t["name"] == name:
            print(f"  team '{name}' already exists ({t['id']})")
            return t["id"]
    result = call(_CREATE_TEAM, {"name": name, "key": key})
    tid = result["teamCreate"]["team"]["id"]
    print(f"  created team '{name}' ({tid})")
    return tid


def get_or_create_project(name: str, team_id: str, project_cache: dict) -> str:
    if name in project_cache:
        return project_cache[name]
    result = call(_CREATE_PROJECT, {"name": name, "teamId": team_id})
    pid = result["projectCreate"]["project"]["id"]
    project_cache[name] = pid
    print(f"  created project '{name}' ({pid})")
    return pid


def get_or_create_label(name: str, team_id: str, label_cache: dict) -> str:
    if name in label_cache:
        return label_cache[name]
    color = LABEL_COLORS.get("module:" if name.startswith("module:") else "type:", "#94a3b8")
    result = call(_CREATE_LABEL, {"name": name, "color": color, "teamId": team_id})
    lid = result["issueLabelCreate"]["issueLabel"]["id"]
    label_cache[name] = lid
    print(f"  created label '{name}' ({lid})")
    return lid


def get_workflow_states(team_id: str) -> dict:
    nodes = call(_GET_STATES, {"teamId": team_id})["workflowStates"]["nodes"]
    states = {}
    for n in nodes:
        if n["type"] == "unstarted" and "Todo" not in states:
            states["Todo"] = n["id"]
        elif n["type"] == "started" and "In Progress" not in states:
            states["In Progress"] = n["id"]
    return states


def setup(team_id: str) -> tuple:
    """Create all projects and seed labels. Returns (project_cache, label_cache, states)."""
    print("\n── Projects ─────────────────────────────────────────────────────")
    # Pre-load existing projects to avoid duplicates
    existing = call(_GET_PROJECTS, {"teamId": team_id})["team"]["projects"]["nodes"]
    project_cache = {p["name"]: p["id"] for p in existing}
    for p in PROJECTS:
        if p["name"] not in project_cache:
            get_or_create_project(p["name"], team_id, project_cache)
        else:
            print(f"  project '{p['name']}' already exists")

    print("\n── Labels ───────────────────────────────────────────────────────")
    existing_labels = call(_GET_LABELS, {"teamId": team_id})["team"]["labels"]["nodes"]
    label_cache = {l["name"]: l["id"] for l in existing_labels}
    for name in MODULE_LABELS + TYPE_LABELS:
        if name not in label_cache:
            get_or_create_label(name, team_id, label_cache)
        else:
            print(f"  label '{name}' already exists")

    print("\n── Workflow states ───────────────────────────────────────────────")
    states = get_workflow_states(team_id)
    print(f"  Todo → {states.get('Todo')}")
    print(f"  In Progress → {states.get('In Progress')}")

    return project_cache, label_cache, states
```

- [ ] **Step 2: Test setup phase with `--setup-only` flag** (add `main` at end of file)

Append to `import.py`:

```python
# ── Entry point ───────────────────────────────────────────────────────────────

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--setup-only", action="store_true",
                        help="Create team/projects/labels but skip issue import")
    parser.add_argument("--dry-run", action="store_true",
                        help="Parse milestones and print what would be created, no API calls")
    args = parser.parse_args()

    REPO_ROOT = Path(__file__).parent.parent.parent

    if args.dry_run:
        from parse import find_milestone_files, parse_milestone_file
        files = find_milestone_files(REPO_ROOT)
        total_issues = total_sub = 0
        for f in files:
            for m in parse_milestone_file(f):
                if m.has_open_work and m.epics:
                    total_issues += 1
                    total_sub += len(m.open_deliverables)
        print(f"Dry run: would create {total_issues} issues, {total_sub} sub-issues")
        sys.exit(0)

    print("── Team ──────────────────────────────────────────────────────────")
    team_id = get_or_create_team("Reflective", "RFL")
    project_cache, label_cache, states = setup(team_id)

    if args.setup_only:
        print("\nSetup complete.")
        sys.exit(0)

    # Import phase added in Task 5
    print("\nRun without --setup-only to import issues (Task 5).")
```

- [ ] **Step 3: Run dry-run — verify counts look right**

```bash
python3 tools/linear-import/import.py --dry-run
```

Expected: `Dry run: would create N issues, M sub-issues` (N roughly 30–50, M roughly 150–250)

- [ ] **Step 4: Run setup-only against Linear**

```bash
export LINEAR_API_KEY=<your-key>
python3 tools/linear-import/import.py --setup-only
```

Expected: lines like `created project 'E1 — Converge: Publishable Platform' (abc123)` for each of 22 projects, then label lines, then state IDs. No errors.

- [ ] **Step 5: Re-run setup-only — confirm idempotency**

```bash
python3 tools/linear-import/import.py --setup-only
```

Expected: all lines read `already exists` — no new items created.

- [ ] **Step 6: Commit**

```bash
git add tools/linear-import/import.py
git commit -m "feat(linear-import): setup phase — team, 22 projects, seed labels"
```

---

### Task 5: Issue import

**Files:**
- Modify: `tools/linear-import/import.py` — add import phase

**Interfaces:**
- Consumes: `setup()` return values, `parse.find_milestone_files`, `parse.parse_milestone_file`
- Produces: Linear issues and sub-issues; printed summary `N issues, M sub-issues created`

- [ ] **Step 1: Add issue creation helpers to `import.py`**

Insert before `if __name__ == "__main__":`:

```python
# ── Issue creation ────────────────────────────────────────────────────────────

_FIND_ISSUES = """
query($teamId: ID!, $title: String!) {
  issues(filter: {
    team: { id: { eq: $teamId } }
    title: { eq: $title }
  }) { nodes { id identifier title } }
}"""

_CREATE_ISSUE = """
mutation(
  $title: String!
  $description: String
  $teamId: String!
  $projectId: String
  $labelIds: [String!]
  $dueDate: TimelessDate
  $stateId: String
  $parentId: String
) {
  issueCreate(input: {
    title: $title
    description: $description
    teamId: $teamId
    projectId: $projectId
    labelIds: $labelIds
    dueDate: $dueDate
    stateId: $stateId
    parentId: $parentId
  }) {
    success issue { id identifier title url }
  }
}"""


def get_or_create_issue(
    title: str,
    description: str,
    team_id: str,
    project_id: str,
    label_ids: list,
    due_date,
    state_id: str,
    parent_id,
    issue_cache: dict,
) -> str:
    if title in issue_cache:
        return issue_cache[title]
    existing = call(_FIND_ISSUES, {"teamId": team_id, "title": title})
    nodes = existing["issues"]["nodes"]
    if nodes:
        iid = nodes[0]["id"]
        issue_cache[title] = iid
        return iid
    variables = {
        "title": title,
        "description": description,
        "teamId": team_id,
        "projectId": project_id,
        "labelIds": label_ids,
        "stateId": state_id,
    }
    if due_date:
        variables["dueDate"] = due_date
    if parent_id:
        variables["parentId"] = parent_id
    result = call(_CREATE_ISSUE, variables)
    issue = result["issueCreate"]["issue"]
    issue_cache[title] = issue["id"]
    return issue["id"]


def import_milestones(team_id: str, project_cache: dict, label_cache: dict, states: dict, root: Path):
    from parse import find_milestone_files, parse_milestone_file

    issue_cache: dict = {}
    created_issues = 0
    created_sub = 0
    skipped = []

    files = find_milestone_files(root)
    print(f"\n── Importing from {len(files)} files ────────────────────────────")

    for path in files:
        milestones = parse_milestone_file(path)
        for m in milestones:
            if not m.has_open_work:
                continue
            if not m.epics:
                skipped.append(f"{path.relative_to(root)}: '{m.title}' (no epic)")
                continue

            primary_epic = m.epics[0]
            project_name = EPIC_TO_PROJECT.get(primary_epic)
            if not project_name:
                skipped.append(f"{path.relative_to(root)}: '{m.title}' (unknown epic {primary_epic})")
                continue

            project_id = project_cache.get(project_name)
            if not project_id:
                skipped.append(f"{path.relative_to(root)}: '{m.title}' (project not found for {project_name})")
                continue

            # Build label IDs
            label_ids = []
            if m.module_label and m.module_label in label_cache:
                label_ids.append(label_cache[m.module_label])
            # Secondary epics as extra module labels if they exist
            for epic in m.epics[1:]:
                extra_label = f"epic:{epic}"
                if extra_label not in label_cache:
                    get_or_create_label(extra_label, team_id, label_cache)
                label_ids.append(label_cache[extra_label])

            state_id = states.get("In Progress" if m.is_in_progress else "Todo")

            print(f"  {m.title[:60]:<60} [{primary_epic}]")
            parent_id = get_or_create_issue(
                title=m.title,
                description=m.body,
                team_id=team_id,
                project_id=project_id,
                label_ids=label_ids,
                due_date=m.deadline,
                state_id=state_id,
                parent_id=None,
                issue_cache=issue_cache,
            )
            created_issues += 1

            for d in m.open_deliverables:
                get_or_create_issue(
                    title=d.text[:255],
                    description=None,
                    team_id=team_id,
                    project_id=project_id,
                    label_ids=label_ids,
                    due_date=None,
                    state_id=states.get("Todo"),
                    parent_id=parent_id,
                    issue_cache=issue_cache,
                )
                created_sub += 1

    print(f"\n── Summary ──────────────────────────────────────────────────────")
    print(f"  Issues created:     {created_issues}")
    print(f"  Sub-issues created: {created_sub}")
    if skipped:
        print(f"\n  Skipped ({len(skipped)}):")
        for s in skipped:
            print(f"    - {s}")
```

- [ ] **Step 2: Wire import into `main` block**

Replace the last line of the `if __name__ == "__main__":` block (`print("\nRun without --setup-only ...")`) with:

```python
    import_milestones(team_id, project_cache, label_cache, states, REPO_ROOT)
    print("\nImport complete.")
```

- [ ] **Step 3: Run the full import**

```bash
export LINEAR_API_KEY=<your-key>
python3 tools/linear-import/import.py
```

Expected: setup lines (all `already exists`), then per-milestone progress lines, then summary with counts and any skipped items.

- [ ] **Step 4: Re-run import — confirm idempotency**

```bash
python3 tools/linear-import/import.py
```

Expected: same output but all issues are found via `_FIND_ISSUES` — `created_issues` and `created_sub` both 0 (or the same as before if counts are printed from cache hits — verify no duplicates appear in Linear).

- [ ] **Step 5: Verify in Linear web UI**

Open `https://linear.app` → Reflective team → Projects. Confirm:
- 22 projects visible
- Issues visible under their correct project
- Sub-issues linked to parent issues
- Due dates set where milestones had `**Deadline:**`
- In Progress status on milestones with partial completion

- [ ] **Step 6: Commit**

```bash
git add tools/linear-import/import.py
git commit -m "feat(linear-import): issue import — milestones → issues, deliverables → sub-issues"
```

---

### Task 6: Agent integration and file retirement

**Files:**
- Modify: `AGENTS.md` — add Project Tracking section + Git branch convention
- Modify: `bedrock-platform/CLAUDE.md` — replace MILESTONES.md reference
- Modify: `runtime-runway/CLAUDE.md` — replace MILESTONES.md reference
- Modify: `mobile-apps/CLAUDE.md` (if exists) — replace MILESTONES.md reference
- Modify: all 43 `MILESTONES.md` files — add deprecation header
- Modify: `EPICS.md`, `bedrock-platform/EPIC.md`, `mobile-apps/EPIC.md` — add deprecation header

- [ ] **Step 1: Update `AGENTS.md` — add Project Tracking section**

Find the `## Cross-Agent Coordination` section in `AGENTS.md` and insert before it:

```markdown
## Project Tracking

Milestone and epic state lives in Linear (workspace: Reflective Labs, team: Reflective).
Do not rely on `MILESTONES.md` or `EPIC.md` files for open work — those files are
archived and kept for historical context only.

### Git branch convention

All feature/fix branches follow:

```
e{N}/{linear-id}-{slug}
# e.g. e5/lin-67-helm-trust-surface-sse
```

- `e{N}` = epic number the work advances (from the Linear project)
- `{linear-id}` = Linear issue identifier (e.g. `lin-67`)
- `{slug}` = short kebab-case description

Worktrees: `worktrees/e{N}-lin{ID}/` — one per active issue, removed on merge.

PR bodies must include the Linear issue URL:
```
Fixes: https://linear.app/reflective/issue/LIN-{ID}
```

Docs-only changes (push direct to main) are exempt from the branch convention.
```
- Mobile milestones likely have no `**Epic:**` field (they use their own Epic 1–10 numbering). The audit will flag them as `no-epic`. Resolve during the manual review gate by adding `**Epic:** M1` through `**Epic:** M10` to the relevant sections.

- [ ] **Step 2: Update `bedrock-platform/CLAUDE.md`**

Replace the `## Session Scope` table entry for Milestones:

Old:
```
| **Milestones:** | `MILESTONES.md` — cross-project rollup with deadlines and priorities |
```

New:
```
| **Milestones:** | Linear — Reflective team, filter by `module:converge`, `module:organism`, `module:axiom`, `module:helms`. MILESTONES.md is archived. |
```

- [ ] **Step 3: Update `runtime-runway/CLAUDE.md`**

Find the `## Session scope` section. Replace:
```
- **Milestones:** `MILESTONES.md` (when it exists)
```
With:
```
- **Milestones:** Linear — Reflective team, filter by `module:runtime-runway`. MILESTONES.md is archived.
```

- [ ] **Step 4: Add deprecation headers to all MILESTONES.md files**

Run this script once to prefix every MILESTONES.md with the deprecation banner:

```bash
python3 - <<'EOF'
from pathlib import Path

BANNER = """> **Archived 2026-07-01** — active milestone tracking moved to Linear (Reflective team).
> This file is kept for historical context only. Do not add new items here.

"""

root = Path(".")
for p in root.rglob("MILESTONES.md"):
    content = p.read_text(encoding="utf-8")
    if "Archived 2026-07-01" not in content:
        p.write_text(BANNER + content, encoding="utf-8")
        print(f"  updated {p}")
    else:
        print(f"  already done: {p}")
EOF
```

- [ ] **Step 5: Add deprecation headers to EPIC.md files**

```bash
python3 - <<'EOF'
from pathlib import Path

BANNER = """> **Archived 2026-07-01** — epics now live as Projects in Linear (Reflective team).
> This file is kept for historical context. E11 conflict (AI Director UX vs Helm Coordination)
> still needs resolution — both are imported as E11a and E11b in Linear.

"""

for name in ["EPICS.md", "bedrock-platform/EPIC.md", "mobile-apps/EPIC.md"]:
    p = Path(name)
    if not p.exists():
        continue
    content = p.read_text(encoding="utf-8")
    if "Archived 2026-07-01" not in content:
        p.write_text(BANNER + content, encoding="utf-8")
        print(f"  updated {p}")
EOF
```

- [ ] **Step 6: Check that all per-project CLAUDE.md files with MILESTONES.md references are updated**

```bash
grep -r "MILESTONES.md" --include="CLAUDE.md" . | grep -v "Archived\|archived\|active tracking moved"
```

Expected: empty output (no unreplaced references). If any appear, add the Linear filter note to that file.

- [ ] **Step 7: Commit all agent integration changes**

```bash
git add AGENTS.md EPICS.md bedrock-platform/ runtime-runway/ mobile-apps/
git add $(git diff --name-only | grep MILESTONES.md)
git commit -m "$(cat <<'EOF'
docs(agents): migrate to Linear — retire MILESTONES.md and EPIC.md files

Updates AGENTS.md with Linear project tracking + git branch convention.
Adds deprecation headers to all 43 MILESTONES.md and 3 EPIC.md files.
Updates per-project CLAUDE.md files to point to Linear instead of local files.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Completion Checklist (superseded — see Status 2026-07-02)

- [x] ~~`python3 -m pytest ...`~~ → `cd tools/linear-import && python3 -m unittest discover -s tests -t .` — 19 tests pass (pytest is not installed; stdlib unittest only)
- [ ] ~~Linear workspace has 22 projects visible~~ → 11 projects (E1–E7, E9–E11 with open work + one shared Mobile project)
- [ ] Issues visible under correct projects with module labels
- [ ] ~~Sub-issues linked to parent issues~~ → no sub-issues; deliverables are checklists in issue descriptions
- [ ] `grep -r "MILESTONES.md" --include="CLAUDE.md" .` returns only archived references
- [x] ~~E11a and E11b both exist~~ → resolved: E11 = Helm Coordination, E12 = AI Director UX (EPICS.md updated 2026-07-02)
- [ ] `python3 tools/linear-import/import.py` re-run exits cleanly (idempotent)

---

## Status 2026-07-02

| Task | Status |
|---|---|
| 1 — Scaffold + client | Done; client rewritten with 429/5xx retry + 30s timeout |
| 2 — Parser | Done; module resolution skips `kb/Planning` nesting; archived-banner files return no milestones |
| 3 — Audit + review gate | Done; all 37 group decisions recorded in the grouped report |
| 4–5 — Setup + import | **Done 2026-07-02.** 123 issues / 11 projects created in team Reflective (RFL); idempotency re-run confirmed 0 created / 123 existed. Fix along the way: `team(id:)` takes `String!`, not `ID!` (GRAPHQL_VALIDATION_FAILED) in `_GET_PROJECTS`/`_GET_LABELS` |
| 6 — Agent integration | **Done 2026-07-02.** `retire.py` banner-stamped all 48 milestone/epic files; stamps committed locally in all 19 repos (root + 18 nested, each on its current branch). Branch policy live: short-lived `e{N}/lin-XX-slug` branches, one worktree per concurrent agent, epic traceability via Linear |

Source-file fixes made during the review (committed in each repo):
- `runtime-runway/MILESTONES.md` — M2, M6 assigned **Epic:** E3 (group 2U)
- `studio-apps/wolfgang-chat/MILESTONES.md` — H3→H4 demote; epic corrected to E4 (group 2Y)
- `bedrock-platform/helms/kb/Planning/Milestones.md` — H3→H4 demote; malformed Epic line fixed (group 2F)
- `marquee-apps/quorum-sense/MILESTONES.md` — M4 → E4; Atlas spike stays E10 (group 2O, modified)
- Root `EPICS.md` — E11/E12 renumbering note
