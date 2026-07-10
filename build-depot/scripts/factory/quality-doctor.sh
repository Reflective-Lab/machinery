#!/usr/bin/env bash
# quality-doctor — canonical implementation owned by Build-Depot
# (machinery/build-depot). Invoked as a thin runner from the root
# workspace Justfile with cwd = workspace root. Semantics:
# build-depot/docs/operations/quality-gates.md
set -uo pipefail
fails=0
echo "── quality-doctor ──"

# 1. Both policy files exist and are tracked.
for f in AGENTS.md QUALITY_BACKLOG.md; do
    if [[ -f "$f" ]] && git ls-files --error-unmatch "$f" >/dev/null 2>&1; then
        echo "✓ $f present and tracked"
    else
        echo "✗ $f missing or untracked"
        fails=$((fails+1))
    fi
done

# 2. Snapshot block "Last review" date is within 14 days.
last_review=$(grep -m1 '^- Last review:' QUALITY_BACKLOG.md | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | head -1)
if [[ -z "$last_review" ]]; then
    echo "✗ Snapshot block has no '- Last review: YYYY-MM-DD' line"
    fails=$((fails+1))
elif days_since=$(python3 -c "from datetime import date; print((date.today() - date.fromisoformat('$last_review')).days)" 2>/dev/null); then
    if [[ "$days_since" -le 14 ]]; then
        echo "✓ Snapshot Last review $last_review ($days_since day(s) ago)"
    else
        echo "✗ Snapshot Last review $last_review is $days_since days old (>14)"
        fails=$((fails+1))
    fi
else
    echo "✗ could not parse Last review date $last_review (python3 missing?)"
    fails=$((fails+1))
fi

# 3. Every QF-* ID cited in AGENTS.md exists in QUALITY_BACKLOG.md.
missing_qfs=()
while IFS= read -r id; do
    [[ -z "$id" ]] && continue
    if ! grep -qF "$id" QUALITY_BACKLOG.md; then
        missing_qfs+=("$id")
    fi
done < <(grep -oE 'QF-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]+' AGENTS.md | sort -u)
if [[ "${#missing_qfs[@]}" -eq 0 ]]; then
    echo "✓ all QF-* IDs cited in AGENTS.md exist in the ledger"
else
    echo "✗ ${#missing_qfs[@]} QF-* ID(s) cited in AGENTS.md missing from the ledger:"
    printf '    %s\n' "${missing_qfs[@]}"
    fails=$((fails+1))
fi

# 4. Every RP-* row's Tracked-by references open findings or '—'.
open_ids=$(awk '
    /^### Bucket [ABCD]/ {in_open=1; next}
    /^## (Accepted Risks|Completed Findings|PR Quality Gates|Review Cycles)/ {in_open=0; next}
    in_open && /^#### QF-/ {
        sub(/^#### /, "")
        print
    }
' QUALITY_BACKLOG.md)
bad_tracked=()
while IFS= read -r line; do
    rp_id=$(echo "$line" | awk -F'|' '{print $2}' | awk '{$1=$1; print}')
    tracked=$(echo "$line" | awk -F'|' '{print $6}' | awk '{$1=$1; print}' | sed -E 's/[[:space:]]+$//')
    # '—' is the legitimate no-tracker marker.
    [[ "$tracked" == "—" || -z "$tracked" ]] && continue
    IFS=',' read -ra ids <<<"$tracked"
    for raw_id in "${ids[@]}"; do
        id=$(echo "$raw_id" | tr -d ' ')
        [[ -z "$id" ]] && continue
        if ! echo "$open_ids" | grep -qx "$id"; then
            bad_tracked+=("$rp_id: $id (not in open buckets)")
        fi
    done
done < <(grep -E '^\| RP-' QUALITY_BACKLOG.md)
if [[ "${#bad_tracked[@]}" -eq 0 ]]; then
    echo "✓ RP-* Tracked-by entries reference open findings or '—'"
else
    echo "✗ ${#bad_tracked[@]} stale RP-* Tracked-by reference(s):"
    printf '    %s\n' "${bad_tracked[@]}"
    fails=$((fails+1))
fi

# 5. Cross-references paths in QUALITY_BACKLOG.md exist or are annotated.
ref_paths=$(awk '/^## Cross-references/{in_block=1; next} in_block && /^## /{in_block=0} in_block' QUALITY_BACKLOG.md | grep -oE '`KB/[^`]+`' | tr -d '`' | sort -u)
missing_paths=()
while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if [[ ! -e "$path" ]]; then
        # Allow 'Created on first use' annotation anywhere the path is mentioned.
        if ! grep -F "$path" QUALITY_BACKLOG.md | grep -qF "Created on first use"; then
            missing_paths+=("$path")
        fi
    fi
done <<<"$ref_paths"
if [[ "${#missing_paths[@]}" -eq 0 ]]; then
    echo "✓ Cross-references paths exist or are annotated 'Created on first use'"
else
    echo "✗ ${#missing_paths[@]} Cross-references path(s) missing without annotation:"
    printf '    %s\n' "${missing_paths[@]}"
    fails=$((fails+1))
fi

# 6. Root agent-pointer files exist and are tracked (QF-2026-06-02-16).
for f in CLAUDE.md CODEX.md GEMINI.md; do
    if [[ -f "$f" ]] && git ls-files --error-unmatch "$f" >/dev/null 2>&1; then
        echo "✓ $f present and tracked"
    else
        echo "✗ $f missing or untracked"
        fails=$((fails+1))
    fi
done

# 7. Snapshot bullet counts match observable state (QF-2026-06-02-17).
snap_out=$(just snapshot 2>/dev/null)
snap_mismatch=0
snap_lines=()
while IFS= read -r expected_line; do
    [[ -z "$expected_line" ]] && continue
    label=$(echo "$expected_line" | sed -E 's/^- ([^:]+):.*$/\1/')
    expected_n=$(echo "$expected_line" | grep -oE '\*\*[0-9]+\*\*' | head -1 | tr -d '*')
    actual_line=$(grep -F -- "- ${label}:" QUALITY_BACKLOG.md | head -1)
    actual_n=$(echo "$actual_line" | grep -oE '\*\*[0-9]+\*\*' | head -1 | tr -d '*')
    if [[ -z "$actual_line" ]]; then
        snap_mismatch=$((snap_mismatch+1))
        snap_lines+=("    ${label}: expected ${expected_n}, file has no matching bullet")
    elif [[ "$expected_n" != "$actual_n" ]]; then
        snap_mismatch=$((snap_mismatch+1))
        snap_lines+=("    ${label}: expected ${expected_n}, file has ${actual_n}")
    fi
done <<<"$snap_out"
if [[ "$snap_mismatch" -eq 0 ]]; then
    echo "✓ Snapshot bullet counts match observable state"
else
    echo "✗ Snapshot bullet counts drifted ($snap_mismatch mismatch(es)); run \`just snapshot\`:"
    printf '%s\n' "${snap_lines[@]}"
    fails=$((fails+1))
fi

# 8. RP-* table in QUALITY_BACKLOG.md matches the JSON source-of-truth
#    (QF-2026-06-02-18). The region between
#    `<!-- BEGIN GENERATED RP-TABLE -->` and `<!-- END GENERATED RP-TABLE -->`
#    is regenerated from `KB/05-engineering/standards/recurring-properties.json`
#    via `just rp-table-sync`. Direct edits to the Markdown table fail this
#    check.
if [[ -f scripts/rp-table-check.py && -f KB/05-engineering/standards/recurring-properties.json ]]; then
    rp_drift=$(python3 scripts/rp-table-check.py 2>/dev/null)
    case "$rp_drift" in
        OK)
            echo "✓ RP-* table in QUALITY_BACKLOG.md matches JSON source"
            ;;
        DRIFT)
            echo "✗ RP-* table drifted from JSON source; run \`just rp-table-sync\`"
            fails=$((fails+1))
            ;;
        MISSING_MARKERS)
            echo "✗ RP-TABLE markers missing in QUALITY_BACKLOG.md"
            fails=$((fails+1))
            ;;
        *)
            echo "✗ RP-table drift check failed unexpectedly (output: $rp_drift)"
            fails=$((fails+1))
            ;;
    esac
fi

# Summary + exit code.
if [[ "$fails" -gt 0 ]]; then
    echo "── ✗ quality-doctor: $fails check(s) failed ──"
else
    echo "── ✓ quality-doctor: all checks passed ──"
fi
exit "$fails"
