# Machinery v1.0: Unified Operations Platform
# Coordinates: Build-Depot, Runtime-Runway, Commerce-Rails, Chart-Room

default:
    @just --list

# ── Build-Depot (Factory Authority) ─────────────────────────────────────

# Run Build-Depot factory doctor (drift checks)
doctor:
    cd build-depot && just doctor

factory-adoption:
    cd build-depot && just factory-adoption-doctor

scorecard:
    cd build-depot && just scorecard

# Validate machinery/release-train.yaml (RP-RELEASE-TRAIN-INTEGRITY): parseable,
# one name+dir per project, every member directory exists.
release-train-check:
    #!/usr/bin/env bash
    set -euo pipefail
    train="release-train.yaml"
    [[ -f "$train" ]] || { echo "✗ $train missing"; exit 1; }
    missing=0
    while IFS=$'\t' read -r name dir; do
        [[ -z "$name" ]] && continue
        if [[ -d "$dir" ]]; then echo "  ✓ $name → $dir"; else echo "  ✗ $name → $dir (missing)"; missing=1; fi
    done < <(awk '
        /^projects:/ { p=1; next }
        p && /^[^[:space:]]/ { p=0 }
        p && /^  - name:/ { n=$3 }
        p && /^    dir:/  { printf "%s\t%s\n", n, $2; n="" }
    ' "$train")
    [[ "$missing" -eq 0 ]] && echo "✓ release-train.yaml: all members present" || { echo "✗ release-train members missing"; exit 1; }

# Machinery factory drift gate (root-independent): quality-doctor + RP-table sync
# + release-train integrity. Mirrors what CI runs in .github/workflows/doctor.yml.
factory-doctor:
    #!/usr/bin/env bash
    set -uo pipefail
    cd build-depot && just quality-doctor; qd=$?
    cd "{{justfile_directory()}}/build-depot" && just rp-table-check | grep -q OK && echo "✓ rp-table-check: OK" || { echo "✗ rp-table-check: DRIFT"; qd=1; }
    cd "{{justfile_directory()}}" && just release-train-check; rt=$?
    total=$((qd + rt))
    [[ "$total" -eq 0 ]] && echo "── ✓ factory-doctor: machinery clean ──" || echo "── ✗ factory-doctor: $total check group(s) failed ──"
    exit "$total"

# ── Full Machinery CI ───────────────────────────────────────────────────

# Release-build all Rust sub-projects (build-depot is Bun/Trigger.dev, no build step)
build:
    #!/usr/bin/env bash
    set -e
    echo "=== Building commerce-rails ==="
    cd commerce-rails && just build
    echo "✅ commerce-rails build passed"
    echo ""
    echo "=== Building runtime-runway ==="
    cd ../runtime-runway && just build
    echo "✅ runtime-runway build passed"

# Check all sub-projects (type-check + compile)
check:
    #!/usr/bin/env bash
    set -e
    echo "=== Checking build-depot ==="
    cd build-depot && just check
    echo "✅ build-depot check passed"
    echo ""
    echo "=== Checking commerce-rails ==="
    cd ../commerce-rails && just check
    echo "✅ commerce-rails check passed"
    echo ""
    echo "=== Checking runtime-runway ==="
    cd ../runtime-runway && just check
    echo "✅ runtime-runway check passed"
    echo ""
    echo "=== Checking chart-room/strategic/validator ==="
    cd ../chart-room/strategic/validator && just check
    echo "✅ strategy-validator check passed"

# Run all tests
test:
    #!/usr/bin/env bash
    set -e
    echo "=== Testing build-depot ==="
    cd build-depot && just test
    echo ""
    echo "=== Testing commerce-rails ==="
    cd ../commerce-rails && just test
    echo ""
    echo "=== Testing runtime-runway ==="
    cd ../runtime-runway && just test
    echo ""
    echo "=== Testing chart-room/strategic/validator ==="
    cd ../chart-room/strategic/validator && just test

# Full CI pipeline (all checks, lints, tests)
ci:
    #!/usr/bin/env bash
    set -e
    echo "=== Build-Depot CI ==="
    cd build-depot && just ci
    echo ""
    echo "=== Commerce-Rails CI ==="
    cd ../commerce-rails && just check && just test
    echo ""
    echo "=== Runtime-Runway CI ==="
    cd ../runtime-runway && just check && just test
    echo ""
    echo "=== Strategy-Validator CI ==="
    cd ../chart-room/strategic/validator && just ci
    echo ""
    echo "✅ All machinery CI passed"

# Run security audits
security-audit:
    #!/usr/bin/env bash
    set -e
    echo "=== build-depot security audit ==="
    cd build-depot && just security-audit || echo "⚠️ build-depot audit reported issues"
    echo ""
    echo "=== commerce-rails security audit ==="
    cd ../commerce-rails && just security-audit || echo "⚠️ commerce-rails audit reported issues"
    echo ""
    echo "=== runtime-runway security audit ==="
    cd ../runtime-runway && just security-audit || echo "⚠️ runtime-runway audit reported issues"
    echo ""
    echo "=== strategy-validator security audit ==="
    cd ../chart-room/strategic/validator && just security-audit || echo "⚠️ strategy-validator audit reported issues"

# ── Local LLM Ops (Tier 1, on-box llama-server) ─────────────────────────
# See: build-depot/docs/operations/model-tiers.md
# Services: dev.reflective.llama.{gemma4-e2b,gemma3-1b} on ports 8080/8081

# Show launchctl status of both Tier-1 llama-server LaunchAgents
llm-status:
    #!/usr/bin/env bash
    launchctl list | awk 'NR==1 || /dev.reflective.llama/'

# Health-check both local llama-servers (127.0.0.1:8080, :8081)
llm-health:
    #!/usr/bin/env bash
    set -u
    for port in 8080 8081; do
      printf "port %s: " "$port"
      curl -sS --max-time 3 "http://127.0.0.1:${port}/health" || echo "unreachable"
      echo
    done

# One-shot chat completion smoke test against both models
llm-test:
    #!/usr/bin/env bash
    set -u
    for entry in "8080|gemma-4-e2b" "8081|gemma-3-1b"; do
      port="${entry%|*}"; name="${entry#*|}"
      echo "=== ${name} (port ${port}) ==="
      curl -sS --max-time 60 "http://127.0.0.1:${port}/v1/chat/completions" \
        -H 'content-type: application/json' \
        -d '{"messages":[{"role":"user","content":"Reply with just the word: ok"}],"max_tokens":10,"temperature":0}' \
        | python3 -c 'import sys,json; d=json.load(sys.stdin); print("reply:", repr(d["choices"][0]["message"]["content"])); t=d.get("timings",{}); print("prompt t/s:", round(t.get("prompt_per_second",0),1), " gen t/s:", round(t.get("predicted_per_second",0),1))'
      echo
    done

# Start (bootstrap) both LaunchAgents; safe to run when already loaded
llm-start:
    #!/usr/bin/env bash
    for label in gemma4-e2b gemma3-1b; do
      plist="$HOME/Library/LaunchAgents/dev.reflective.llama.${label}.plist"
      launchctl bootstrap "gui/$(id -u)" "$plist" 2>&1 | grep -v 'service already loaded' || true
      echo "started: $label"
    done

# Stop (bootout) both LaunchAgents
llm-stop:
    #!/usr/bin/env bash
    for label in gemma4-e2b gemma3-1b; do
      plist="$HOME/Library/LaunchAgents/dev.reflective.llama.${label}.plist"
      launchctl bootout "gui/$(id -u)" "$plist" 2>&1 | grep -v 'No such process' || true
      echo "stopped: $label"
    done

# Restart both services (kickstart, faster than stop+start)
llm-restart:
    #!/usr/bin/env bash
    for label in gemma4-e2b gemma3-1b; do
      launchctl kickstart -k "gui/$(id -u)/dev.reflective.llama.${label}"
      echo "restarted: $label"
    done

# Tail last 40 lines of both models' stderr logs (llama-server writes there)
llm-logs:
    #!/usr/bin/env bash
    for label in gemma4-e2b gemma3-1b; do
      echo "=== $label (last 40) ==="
      tail -n 40 "/Volumes/Lagring/tools/logs/${label}.err.log" 2>/dev/null \
        || echo "(no log yet)"
      echo
    done

# ── Machinery Monorepo Info ─────────────────────────────────────────────

status:
    #!/usr/bin/env bash
    echo "=== Machinery v1.0 Status ==="
    echo ""
    echo "Sub-projects:"
    echo "  • build-depot       (Node.js/Bun, Factory Authority)"
    echo "  • runtime-runway    (Rust, 405+ tests)"
    echo "  • commerce-rails    (Rust, 14+ tests)"
    echo "  • chart-room        (Documentation/Governance)"
    echo "  • strategy-validator (Rust, chart-room/strategic/validator, 106 tests)"
    echo ""
    echo "Git status:"
    git status --short
    echo ""
    echo "Latest commit:"
    git log -1 --oneline
