#!/usr/bin/env bash
# project-doctor — canonical implementation owned by Build-Depot
# (machinery/build-depot). Invoked as a thin runner from the root
# workspace Justfile with cwd = workspace root. Semantics:
# build-depot/docs/operations/quality-gates.md
set -uo pipefail
fails=0
echo "── project-doctor ──"

# 1. RP-RELEASE-TRAIN-INTEGRITY — release-train.yaml is the only source
#    of truth (QF-2026-06-06-02, closed 2026-06-08). The Justfile reads
#    `release_order` and `_release-dir` from it at runtime via awk, so the
#    old sync-against-Justfile half is gone. This check validates: yaml
#    parseable + every named member directory exists.
train_yaml="release-train.yaml"
if [[ ! -f "$train_yaml" ]]; then
    echo "✗ $train_yaml missing"
    fails=$((fails+1))
else
    project_count=$(awk '
        /^projects:/ { in_proj = 1; next }
        in_proj && /^[^[:space:]]/ { in_proj = 0 }
        in_proj && /^  - name:/ { n++ }
        END { print n+0 }
    ' "$train_yaml")
    if [[ "$project_count" -lt 1 ]]; then
        echo "✗ release-train.yaml has no projects: entries"
        fails=$((fails+1))
    fi
    missing_dirs=()
    while IFS=$'\t' read -r name dir; do
        [[ -z "$name" ]] && continue
        [[ -d "$dir" ]] || missing_dirs+=("$name → $dir")
    done < <(awk '
        /^projects:/ { in_proj = 1; next }
        in_proj && /^[^[:space:]]/ { in_proj = 0 }
        in_proj && /^  - name:/ { name=$3 }
        in_proj && /^    dir:/  { dir=$2; printf "%s\t%s\n", name, dir; name=""; dir="" }
    ' "$train_yaml")
    if [[ ${#missing_dirs[@]} -eq 0 ]]; then
        echo "✓ release-train.yaml parseable + every member directory exists"
    else
        echo "✗ ${#missing_dirs[@]} release-train member dir(s) missing:"
        printf '    %s\n' "${missing_dirs[@]}"
        fails=$((fails+1))
    fi
fi

# 2. RP-LAYERING — a publishable crate may not path-dep an unpublishable
#    (publish=false / UNLICENSED) one. Walks every train workspace via
#    cargo metadata. Motivated by QF-2026-06-02-08 (commerce-rails-stripe
#    blocking runway-accounts).
layer_viol=$(python3 - <<'PY'
import json, os, subprocess
workspaces = [
    "framework/bedrock",
    "machinery/build-depot", "machinery/runtime-runway",
    "machinery/commerce-rails", "machinery/chart-room",
]
for ws in workspaces:
    if not os.path.isfile(os.path.join(ws, "Cargo.toml")):
        continue
    try:
        out = subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=ws, stderr=subprocess.DEVNULL, timeout=30,
        )
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
        print(f"{ws}: cargo metadata failed")
        continue
    meta = json.loads(out)
    unpub = {p["name"] for p in meta["packages"] if p.get("publish") == []}
    for p in meta["packages"]:
        if p.get("publish") == []:
            continue
        for d in p.get("dependencies", []):
            if d.get("path") and d["name"] in unpub:
                print(f"{ws}: {p['name']} → {d['name']} (publish=false)")
PY
)
if [[ -z "$layer_viol" ]]; then
    echo "✓ no publishable crate path-deps an unpublishable one"
else
    line_count=$(echo "$layer_viol" | wc -l | tr -d ' ')
    echo "✗ $line_count layering violation(s):"
    echo "$layer_viol" | sed 's/^/    /'
    fails=$((fails+1))
fi

# 3. RP-CRATE-SIZE-BUDGET — leading indicator for the crates.io 10 MiB
#    ceiling. Any git-tracked file > 1 MiB in a publishable workspace
#    is a smell. Walks `git ls-files` (not `find`) so gitignored caches —
#    `.fastembed_cache`, `.terraform/` providers, vendored solver
#    binaries, local databases — are correctly excluded: `cargo package`
#    respects `.gitignore` so untracked files never ship anyway.
#    Motivated by QF-2026-06-02-09 (runway-storage-contract hit the
#    10 MiB cap with real shipping bytes).
train_dirs="framework/bedrock machinery/runtime-runway machinery/commerce-rails"
big_files=()
for d in $train_dirs; do
    [[ -d "$d/.git" ]] || continue
    while IFS= read -r f; do
        [[ -z "$f" ]] && continue
        fullpath="$d/$f"
        size=$(stat -f%z "$fullpath" 2>/dev/null || stat -c%s "$fullpath" 2>/dev/null)
        if [[ -n "$size" && "$size" -gt 1048576 ]]; then
            kb=$((size/1024))
            big_files+=("${fullpath} (${kb} KiB)")
        fi
    done < <(git -C "$d" ls-files 2>/dev/null)
done
if [[ ${#big_files[@]} -eq 0 ]]; then
    echo "✓ no source file > 1 MiB in publishable workspaces"
else
    echo "✗ ${#big_files[@]} oversize source file(s) in publishable workspaces:"
    printf '    %s\n' "${big_files[@]}"
    fails=$((fails+1))
fi

# 4. RP-SNAPSHOT-PORTABLE — trybuild .stderr fixtures must not contain
#    machine-specific absolute paths. Catches the failure mode where local
#    blessing leaks /Users/<name>/ into fixtures that then fail on CI or
#    on another contributor's machine (QF-2026-06-02-06).
leaks=()
while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    hits=$(grep -nE '/Users/|/home/[a-z]+/|/private/var/folders/|/tmp/[A-Za-z0-9]{6,}' "$f" 2>/dev/null | head -3)
    if [[ -n "$hits" ]]; then
        while IFS= read -r h; do
            leaks+=("${f}:${h}")
        done <<<"$hits"
    fi
done < <(find framework/bedrock machinery/runtime-runway machinery/commerce-rails \
    \( -path '*/target' -o -path '*/.git' -o -path '*/node_modules' \) -prune \
    -o -type f -name '*.stderr' -print 2>/dev/null)
if [[ ${#leaks[@]} -eq 0 ]]; then
    echo "✓ no absolute paths in .stderr fixtures"
else
    echo "✗ ${#leaks[@]} .stderr fixture line(s) leak machine-specific paths:"
    printf '    %s\n' "${leaks[@]}"
    fails=$((fails+1))
fi

# 5. RP-RUSTC-DRIFT-CONTAINED (pinning half) — every train workspace root
#    must pin rustc to an exact channel ("1.X.Y" or a dated nightly), never
#    "stable" / "nightly" / "beta". Lets rustc bumps land in dedicated PRs
#    with classified diffs instead of slipping in via `rustup update`.
pinning_misses=()
for d in framework/bedrock machinery/runtime-runway machinery/commerce-rails; do
    [[ -f "$d/Cargo.toml" ]] || continue
    toolchain="$d/rust-toolchain.toml"
    if [[ ! -f "$toolchain" ]]; then
        pinning_misses+=("$d: rust-toolchain.toml missing")
        continue
    fi
    channel=$(grep -E '^channel[[:space:]]*=' "$toolchain" | head -1 | sed -E 's/.*"([^"]*)".*/\1/')
    if [[ "$channel" =~ ^1\.[0-9]+\.[0-9]+$ ]] || [[ "$channel" =~ ^nightly-[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
        : # pinned
    else
        pinning_misses+=("$d: channel=\"$channel\" (need 1.X.Y or nightly-YYYY-MM-DD)")
    fi
done
if [[ ${#pinning_misses[@]} -eq 0 ]]; then
    echo "✓ every train workspace pins rustc to an exact channel"
else
    echo "✗ ${#pinning_misses[@]} train workspace(s) have unpinned rustc:"
    printf '    %s\n' "${pinning_misses[@]}"
    fails=$((fails+1))
fi

# 6. RP-YANK-DISCOVERABLE — KB/release-history.md is the yank trail.
#    Every `### <crate> v<ver>` entry must declare the four required
#    fields (Yanked, Reason, Successor, Migration). Lints structure
#    only — content is reviewed at PR time. The actual crates.io
#    cross-check (every yanked version on crates.io has a row here)
#    is a future tightening and lives outside this fast lint.
yank_viol=$(python3 - <<'PY'
import re
from pathlib import Path
p = Path("KB/release-history.md")
if not p.exists():
    print("FILE_MISSING")
else:
    text = p.read_text()
    # Split into entry blocks. Header is "### <crate-name> v<version>"
    # where crate-name matches Cargo's allowed name set.
    pattern = re.compile(r'^### ([a-zA-Z0-9][a-zA-Z0-9_-]*) v(\S+)\s*$', re.MULTILINE)
    matches = list(pattern.finditer(text))
    for i, m in enumerate(matches):
        name, ver = m.group(1), m.group(2)
        start = m.end()
        end = matches[i+1].start() if i+1 < len(matches) else len(text)
        block = text[start:end]
        for field in ("Yanked:", "Reason:", "Successor:", "Migration:"):
            if f"- {field}" not in block:
                print(f"{name} v{ver}: missing required field '{field}'")
PY
)
if [[ "$yank_viol" == "FILE_MISSING" ]]; then
    echo "✗ KB/release-history.md missing"
    fails=$((fails+1))
elif [[ -z "$yank_viol" ]]; then
    echo "✓ KB/release-history.md entries have required fields"
else
    line_count=$(echo "$yank_viol" | wc -l | tr -d ' ')
    echo "✗ $line_count release-history entry/entries missing required field(s):"
    echo "$yank_viol" | sed 's/^/    /'
    fails=$((fails+1))
fi

# 7. Repository-boundary layering — foundation, extension, showcase, and
#    test workspaces (platform layer) must NOT path-dep into product
#    workspaces (marquee-apps, studio-apps, beacon-sites, mobile-apps).
#    Products consume the platform; never the other way. Same shape as
#    check 2 (RP-LAYERING) but checks a different axis — repository
#    boundary rather than publish status.
#
#    Standard: KB/05-engineering/standards/repo-layering.md.
boundary_viol=$(python3 - <<'PY'
import json, os, subprocess
# Workspaces that sit upstream of product apps in the architecture.
upstream_only = [
    "framework/bedrock",
    "machinery/runtime-runway",
    "machinery/commerce-rails",
]
# Path fragments that designate product (end-user app) workspaces.
product_dirs = ["marquee", "studio", "mobile"]
for ws in upstream_only:
    if not os.path.isfile(os.path.join(ws, "Cargo.toml")):
        continue
    try:
        out = subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=ws, stderr=subprocess.DEVNULL, timeout=30,
        )
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
        # cargo metadata failure is reported by check 2; skip here.
        continue
    meta = json.loads(out)
    for p in meta["packages"]:
        for d in p.get("dependencies", []):
            dep_path = d.get("path") or ""
            for prod in product_dirs:
                if f"/{prod}/" in dep_path:
                    print(f"{ws}: {p['name']} → {d['name']} (path-dep into {prod}/)")
                    break
PY
)
if [[ -z "$boundary_viol" ]]; then
    echo "✓ no upstream workspace path-deps into a product workspace"
else
    line_count=$(echo "$boundary_viol" | wc -l | tr -d ' ')
    echo "✗ $line_count repository-boundary layering violation(s):"
    echo "$boundary_viol" | sed 's/^/    /'
    fails=$((fails+1))
fi

# 8. RP-BRANCH-HYGIENE — no stranded work outside traceable branches
#    (QF-2026-07-02-06). Stashes carry no intent and rot on one machine;
#    WIP belongs on an e{N}/lin-XX-slug branch as (wip:) commits. Any
#    stash anywhere in the fleet fails the check. Dirty tracked files
#    are a warning only: normal mid-session, and CI checkouts are
#    always clean — the warning is the local tight-loop nudge.
#
#    Standard: KB/05-engineering/standards/branch-hygiene.md.
stash_viol=""
hygiene_warn=""
while IFS= read -r gitdir; do
    repo=$(dirname "$gitdir"); repo=${repo#./}; [[ -z "$repo" ]] && repo="."
    n=$(git -C "$repo" stash list 2>/dev/null | wc -l | tr -d ' ')
    if [[ "$n" -gt 0 ]]; then
        stash_viol+="$repo: $n stash(es) — $(git -C "$repo" stash list --format='%gs' | head -1)"$'\n'
    fi
    d=$(git -C "$repo" status --porcelain 2>/dev/null | grep -c -v '^??')
    if [[ "$d" -gt 0 ]]; then
        hygiene_warn+="$repo: $d dirty tracked file(s) on $(git -C "$repo" branch --show-current)"$'\n'
    fi
done < <(find . -maxdepth 3 -name .git \( -type d -o -type f \) 2>/dev/null | sort)
if [[ -z "$stash_viol" ]]; then
    echo "✓ no stashes across the fleet (RP-BRANCH-HYGIENE)"
else
    stash_count=$(echo -n "$stash_viol" | grep -c .)
    echo "✗ $stash_count repo(s) with stranded stashes (WIP belongs on a lin-XX branch):"
    echo -n "$stash_viol" | sed 's/^/    /'
    fails=$((fails+1))
fi
if [[ -n "$hygiene_warn" ]]; then
    echo "⚠ dirty tracked files (fine mid-session; debt if they outlive it):"
    echo -n "$hygiene_warn" | sed 's/^/    /'
fi

# 9. RP-HELMS-SUBSTRATE-SEAM — Foundation crates (framework/bedrock/)
#    must not path-dep machinery/runtime-runway without an explicit
#    `# RP-HELMS-SUBSTRATE-SEAM` exemption comment in the same Cargo.toml.
#    Approved edges: helm-coordination, helm-governed-jobs, helm-session-host
#    (all carry the comment; documented in repo-layering.md seam table).
#
#    Standard: KB/05-engineering/standards/repo-layering.md (check added RFL-128, 2026-07-04).
seam_viol=""
while IFS= read -r cargo_toml; do
    if grep -qE 'path\s*=\s*"[^"]*runtime-runway' "$cargo_toml"; then
        if ! grep -q '# RP-HELMS-SUBSTRATE-SEAM' "$cargo_toml"; then
            crate=$(awk -F'"' '/^name[[:space:]]*=/ {print $2; exit}' "$cargo_toml")
            seam_viol+="${cargo_toml}: ${crate} path-deps runtime-runway without # RP-HELMS-SUBSTRATE-SEAM"$'\n'
        fi
    fi
done < <(find framework/bedrock -name "Cargo.toml" -not -path "*/target/*" 2>/dev/null | sort)
if [[ -z "$seam_viol" ]]; then
    echo "✓ no unapproved Foundation→runtime-runway path-deps (RP-HELMS-SUBSTRATE-SEAM)"
else
    seam_count=$(echo -n "$seam_viol" | grep -c .)
    echo "✗ $seam_count unapproved Foundation→runtime-runway dep(s) (add # RP-HELMS-SUBSTRATE-SEAM or remove):"
    echo -n "$seam_viol" | sed 's/^/    /'
    fails=$((fails+1))
fi

# Summary + exit code.
if [[ "$fails" -gt 0 ]]; then
    echo "── ✗ project-doctor: $fails check(s) failed ──"
else
    echo "── ✓ project-doctor: all checks passed ──"
fi
exit "$fails"
