#!/usr/bin/env bash
set -euo pipefail

workspace="${GITHUB_WORKSPACE:-$(git rev-parse --show-toplevel)}"

# REFLECTIVE_SIBLING_REF controls which branch siblings are cloned from (default: main).
# Validate to only allow chars legal in git branch names; reject anything else so a
# malformed value cannot be used for argument injection in the git clone call below.
_raw_ref="${REFLECTIVE_SIBLING_REF:-main}"
if [[ ! "$_raw_ref" =~ ^[a-zA-Z0-9/._-]+$ ]]; then
  echo "warning: REFLECTIVE_SIBLING_REF='${_raw_ref}' contains disallowed characters; using main" >&2
  _raw_ref=main
fi
REFLECTIVE_SIBLING_REF="$_raw_ref"
unset _raw_ref

checkout_reflective_repo() {
  local repo="$1"
  local relative_path="$2"
  local dest="${workspace}/${relative_path}"
  local sibling_ref="${REFLECTIVE_SIBLING_REF:-main}"

  if [[ -d "$dest/.git" ]]; then
    echo "ok: ${relative_path} already checked out"
    return
  fi

  if [[ -e "$dest" ]]; then
    echo "error: ${dest} exists but is not a git checkout" >&2
    exit 1
  fi

  mkdir -p "$(dirname "$dest")"

  if [[ "$sibling_ref" != "main" ]]; then
    echo "==> checkout Reflective-Lab/${repo}@${sibling_ref} -> ${relative_path}"
    if GIT_TERMINAL_PROMPT=0 git clone --depth=1 --branch "$sibling_ref" --quiet \
        "https://github.com/Reflective-Lab/${repo}.git" "$dest" 2>/dev/null; then
      return
    fi
    echo "    (branch '${sibling_ref}' not found; falling back to main)"
    rm -rf "$dest"
  fi

  echo "==> checkout Reflective-Lab/${repo}@main -> ${relative_path}"
  GIT_TERMINAL_PROMPT=0 git clone --depth=1 --quiet "https://github.com/Reflective-Lab/${repo}.git" "$dest"
}

checkout_reflective_repo commerce-rails ../commerce-rails
# helm-module-contracts is a path-dep of runway-app-host (RFL-128). The crate is
# standalone (not a workspace member) so cloning just helms is sufficient — Cargo
# won't load the full helms workspace when following the path dep.
checkout_reflective_repo helms ../bedrock-platform/helms
