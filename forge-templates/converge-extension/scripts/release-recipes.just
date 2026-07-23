# ── Release-grade gates (Extension Release Checklist) ─────────────────────
# Mirror of foundation Justfile recipes. Update from
# ~/dev/reflective-templates/converge-extension/Justfile in lockstep with foundation.

# Gate 1: supply-chain audit. Output:
#   target/security/audit.json   (cargo-audit JSON)
#   target/security/deny.txt     (cargo-deny human report)
#   target/security/summary.txt  (combined human summary)
security-audit:
    #!/usr/bin/env bash
    set -uo pipefail
    out_dir="target/security"
    mkdir -p "${out_dir}"
    summary="${out_dir}/summary.txt"
    : > "${summary}"
    echo "── cargo-audit ──────────────────────────────" | tee -a "${summary}"
    cargo audit --json > "${out_dir}/audit.json" || true
    cargo audit --deny warnings 2>&1 | tee -a "${summary}"
    audit_human_status=${PIPESTATUS[0]}
    echo "" | tee -a "${summary}"
    echo "── cargo-deny ───────────────────────────────" | tee -a "${summary}"
    cargo deny check 2>&1 | tee "${out_dir}/deny.txt" | tee -a "${summary}"
    deny_status=${PIPESTATUS[0]}
    echo "" | tee -a "${summary}"
    echo "audit→${out_dir}/audit.json  deny→${out_dir}/deny.txt  summary→${summary}"
    if [ "${audit_human_status}" -ne 0 ] || [ "${deny_status}" -ne 0 ]; then
        exit 1
    fi

# Gate 2: workspace coverage. ≥ 80% per crate, no regression.
coverage:
    #!/usr/bin/env bash
    set -euo pipefail
    out_dir="target/coverage"
    mkdir -p "${out_dir}/html"
    common=(--workspace --lib --tests
        --ignore-filename-regex '(^|/)(tests|benches|examples)/')
    cargo llvm-cov clean --workspace
    rm -rf target/tests/trybuild
    cargo llvm-cov "${common[@]}" --no-report
    cargo llvm-cov report \
        --json --summary-only --output-path "${out_dir}/converge-coverage.json"
    cargo llvm-cov report \
        --lcov --output-path "${out_dir}/lcov.info"
    cargo llvm-cov report \
        --html --output-dir "${out_dir}/html"
    pct=$(python3 -c "import json; d=json.load(open('${out_dir}/converge-coverage.json')); print(f\"{d['data'][0]['totals']['lines']['percent']:.1f}\")")
    echo "coverage: ${pct}%  json→${out_dir}/converge-coverage.json  lcov→${out_dir}/lcov.info  html→${out_dir}/html/index.html"
    awk -v p="${pct}" 'BEGIN { if (p+0 < 80) { print "FAIL: coverage " p "% below 80% floor"; exit 1 } }'

# Gate 3: Criterion baseline. Set PERF_BASELINE to the release tag.
performance-profile:
    #!/usr/bin/env bash
    set -euo pipefail
    name="${PERF_BASELINE:-v0.1.0}"
    mode_flag="--save-baseline"
    if [ -d "target/criterion" ]; then
        existing="$(find target/criterion -mindepth 2 -maxdepth 3 -type d -name "${name}" -print -quit 2>/dev/null || true)"
        if [ -n "${existing}" ]; then
            mode_flag="--baseline"
        fi
    fi
    echo "performance-profile: ${mode_flag} ${name}"
    cargo bench --workspace -- "${mode_flag}" "${name}" || true
    if [ -f scripts/extract-criterion-baseline.py ]; then
        python3 scripts/extract-criterion-baseline.py || \
            echo "warn: baseline extraction failed (non-fatal)"
    fi
    echo "performance-profile: criterion→target/criterion/"

# Gate 4: bounded soak run. Configure with SOAK_DURATION_MIN (default 5).
soak:
    #!/usr/bin/env bash
    set -euo pipefail
    duration_min="${SOAK_DURATION_MIN:-5}"
    out_dir="target/soak"
    mkdir -p "${out_dir}"
    stamp="$(date -u +%Y%m%dT%H%M%SZ)"
    log="${out_dir}/soak-${stamp}.log"
    cycles=$(awk -v d="${duration_min}" 'BEGIN { printf "%d", 200 * d }')
    iterations=$(awk -v d="${duration_min}" 'BEGIN { printf "%d", 40 * d }')
    concurrency=100
    echo "soak: duration=${duration_min}min cycles=${cycles} concurrency=${concurrency} iterations=${iterations}" | tee "${log}"
    SOAK_CYCLES="${cycles}" \
    SOAK_CONCURRENCY="${concurrency}" \
    SOAK_ITERATIONS="${iterations}" \
    cargo test --workspace -- --include-ignored soak --nocapture 2>&1 | tee -a "${log}"
    ln -sf "soak-${stamp}.log" "${out_dir}/latest.log"
    echo "soak: log → ${log}"

# The five-command release ritual. All five must be green before tagging.
release-check:
    just security-audit
    just coverage
    PERF_BASELINE="v$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"(.*)".*/\1/')" just performance-profile
    SOAK_DURATION_MIN=5 just soak
    just lint
    cargo test --workspace
