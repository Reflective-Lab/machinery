#!/usr/bin/env bash
# Fleet CI board (the factory's andon light) — see
# KB/05-engineering/standards/ci-parity.md, "The feedback loop".
#
# Reads the fleet from release-train.yaml — projects in train order, with
# the mosaic container expanded to its sub-workspace aliases — and prints
# the latest main-branch conclusion per workflow per repo.
#
# Verdict: exit 1 if any repo has a failing workflow on main.
# "Dependabot Updates" is displayed but excluded from the verdict —
# dependabot's own update-job failures track dependency resolution,
# not the state of main. An andon light that cries wolf gets ignored.
#
# Works anywhere `gh` is authenticated: locally via `just factory-status`
# and in the scheduled factory-alert workflow.
set -uo pipefail
cd "$(dirname "$0")/../.."

ORG="Reflective-Lab"

dirs=()
while IFS= read -r d; do
    if [[ "$d" == "mosaic-extensions" ]]; then
        while IFS= read -r a; do dirs+=("$a"); done \
            < <(awk '/^aliases:/{p=1;next} /^[a-z_]+:/{if(p)exit} p&&NF>=2{print $2}' release-train.yaml)
    else
        dirs+=("$d")
    fi
done < <(awk '/^projects:/{p=1;next} /^[a-z_]+:/{if(p)exit} p&&/dir:/{print $2}' release-train.yaml)

red_repos=0
unknown_repos=0
for d in "${dirs[@]}"; do
    repo="$ORG/${d##*/}"
    # One retry: a transient API failure (503) must read as UNKNOWN, not
    # as "no runs" — unknown is not green.
    runs=$(gh run list -R "$repo" --branch main --limit 20 \
        --json workflowName,conclusion,status,updatedAt 2>/dev/null) \
      || { sleep 3; runs=$(gh run list -R "$repo" --branch main --limit 20 \
        --json workflowName,conclusion,status,updatedAt 2>/dev/null); } \
      || runs=""
    if [[ -z "$runs" ]]; then
        printf '%-22s  ? fetch failed (API error) — status unknown\n' "${d##*/}"
        unknown_repos=$((unknown_repos + 1))
        continue
    fi
    if [[ "$runs" == "[]" ]]; then
        printf '%-22s  ∅ no CI runs on main\n' "${d##*/}"
        continue
    fi
    line=$(jq -r '
        [group_by(.workflowName)[] | max_by(.updatedAt)]
        | map(
            (if .status != "completed" then "⏳"
             elif .conclusion == "success" then "✓"
             elif .conclusion == "failure" then "✗"
             else "•" end) + " " + .workflowName)
        | join("   ")' <<<"$runs")
    reds=$(jq '
        [group_by(.workflowName)[] | max_by(.updatedAt)
         | select(.workflowName != "Dependabot Updates"
                  and .status == "completed"
                  and .conclusion == "failure")]
        | length' <<<"$runs")
    printf '%-22s  %s\n' "${d##*/}" "$line"
    if (( reds > 0 )); then
        red_repos=$((red_repos + 1))
    fi
done

echo
if (( red_repos > 0 )); then
    echo "✗ FACTORY RED — ${red_repos} repo(s) with failing workflows on main"
    exit 1
fi
if (( unknown_repos > 0 )); then
    echo "? FACTORY UNKNOWN — ${unknown_repos} repo(s) unreadable (API errors); not claiming green"
    exit 2
fi
echo "✓ factory green — all fleet repos passing on main"
