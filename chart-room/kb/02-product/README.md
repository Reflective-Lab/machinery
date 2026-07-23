# Product & User Experience Architecture

This domain explains how users experience the platform and products. It should
stay grounded in shipped or scaffolded surfaces, not future wishes.

## Current Product Surfaces

| Surface | Home | Current role |
|---------|------|--------------|
| Helm / Helms | `bedrock-platform/helms` | Bedrock trust-transfer surface: approvals, truth visibility, operator control, and app mediation |
| Catalyst | `marquee-apps/catalyst-biz` | SMB business-ops app; doctrinally Reframe-watch under governed decision translation |
| Shoal | `blueprint-apps/shoal-meta` | Blueprint KB-only meta-app that explains the composed-decision stack across the apps |
| Keystone | `blueprint-apps/keystone-architecture` | Blueprint KB-first constraint-driven structure search for governed decisions |
| Wolfgang | `studio-apps/wolfgang-chat` | Studio research companion with desktop app, web app, backend, personas, and knowledgebase flows |
| apps.reflective.se | `beacon-sites/apps.reflective.se` | Portal that hosts app-facing web surfaces, including Wolfgang web under `/wolfgang-chat/**` |
| Public sites | `beacon-sites/www.*` | Marketing and positioning surfaces for Converge, Organism, Axiom, Helms, Reflective |
| Mobile clients | `mobile-apps/converge-android`, `mobile-apps/converge-ios` | Native Converge client projects |
| Marquee apps | `marquee-apps/*` | Category drivers (quorum, plumb); other resident apps are Applied — sorted by commitment shape (convened burst / multi-sovereign / standing governance) — or Reframe-watch per reflective-paradigm §6 |
| Blueprint apps | `blueprint-apps/*` | KB-first doctrine/meta apps (Shoal, Keystone) — not shipped products |
| Studio apps | `studio-apps/*` | Creative/productivity apps moved out of marquee when their domain fit the studio portfolio |

## User Experience Boundaries

- Product repositories own UX, app-specific projections, and domain consequence.
- Apps are thin and JTBD-oriented; reusable platform behavior belongs in Bedrock
  and Mosaic, not inside app repos.
- Applets should start from an Intent Codec entry: functional, emotional, and
  relational demand plus authority, evidence, runtime, and commerce boundaries.
- Helm/Helms surfaces are control layers: they show truths, approvals, exceptions, and operator state.
- Apps should consume shared runtime, commerce, and platform contracts instead of owning those foundations.
- Desktop, web, and mobile may expose different affordances, but should not invent separate domain truths for the same job.

## Current Capability Map

- Helm currently has a 23-definition truth catalog and four executable truth paths exposed through `workbench-backend`.
- Catalyst is the current SMB business-ops marquee app and depends on Helm-facing JTBD execution.
- Shoal and Keystone are KB-first blueprint apps (in `blueprint-apps/`) that clarify cross-app composition and structure-search pressure.
- Wolfgang has desktop, web, backend, shared UI, personas, knowledgebase ingestion, auth, and deployment scaffolding.
- apps.reflective.se currently packages a portal plus a copied Wolfgang web build.
- Mobile projects exist separately from the current Converge spec archive.

## Canonical Links

- [[intent-codec-jtbd-schema|Intent Codec JTBD Schema]]
- [[templates/intent-codec-applet|Intent Codec Applet Template]]
- `templates/intent-codec-applet.manifest.schema.json`
- [[applets/activate-subscription|Applied applet: Activate Subscription]]
- [[applets/refill-prepaid-ai-credits|Applied applet: Refill Prepaid AI Credits]]
- [[../outcome-workbench/kb/Home|Helm workspace docs]]
- [[../outcome-workbench/kb/Architecture/Module Map|Helm Module Map]]
- `../../marquee-apps/catalyst-biz/README.md`
- `../../blueprint-apps/shoal-meta/README.md`
- `../../blueprint-apps/keystone-architecture/README.md`
- [[../outcome-workbench/kb/Product/Desktop Notes Direction|Desktop Notes Direction]]
- [[../wolfgang-business/00-index|Wolfgang Business Vault]]
- [[../typography-and-design|Typography and Design]]
