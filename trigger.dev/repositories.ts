export interface RepositoryContext {
  language: string;
  layer: string;
  purpose: string;
  sentryProject?: string;
  linearLabel?: string;
  deployedTo?: string;
}

export interface RepositoryNodeData {
  name: string;
  language: string;
  purpose?: string;
  layer?: string;
  ci_status?: string;
  open_findings?: number;
  last_release?: string;
  sentry_project?: string;
  linear_label?: string;
  deployed_to?: string;
  adoption_cohort?: string;
  adoption_tier?: string;
  adoption_state?: string;
  adoption_missing_required?: string;
  adoption_exception_refs?: string;
  adoption_last_scanned?: string;
  adoption_scan_revision?: string;
}

export const REPO_CONTEXT: Record<string, RepositoryContext> = {
  "runtime-runway": {
    language: "Rust",
    layer: "runtime",
    purpose:
      "Deployment substrate: auth, storage, secrets, telemetry, LLM paths, Cloud Run",
    sentryProject: "runtime-runway",
    linearLabel: "module:runtime-runway",
    deployedTo: "cloud-run-europe-west1",
  },
  "commerce-rails": {
    language: "Rust",
    layer: "commerce",
    purpose:
      "Billing and commercial authority: subscriptions, entitlements, Stripe Connect",
    linearLabel: "module:commerce-rails",
  },
  converge: {
    language: "Rust",
    layer: "platform",
    purpose:
      "Decision engine: Suggestor/Formation/Invariant/ConvergeResult, gRPC streaming",
    deployedTo: "crates.io",
  },
  organism: {
    language: "Rust",
    layer: "platform",
    purpose:
      "Reasoning and planning: 6 reasoning systems, skepticism kinds, simulation",
  },
  axiom: {
    language: "Rust",
    layer: "platform",
    purpose: "Truth and JTBD layer with cz CLI",
  },
  helms: {
    language: "Rust",
    layer: "platform",
    purpose:
      "35-crate operator workbench: Tauri 2 desktop, prio-* gRPC+OpenAPI+GraphQL",
  },
  "quorum-sense": {
    language: "Rust",
    layer: "app",
    purpose: "Marquee app: live on Cloud Run Europe West 1",
    sentryProject: "quorum-sense",
    linearLabel: "module:quorum-sense",
    deployedTo: "cloud-run-europe-west1",
  },
  "build-depot": {
    language: "TypeScript",
    layer: "factory",
    purpose:
      "Software Factory: engineering operating system for the Reflective platform",
    linearLabel: "module:build-depot",
  },
  "bedrock-consolidated": {
    language: "Rust",
    layer: "platform",
    purpose:
      "Consolidated Reflective foundation workspace: Converge, Organism, Axiom, Helm, extensions, Arena, and Atelier",
    linearLabel: "module:bedrock-consolidated",
    deployedTo: "shipyard.rs/reflective-labs",
  },
  "alias-apps": {
    language: "Markdown",
    layer: "app-family",
    purpose: "KB-only app-family proofs for non-convened commitment shapes",
    linearLabel: "module:alias-apps",
  },
  "blueprint-apps": {
    language: "Markdown",
    layer: "app-family",
    purpose: "KB-only doctrine projects cited by application portfolios",
    linearLabel: "module:blueprint-apps",
  },
  "marquee-apps": {
    language: "Markdown",
    layer: "app-portfolio",
    purpose: "Portfolio root for end-user marquee and applied app projects",
    linearLabel: "module:marquee-apps",
  },
  "studio-apps": {
    language: "Markdown",
    layer: "app-portfolio",
    purpose: "Coordination root for studio-product app families",
    linearLabel: "module:studio-apps",
  },
};

export function repositoryData(
  name: string,
  patch: Partial<RepositoryNodeData> = {}
): RepositoryNodeData {
  const ctx = REPO_CONTEXT[name];
  const base: RepositoryNodeData = ctx
    ? {
        name,
        language: ctx.language,
        purpose: ctx.purpose,
        layer: ctx.layer,
      }
    : {
        name,
        language: "Unknown",
        layer: "unknown",
      };

  if (ctx?.sentryProject) base.sentry_project = ctx.sentryProject;
  if (ctx?.linearLabel) base.linear_label = ctx.linearLabel;
  if (ctx?.deployedTo) base.deployed_to = ctx.deployedTo;

  if (patch.purpose) base.purpose = patch.purpose;
  if (patch.layer) base.layer = patch.layer;
  if (patch.ci_status) base.ci_status = patch.ci_status;
  if (patch.open_findings !== undefined) base.open_findings = patch.open_findings;
  if (patch.last_release) base.last_release = patch.last_release;
  if (patch.sentry_project) base.sentry_project = patch.sentry_project;
  if (patch.linear_label) base.linear_label = patch.linear_label;
  if (patch.deployed_to) base.deployed_to = patch.deployed_to;
  if (patch.adoption_cohort) base.adoption_cohort = patch.adoption_cohort;
  if (patch.adoption_tier) base.adoption_tier = patch.adoption_tier;
  if (patch.adoption_state) base.adoption_state = patch.adoption_state;
  if (patch.adoption_missing_required) {
    base.adoption_missing_required = patch.adoption_missing_required;
  }
  if (patch.adoption_exception_refs) {
    base.adoption_exception_refs = patch.adoption_exception_refs;
  }
  if (patch.adoption_last_scanned) {
    base.adoption_last_scanned = patch.adoption_last_scanned;
  }
  if (patch.adoption_scan_revision) {
    base.adoption_scan_revision = patch.adoption_scan_revision;
  }

  return base;
}

export function repoFromLinearLabels(labels: string[]): string | undefined {
  const normalized = labels.map((label) => label.toLowerCase());

  for (const [repo, ctx] of Object.entries(REPO_CONTEXT)) {
    if (normalized.includes(repo.toLowerCase())) {
      return repo;
    }

    if (ctx.linearLabel && normalized.includes(ctx.linearLabel.toLowerCase())) {
      return repo;
    }
  }

  const moduleLabel = normalized.find((label) => label.startsWith("module:"));
  return moduleLabel?.slice("module:".length);
}

export function repoFromSentryProject(project: string): string | undefined {
  const target = project.toLowerCase();
  for (const [repo, ctx] of Object.entries(REPO_CONTEXT)) {
    if (repo.toLowerCase() === target || ctx.sentryProject?.toLowerCase() === target) {
      return repo;
    }
  }
  return undefined;
}
