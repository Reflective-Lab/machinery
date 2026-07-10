#!/usr/bin/env bash
set -euo pipefail

# commerce-rails-stripe depends on runway-storage via a sibling path
# dependency (`../runtime-runway/crates/runway-storage` relative to the
# workspace root). CI checks the sibling out next to this repo so the
# workspace resolves exactly like a local ~/dev/reflective checkout.
# Modeled on atelier-showcase/scripts/ci/checkout-reflective-siblings.sh.

workspace="${GITHUB_WORKSPACE:-$(git rev-parse --show-toplevel)}"

checkout_reflective_repo() {
  local repo="$1"
  local relative_path="$2"
  local dest="${workspace}/${relative_path}"

  if [[ -d "$dest/.git" ]]; then
    echo "ok: ${relative_path} already checked out"
    return
  fi

  if [[ -e "$dest" ]]; then
    echo "error: ${dest} exists but is not a git checkout" >&2
    exit 1
  fi

  mkdir -p "$(dirname "$dest")"
  echo "==> checkout Reflective-Lab/${repo} -> ${relative_path}"
  GIT_TERMINAL_PROMPT=0 git clone --depth=1 --quiet "https://github.com/Reflective-Lab/${repo}.git" "$dest"
}

checkout_reflective_repo runtime-runway ../runtime-runway
