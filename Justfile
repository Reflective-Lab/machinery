default:
    @just --list

# Canonical CI aggregate (RP-CI-PARITY): CI runs exactly `just ci`.
# (KB/05-engineering/standards/ci-parity.md in the root reflective
# repo). Runs every gate CI runs, in CI order. Local exit code should
# match GitHub Actions' verdict on the same commit.
#
# Order matters: fmt-check is fastest fail, then check (compile),
# then lint (clippy needs compile), then test. security-audit runs in
# the Security workflow (and locally on demand), not in `ci`.
ci: fmt-check check lint test

# Type-check the entire workspace including tests, benches, examples.
# --all-targets is non-negotiable: it's what catches symbols only used
# in test code from drifting out of the API surface.
check:
    cargo check --workspace --all-targets

# Run the full test suite, also covering tests/benches/examples.
test:
    cargo test --workspace --all-targets

# Format every file in the workspace. `fmt` is the write action;
# `fmt-check` is the read-only verifier (CI runs fmt-check).
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Clippy with `-D warnings`: every warning is treated as an error so
# the local + CI verdicts always agree. The workspace-level
# `[workspace.lints.clippy]` config decides which lints fire; this
# recipe enforces the policy.
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# `cargo audit` with the ignore list at `.audit-ignores`. The file
# format is one RUSTSEC ID per line; `#` comments and blank lines
# allowed. Drift between repos is trackable by reading one file per
# workspace.
security-audit:
    #!/usr/bin/env bash
    set -uo pipefail
    if [[ ! -f .audit-ignores ]]; then
        cargo audit --deny warnings
    else
        ignores=""
        while IFS= read -r line; do
            line="${line%%#*}"
            line="${line## }"
            line="${line%% }"
            [[ -z "$line" ]] && continue
            ignores="$ignores --ignore $line"
        done < .audit-ignores
        cargo audit --deny warnings $ignores
    fi

# Build-Depot delivery gate. Commerce Rails is a commercial library and deploy
# recipe authority; Runtime Runway owns hosted runtime deployment.
delivery-preflight: ci
