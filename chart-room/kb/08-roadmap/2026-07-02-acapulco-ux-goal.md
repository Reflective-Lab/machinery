---
name: acapulco-ux-goal
description: The UX goal for release Acapulco — the meeting arc (prep, context, live state, ledger, learning) that the five promises must add up to.
metadata:
  type: project
  date: 2026-07-02
  status: adopted 2026-07-02 — interprets the confirmed Acapulco scope, does not change it
---

# Acapulco UX Goal

Companion to [[release-acapulco]]. The five promises say what ships; this
says what it must **feel like** in one meeting. The vocabulary below is not
aspirational — every term names something already in the quorum-sense /
helms / helm-coordination codebase.

## The goal, in one sentence

> **Walk in with a question, walk out with a receipt — and the next meeting
> starts smarter.**

A meeting run on the Acapulco train: the organizer states an intent, not an
agenda; participants have already spoken, on their own devices, under their
own consent; the session opens with shared context instead of a recap; the
group watches itself converge while it happens; every decision leaves a
verifiable trace; and the next session starts from that trace instead of
from zero.

## The meeting arc — seven questions, seven answers

### 1. How does the organizer prep?

By stating the **core question**, not building an agenda. The Session Leader
compile UX (`#/leader/start`, Organism path — an open M4 deliverable inside
RFL-62) compiles the leader's intent into an `InquiryContract`: decision
rules, anonymity policy, scoring policy, research policy — with a compile
receipt. Prep is reviewing and adjusting that compiled contract and inviting
people. Minutes, not an afternoon of slides. The contract is the agenda.

### 2. How do participants prepare?

By **capturing signals in advance** — speech or text on their own phone:
draft → review → explicit consent → durable queue → sync (the M3 capture
slice; RFL-124 puts it on a physical iPhone). Participant prep is "say what
you actually think, ahead of time, under your own consent" — not "read the
pre-read." Signals queue offline and are admitted with receipts; the
contract's anonymity/sealed-round policy decides what identity travels
with them.

### 3. Is there a "Context" when a meeting starts?

Yes, and everyone sees the same one. Opening context =

- the compiled **contract** (why we're here, what rules govern decisions),
- the **SenseMap** state (what the org already believes: ranked insights,
  open hypotheses, anticipatory predictions),
- the **pre-seeded consented signals** from step 2,
- **recall of prior related inquiries** (`QuorumAmbientHandler`
  mnemos-recall).

The context is queryable state, not a deck someone made the night before.

### 4. Is there a long-term ledger with decisions and traces?

Per inquiry, yes — and it is the strongest part of the system: every event
is receipt-bearing on a Lamport + Merkle integrity chain; decisions and
amendments are explicit chain-recorded acts; the whole run exports as a
process receipt that is **externally verifiable** (`/inquiry/{id}/events`,
`/integrity`, `/process-receipt`). That is the Acapulco ledger bar.

Honest gap: the **durable cross-restart registries** (`SessionRegistry`,
`PresenceRegistry`, persistent `DecisionLedger`) are open helms work in
RFL-25 — since 2026-07-02 the anchor issue for promise 2, so they are
tracked inside Acapulco. The outward claim is still only auditable
artifacts, which the chain + receipts already deliver; durability is the
internal bar, not the announcement.

### 5. Does the group session learn and get better each time?

In Acapulco: **the memory learns, not yet the facilitator.** SenseMap
accrues insights and falsifiable predictions across sessions; mnemos-recall
feeds session N's traces into session N+1's opening context; drift-scan
notices when the org's signals move against prior sessions. So the second
meeting on a topic demonstrably starts from what the first one established
— that is the Acapulco bar. Self-tuning facilitation (probe strategies that
improve from outcomes; the Prism/Ferrox swap triggers documented in the
Facilitation Heuristics Floor) is post-Acapulco — a Bruges candidate, not a
silent promise.

### 6. How is group state reflected back to the members?

Live, on the run surface (promise 1), and honestly:

- presence and admitted signals as they land,
- hypothesis confidence as **fuzzy confidence**, not fake precision,
- quorum as **evidence topology** — diversity, independence, dissent,
  recency, role coverage — never a single percent-agreement bar,
- dissent preserved as first-class state, visible in the topology.

Participants see the group converging without seeing sealed identities.
The facilitator sees more (reachability, evidence gaps) but every mutating
control maps to a receipt-bearing API — there is no hidden facilitation
state that the ledger doesn't show.

### 7. How does that affect the individual flow?

Three ways:

- **The group state steers what you're asked next.** Adaptive probes are
  routed toward evidence gaps (`CoordinatorSuggestor`), so an individual's
  next question depends on what the group hasn't covered — not a fixed
  script.
- **You can trace your own signal.** Admission receipt → hypothesis
  contribution → decision trace. Being in the minority is recorded as
  dissent in the topology, not erased by the majority — you can lose the
  decision and still see that you were heard.
- **Contribution detaches from attendance.** Because sensing happens before
  and around the meeting (on the phone, consent-gated, offline-tolerant),
  the synchronous session spends its time converging, not collecting.
  Meetings get shorter because they stop being the data-capture step.

## The bar Acapulco must clear

The M4 product-proof run (RFL-62) is the test of this whole page: **one real
guided inquiry where all seven answers above are demonstrably true** — with
at least one dissent path, one adaptive probe, one explicit decision or
deferral, a final process receipt, and at least one participant whose
signals came in from a physical iPhone (RFL-124). If a demo can't show an
answer, the answer above is marked wrong or the gap is recorded in
[[release-acapulco]] Decisions.

## What this deliberately does not promise

- Self-improving facilitation (see §5) — Bruges candidate.
- Durable coordination registries as an outward claim (see §4) — helms
  RFL-25, internal hardening.
- Any mobile surface beyond capture-and-consent on device — M6–M9 stay
  Backlog per [[release-acapulco]].
