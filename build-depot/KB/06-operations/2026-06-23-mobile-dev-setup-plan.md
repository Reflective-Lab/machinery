# Mobile Dev Setup Script — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Write `scripts/setup-mobile-dev.sh` — a single shell script that clones the exact nested repo structure needed for mobile development into `~/dev/reflective/`.

**Architecture:** Sequential `git clone` calls under `set -euo pipefail`. Conditionally clones the `reflective` root only if `~/dev/reflective` does not yet exist, then clones all sub-repos into their correct nested positions. No parallelism, no re-run safety — designed for one-time use on a fresh machine.

**Tech Stack:** bash, git (SSH)

## Global Constraints

- All clones target `main` branch via `--branch main`
- All clones use SSH remotes (`git@github.com:...`)
- `set -euo pipefail` — any failure exits immediately
- No toolchain installation, no SSH setup, no direnv — git structure only
- Script lives at `reflective/scripts/setup-mobile-dev.sh`

---

### Task 1: Write, verify, and commit the setup script

**Files:**
- Create: `scripts/setup-mobile-dev.sh`

**Interfaces:**
- Produces: executable shell script at `scripts/setup-mobile-dev.sh`

- [ ] **Step 1: Write the script**

Create `scripts/setup-mobile-dev.sh` with the following content:

```bash
#!/usr/bin/env bash
set -euo pipefail

BASE="$HOME/dev/reflective"
ROOT_REMOTE="git@github.com:Reflective-Lab/reflective.git"

if [ ! -d "$BASE" ]; then
  echo "→ Cloning reflective root..."
  git clone --branch main "$ROOT_REMOTE" "$BASE"
fi

cd "$BASE"

echo "→ Cloning marquee-apps..."
git clone --branch main git@github.com:Reflective-Lab/marquee-apps.git marquee-apps

echo "→ Cloning quorum-sense..."
git clone --branch main git@github.com:Reflective-Lab/quorum-sense.git marquee-apps/quorum-sense

echo "→ Cloning atlas-integration..."
git clone --branch main git@github.com:Reflective-Lab/atlas-integration.git marquee-apps/atlas-integration

echo "→ Cloning vouch-lending..."
git clone --branch main git@github.com:Reflective-Lab/vouch-lending.git marquee-apps/vouch-lending

echo "→ Cloning bedrock-platform..."
git clone --branch main git@github.com:Reflective-Lab/bedrock-platform.git bedrock-platform

echo "→ Cloning axiom..."
git clone --branch main git@github.com:Reflective-Lab/axiom.git bedrock-platform/axiom

echo "→ Cloning converge..."
git clone --branch main git@github.com:Reflective-Lab/converge.git bedrock-platform/converge

echo "→ Cloning organism..."
git clone --branch main git@github.com:Reflective-Lab/organism.git bedrock-platform/organism

echo "→ Cloning helms..."
git clone --branch main git@github.com:Reflective-Lab/helms.git bedrock-platform/helms

echo "→ Cloning runtime-runway..."
git clone --branch main git@github.com:Reflective-Lab/runtime-runway.git runtime-runway

echo "→ Cloning commerce-rails..."
git clone --branch main git@github.com:Reflective-Lab/commerce-rails.git commerce-rails

echo "→ Cloning mosaic-extensions..."
git clone --branch main git@github.com:Reflective-Lab/mosaic-extensions.git mosaic-extensions

echo ""
echo "✓ Mobile dev workspace ready at $BASE"
```

- [ ] **Step 2: Verify syntax**

```bash
bash -n scripts/setup-mobile-dev.sh
```

Expected: no output, exit 0.

- [ ] **Step 3: Make executable**

```bash
chmod +x scripts/setup-mobile-dev.sh
```

- [ ] **Step 4: Commit**

```bash
git add scripts/setup-mobile-dev.sh
git commit -m "chore: add mobile dev workspace setup script"
```
