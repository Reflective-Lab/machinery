#!/usr/bin/env bash
#
# Mechanical detector for RP-TEST-CODE-ATTRIBUTION at PR time.
# Closes QF-2026-06-07-01 (the hook/CI half; convention half shipped in
# AGENTS.md > Test/code attribution 2026-06-07).
#
# Lists files changed between BASE_REF and HEAD_REF, detects src+test
# file pairs via heuristics A and B (heuristics C and D — in-source
# `#[cfg(test)] mod tests` and doc-tests — deliberately out of scope
# for this pilot per the standard), and requires a classification line
# in either the PR body or any commit message body.
#
# Heuristics:
#   A. src/X.rs ↔ tests/X.rs OR tests/test_X.rs  (flat src files)
#   B. src/<path>/mod.rs ↔ src/<path>/tests.rs   (module-folder tests)
#
# Classification regex (one of these on a line by itself in PR body or
# commit message body):
#   ^Contract update:
#   ^Fixture refresh:
#   ^Real bug fix:
#
# Bypass: literal token `[skip-attribution]` anywhere in PR body or
# commit messages. For legitimate non-attribution cases like pure
# renames.
#
# Invoked by .github/workflows/test-code-attribution.yml. Standalone
# locally: BASE_REF=origin/main HEAD_REF=HEAD PR_BODY=... bash
# scripts/check-test-code-attribution.sh

set -uo pipefail

base="${BASE_REF:-origin/main}"
head="${HEAD_REF:-HEAD}"

changed=$(git diff --name-only "$base...$head" 2>/dev/null)
if [[ -z "$changed" ]]; then
    echo "✓ no changed files; skip"
    exit 0
fi

pairs=()
while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    # Heuristic A: flat src/X.rs (no nested path; [^/]+ in regex).
    if [[ "$f" =~ ^(.+)/src/([^/]+)\.rs$ ]]; then
        crate_root="${BASH_REMATCH[1]}"
        base_name="${BASH_REMATCH[2]}"
        for candidate in "$crate_root/tests/$base_name.rs" "$crate_root/tests/test_$base_name.rs"; do
            if printf '%s\n' "$changed" | grep -qFx -- "$candidate"; then
                pairs+=("$f  ↔  $candidate")
            fi
        done
    fi
    # Heuristic B: src/<path>/mod.rs ↔ src/<path>/tests.rs.
    if [[ "$f" =~ ^(.+)/mod\.rs$ ]]; then
        mod_dir="${BASH_REMATCH[1]}"
        candidate="$mod_dir/tests.rs"
        if printf '%s\n' "$changed" | grep -qFx -- "$candidate"; then
            pairs+=("$f  ↔  $candidate")
        fi
    fi
done <<<"$changed"

if [[ ${#pairs[@]} -eq 0 ]]; then
    echo "✓ no src+test pairs in this diff"
    exit 0
fi

echo "── src+test pairs detected ──"
printf '    %s\n' "${pairs[@]}"
echo

search_text="${PR_BODY:-}"
search_text="${search_text}"$'\n'"$(git log --format=%B "$base...$head" 2>/dev/null)"

if grep -qF -- "[skip-attribution]" <<<"$search_text"; then
    echo "✓ '[skip-attribution]' bypass token found; skip"
    exit 0
fi

if grep -qE '^(Contract update|Fixture refresh|Real bug fix):' <<<"$search_text"; then
    echo "✓ classification line found in PR body or commit messages"
    exit 0
fi

cat <<'EOF'
✗ src+test pair detected without classification.

This PR modifies production source and a directly-corresponding test
in the same change. Per AGENTS.md > Test/code attribution and
RP-TEST-CODE-ATTRIBUTION, you must classify this change in the PR body
or in a commit message body. Choose one:

  Contract update: <why the test had to follow the production change>
  Fixture refresh: <why the production code had to follow the test change>
  Real bug fix:    <why both moved to fix a real defect>

To bypass for a legitimate non-attribution case (e.g. a pure rename),
include the literal token '[skip-attribution]' in the PR body.

See AGENTS.md > Test/code attribution for the policy. QF-2026-06-07-01
tracked this enforcement.
EOF
exit 1
