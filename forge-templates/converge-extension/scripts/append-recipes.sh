#!/usr/bin/env bash
# Append missing release-grade recipes to a target Justfile.
#
# Reads the canonical recipe block from
# ~/dev/reflective/templates/converge-extension/scripts/release-recipes.just and
# appends only the recipes that are not already defined in the target.
#
# Usage:
#   append-recipes.sh <target-justfile>

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <target-justfile>" >&2
    exit 2
fi

target="$1"
template_dir="$(cd "$(dirname "$0")" && pwd)"
recipes="${template_dir}/release-recipes.just"

if [ ! -f "$target" ]; then
    echo "FAIL: target Justfile $target not found" >&2
    exit 1
fi
if [ ! -f "$recipes" ]; then
    echo "FAIL: canonical recipes file $recipes not found" >&2
    exit 1
fi

# All recipes the canonical block defines.
canonical_recipes=(security-audit coverage performance-profile soak release-check)

# Which are already in the target?
present=()
missing=()
for r in "${canonical_recipes[@]}"; do
    if grep -qE "^${r}:" "$target"; then
        present+=("$r")
    else
        missing+=("$r")
    fi
done

if [ "${#missing[@]}" -eq 0 ]; then
    echo "all recipes present, nothing to append: $target"
    exit 0
fi

echo "appending to $target: ${missing[*]}"
echo "(already present: ${present[*]:-none})"

# Extract just the missing recipes from the canonical file. Each recipe
# block runs from `^<name>:` until the next blank line followed by `^[a-z]`
# or end-of-file. Use awk to slice.
extract_recipe() {
    local name="$1"
    awk -v name="$name" '
        $0 ~ "^"name":" {
            in_block = 1
            # Capture preceding comment lines (already buffered)
            for (i = 0; i < buf_n; i++) print buf[i]
            buf_n = 0
            print
            next
        }
        in_block {
            # End of block: a line that begins at column 0 with letters/comment
            # AND is not indented and is not a blank line
            if (/^$/) {
                print
                next
            }
            if (/^[a-zA-Z_-]+:/) {
                in_block = 0
                next
            }
            print
            next
        }
        # Buffer comment/blank lines so we can attach them when the next recipe matches
        {
            if (/^#/) {
                buf[buf_n++] = $0
            } else if (/^$/) {
                # Reset buffer on blank if not yet a comment-doc target
                if (buf_n > 0) { buf[buf_n++] = $0 }
            } else {
                buf_n = 0
            }
        }
    ' "$recipes"
}

# Build append payload
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
{
    echo ""
    echo "# ── Release-grade gates (appended from ~/dev/reflective/templates/converge-extension) ─"
    echo "# Standard: https://github.com/Reflective-Lab/converge/blob/main/kb/Standards/Extension%20Release%20Checklist.md"
    for r in "${missing[@]}"; do
        echo ""
        extract_recipe "$r"
    done
} > "$tmp"

cat "$tmp" >> "$target"
echo "appended $(wc -l < "$tmp") lines to $target"
