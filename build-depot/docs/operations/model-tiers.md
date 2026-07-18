# Autonomous Ops — Three-Tier Model Strategy

**Decision (2026-07-18):** Autonomous work on the build machine (Sentry
triage, Linear task pickup, dep upgrades, CI/CD orchestration via Build-Depot)
routes across three tiers of LLMs, each handling a bounded class of decisions
and escalating the rest. Tier 1 is local to the build machine and runs today;
Tiers 2 and 3 are planned.

This straddles Build-Depot (workflow orchestration) and Runtime-Runway
(inference runtime). Doctrine call: it lives under Build-Depot because the
consumer is the factory workflows. The runtime bits are a supporting detail,
not the primary artifact.

Companion doc: `build-machine-stack.md` (DNS/registry/hosting on the same box).

## Tier 1 — Local (this build machine)

- **Hardware:** Intel Xeon X5690 (Westmere, no AVX), 96 GB RAM,
  AMD Radeon RX Vega 56 (8 GB HBM2, Metal 2).
- **Runtime:** `llama.cpp`, CPU-only. Two builds exist under
  `/Volumes/Lagring/tools/llama.cpp/`, but only the CPU build is used:
  - `/Volumes/Lagring/tools/llama-cli` → CPU build (production)
  - `/Volumes/Lagring/tools/llama-server` → CPU build (production)
  - `/Volumes/Lagring/tools/llama-plus-cli` / `llama-plus-server` → Metal
    build. **Do not use for inference** (see Metal finding below). Kept only
    as the reference binary should llama.cpp fix Vega/Metal ops in a future
    release.
- **Picks (Q4_K_M GGUF under `/Volumes/Lagring/tools/models/`):**
  - **`gemma-4-E2B-it-Q4_K_M.gguf`** — default triage/classification model.
    Best quality/speed balance we measured. Use for Sentry bucket-routing,
    Linear field extraction, PR review first pass, JSON schema output.
  - **`gemma-3-1b-it-Q4_K_M.gguf`** — fast-path router. Use when latency
    matters more than nuance (short prompts, small closed-set outputs,
    high-volume webhook fanout).
  - `gemma-3-270m-it-Q4_K_M.gguf` — kept as latency reference only.
- **Not on this tier:** Qwen3 8B (1.3 t/s here, and its thinking mode wastes
  the token budget without prompt gymnastics) → belongs on Tier 2. Phi-4-mini
  was slower than Gemma 4 E2B at the same size class — no reason to keep it.
- **Reliable use cases:**
  - Classify Sentry event → known bucket (regression / flake / infra / novel)
  - Extract structured fields from Linear tickets
  - Match dep-bump PR against safe-pattern catalog
  - Draft first-pass PR comments, incident summaries, triage notes
  - Route work to the correct downstream script (router, not reasoner)
- **Escalates when:** confidence < threshold, novel pattern, cross-repo scope,
  or output fails JSON-schema validation after N retries.

## Tier 2 — Networked NVIDIA (LAN)

- **Role:** Real reasoning inside the privacy envelope.
- **Model class:** Qwen3 32B / Llama 3.3 70B, run locally on the NVIDIA box.
- **Reliable use cases:**
  - Triage with codebase context loaded (retrieval + reasoning)
  - Patch generation for known-shape dep bumps and minor-version upgrades
  - First-pass PR review with rubric
  - Multi-step plans of bounded depth (≤ 3 steps, all reversible)
- **Escalates when:** the change is architectural, spans repo boundaries, or
  the model's `confidence` field is below the Tier 3 gate.

## Tier 3 — Cloud (Sonnet / Opus)

- **Role:** Last resort for novel or high-stakes reasoning. Rate-limited by
  the Tier 2 confidence gate — never called directly from a webhook.
- **Handles:** unfamiliar bugs, architectural decisions, anything where being
  wrong is expensive.
- **Guardrail:** output is a *proposed plan*, not an action. Human approves
  before Tier 1/2 executes.

## Runtime service topology

Two `llama-server` LaunchAgents on the build machine, one per Tier 1 model,
on separate ports. Both bind `127.0.0.1` only — no external exposure.

| Service label                         | Port | Model            | Threads |
| ------------------------------------- | ---- | ---------------- | ------- |
| `dev.reflective.llama.gemma4-e2b`     | 8080 | Gemma 4 E2B      | 8       |
| `dev.reflective.llama.gemma3-1b`      | 8081 | Gemma 3 1B       | 4       |

Plists: `~/Library/LaunchAgents/dev.reflective.llama.*.plist`
Logs: `/Volumes/Lagring/tools/logs/*.log`
Ops recipes: `just llm-*` at the Machinery root Justfile.

The API is HTTP OpenAI-compatible (`POST /v1/chat/completions`). Rust and
TypeScript callers both hit the same endpoint — no FFI, no Python. When Tier
2/3 come online they speak the same shape, differing only in base URL.

**Server default: `--reasoning off`.** Gemma 4 (and increasingly other small
models) ship with chain-of-thought "thinking" mode enabled by default. On
Tier 1, thinking is counterproductive — the whole budget is a monologue that
lands in `message.reasoning_content` while `message.content` stays empty.
Tier 1's use cases (classification, JSON extraction, routing) want direct
structured output, not deliberation. Callers that specifically want thinking
can opt in per request with
`chat_template_kwargs: {"enable_thinking": true}`; that traffic should almost
always be routed to Tier 2 instead.

### Client-side defaults for Tier 1 callers

The server exposes generic OpenAI-compatible endpoints; taste is set by the
caller. For Tier 1 workloads, two defaults are worth hard-coding into the
client wrappers.

**1. `max_tokens` by use-case class** — Gemma 4 E2B is verbose by nature
(headers, multiple approaches, docstrings). Under-set `max_tokens` and it
truncates mid-sentence; over-set and it wastes wall-clock time producing
material the caller will throw away.

| Use case                                     | Recommended `max_tokens` |
| -------------------------------------------- | ------------------------ |
| Structured JSON classification (Tier 1 core) | 128–256                  |
| Short natural-language reply / triage note   | 256                      |
| Single-function code answer                  | 300–500                  |
| Anything longer                              | escalate to Tier 2       |

Gemma 3 1B needs roughly half of these ceilings for the same task; it is
terser out of the box.

**2. Terse system prompt for code/answer tasks.** Without a system prompt,
Gemma 4 E2B on a "write me X" query emits section headers, multiple
implementations, and a tutorial. Adding a one-line system prompt like:

> Provide exactly one solution. No alternatives, no section headers, no
> preamble. Return only the code inside a single fenced block, plus at most
> one sentence of explanation after it. Stop.

collapses the same fibonacci-script request from 500 tokens / 88 s (still
truncated) to **139 tokens / 22.6 s with `finish_reason: stop`** — a clean,
self-terminating, single-function answer. ~4× wall-clock improvement per
useful response, and the output is easier to consume downstream.

Both defaults belong in a thin Tier 1 client wrapper so individual callers
inherit them rather than repeatedly rediscovering the tuning.

## Escalation Contract

Every tier speaks the same JSON schema in and out. Swapping which tier owns
which class is a config change, not a rewrite.

**Input envelope:**
```json
{
  "task_id": "string",
  "task_type": "sentry_triage | linear_pickup | dep_upgrade | ci_decision | ...",
  "payload": { ... },
  "context_refs": ["url", "file:line", ...]
}
```

**Output envelope:**
```json
{
  "task_id": "string",
  "decision": "run_script | open_ticket | escalate | reject",
  "action": { ... },
  "confidence": 0.0,
  "escalate_reason": "string | null",
  "notes": "string"
}
```

Rules:
- `confidence < 0.7` → auto-escalate to next tier, no matter the decision.
- `decision == "escalate"` → next tier receives the same envelope plus the
  previous tier's `notes` as extra context.
- Output that fails schema validation is treated as `decision: reject` with
  `confidence: 0`.

## Decision Flow

```
webhook / cron
      │
      ▼
  Tier 1 (local)
      │
      ├─ confidence ≥ 0.7 + known pattern ──▶ execute script
      ├─ confidence ≥ 0.7 + human-only tag ──▶ open Linear ticket
      └─ else ──▶ Tier 2 (LAN NVIDIA)
                     │
                     ├─ confidence ≥ 0.8 ──▶ execute or file PR draft
                     └─ else ──▶ Tier 3 (Cloud)
                                    │
                                    └─ human approves plan ──▶ Tier 1/2 executes
```

## Operating Notes

- Structured output is non-negotiable — free-form "just do the right thing"
  is not a tier the build machine supports.
- Tier thresholds (`0.7`, `0.8`) are seed values. Log every decision +
  outcome so they can be re-tuned against ground truth.
- Every escalation carries the full prior-tier trace. No hidden context.
- Tier 3 never executes; it proposes. This is the invariant that keeps
  cloud-model failures bounded.

## Metal-on-Vega Finding (dead end)

The Metal-enabled build (`llama-plus-cli`, from `build-metal/`) links against
the AMD Radeon RX Vega 56 and does allocate all layers to `MTL0` at load
time. It nonetheless produced worse results on every model we tested versus
the pure-CPU build on this Xeon:

| Model            | CPU prompt / gen (t/s) | Metal prompt / gen (t/s) | Metal output |
| ---------------- | ---------------------- | ------------------------ | ------------ |
| Gemma 3 270m     | 118.7 / 52.8           | 23.7 / 5.0               | garbage      |
| Phi-4-mini 3.8B  | 6.2 / 2.9              | 4.5 / 1.6                | ok, slower   |
| Gemma 4 E2B      | 10.0 / 6.5             | 6.4 / (broken)           | zero tokens  |

Two root causes visible in the verbose Metal init log:

1. `resolve_fused_ops: layer 0 is assigned to device MTL0 but Flash Attention
   is assigned to device CPU (usually due to missing support)` — the Vega's
   Metal implementation is missing kernels llama.cpp needs, so it silently
   splits the graph across GPU and CPU on every forward pass.
2. GPU↔CPU marshalling for the fallback ops costs more than the pure-CPU
   forward pass on this old Westmere chip. And for architectures using
   interleaved SWA (Gemma 3/4), the Metal path produces numerically wrong
   output on top of being slow.

**Do not use `llama-plus-cli` for inference.** Re-test only after a llama.cpp
release that specifically notes AMD Metal op coverage.

## Benchmark Reference (CPU-only, `-t 8`, Q4_K_M)

Baselines used to set Tier 1 picks. Prompt: `"In one sentence, what is a
compiler?"`, `-n 40`–`60`, `--temp 0`.

| Model            | Size    | Prompt t/s | Gen t/s | Notes                       |
| ---------------- | ------- | ---------- | ------- | --------------------------- |
| Gemma 3 270m     | 253 MB  | 118.7      | 52.8    | Toy, latency baseline       |
| Gemma 3 1B       | 806 MB  | 18.7       | 12.2    | Fast-path router pick       |
| **Gemma 4 E2B**  | 3.1 GB  | **10.0**   | **6.5** | **Tier 1 default pick**     |
| Gemma 3 4B       | 2.5 GB  | 5.8        | 4.1     | Superseded by Gemma 4 E2B   |
| Phi-4-mini 3.8B  | 2.5 GB  | 6.2        | 2.9     | Slower than E2B, dropped    |
| Qwen3 8B         | 5.0 GB  | 2.9        | 1.3     | Reasoning mode; Tier 2 only |
