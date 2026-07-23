# Cross-Repository Reconciliation - 2026-05-31

This pass compared the top-level platform docs with the current code,
manifests, local READMEs, and KB notes in the active stack repositories. Code,
tests, manifests, generated catalogs, and runtime configuration were treated as
stronger evidence than prose.

## Reconciliation: bedrock-platform

### Role in Platform

`bedrock-platform` is the foundational implementation layer for Axiom,
Organism, Converge, and Helm / Outcome Workbench. It supports architecture,
product, engineering, and operations domains. Developers and downstream app
builders are the primary users.

### Relevant Top-Level Domains

Platform Vision, Product & UX, System Architecture, Engineering, Operations,
Knowledge, Roadmap.

### What Local Code Confirms

- The implemented bedrock order is Axiom -> Organism -> Converge -> Helm /
  Outcome Workbench.
- `bedrock-platform/helms` is the current Helm / Outcome Workbench code
  home, not `helms/`.
- Commerce Rails is a root-level peer at `commerce-rails/`, not under
  `movement/`.

### What Local Code Contradicts

- Older docs still treated `converge-provider-api` as the public provider
  crate; the current Converge crate is `converge-provider`.
- Older docs treated Helm naming and `helms/` paths as current structure.

### What Local Docs Needed Updating

Updated bedrock README, KB index, ecosystem overview, milestones, and local
Outcome Workbench notes for current paths, crate names, and Helm state.

### What Should Be Promoted to Top-Level Docs

The top-level architecture should keep presenting bedrock as a stack of
separate foundations rather than a monolith: Axiom translates intent, Organism
forms plans, Converge admits and promotes facts, Helm proves application-level
truths.

### Open Questions

- ~~Whether the `2/` directory name should remain the long-term source path~~
  **Resolved (2026-06):** `helms/` is the canonical path; `2/` was a stale
  workspace alias and is retired from agent entrypoints.

### Suggested Top-Level Changes

- Keep the root KB decision that Helm / Outcome Workbench currently lives at
  `bedrock-platform/helms`.
- Mark old `helms/` and `converge-provider-api` references as historical when
  they appear in chronology or release notes.

## Reconciliation: converge

### Role in Platform

`bedrock-platform/converge` is the semantic governance and promotion
kernel. It owns fact admission, promoted claims, packs, provider contracts,
models, protocols, client APIs, storage/runtime internals, and validation.

### Relevant Top-Level Domains

Platform Vision, System Architecture, Engineering, Operations, Knowledge,
Roadmap.

### What Local Code Confirms

- Current workspace version is `3.9.1`.
- The workspace has 11 crates.
- The canonical external/public crate surface is `converge-pack`,
  `converge-provider`, `converge-model`, `converge-kernel`,
  `converge-protocol`, and `converge-client`.

### What Local Code Contradicts

- Docs that describe `converge-provider-api` as current are stale.
- Docs that still use Converge `3.8.x` as the active Mosaic baseline are stale.

### What Local Docs Needed Updating

Updated bedrock and Mosaic docs to use `converge-provider` and Converge
`3.9.1` as the current integration baseline.

### What Should Be Promoted to Top-Level Docs

Use the six-crate external surface as the platform-wide Converge integration
model. Treat other Converge crates as internal or workspace-local unless a
manifest and README say otherwise.

### Open Questions

- Which Converge internals should remain publishable crates versus private
  implementation details.

### Suggested Top-Level Changes

- Keep current-system-map references to Converge aligned with the six public
  crates and `3.9.1` baseline.

### 2026-06-02 Runtime Retirement Update

The open question about Converge internals now has a concrete boundary for
`converge-runtime`: it remains as a compatibility-only internal service shell,
not a canonical deployed runtime. Runtime Runway owns live app hosting, auth,
storage, secrets, telemetry, process lifecycle, and deployment. Lattice Mesh
owns distributed work coordination. Commerce Rails owns commercial authority.
The former converge.zone `/api/**` rewrite to Cloud Run service
`converge-runtime` was removed from the active Firebase config.

## Reconciliation: organism

### Role in Platform

`bedrock-platform/organism` is the formation, planning, agent, memory,
and intelligence runtime foundation. It is consumed by Helm, Atelier demos,
and downstream applications.

### Relevant Top-Level Domains

Platform Vision, Product & UX, System Architecture, Engineering, Roadmap.

### What Local Code Confirms

- Current workspace version is `1.9.1`.
- The workspace has 12 crates.
- Atelier references Organism domain crates from the root-level
  `atelier-showcase/` repo.

### What Local Code Contradicts

- Any docs implying Atelier lives under `stack/atelier-showcase` are stale.

### What Local Docs Needed Updating

Updated Organism README path examples for the root-level Atelier repo.

### What Should Be Promoted to Top-Level Docs

Organism remains the formation layer between Axiom intent and Converge
governance; proof repos should depend on it rather than duplicate its
formation abstractions.

### Open Questions

- Which Organism examples should move into Atelier versus stay in the
  foundation README.

### Suggested Top-Level Changes

- Keep Atelier listed as a root-level peer proof repo in architecture and
  engineering docs.

## Reconciliation: axiom

### Role in Platform

`bedrock-platform/axiom` is the language and truth translation layer. It
supports authoring and converting higher-level intent into lower-level
platform contracts.

### Relevant Top-Level Domains

Platform Vision, Product & UX, System Architecture, Engineering, Knowledge.

### What Local Code Confirms

- Current crate is `axiom-truth` at version `0.15.0`.
- The local binary is `cz`.

### What Local Code Contradicts

- No current contradiction found in this pass.

### What Local Docs Needed Updating

No targeted Axiom README changes were required.

### What Should Be Promoted to Top-Level Docs

Axiom should be described as a translation layer, not as an application or
runtime owner.

### Open Questions

- Whether `cz` should be documented in top-level developer onboarding.

### Suggested Top-Level Changes

- Keep Axiom in the system map as the upstream intent/truth translation layer.

## Reconciliation: Helm / Outcome Workbench

### Role in Platform

`bedrock-platform/helms` is the executable application and truth workbench.
It demonstrates product workflows over Converge, Organism, Mosaic, and
business-domain crates.

### Relevant Top-Level Domains

Product & UX, Business & Commerce, System Architecture, Engineering,
Operations, Roadmap.

### What Local Code Confirms

- Current workspace version is `0.2.0`.
- Cargo metadata resolves 35 workspace members.
- The catalog currently has 23 truth definitions.
- The executable workbench truth keys are `qualify-inbound-lead`,
  `submit-expense-report`, `activate-subscription`, and
  `refill-prepaid-ai-credits`.
- Proto namespaces remain under `proto/prio/*/v1`.

### What Local Code Contradicts

- Docs claiming nine executable truths are ahead of the current code-backed
  surface.
- Docs that treat all `prio-*` domains as product-complete are overstating the
  current implementation state.

### What Local Docs Needed Updating

Updated the Outcome Workbench README, session preamble, foundation contracts,
and browser extension demo notes with the current workspace shape and
executable truth count.

### What Should Be Promoted to Top-Level Docs

Top-level product and architecture docs should distinguish cataloged truth
definitions from workbench-executable truth paths.

### Open Questions

- Which of the 23 cataloged truth definitions are the next promotion targets
  for executable workbench flows.

### Suggested Top-Level Changes

- Make the 23 definitions / 4 executable truths distinction visible anywhere
  Helm proof coverage is discussed.

## Reconciliation: mosaic-extensions

### Role in Platform

`mosaic-extensions` is a multi-repository container for reusable
Converge-adjacent specialist capabilities: policy, ML, ports, solvers,
adapters, memory, analytics, safety, and integration harnesses.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations, Knowledge, Roadmap.

### What Local Code Confirms

- Mosaic is not one Cargo workspace; each extension repo keeps its own
  manifest and release surface.
- Current versions observed in manifests are Arbiter `2.0.1`, Crucible
  `0.3.0`, Embassy `1.3.0`, Ferrox `0.7.1`, Manifold `1.1.1`, Mnemos
  `1.2.2`, Prism `2.0.1`, and Soter `0.2.2`.
- The current shared Converge baseline is `3.9.1`.

### What Local Code Contradicts

- Docs that frame Mosaic as a single workspace or as sharing one release train
  are incomplete.
- Docs that place model fitting in Prism are stale.

### What Local Docs Needed Updating

Updated Mosaic README, developer guide, release/versioning notes, module KBs,
and local extension READMEs for current versions, boundaries, and dependency
rules.

### What Should Be Promoted to Top-Level Docs

The platform model should describe Mosaic as a portfolio of independently
versioned extension repos, coordinated by Converge contracts and path patches
during stack development.

### Open Questions

- Whether `integration-harness/` should become a separately versioned repo or
  remain a container-local validation harness.

### Suggested Top-Level Changes

- Add independent Mosaic extension versioning and local `[patch.crates-io]`
  redirects to engineering guidance.

## Reconciliation: arbiter-policy

### Role in Platform

Arbiter owns Cedar-backed policy gates and authorization suggestors for
runtime decisions.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations.

### What Local Code Confirms

- Current version is `2.0.1`.
- Arbiter belongs in Mosaic, not Converge core.

### What Local Code Contradicts

- No current contradiction found in this pass.

### What Local Docs Needed Updating

Updated Mosaic version tables.

### What Should Be Promoted to Top-Level Docs

Policy engines are specialist capabilities; platform-wide contracts should
call them through Converge-compatible suggestor boundaries.

### Open Questions

- Which product flows should require Arbiter policy gates by default.

### Suggested Top-Level Changes

- Reference Arbiter from operations and architecture where runtime policy
  checks are described.

## Reconciliation: crucible-models

### Role in Platform

Crucible owns trained models, training pipelines, fitted artifacts, classifier
suggestors, and typed model provenance.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations, Roadmap.

### What Local Code Confirms

- Current version is `0.3.0`.
- Training is implemented under `crates/crucible/src/training/`.
- Data ingestion is in `crates/crucible/src/ingest.rs`.
- There is no current `crucible::storage` module or storage feature.

### What Local Code Contradicts

- Docs pointing to `crucible/src/training.rs` are stale after the split.
- Docs that describe Crucible as owning storage contracts are stale.

### What Local Docs Needed Updating

Updated Crucible README, crate docs, architecture surface notes, developer
guide entries, and module KB notes.

### What Should Be Promoted to Top-Level Docs

The Prism/Crucible split is a platform-wide reasoning boundary: Prism owns
closed-form inference; Crucible owns anything fit to data.

### Open Questions

- Which retraining triggers are product requirements versus roadmap targets.

### Suggested Top-Level Changes

- Keep continuous learning described as a roadmap direction unless a runtime
  retrain trigger exists in code.

## Reconciliation: embassy-ports

### Role in Platform

Embassy owns source-specific connector ports where identity, terms, provenance,
and source semantics are part of the contract.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations, Business & Commerce.

### What Local Code Confirms

- Current version is `1.3.0`.
- Embassy includes many source-specific ports, including LinkedIn, SEC EDGAR,
  Bolagsverket, GLEIF, VIES, sanctions, SAM.gov, USASpending, TED,
  Skatteverket, USPTO, Crunchbase, GitHub, PubMed, arXiv, OpenAlex, Wikidata,
  Companies House, SCB, and EPO.

### What Local Code Contradicts

- Docs that describe Embassy as LinkedIn-only are stale.

### What Local Docs Needed Updating

Updated Embassy README and Mosaic docs to show current port families.

### What Should Be Promoted to Top-Level Docs

Use the Embassy-vs-Manifold decision rule platform-wide: source-specific
semantic ports go to Embassy; generic provider/storage/tool adapters go to
Manifold.

### Open Questions

- Which external ports require explicit commercial or compliance ownership in
  Business & Commerce docs.

### Suggested Top-Level Changes

- Add connector provenance and source-specific terms to operations guidance.

## Reconciliation: ferrox-solvers

### Role in Platform

Ferrox owns optimization and solver-backed suggestors.

### Relevant Top-Level Domains

System Architecture, Engineering, Roadmap.

### What Local Code Confirms

- Current version is `0.7.1`.
- Solver functionality is a Mosaic specialist capability, not application
  domain logic.

### What Local Code Contradicts

- No current contradiction found in this pass.

### What Local Docs Needed Updating

Updated Mosaic version tables.

### What Should Be Promoted to Top-Level Docs

Optimization logic should remain behind specialist suggestor boundaries.

### Open Questions

- Which Helm truth paths should expose optimization-backed recommendations.

### Suggested Top-Level Changes

- Mention Ferrox in architecture as the optimization home, not as a product
  feature by itself.

## Reconciliation: manifold-adapters

### Role in Platform

Manifold owns generic provider, storage, and tool adapters that are not
source-specific semantic ports.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations.

### What Local Code Confirms

- Current version is `1.1.1`.
- Current Converge baseline is `3.9.1` or newer.

### What Local Code Contradicts

- Docs pinned to older Converge integration baselines are stale.

### What Local Docs Needed Updating

Updated Manifold README and Mosaic developer docs.

### What Should Be Promoted to Top-Level Docs

Generic adapter contracts should be distinguished from Embassy source ports.

### Open Questions

- Which adapter families should become platform-required runtime dependencies.

### Suggested Top-Level Changes

- Add Manifold to runtime assembly guidance where generic providers are wired.

## Reconciliation: mnemos-knowledge

### Role in Platform

Mnemos owns knowledge, recall, memory, and retrieval suggestors.

### Relevant Top-Level Domains

System Architecture, Engineering, Product & UX, Roadmap.

### What Local Code Confirms

- Current version is `1.2.2`.
- Memory remains a Mosaic specialist capability.

### What Local Code Contradicts

- No current contradiction found in this pass.

### What Local Docs Needed Updating

Updated Mosaic version tables.

### What Should Be Promoted to Top-Level Docs

Memory and recall should be modeled as governed capability calls, not silent
ambient state.

### Open Questions

- Which experience records become the source of labels for future Crucible
  retraining.

### Suggested Top-Level Changes

- Tie Mnemos to the roadmap for continuous learning, while keeping retraining
  described as future until implemented.

## Reconciliation: prism-analytics

### Role in Platform

Prism owns closed-form analytics, fuzzy inference, feature extraction, and
hand-authored inference.

### Relevant Top-Level Domains

System Architecture, Engineering, Product & UX.

### What Local Code Confirms

- Current version is `2.0.1`.
- Prism no longer owns training pipelines or model registry behavior.

### What Local Code Contradicts

- Docs that place trained models, registry, monitoring, or deployment in Prism
  are stale.

### What Local Docs Needed Updating

Updated Prism README and Mosaic docs to restore the Prism/Crucible boundary.

### What Should Be Promoted to Top-Level Docs

Closed-form inference and fitted models are different platform concerns and
should not share ownership language.

### Open Questions

- Whether Prism feature extraction outputs should use a shared schema with
  Crucible training features.

### Suggested Top-Level Changes

- Update architecture examples to route hand-authored inference to Prism and
  fitted models to Crucible.

## Reconciliation: soter-smt

### Role in Platform

Soter owns SMT-backed safety evidence and solver-backed correctness checks.

### Relevant Top-Level Domains

System Architecture, Engineering, Operations, Roadmap.

### What Local Code Confirms

- Current version is `0.2.2`.
- Safety evidence is a specialist Mosaic capability.

### What Local Code Contradicts

- No current contradiction found in this pass.

### What Local Docs Needed Updating

Updated Mosaic version tables.

### What Should Be Promoted to Top-Level Docs

Safety evidence should be a first-class reasoning mode rather than an
after-the-fact test-only activity.

### Open Questions

- Which production flows require SMT evidence before promotion.

### Suggested Top-Level Changes

- Reference Soter in operations guidance for high-stakes promotion checks.

## Reconciliation: integration-harness

### Role in Platform

`mosaic-extensions/integration-harness` is the cross-extension validation
home for proving Mosaic modules work together.

### Relevant Top-Level Domains

Engineering, Operations, Knowledge.

### What Local Code Confirms

- Mosaic has an executable integration-harness concept distinct from the
  extension crate families.

### What Local Code Contradicts

- Docs that list only the eight extension repos omit an important validation
  surface.

### What Local Docs Needed Updating

Added the integration harness to the Mosaic README.

### What Should Be Promoted to Top-Level Docs

Cross-extension golden flows should be treated as platform evidence, not just
local tests.

### Open Questions

- Whether the harness should validate root-level Arena cases or stay purely
  Mosaic-local.

### Suggested Top-Level Changes

- Add integration-harness expectations to Engineering and Operations docs.

## Reconciliation: atelier-showcase

### Role in Platform

`atelier-showcase` is a root-level proof and demo repo for tutorialized,
realistic platform scenarios. It exercises Organism, Converge, Mosaic, and
domain crates without becoming a fake-contract test bucket.

### Relevant Top-Level Domains

Product & UX, System Architecture, Engineering, Roadmap.

### What Local Code Confirms

- Current workspace version is `1.0.0`.
- It is a root-level peer repo at `/Users/kpernyer/dev/reflective/atelier-showcase`.
- Its manifests resolve against root-level `bedrock-platform` and
  `mosaic-extensions` paths.

### What Local Code Contradicts

- Docs and manifests that point to `stack/atelier-showcase` are stale.

### What Local Docs Needed Updating

Updated root-relative path references in Organism and Outcome Workbench
manifests/docs, and updated Atelier's README to describe the current local
path-patch build model.

### What Should Be Promoted to Top-Level Docs

Atelier is the right home for polished tutorial and scenario galleries with
real resources and user-facing narrative.

### Open Questions

- Which showcased flows should be treated as product commitments versus demo
  proof.

### Suggested Top-Level Changes

- Keep Atelier listed as a root-level proof repo in the platform and
  engineering docs.

## Reconciliation: arena-tests

### Role in Platform

`arena-tests` is a root-level test-only repo for cross-extension smoke tests,
intent cases, and convergence scenarios that should not be polished into
Atelier demos.

### Relevant Top-Level Domains

Engineering, System Architecture, Operations, Knowledge.

### What Local Code Confirms

- Current workspace version is `0.1.0`.
- The repo contains `cross-extension-smoke`, `intent-cases`, and
  `counterparty-kyc-convergence` crates.
- Its manifests resolve against root-level `bedrock-platform` and
  `mosaic-extensions` paths.

### What Local Code Contradicts

- Docs and manifests that point to `stack/arena-tests` are stale.

### What Local Docs Needed Updating

Updated Arena README and Cargo manifests to root-level relative paths.

### What Should Be Promoted to Top-Level Docs

Arena is the right home for mixed-resource, fake-service, drift-detection, and
contract-shape tests that would weaken Atelier if presented as product demos.

### Open Questions

- Which Arena cases should graduate into Mosaic integration harnesses versus
  remain root-level cross-stack tests.

### Suggested Top-Level Changes

- Keep the Atelier/Arena split in Engineering Principles as the default proof
  repo pattern.

## Top-Level Documentation Changes Needed

- Keep all eight canonical docs aligned with the root-level repo shape:
  `atelier-showcase/` and `arena-tests/` are peers of `stack/`, not children.
- Preserve the distinction between cataloged Helm truth definitions and
  executable workbench truth paths.
- Keep Converge integration guidance pinned to `3.9.1` and the six external
  public crates until manifests prove otherwise.
- Describe Mosaic as independently versioned extension repos coordinated by
  Converge-compatible contracts and local path patches.
- Document the Prism/Crucible boundary everywhere reasoning modes are
  discussed.
- Mark historical `helms/`, `converge-provider-api`, `stack/atelier-showcase`,
  and `stack/arena-tests` references as historical or superseded when they are
  not part of current source layout.

## Promoted Patterns

- Proof repos split by intent: Atelier for polished scenario galleries, Arena
  for contract-shape and mixed-resource test cases.
- Specialist capability boundaries: Prism for closed-form inference, Crucible
  for fitted models, Embassy for source-specific ports, Manifold for generic
  adapters.
- Code-backed truth levels: cataloged definitions and executable flows must be
  counted separately.
- Local stack development uses path patches to test unreleased Converge and
  Mosaic changes together.
- Cross-extension integration harnesses are platform evidence and should be
  maintained alongside crate-level tests.

## Contradictions And Unresolved Questions

- `bedrock-platform/helms` is the current code home but remains an unclear
  long-term directory name.
- Continuous learning is supported by Crucible components but runtime retrain
  triggers remain roadmap, not current reality.
- Some historical KB and release notes intentionally preserve obsolete names;
  future edits should mark those sections historical rather than rewriting
  chronology.
- Ownership between Mosaic `integration-harness` and root-level `arena-tests`
  should be made explicit before adding more golden flows.

## Recommended Update Plan For The Canonical Documents

- `01-platform`: Keep the stack story focused on Axiom -> Organism ->
  Converge -> Helm, with Mosaic as specialist capability portfolio and
  Commerce Rails as commercial truth owner.
- `02-product`: Reflect Helm's four executable truth paths separately from
  catalog breadth; describe Atelier as proof/demo, not product commitment.
- `03-commerce`: Keep Commerce Rails as the commercial source of truth and map
  Embassy external ports that carry compliance or partner obligations.
- `04-architecture`: Keep the current-system-map aligned with root-level
  proof repos, Converge `3.9.1`, Mosaic independent versioning, and the
  Prism/Crucible split.
- `05-engineering`: Keep the Atelier/Arena proof-repo pattern, path-patch
  guidance, and code-over-docs reconciliation rule.
- `06-operations`: Add source-specific connector provenance expectations,
  Mosaic integration-harness validation, and high-stakes safety evidence hooks
  where relevant.
- `07-knowledge`: Track active/superseded names and directory moves here
  before deleting historical references elsewhere.
- `08-roadmap`: Keep continuous learning, retrain triggers, and broader Helm
  executable-truth expansion as roadmap until code proves those behaviors.
