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

# ── Full Machinery CI ───────────────────────────────────────────────────

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
