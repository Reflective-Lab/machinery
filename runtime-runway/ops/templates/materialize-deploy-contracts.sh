#!/usr/bin/env bash
# RR D4 — materialize deploy_contracts into Cloud Run env/secret flags.
#
# Reads an app's `runway.app.json:deploy_contracts: [{key, version}]` (the D6
# manifest field) and, for each contract, the matching Commerce-Rails deploy
# recipe (`<key>@<version>.yaml`, QF-CR-04 format). It emits eval-able shell
# assignments that a deploy wrapper folds into `gcloud run deploy`:
#
#   RUNWAY_DEPLOY_ENV_VARS='NAME=value,NAME2=value2'
#   RUNWAY_DEPLOY_SECRETS='NAME=secret-id:version,...'
#
# This is the mechanism that keeps provider env-var NAMES (STRIPE_*, CR_*) out
# of every app's deploy script: the app only declares the contract; RR owns the
# names; the recipe (CR-owned) is the single source of truth.
#
# Materialization rules (per the recipe's `source` taxonomy):
#   source: secret  -> --set-secrets  NAME=<secret-id>:<version>
#                      secret-id convention: ${SECRET_PREFIX}-<lower(name|_->->)>
#                      (e.g. STRIPE_SECRET_KEY -> quorum-stripe-secret-key)
#   source: config  -> --set-env-vars NAME=<value-from-deploy-env>
#                      value read from the same-named env var at deploy time;
#                      required+unset is a hard error, optional+unset is skipped.
#
# Dependencies: bash, jq, awk. (No yq — the recipe parser is keyed to the
# version-pinned recipe_format_version "1.0" and guards against drift.)
#
# Usage:
#   eval "$(ops/templates/materialize-deploy-contracts.sh <app_dir>)"
#   gcloud run deploy ... --set-env-vars="$RUNWAY_DEPLOY_ENV_VARS" \
#                         --set-secrets="$RUNWAY_DEPLOY_SECRETS"
#
# Env overrides:
#   CR_RECIPES_DIR   directory of recipe yaml files
#                    (default: ../commerce-rails/kb/Contracts/Deploy Recipes)
#   SECRET_PREFIX    secret-id prefix (default: runway.app.json app_id)
#   SECRET_VERSION   secret version suffix (default: latest)
set -euo pipefail

SUPPORTED_RECIPE_FORMAT="1.0"

die() {
  echo "materialize-deploy-contracts: $*" >&2
  exit 1
}

APP_DIR="${1:-.}"
MANIFEST="${APP_DIR%/}/runway.app.json"
[[ -f "$MANIFEST" ]] || die "no runway.app.json at '$MANIFEST'"
command -v jq >/dev/null 2>&1 || die "jq is required"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RR_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CR_RECIPES_DIR="${CR_RECIPES_DIR:-$RR_ROOT/../commerce-rails/kb/Contracts/Deploy Recipes}"

APP_ID="$(jq -r '.app_id // empty' "$MANIFEST")"
SECRET_PREFIX="${SECRET_PREFIX:-$APP_ID}"
SECRET_VERSION="${SECRET_VERSION:-latest}"
[[ -n "$SECRET_PREFIX" ]] || die "could not determine SECRET_PREFIX (no app_id in manifest); set SECRET_PREFIX"

# Extract (required|optional, source, name) tuples from a format-1.0 recipe.
# Anchored on exact indentation so prose in `semantics:` blocks can't match.
parse_recipe() {
  local recipe="$1"
  local fmt
  fmt="$(awk -F'"' '/^recipe_format_version:/ {print $2; exit}' "$recipe")"
  [[ "$fmt" == "$SUPPORTED_RECIPE_FORMAT" ]] \
    || die "recipe '$recipe' format '$fmt' != supported '$SUPPORTED_RECIPE_FORMAT'"
  awk '
    /^env:/ { inenv=1; next }
    inenv && /^  required:/ { sect="required"; next }
    inenv && /^  optional:/ { sect="optional"; next }
    inenv && /^[^[:space:]]/ { inenv=0; sect="" }
    sect && /^    - name: / { cur=$3 }
    sect && /^      source: / { print sect"\t"$2"\t"cur }
  ' "$recipe"
}

secret_id_for() {
  local name="$1"
  echo "${SECRET_PREFIX}-$(echo "$name" | tr 'A-Z_' 'a-z-')"
}

env_pairs=()
secret_pairs=()

contracts="$(jq -c '.deploy_contracts // [] | .[]' "$MANIFEST")"
while IFS= read -r contract; do
  [[ -n "$contract" ]] || continue
  key="$(jq -r '.key' <<<"$contract")"
  version="$(jq -r '.version' <<<"$contract")"
  recipe="$CR_RECIPES_DIR/${key}@${version}.yaml"
  [[ -f "$recipe" ]] || die "no recipe for contract ${key}@${version} at '$recipe'"

  while IFS=$'\t' read -r tier source name; do
    [[ -n "$name" ]] || continue
    case "$source" in
      secret)
        secret_pairs+=("${name}=$(secret_id_for "$name"):${SECRET_VERSION}")
        ;;
      config)
        value="${!name:-}"
        if [[ -z "$value" ]]; then
          [[ "$tier" == "required" ]] \
            && die "recipe ${key}@${version} requires config var '$name' but it is unset in the deploy environment"
          continue
        fi
        env_pairs+=("${name}=${value}")
        ;;
      *)
        die "recipe ${key}@${version} var '$name' has unknown source '$source'"
        ;;
    esac
  done < <(parse_recipe "$recipe")
done <<<"$contracts"

join_csv() { local IFS=,; echo "$*"; }

printf "RUNWAY_DEPLOY_ENV_VARS=%q\n" "$(join_csv "${env_pairs[@]:-}")"
printf "RUNWAY_DEPLOY_SECRETS=%q\n" "$(join_csv "${secret_pairs[@]:-}")"
