---
name: release-acapulco
description: Definition of release Acapulco — what it contains, what "in Acapulco" means, and its ship criteria. First named release of the alphabetical city sequence.
metadata:
  type: project
  date: 2026-07-02
  status: CONFIRMED 2026-07-02 — scope frozen; changes are recorded de-scope/add decisions
---

# Release Acapulco

**First named release. First proof that the thesis works.**

## Remember Why We Do This

Meetings are where organizations think. Most meetings waste time.

- 15 minutes: process, repetition, hierarchy
- 20 minutes: the loudest person talking
- 5 minutes: the person who actually knows something
- 20 minutes: the same ground covered twice

**What if every minute counted?**

What if the person who knew something was *heard*. What if disagreement surfaced as data, not conflict. What if the group's reasoning was *auditable* — you could trace why you decided what you decided, see where you still disagreed, understand what you learned together.

**That is Acapulco.**

One real meeting. Six to eight people. A real decision to make. Sixty minutes where every participant is their best self, amplified by AI, reasoning is made visible, and you walk out with a process receipt.

Not a demo. Not a mockup. A real meeting, with real people, real stakes, real artifacts.

When that works, everything changes.

---

## What Acapulco Ships

First named release of the Reflective release train ([[release-naming]]).
**A named release is an outward promise**: specific content and functionality
committed to the outside world, shipped as one coordinated train movement
(`KB/08-roadmap/release-train.yaml` defines membership and publish order).
It is a wrapper around work drawn from several epics — a capability slice
across the train, not a bundle of whole epics.

**Acapulco proves one thing: AI-orchestrated meetings work.**

Three artifacts ship:
1. **Quorum-Sense** — web app (Svelte/Tauri) + mobile (iOS/Android FFI) where participants join a real meeting
2. **Helms coordination** — the backbone that keeps everyone's thinking in sync (sessions, presence, decisions, SSE stream)
3. **One proof run** — a real inquiry with real people, recorded and analyzed, showing what this actually looks like

## The Outside Story — What to Say to Circle A

**To Axfood, Epidemic Sound, Monterro:**

"We just built something different: a meeting where people think better together.

One hour. A real decision. Mix of expertise in the room. AI that surfaces what each person actually knows (and didn't say), removes the politics from disagreement, and keeps the group on hard questions.

The output isn't a recommendation from a black box. It's a trace of *why* you decided what you decided — who thought what, where you disagreed, what you learned.

We ran one proof. Here's what happened. [Process receipt, participant feedback, reasoning trace]

Want to run the next one with your team in September? Real decision, real stakes, real learning."

**Three words to remember and repeat:**
1. **Audible** — Everyone's thinking is visible
2. **Alive** — The reasoning adapts as you go (Organism learns from disagreement)
3. **Authentic** — Expertise surfaces; politics disappears

---

## How Acapulco is Referenced

| Context | Meaning |
|---|---|
| **Linear** | The `release:acapulco` label. An issue is "in Acapulco" iff it carries the label. Linear is the live status; this doc is the promise. |
| **This doc** | The promise: the five promises below, frozen once confirmed. Scope changes after confirmation are explicit de-scope/add decisions recorded here, not silent label edits. |
| **Git** | Machine form `acapulco` ([[release-naming]] conventions). At ship time each train repo gets an annotated tag `acapulco` on the released commit. Branches stay per-issue (`e{N}/lin-XX-slug`); there is no long-lived acapulco branch. |
| **Outside world** | The announcement (converge.zone, LinkedIn, customer conversations) names Acapulco and lists the shipped capabilities — never internal epic numbers. |
| **Claude** | This doc lives in `KB/08-roadmap/` and is linked from Linear issue descriptions. When Claude works on Acapulco tasks, it reads this doc first to align on thesis and timeline. |

"Shipped" means: every `release:acapulco` issue is Done, the train publishes
in `release-train.yaml` order, the tags exist, and the Status row in
[[release-naming]] flips to `Shipped (YYYY-MM-DD)`.

A release name is **not** an epic, a milestone, a branch, or a version
number. Crate versions stay semver per-crate; Acapulco is the name of the
coordinated shipment that carries them.

## Confirmed scope (2026-07-02)

Theme: **Quorum-Sense runs a real session end-to-end on the full train.**
One market application, live, on proven platform components.

| # | Promise (outward wording) | Issue | Epic |
|---|---|---|---|
| 1 | Quorum-Sense product proof: a real multi-participant session runs live, with a run surface you can watch | RFL-62 (quorum-sense M4 — Product Proof + Live Run Surface) | E4 |
| 2 | Helm session backbone: liveness and realtime upstream events (SSE) powering that session | RFL-25 (helms scale unlock: `SessionOwnershipLayer`, durable registries + persistent `DecisionLedger`, mobile SSE resilience, Cloud Run deploy) | E5 |
| 3 | Coordination model proven headlessly — many humans + agents converging in one scripted session, staged as an **atelier-showcase** scenario | RFL-17 (bedrock-platform helm-coordination M0); scenario lands in atelier-showcase | E11 |
| 4 | Platform floor published: the train ships with Converge, Organism, Axiom, Helms versions aligned and consumable from crates.io | RFL-125 (train publish, converge-first order) | E1–E3 |
| 5 | Quorum-Sense on iOS, on a real device: the capture slice (speech/text → draft → consent) runs on a physical iPhone | RFL-124 (on-device signing, install, hardware verification; M3 slice done 2026-06-30) | Mobile |

Deliberately **out** of Acapulco:

- Mobile beyond the on-device capture slice: compute placement (M6),
  realtime collaboration UX (M7), portfolio pattern (M8), and App Store
  release/privacy ops (M9) stay Backlog. Promise 5 is "runs on my iPhone",
  not "in the App Store".
- Axioms-dependent promises (E7) — unproven; promising it outward is premature.
- Commercial spikes (E10, incl. the Atlas cross-app spike) — exploration is
  never an outward promise. If a spike lands something shippable, it enters
  Bruges by decision, not by drift.
- Wolfgang Stage 1 — real goal, wrong train: it has its own storefront/billing
  arc. Candidate headline for **Bruges**.

## Mechanics

1. ~~Create the `release:acapulco` label and apply it~~ Done 2026-07-02:
   label created (`#10b981`), applied to RFL-62, RFL-26, RFL-17, RFL-125,
   RFL-124.
2. Scope discipline: adding the label to a new issue after confirmation =
   scope change; record it under Decisions below.
3. At ship: tag train repos `acapulco`, flip [[release-naming]], write the
   announcement from the Promise column — it is worded to be quotable as-is.

## The Journey — A Predictable Path with Uncertainties

**56 days. Five known blockers. Drive through.**

Ship date: **2026-08-28** (internal; confirm outward-safe in RFL-127).

### The Five Blockers (Knowable, Fixable)

1. **Quorum server build is broken** — `with_ambient_handler` method missing (runway-ambient integration)
2. **Organism live reasoning not wired** — M3 done; M4 needs real Organism adapting during rounds
3. **Mobile ⇄ Web coordination untested** — FFI bridge exists; SSE coordination exists; haven't run together
4. **M4 Review Charter acceptance test partially done** — 6 sections; some checks not wired yet
5. **Proof run logistics undefined** — where, when, who, what domain?

All five are fixable. None are architectural surprises.

### Milestones to 2026-08-28

| # | Week | Target | What | Why | Blocker(s) |
|---|---|---|---|---|---|
| **1** | Jul 3–9 | Unblock builds | Fix `with_ambient_handler`, wire Organism reasoning, get `just check` green | Can't move forward if builds fail | 1, 2 |
| **2** | Jul 10–17 | Product surface complete | Facilitator + participant console map every control to API route; sealed rounds don't leak; M4 Review Charter checks wired | The product proof is the whole point | 4 |
| **3** | Jul 18–31 | Coordination proven | Web + mobile talk to each other through coordination routes; SSE stream works end-to-end; atelier scenario runs headlessly | Both apps ship; if they can't coordinate, the whole thesis fails | 3 |
| **4** | Aug 1–8 | Deploy to production | Quorum on Cloud Run via Runtime Runway auth; mobile app signed and testable on real device; one internal dry run | "Live" means deployed, not localhost | (milestone #3 done) |
| **5** | Aug 9–21 | Proof run | Real participants (Axfood/Epidemic/Monterro), real decision, 60min recorded meeting, process receipt generated and analyzed | The proof is everything | 5 + logistics |
| **6** | Aug 22–28 | Ship train | Crates published, tags created, announcement live | Promise kept | (all prior done) |

### Week-by-Week: What "Drive Through" Means

**Week 1 (Jul 3–9): Unblock**
- [Commit: Fix runway-ambient wiring] + PR → merge to next
- [Commit: Wire Organism live reasoning into round planning] + PR → merge
- [Commit: Mobile FFI sync if stale] + PR → merge
- Success: `cargo build -p quorum-server` exits 0; `cargo build -p quorum-mobile-ffi` exits 0
- If any fails: debug + retry same week. No punt.

**Week 2 (Jul 10–17): Charter**
- [M4 Review Charter](../../../marquee-apps/quorum-sense/kb/Architecture/M4%20Review%20Charter.md) section 1 (facilitator surface): every mutating control maps to API route with audit trail. Test it.
- Section 2 (participant surface): consent, presence, signal submission, optimistic receipts on both web + mobile. Test it.
- Section 3–6: sealed rounds, SenseMap/predictions, process receipt, scoring semantics. Check them.
- Success: internal checklist pass (Karl + 2 engineers run a 30-min test inquiry; all section checks pass)
- If any section fails: fix the code, not the test.

**Week 3 (Jul 18–31): Coordination**
- Wire web app SSE coordinator to mobile FFI; test signal submission from mobile → appears in web in real-time
- Wire presence claims from iOS → web sees them; web gates → iOS sees them
- Run atelier headless scenario (many agents + one human, converge on decision through coordination routes)
- Success: atelier scenario passes (RFL-126 definition of done)
- If SSE drops: improve mobile resilience. If presence claim fails: debug Helms coordination. Fix, iterate, move on.

**Week 4 (Aug 1–8): Production**
- Deploy Quorum server to Cloud Run with Runtime Runway auth
- Sign mobile app for iOS; install on real device; test the hot path (open inquiry → join presence → submit signal → see it live in web)
- One internal dry run: Karl, 2 engineers, 1 Organism agent, 30min inquiry (supply decision). Record it. Analyze: Did Karl feel heard? Did Organism surface anything new? Was the reasoning auditable?
- Success: dry run receipt generated; M4 Review Charter all 6 sections pass; no critical production bugs
- If production deploy fails: rollback, fix, redeploy same week.

**Week 5 (Aug 9–21): The Proof**
- Pick 6–8 real participants (e.g., 2 from Axfood, 2 from Epoch Sound, 1 PM, 1 facilitator = real decision)
- Pick a real domain (supply chain move, licensing strategy, M&A integration)
- Run the 60-minute meeting: opening, rounds, signals, dissent, decision, process receipt
- Analyze: What changed in their thinking? Did the process feel auditable? Would they run another?
- Success: receipt generated; participants signed off; the proof is real
- If proof run has critical bugs: roll back to week 4 state, fix, rerun before Aug 21.

**Week 6 (Aug 22–28): Ship**
- Tag all train repos `acapulco`; publish crates.io (converge, organism, axiom, ferrox, helms, runtime, etc.)
- Write announcement (converge.zone, LinkedIn): "We proved AI-orchestrated meetings work. Here's what happened."
- Update [[release-naming]]: Acapulco → Shipped
- Close all `release:acapulco` issues in Linear
- Success: tags exist; crates.io shows Acapulco versions; announcement live
- If crate upload fails: retry, no panic (crates.io has a rate limit; it's expected).

### Known Uncertainties (Not Blockers; You'll Learn as You Go)

- **Apple Developer Program.** You need to sign the iOS app; requires Apple membership + device provisioning. Start this in week 1 (CP0 decision per RFL-127). Expected friction: none if Karl is already a member; small if not.
- **Proof-run participant availability.** Axfood/Epidemic/Monterro — are they available mid-August? Lock this down in week 2 (ask for a calendar hold; confirm topic).
- **Organism live reasoning quality.** Will Organism's real-time suggestions surface useful new ideas in the meeting? Or will they be noise? You'll find out in week 4's dry run. If noise: tune the prompts. If useful: great; if not yet useful: that's data for Bruges.
- **Mobile ⇄ Web coordination under real load.** SSE can be flaky; presence claims can race. Will you hit edge cases under real-meeting load? Week 3 dry run will surface them. Week 4 production deploy will stress-test. If it breaks: fix it before the proof run.

**None of these kill the release. They inform it.**

## Dependencies (internal enablers — not outward promises)

- **RFL-88** (runtime-runway M2, GCP production readiness) gates the
  deployed demo. "Live" is defined as: deployed on Cloud Run via Runtime
  Runway-issued auth — never the local dev stack.
- **Acceptance bar**: `marquee-apps/quorum-sense/kb/Architecture/M4 Review
  Charter.md` (+ its Acapulco Additions section). RFL-62 is done when one
  run passes every check.
- **Train reality** (corrected by CP0 dry-runs): the train IS published —
  converge 3.9.2, organism 1.9.3, axiom-truth 0.15.2, ferrox-solver 0.7.2
  on crates.io, and every quorum-sense floor version is already satisfiable
  from the registry. The `[patch.crates-io]` redirects are dev-loop
  convenience, not evidence of absence. The real issue is
  **drift-without-bump**: every repo head carries unpublished code past its
  tag while the manifest still holds the published version, and
  organism/ferrox heads require unpublished converge APIs. Promise 4 is a
  routine bump-and-publish in dependency order, not a first-ever publish.
  Per-member dry-run results live in `KB/08-roadmap/release-train.yaml`.
- **Helms publishes** (decided 2026-07-02): helms ships as crates like the
  rest of bedrock-platform and mosaic-extensions. Publish-readiness fix on
  `e5/lin-125-helms-publishable` in the helms repo (publish flag, metadata,
  dep versions, `notes` → `helm-notes` rename — the bare name is squatted).
  First-ever publish at 0.2.1 — all 40 crates queued 2026-07-02; the
  runway layering fork that briefly held the five-crate helm cone is
  resolved (see Decisions), and an unattended two-phase loop is draining
  the uploads through the crates.io new-crate rate limit. The atelier
  follow-up is done: `organism-domain` 1.5.0 republished past the stale
  1.4.0.

## Decisions

- 2026-07-02 — doc created; scope proposed, not yet confirmed by Karl.
- 2026-07-02 — Karl: the blanket "mobile stays Backlog" exclusion was stale —
  the M1–M5 foundation (incl. the full iOS capture slice) is already done.
  Added promise 5: iOS app on a physical device (RFL-124, Todo in the Mobile
  project). M6–M9 remain Backlog.
- 2026-07-02 — Karl: promise 3 names **atelier-showcase** as the vehicle —
  the E11 headless coordination proof lands as an atelier-showcase scenario.
  (The existing atelier issues RFL-1/2/3 are E7 Mosaic scenarios and stay in
  E7; the coordination scenario is E11 work under helm-coordination M0.)
- 2026-07-02 — **Karl confirmed the five-promise scope.** `release:acapulco`
  created and applied. RFL-125 created as the tracking home for promise 4
  (train publish — cross-repo, no single MILESTONES.md owned it). Scope is
  now frozen; changes from here are recorded de-scope/add decisions.
- 2026-07-02 — UX goal adopted: [[acapulco-ux-goal]] ("walk in with a
  question, walk out with a receipt — and the next meeting starts smarter").
  It interprets the promises, does not extend them; the M4 product-proof run
  (RFL-62) is its acceptance test. Noted there: durable coordination
  registries (helms RFL-25) stay out of scope as an outward claim.
- 2026-07-02 — **Promise 2 re-anchored RFL-26 → RFL-25.** RFL-26 was imported
  from the stale `kb/Planning/MILESTONES.md`; canonical helms MILESTONES
  shows Stage 1 shipped as v0.1.1 on 2026-04-25. RFL-26 closed as Done, label
  moved to RFL-25 (the open backbone work). The outward wording of promise 2
  is unchanged — durability remains an internal bar, not an announced claim.
- 2026-07-02 — CP0 executed: "live" defined (Cloud Run via Runtime Runway),
  checkpoint plan + due dates set (target ship 2026-08-28), RFL-126 created
  (atelier helm-coordination-convergence scenario, labeled — it *is* the
  staging of promise 3, not new scope), RFL-127 created (CP0 spec-hardening
  tracker), M4 Review Charter extended with the five UX-goal checks,
  `release-train.yaml` created. Discovery: no train crate is published yet
  (`[patch.crates-io]` redirects everywhere) and `ferrox-solver` is a
  candidate sixth train member — decide before RFL-125 starts.
- 2026-07-02 — **CP0 dry-runs run; "nothing is published" was wrong.** The
  sparse index shows the whole train on crates.io at exactly the local
  version numbers (converge 3.9.2, organism 1.9.3, axiom-truth 0.15.2,
  ferrox 0.7.2). Dry-run verdicts: converge and axiom package+verify clean
  but need version bumps (their versions are already uploaded); organism and
  ferrox FAIL verify because their heads use unpublished converge APIs —
  converge must bump+publish first; helms is publish-disabled workspace-wide
  with missing descriptions and versionless path deps, and may not need to
  publish at all (new open question above). RFL-125 reframed from
  "first-ever publish" to "bump-and-publish drifted heads in dependency
  order". Dependencies section and release-train.yaml corrected.
- 2026-07-02 — **Karl decided: helms publishes** — "fix and publish helms as
  all bedrock-platform and mosaic-extensions." Publish-readiness fix landed
  on `e5/lin-125-helms-publishable`: `publish = true`, repository metadata
  inherited workspace-wide, per-crate descriptions for all 40 publishable
  crates, versions added to every path dependency, and `notes` renamed
  `helm-notes` (the bare name is taken on crates.io by an unrelated crate).
  All 40 package cleanly. Two new train facts recorded in
  release-train.yaml: crates.io carries a stale `organism-domain` 1.4.0
  ahead of the atelier workspace's 1.0.2 (republish-past-it required), and
  the four mosaic-extensions repos show the same drift-without-bump pattern
  (+8…+13 commits past their tags).
- 2026-07-02 — **Promise 4 substantially executed: the train is bumped and
  published on crates.io** in dependency order, from each repo's `next`
  head (release commits + v-tags local; nothing pushed to main). Live:
  converge 3.9.3, organism 1.9.4, converge-ferrox-solver 0.7.3 (+2 sys),
  axiom-truth 0.15.3, converge-prism-analytics 2.0.3,
  converge-arbiter-policy 2.0.3, converge-mnemos-knowledge 1.2.4,
  converge-manifold-adapters 1.1.3, converge-atelier-domain 1.5.0,
  organism-domain 1.5.0 (skew resolved — republished past the stale 1.4.0).
  helms 0.2.1 first publish in progress: all 40 crates verified against the
  registry; crates.io's new-crate rate limit throttled the upload burst and
  an unattended retry loop is draining the remaining ~30 uploads.
- 2026-07-02 — **OPEN DECISION (Karl): the runway layering fork** *(resolved
  — see next entry)*. Five
  helms crates (helm-truth-execution, helm-operator-control,
  helm-governed-jobs, helm-coordination, helm-session-host) need
  runway-app-host HEAD APIs (`ModuleState`, `HelmModule::module_state`,
  `sse::event_stream`, awaitable `EventSubscription`) — but runway-app-host
  head is `publish = false` by documented RP-LAYERING design (it
  normal-depends on the internal `runway-ambient` substrate, which "can
  never ship to crates.io"). The five are held with `publish = false` +
  comment rather than overriding that design. Options: make the substrate
  publishable, refactor app-host so its public surface publishes without
  the substrate, or keep the cone repo-only. Until resolved, promise 4's
  "Helms" floor on crates.io is the application/capability layer (35
  crates), not the helm session/coordination cone.
- 2026-07-02 — **Runway fork RESOLVED: dependency-inversion seam.** Karl's
  direction: interfaces flow upstream, concrete implementations are created
  and injected downstream — and any publish-boundary fork must be explicit
  and documented. Executed as: `runway-app-host` now declares a minimal
  `JobsRuntime` trait and carries zero `runway-ambient` references;
  `runway-ambient` implements it (`AmbientJobs`) and hosts inject it at
  build time. runtime-runway **v3.5.0** published (all 10 crates), after
  commerce-rails **0.2.2** unblocked `runway-accounts` (its `runway-storage`
  path dep lacked a version). The seam pattern is codified in
  `KB/05-engineering/standards/repo-layering.md` ("Publish-boundary seams" —
  every fork documented at both ends + registered in the seam table;
  `JobsRuntime` is entry #1). The deeper debt — the substrate defines the
  app-branded `HelmModule`/`ModuleState` contract, and Foundation consumes
  substrate — is recorded there and tracked as **RFL-128** (extract the
  contract into a neutral crate). The five helm-* cone crates are unheld
  (helms `ca70b0d`: `publish.workspace = true` restored, app-host floor
  3.5.0) and queued behind the drain with full verification.
- 2026-07-03 — **Promise 4 COMPLETE: all 40 helms crates live on crates.io**
  (index-verified). The helm-* cone passed full verification against
  registry-only deps — the JobsRuntime seam composes from crates.io alone.
  All train tags pushed to origin (11 repos) plus each repo's `next`;
  helms `next` and `main` pushed (merge `6f12549`). Outstanding: a
  pre-existing `v0.2.1` tag (local + origin) points at `bd0a5c3`, 53
  commits behind the release head — force-moving it needs Karl's explicit
  call; and the clean-room `cargo add` gate result.
- 2026-07-03 — **Planning review; gaps actioned:**
  1. **RFL-131 created** (Acapulco proof run — participants, date, domain,
     recording, success criteria; Urgent, due Aug 21). The proof run (the
     whole point of the release) had no owned Linear issue. RFL-131 is the
     home; its first action (send calendar invites to Axfood/Epidemic/Monterro)
     must happen this week (Jul 3–9) — longest external lead time.
  2. **RFL-88 labeled `release:acapulco`** — the GCP production readiness
     issue gates promise 2 (Helms on Cloud Run) and was an unlabeled hidden
     dependency. Now visible in the release board.
  3. **RFL-125 → In Progress** — platform floor is substantially executed;
     one outstanding item requires Karl's call (helms v0.2.1 tag conflict,
     see last Decisions entry). Moving to In Progress reflects reality.
  4. **RFL-127 → In Progress** — due Jul 10; two CP0 items with external
     lead time still open: Apple Developer Program verification (unblocks
     RFL-124) and proof-run participant invites (now owned by RFL-131).
     Staying Todo would mask the urgency.
  5. **Scope note (RFL-62):** the issue still lists Atlas ⇄ Quorum deployed
     demo and `quorum://` citation resolver. Neither appears in the five
     promises. Recommend: explicitly de-scope or add as a named decision
     before Week 2 build pressure starts.
- 2026-07-03 — **Promise 4 CLOSED (RFL-125 → Done).** Karl force-moved
  `v0.2.1` to the release head `ca70b0d` (local + origin verified
  identical). Clean-room `cargo add` gate had already PASSED. Helms repo
  end-state verified: main/next synced with origin, CI on main, merged
  issue branch `e5/lin-125-helms-publishable` deleted, no stashes, clean
  tree. Nothing outstanding on the platform floor.
