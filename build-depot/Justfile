project := "proj_idrnrbkbldpxamvcvhqr"
api     := "https://api.trigger.dev/api/v1"

# Show available recipes
default:
    @just --list

# Typecheck project TypeScript
check:
    bun run check

# Run Bun tests
test:
    bun run test

# Run the local CI gate
ci:
    bun run ci

# Run all read-only factory drift checks
doctor:
    bun run doctor

# Check quality setup drift
quality-doctor:
    bun run quality:doctor

# Check security setup drift
security-doctor:
    bun run security:doctor

# Check delivery setup drift
delivery-doctor:
    bun run delivery:doctor

# Scan tracked files for obvious committed secrets
secrets-scan:
    bun run security:secrets

# Run dependency audit plus secret scan
security-audit:
    bun run security:audit

# Scan workspace repository adoption against the Build-Depot factory contract
factory-adoption-doctor:
    bun run factory:adoption

# Emit the current local factory scorecard as JSON
scorecard:
    bun run scorecard

# Alias for security-audit
audit:
    just security-audit

# Run the pre-deploy gate
delivery-preflight:
    bun run delivery:preflight

# Regenerate seed/seed.jsonl from the workspace quality ledger
seed:
    bun run seed

# Initialize Omnigraph cluster and load seed data
setup:
    bun run setup

# Start the local dev worker
dev:
    bun run dev

# Deploy to production
deploy:
    bun run deploy

# ── Environment variables ────────────────────────────────────────────────────

# Set an env var in Trigger.dev: just env-set GITHUB_TOKEN ghp_xxx
env-set key value:
    @curl -sf -X PUT "{{api}}/projects/{{project}}/envvars/dev" \
        -H "Authorization: Bearer $(security find-generic-password -s TRIGGERDEV_API_KEY -w)" \
        -H "Content-Type: application/json" \
        -d "{\"name\":\"{{key}}\",\"value\":\"{{value}}\"}" | jq .
    @curl -sf -X PUT "{{api}}/projects/{{project}}/envvars/prod" \
        -H "Authorization: Bearer $(security find-generic-password -s TRIGGERDEV_API_KEY -w)" \
        -H "Content-Type: application/json" \
        -d "{\"name\":\"{{key}}\",\"value\":\"{{value}}\"}" | jq .

# List env vars
env-list:
    @curl -sf "{{api}}/projects/{{project}}/envvars/dev" \
        -H "Authorization: Bearer $(security find-generic-password -s TRIGGERDEV_API_KEY -w)" | jq '.data[].name'

# ── Task triggering ──────────────────────────────────────────────────────────

# Trigger any task with a JSON payload file: just run pr-gate payload.json
run task payload_file:
    @curl -sf -X POST "{{api}}/tasks/{{task}}/trigger" \
        -H "Authorization: Bearer $(security find-generic-password -s TRIGGERDEV_API_KEY -w)" \
        -H "Content-Type: application/json" \
        -d @{{payload_file}} | jq .

# Smoke test pr-gate against a real PR: just test-pr-gate runtime-runway 42
test-pr-gate repo pr:
    @just run pr-gate <(jq -n \
        --arg repo "{{repo}}" \
        --argjson pr {{pr}} \
        '{action:"opened",number:$pr,repository:{name:$repo,full_name:("Reflective-Lab/"+$repo),owner:{login:"Reflective-Lab"}},pull_request:{title:"just test-pr-gate",body:"triggered from Justfile",head:{sha:"HEAD"},base:{ref:"main"},additions:1,deletions:0,changed_files:1,html_url:("https://github.com/Reflective-Lab/"+$repo+"/pull/"+($pr|tostring))}}')
