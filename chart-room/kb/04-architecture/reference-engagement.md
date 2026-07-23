# Reference Engagement

The canonical starting point for a new Reflective Labs engagement built on the Converge / Organism / Axiom / Helms / Mosaic stack.

## The reference

> **`studio-apps/folio-editor/`** (project codename: "Newspaper") is the reference engagement implementation.

It is maintained because Karl uses it for the Sölvesborg Lede pilot. It tracks current platform head, follows the current crate-naming conventions, and ships a real Tauri + SvelteKit + 7-crate Rust workspace + KB structure.

See [[studio-apps/Architecture - Overview|studio-apps overview]] for the broader studio-apps context and [[../forge-templates/Architecture - Overview|forge-templates overview]] for what remains of the templates folder (the `converge-extension` template is still active for starting new Mosaic extensions).

## Why a reference, not a template

A static template carries floor-version tables and frozen scaffold conventions that decay the moment the platform releases. The `forge-templates/converge-engagement/` template was archived 2026-06-07 because its floor table was 1–8 minor versions stale (see [[decisions/2026-06-07-retire-engagement-template|retirement ADR]]). A live reference repo can't drift past its own dep bumps.

This is the same logic [[decisions/2026-05-23-runway-config-injection|runway-config-injection]] applied at the code level: prefer the source of truth that's already alive over a snapshot that has to be hand-synced.

## How to start a new engagement

1. **Copy folio-editor.** From `~/dev/reflective/`:
   ```sh
   cp -R studio-apps/folio-editor/ engagements/<new-name>/
   # or wherever new engagements land — see workspace conventions
   ```

2. **Rename `newspaper` to `<new-name>` across the workspace.** The 7 crates are named `newspaper-{domain,kernel,truths,app,platform,server}` plus `apps/desktop/src-tauri`. Find/replace `newspaper` → `<new-name>` in Cargo.toml manifests, lib.rs `pub use` re-exports, and any documentation that names them.

3. **Verify floor versions are current.** Run `cargo update` then `cargo check --workspace`. The dep versions in folio-editor's workspace `Cargo.toml` are the current floor — they don't need bumping unless you're tracking unreleased changes via `[patch.crates-io]`.

4. **Adapt the KB.** Copy `studio-apps/folio-editor/kb/` and update it for your engagement. Keep the same skeleton (Home / INDEX / LOG / Architecture / Standards / Planning).

5. **Update the workspace `[patch.crates-io]` block if doing local SDK work.** Folio-editor uses local paths to bedrock-platform/{converge,organism,axiom} + mosaic-extensions/* when iterating against unreleased platform changes. Same pattern works for any engagement.

6. **Register the new engagement in [[current-system-map|current-system-map.md]]** as a new Project Boundary Anchor. Add the `## Boundary` block to its README citing the new anchor (see [[bedrock-platform/Architecture - Converge|Converge]] or [[runtime-runway/Architecture - Overview|runtime-runway]] for the pattern).

## Reference extension (still template-driven)

The reference-implementation principle does not yet apply to new Mosaic extensions — `forge-templates/converge-extension/` carries working CI workflows, release-checklist enforcement scripts, and a criterion benchmark baseline extractor that are not yet in any individual mosaic-extensions repo as a "copy me" pattern. For new extensions, the template is still the recommended starting point. See [[forge-templates/Architecture - Overview|forge-templates overview]].

## Cross-references

- [[studio-apps/Architecture - Overview|studio-apps overview]] — folio-editor sits here
- [[forge-templates/Architecture - Overview|forge-templates overview]] — converge-extension still active
- [[decisions/2026-06-07-retire-engagement-template|2026-06-07 retire-engagement-template ADR]] — the decision
- [[current-system-map|current-system-map]] — boundary registry
