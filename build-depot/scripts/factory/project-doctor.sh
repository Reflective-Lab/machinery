#!/usr/bin/env bash
# project-doctor — canonical implementation owned by Build-Depot
# (machinery/build-depot). Release-train-driven and reusable: it derives the
# workspace set from `release-train.yaml` in cwd (fresh_workspaces), so it works
# from the fleet root (which lists framework/bedrock + machinery/* dirs) AND from
# any single consumer repo (whose fresh_workspaces is `dir: .`). Invoked as a thin
# runner with cwd = repo root, or via the reusable factory-project-doctor.yml.
# Semantics: build-depot/docs/operations/quality-gates.md
set -uo pipefail
fails=0
echo "── project-doctor ──"

# 1. RP-RELEASE-TRAIN-INTEGRITY — release-train.yaml is the only source of truth.
#    Validates: yaml parseable + every named member directory exists.
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

# Derive the Rust workspace roots from release-train.yaml `fresh_workspaces`.
# Fleet root → framework/bedrock + machinery/runtime-runway + machinery/commerce-rails
# (reproduces the previous hardcoded train_dirs); a single consumer repo → its own
# workspace(s), typically ".". Everything below iterates this set — no repo hardcoded.
WORKSPACES=()
if [[ -f "$train_yaml" ]]; then
    while IFS= read -r d; do
        [[ -n "$d" ]] && WORKSPACES+=("$d")
    done < <(awk '
        /^fresh_workspaces:/ { in_sec=1; next }
        /^[^[:space:]]/ { in_sec=0 }
        in_sec { for (i=1;i<=NF;i++) if ($i=="dir:") { print $(i+1); break } }
    ' "$train_yaml" | sort -u)
fi
export PD_WORKSPACES="${WORKSPACES[*]:-}"

# 2. RP-LAYERING — a publishable crate may not path-dep an unpublishable
#    (publish=false / UNLICENSED) one. Walks every workspace via cargo metadata.
layer_viol=$(python3 - <<'PY'
import json, os, subprocess
workspaces = os.environ.get("PD_WORKSPACES", "").split()
for ws in workspaces:
    if not os.path.isfile(os.path.join(ws, "Cargo.toml")):
        continue
    try:
        out = subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=ws, stderr=subprocess.DEVNULL, timeout=60,
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

# 3. RP-CRATE-SIZE-BUDGET — leading indicator for the crates.io 10 MiB ceiling.
#    Any git-tracked file > 1 MiB in a workspace is a smell. Uses `git ls-files`
#    so gitignored caches are excluded. Resolves each workspace to its enclosing
#    git repo, so sub-workspaces of a monorepo are handled too.
big_files=()
for d in "${WORKSPACES[@]}"; do
    [[ -d "$d" ]] || continue
    repo_root=$(git -C "$d" rev-parse --show-toplevel 2>/dev/null) || continue
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
#    machine-specific absolute paths.
leaks=()
for d in "${WORKSPACES[@]}"; do
    [[ -d "$d" ]] || continue
    while IFS= read -r f; do
        [[ -z "$f" ]] && continue
        hits=$(grep -nE '/Users/|/home/[a-z]+/|/private/var/folders/|/tmp/[A-Za-z0-9]{6,}' "$f" 2>/dev/null | head -3)
        if [[ -n "$hits" ]]; then
            while IFS= read -r h; do
                leaks+=("${f}:${h}")
            done <<<"$hits"
        fi
    done < <(find "$d" \
        \( -path '*/target' -o -path '*/.git' -o -path '*/node_modules' \) -prune \
        -o -type f -name '*.stderr' -print 2>/dev/null)
done
if [[ ${#leaks[@]} -eq 0 ]]; then
    echo "✓ no absolute paths in .stderr fixtures"
else
    echo "✗ ${#leaks[@]} .stderr fixture line(s) leak machine-specific paths:"
    printf '    %s\n' "${leaks[@]}"
    fails=$((fails+1))
fi

# 5. RP-RUSTC-DRIFT-CONTAINED (pinning half) — every workspace root must pin rustc
#    to an exact channel ("1.X.Y" or a dated nightly).
pinning_misses=()
for d in "${WORKSPACES[@]}"; do
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
    echo "✓ every workspace pins rustc to an exact channel"
else
    echo "✗ ${#pinning_misses[@]} workspace(s) have unpinned rustc:"
    printf '    %s\n' "${pinning_misses[@]}"
    fails=$((fails+1))
fi

# 6. RP-YANK-DISCOVERABLE — KB/release-history.md is the yank trail (fleet-level
#    artifact). Every `### <crate> v<ver>` entry must declare the four required
#    fields. Skipped where the repo carries no release-history (not every consumer
#    owns the fleet yank trail).
if [[ ! -f "KB/release-history.md" ]]; then
    echo "○ RP-YANK-DISCOVERABLE skipped (no KB/release-history.md in this repo)"
else
    yank_viol=$(python3 - <<'PY'
import re
from pathlib import Path
text = Path("KB/release-history.md").read_text()
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
    if [[ -z "$yank_viol" ]]; then
        echo "✓ KB/release-history.md entries have required fields"
    else
        line_count=$(echo "$yank_viol" | wc -l | tr -d ' ')
        echo "✗ $line_count release-history entry/entries missing required field(s):"
        echo "$yank_viol" | sed 's/^/    /'
        fails=$((fails+1))
    fi
fi

# 7. Repository-boundary layering — an upstream (platform) workspace must NOT
#    path-dep into a product (end-user app) workspace. Scans the derived workspace
#    set; passes trivially in a repo that has no product path-deps.
#    Standard: KB/05-engineering/standards/repo-layering.md.
boundary_viol=$(python3 - <<'PY'
import json, os, subprocess
workspaces = os.environ.get("PD_WORKSPACES", "").split()
product_dirs = ["marquee", "studio", "mobile"]
for ws in workspaces:
    if not os.path.isfile(os.path.join(ws, "Cargo.toml")):
        continue
    try:
        out = subprocess.check_output(
            ["cargo", "metadata", "--no-deps", "--format-version", "1"],
            cwd=ws, stderr=subprocess.DEVNULL, timeout=60,
        )
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
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

# 8. RP-BRANCH-HYGIENE — no stranded stashes across nested repos under cwd.
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

# 9. RP-HELMS-SUBSTRATE-SEAM — a workspace Cargo.toml must not path-dep
#    `runtime-runway` without an explicit `# RP-HELMS-SUBSTRATE-SEAM` exemption.
#    Scans the derived workspace set; in a repo with no runtime-runway path-deps
#    (e.g. bedrock's own checkout) it passes trivially.
#    Standard: KB/05-engineering/standards/repo-layering.md.
seam_viol=""
for d in "${WORKSPACES[@]}"; do
    [[ -d "$d" ]] || continue
    while IFS= read -r cargo_toml; do
        if grep -qE 'path\s*=\s*"[^"]*runtime-runway' "$cargo_toml"; then
            if ! grep -q '# RP-HELMS-SUBSTRATE-SEAM' "$cargo_toml"; then
                crate=$(awk -F'"' '/^name[[:space:]]*=/ {print $2; exit}' "$cargo_toml")
                seam_viol+="${cargo_toml}: ${crate} path-deps runtime-runway without # RP-HELMS-SUBSTRATE-SEAM"$'\n'
            fi
        fi
    done < <(find "$d" -name "Cargo.toml" -not -path "*/target/*" 2>/dev/null | sort)
done
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
