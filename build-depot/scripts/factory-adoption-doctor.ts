import {
  type Dirent,
  existsSync,
  readdirSync,
  readFileSync,
  statSync,
} from "node:fs";
import { spawnSync } from "node:child_process";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  REPO_CONTEXT,
  repositoryData,
  type RepositoryNodeData,
} from "../trigger.dev/repositories";

export type AdoptionTier = "Tier0" | "Tier1" | "Tier2" | "Tier3";
export type AdoptionState = "adopted" | "partial" | "blocked" | "exempt";
export type SignalStatus = "present" | "missing" | "exception" | "not_applicable";

export interface AdoptionSignalResult {
  id: string;
  title: string;
  required: boolean;
  status: SignalStatus;
  detail: string;
  reference?: string;
}

export interface RepositoryAdoptionResult {
  name: string;
  path: string;
  cohort?: string;
  tier: AdoptionTier;
  state: AdoptionState;
  signals: AdoptionSignalResult[];
  missingRequiredSignals: string[];
}

export interface AdoptionScanResult {
  scannedAt: string;
  sourceRevision: string;
  workspaceRoot: string;
  repositories: RepositoryAdoptionResult[];
  blockingFailures: number;
}

export interface ScanOptions {
  workspaceRoot?: string;
  now?: Date;
  sourceRevision?: string;
}

export type AdoptionGraphRecord = {
  type: "Repository";
  data: RepositoryNodeData;
};

interface RepoCandidate {
  name: string;
  path: string;
}

interface CohortMember {
  repo: string;
  tier?: AdoptionTier;
  role?: string;
}

interface CohortDefinition {
  id: string;
  name?: string;
  members?: CohortMember[];
}

interface CohortConfig {
  cohorts?: CohortDefinition[];
}

interface RepoCohort {
  id: string;
  member: CohortMember;
}

interface RepoFiles {
  agents: string;
  justfile: string;
  markdown: string;
  workflows: string[];
  toolPointers: Array<{ path: string; text: string }>;
  cargoManifests: Array<{ path: string; text: string }>;
  cargoConfigs: Array<{ path: string; text: string }>;
}

interface SignalInput {
  id: string;
  title: string;
  required: boolean;
  present: boolean;
  detail: string;
  repositoryText: string;
  exceptionKeywords: string[];
  notApplicable?: boolean;
}

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(SCRIPT_DIR, "..");
const DEFAULT_WORKSPACE_ROOT = resolve(PROJECT_ROOT, "..");
const TOOL_POINTER_FILES = ["CLAUDE.md", "CODEX.md", "GEMINI.md"];
const DOC_DIRECTORIES = ["docs", "KB", "kb"];
const SKIPPED_DIRECTORIES = new Set([
  ".git",
  ".jj",
  "node_modules",
  "target",
  "dist",
  "build",
  ".next",
  ".turbo",
  ".cache",
]);

export function scanWorkspace(options: ScanOptions = {}): AdoptionScanResult {
  const workspaceRoot = resolve(options.workspaceRoot ?? DEFAULT_WORKSPACE_ROOT);
  const now = options.now ?? new Date();
  const sourceRevision = options.sourceRevision ?? readGitRevision(workspaceRoot);
  const releaseTrainRepos = parseReleaseTrainRepos(workspaceRoot);
  const cohorts = parseCohorts(workspaceRoot);
  const repositories = discoverRepositories(workspaceRoot).map((repo) =>
    evaluateRepository(repo, releaseTrainRepos, cohorts)
  );
  const blockingFailures = repositories.filter((repo) => repo.state === "blocked")
    .length;

  return {
    scannedAt: now.toISOString(),
    sourceRevision,
    workspaceRoot,
    repositories,
    blockingFailures,
  };
}

export function evaluateRepository(
  repo: RepoCandidate,
  releaseTrainRepos: Set<string> = new Set(),
  cohorts: Map<string, RepoCohort> = new Map()
): RepositoryAdoptionResult {
  const cohort = cohorts.get(repo.name);
  const tier = classifyTier(repo, releaseTrainRepos, cohorts);
  const files = readRepoFiles(repo.path);
  const repositoryText = [
    files.agents,
    files.justfile,
    files.markdown,
    ...files.workflows,
    ...files.toolPointers.map((pointer) => pointer.text),
    ...files.cargoManifests.map((manifest) => manifest.text),
    ...files.cargoConfigs.map((config) => config.text),
  ].join("\n");
  const productionRuntime = isProductionRuntime(repo.name, tier, repositoryText);
  const shipyardDistribution = isShipyardDistributionRepo(repo.name, files);
  const checkoutState = readCheckoutState(repo.path);
  const activeRepo = tier !== "Tier3";

  const signals: AdoptionSignalResult[] = [
    signal({
      id: "checkout-current",
      title: "Scanned checkout is current enough for adoption verdict",
      required: activeRepo,
      present: checkoutState.present,
      detail: checkoutState.detail,
      repositoryText,
      exceptionKeywords: [],
    }),
    signal({
      id: "agents",
      title: "AGENTS.md points to Build-Depot",
      required: activeRepo,
      present: agentsPointsToBuildDepot(repo.name, files.agents),
      detail:
        "Expected AGENTS.md to point to Build-Depot for software-factory policy.",
      repositoryText,
      exceptionKeywords: ["agents", "software-factory", "software factory"],
    }),
    evaluateToolPointers(files),
    signal({
      id: "local-policy",
      title: "Local docs do not redefine factory policy",
      required: activeRepo && repo.name !== "build-depot",
      present: repo.name === "build-depot" || !hasUnownedFactoryPolicy(files.markdown),
      detail:
        "Expected repo-local factory docs to be compatibility pointers, not independent policy.",
      repositoryText,
      exceptionKeywords: ["factory policy", "software-factory", "quality gate"],
    }),
    signal({
      id: "just-ci",
      title: "Justfile exposes just ci",
      required: activeRepo,
      present: hasRecipe(files.justfile, "ci"),
      detail: "Expected Justfile with a ci recipe.",
      repositoryText,
      exceptionKeywords: ["just ci", "ci gate", "quality gate"],
    }),
    signal({
      id: "ci-workflow",
      title: "CI workflows call Just",
      required: activeRepo,
      present: workflowsCallJust(files.workflows),
      detail: "Expected GitHub workflows to call Just recipes.",
      repositoryText,
      exceptionKeywords: ["ci workflow", "github actions", "just ci"],
    }),
    signal({
      id: "security",
      title: "Security gate or accepted risk exists",
      required: activeRepo,
      present:
        hasRecipe(files.justfile, "security-audit") ||
        hasRecipe(files.justfile, "audit") ||
        hasRecipe(files.justfile, "security-doctor"),
      detail: "Expected security-audit, audit, or security-doctor Just recipe.",
      repositoryText,
      exceptionKeywords: ["security", "audit", "dependency", "secret"],
    }),
    signal({
      id: "delivery",
      title: "Delivery or release gate exists",
      required: activeRepo && (tier === "Tier0" || tier === "Tier1"),
      present:
        hasRecipe(files.justfile, "delivery-preflight") ||
        hasRecipe(files.justfile, "release-preflight") ||
        hasRecipe(files.justfile, "release") ||
        hasRecipe(files.justfile, "deploy"),
      detail:
        "Expected delivery-preflight, release-preflight, release, or deploy Just recipe.",
      repositoryText,
      exceptionKeywords: ["delivery", "release", "deploy", "preflight"],
      notApplicable: !activeRepo || (tier !== "Tier0" && tier !== "Tier1"),
    }),
    signal({
      id: "shipyard-registry",
      title: "Shipyard crates keep Cargo registry attribution",
      required: activeRepo && shipyardDistribution,
      present: hasReflectiveLabsRegistryAttribution(files.cargoManifests),
      detail:
        'Expected publishable Rust manifests to retain registry = "reflective-labs" Cargo attribution.',
      repositoryText,
      exceptionKeywords: [],
      notApplicable: !shipyardDistribution,
    }),
    signal({
      id: "depot-distribution",
      title: "Private distribution operations are Build-Depot-owned",
      required: activeRepo && shipyardDistribution,
      present:
        shipyardDistribution && privateDistributionSurfaceIsDepotOwned(files),
      detail:
        "Expected Shipyard publishing workflows, private registry guides, and Shipyard secrets to live in Build-Depot rather than the published Rust workspace.",
      repositoryText,
      exceptionKeywords: [],
      notApplicable: !shipyardDistribution,
    }),
    signal({
      id: "linear-label",
      title: "Linear repo label exists",
      required: activeRepo,
      present: hasLinearLabel(repo.name, repositoryText),
      detail: "Expected stable module:<repo> Linear label metadata or docs.",
      repositoryText,
      exceptionKeywords: ["linear", "module", "label"],
    }),
    signal({
      id: "sentry",
      title: "Sentry mapping or not-applicable declaration exists",
      required: activeRepo && (tier === "Tier1" || productionRuntime),
      present: hasSentryMapping(repo.name, repositoryText),
      detail:
        "Expected Sentry project mapping or an explicit non-runtime/not-applicable declaration.",
      repositoryText,
      exceptionKeywords: ["sentry", "incident", "runtime"],
      notApplicable: activeRepo && tier !== "Tier1" && !productionRuntime,
    }),
    signal({
      id: "compatibility-docs",
      title: "Compatibility docs point to Build-Depot",
      required: activeRepo,
      present: repo.name === "build-depot" || compatibilityDocsPointToBuildDepot(files.markdown),
      detail: "Expected local software-factory references to point to Build-Depot.",
      repositoryText,
      exceptionKeywords: ["compatibility", "build-depot", "factory docs"],
    }),
  ];

  const missingRequiredSignals = signals
    .filter((result) => result.required && result.status === "missing")
    .map((result) => result.id);
  const state = adoptionState(tier, missingRequiredSignals.length);

  return {
    name: repo.name,
    path: repo.path,
    ...(cohort ? { cohort: cohort.id } : {}),
    tier,
    state,
    signals,
    missingRequiredSignals,
  };
}

export function formatScan(scan: AdoptionScanResult): string {
  const lines = [
    `Factory adoption scan ${scan.scannedAt}`,
    `Workspace: ${scan.workspaceRoot}`,
    `Revision: ${scan.sourceRevision}`,
    "",
  ];

  for (const repo of scan.repositories) {
    const missing =
      repo.missingRequiredSignals.length === 0
        ? "none"
        : repo.missingRequiredSignals.join(", ");
    lines.push(`${repo.state.toUpperCase()} ${repo.tier} ${repo.name}`);
    if (repo.cohort) {
      lines.push(`  cohort: ${repo.cohort}`);
    }
    lines.push(`  path: ${relative(scan.workspaceRoot, repo.path) || "."}`);
    lines.push(`  missing required: ${missing}`);

    for (const signalResult of repo.signals) {
      const required = signalResult.required ? "required" : "optional";
      lines.push(
        `  - ${signalResult.status} ${signalResult.id} (${required}): ${signalResult.title}`
      );
      if (signalResult.status !== "present") {
        lines.push(`    ${signalResult.reference ?? signalResult.detail}`);
      }
    }
    lines.push("");
  }

  lines.push(`Blocking failures: ${scan.blockingFailures}`);
  return lines.join("\n");
}

export function adoptionRecordsFromScan(
  scan: AdoptionScanResult
): AdoptionGraphRecord[] {
  return scan.repositories.map((repo) => {
    const exceptions = repo.signals
      .filter((signalResult) => signalResult.status === "exception")
      .flatMap((signalResult) =>
        signalResult.reference ? [`${signalResult.id}: ${signalResult.reference}`] : []
      );
    const patch: Partial<RepositoryNodeData> = {
      ...(repo.cohort ? { adoption_cohort: repo.cohort } : {}),
      adoption_tier: repo.tier,
      adoption_state: repo.state,
      adoption_last_scanned: scan.scannedAt,
      adoption_scan_revision: scan.sourceRevision,
    };

    if (repo.missingRequiredSignals.length > 0) {
      patch.adoption_missing_required = repo.missingRequiredSignals.join(",");
    }

    if (exceptions.length > 0) {
      patch.adoption_exception_refs = exceptions.join(" | ");
    }

    return {
      type: "Repository",
      data: repositoryData(repo.name, patch),
    };
  });
}

function discoverRepositories(workspaceRoot: string): RepoCandidate[] {
  const repos: RepoCandidate[] = [];

  if (existsSync(join(workspaceRoot, ".git"))) {
    repos.push({ name: basename(workspaceRoot), path: workspaceRoot });
  }

  for (const entry of safeReadDir(workspaceRoot)) {
    if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
    const repoPath = join(workspaceRoot, entry.name);
    if (existsSync(join(repoPath, ".git"))) {
      repos.push({ name: entry.name, path: repoPath });
    }
  }

  return repos.sort((left, right) => left.name.localeCompare(right.name));
}

function classifyTier(
  repo: RepoCandidate,
  releaseTrainRepos: Set<string>,
  cohorts: Map<string, RepoCohort>
): AdoptionTier {
  if (repo.name === "build-depot") return "Tier0";
  const cohortTier = cohorts.get(repo.name)?.member.tier;
  if (cohortTier) return cohortTier;
  if (releaseTrainRepos.has(repo.name) || isKnownProductionRepo(repo.name)) {
    return "Tier1";
  }
  if (isTier3Repo(repo)) return "Tier3";
  return "Tier2";
}

function parseReleaseTrainRepos(workspaceRoot: string): Set<string> {
  const releaseTrain = readFile(join(workspaceRoot, "release-train.yaml"));
  const repos = new Set<string>();

  for (const line of releaseTrain.split("\n")) {
    const match = line.match(/^\s*dir:\s*([^#\s]+)/);
    if (!match?.[1]) continue;
    repos.add(match[1].split("/")[0] ?? match[1]);
  }

  return repos;
}

function parseCohorts(workspaceRoot: string): Map<string, RepoCohort> {
  const text = cohortConfigPaths(workspaceRoot)
    .map((path) => readFile(path))
    .find((contents) => contents.trim().length > 0);
  const cohorts = new Map<string, RepoCohort>();
  if (!text) return cohorts;

  let parsed: CohortConfig;
  try {
    parsed = JSON.parse(text) as CohortConfig;
  } catch {
    return cohorts;
  }

  for (const cohort of parsed.cohorts ?? []) {
    if (!cohort.id) continue;

    for (const member of cohort.members ?? []) {
      if (!member.repo) continue;
      cohorts.set(member.repo, {
        id: cohort.id,
        member,
      });
    }
  }

  return cohorts;
}

function cohortConfigPaths(workspaceRoot: string): string[] {
  const root = resolve(workspaceRoot);
  const paths = [
    join(root, "factory-cohorts.json"),
    join(root, "build-depot", "factory-cohorts.json"),
  ];

  if (root === DEFAULT_WORKSPACE_ROOT || root === PROJECT_ROOT) {
    paths.push(join(PROJECT_ROOT, "factory-cohorts.json"));
  }

  return paths;
}

function readRepoFiles(repoPath: string): RepoFiles {
  const toolPointers = TOOL_POINTER_FILES.flatMap((name) => {
    const filePath = join(repoPath, name);
    if (!existsSync(filePath)) return [];
    return [{ path: name, text: readFile(filePath) }];
  });

  return {
    agents: readFile(join(repoPath, "AGENTS.md")),
    justfile: readFile(join(repoPath, "Justfile")),
    markdown: collectMarkdown(repoPath),
    workflows: collectWorkflowFiles(repoPath),
    toolPointers,
    cargoManifests: collectNamedFiles(repoPath, "Cargo.toml", 0),
    cargoConfigs: collectCargoConfigs(repoPath),
  };
}

function signal(input: SignalInput): AdoptionSignalResult {
  if (input.notApplicable === true && !input.present) {
    return {
      id: input.id,
      title: input.title,
      required: input.required,
      status: "not_applicable",
      detail: input.detail,
    };
  }

  if (input.present) {
    return {
      id: input.id,
      title: input.title,
      required: input.required,
      status: "present",
      detail: input.detail,
    };
  }

  const reference = findExceptionReference(
    input.repositoryText,
    input.exceptionKeywords
  );
  if (reference) {
    return {
      id: input.id,
      title: input.title,
      required: input.required,
      status: "exception",
      detail: input.detail,
      reference,
    };
  }

  return {
    id: input.id,
    title: input.title,
    required: input.required,
    status: "missing",
    detail: input.detail,
  };
}

function evaluateToolPointers(files: RepoFiles): AdoptionSignalResult {
  if (files.toolPointers.length === 0) {
    return {
      id: "tool-pointers",
      title: "Tool pointer files are short AGENTS.md pointers",
      required: false,
      status: "not_applicable",
      detail: "No tool-specific pointer files are present.",
    };
  }

  const badPointers = files.toolPointers.filter((pointer) => {
    const lineCount = pointer.text.split("\n").length;
    return lineCount > 80 || !pointer.text.includes("AGENTS.md");
  });

  if (badPointers.length === 0) {
    return {
      id: "tool-pointers",
      title: "Tool pointer files are short AGENTS.md pointers",
      required: false,
      status: "present",
      detail: "Tool-specific files point to AGENTS.md.",
    };
  }

  return {
    id: "tool-pointers",
    title: "Tool pointer files are short AGENTS.md pointers",
    required: false,
    status: "missing",
    detail: `Expected short AGENTS.md pointers; review ${badPointers
      .map((pointer) => pointer.path)
      .join(", ")}.`,
  };
}

function agentsPointsToBuildDepot(repoName: string, agents: string): boolean {
  const lower = agents.toLowerCase();
  if (!lower.includes("build-depot")) return false;
  if (repoName === "build-depot") {
    return lower.includes("canonical") || lower.includes("software-factory");
  }
  return lower.includes("software-factory") || lower.includes("software factory");
}

function hasUnownedFactoryPolicy(markdown: string): boolean {
  const lower = markdown.toLowerCase();
  const mentionsFactoryPolicy =
    lower.includes("software-factory quality system") ||
    lower.includes("quality-gate semantics") ||
    lower.includes("factory scorecard") ||
    lower.includes("recurring system properties");

  return mentionsFactoryPolicy && !lower.includes("build-depot");
}

function compatibilityDocsPointToBuildDepot(markdown: string): boolean {
  const lower = markdown.toLowerCase();
  const mentionsFactory =
    lower.includes("software-factory") ||
    lower.includes("software factory") ||
    lower.includes("quality gate") ||
    lower.includes("factory scorecard");

  return !mentionsFactory || lower.includes("build-depot");
}

function hasRecipe(justfile: string, recipe: string): boolean {
  return new RegExp(`^${escapeRegExp(recipe)}(?:\\s[^:]*)?:`, "m").test(justfile);
}

function workflowsCallJust(workflows: string[]): boolean {
  return workflows.some((workflow) =>
    /\bjust\s+(ci|doctor|check|test|security-audit|delivery-preflight|release-preflight|release|deploy)\b/.test(
      workflow
    )
  );
}

function hasLinearLabel(repoName: string, repositoryText: string): boolean {
  const context = REPO_CONTEXT[repoName];
  if (context?.linearLabel) return true;

  const lower = repositoryText.toLowerCase();
  return (
    lower.includes(`module:${repoName.toLowerCase()}`) ||
    lower.includes(`module:${repoName.toLowerCase().replaceAll("-", "_")}`)
  );
}

function hasSentryMapping(repoName: string, repositoryText: string): boolean {
  const context = REPO_CONTEXT[repoName];
  if (context?.sentryProject) return true;

  const lower = repositoryText.toLowerCase();
  return (
    lower.includes("sentry") &&
    (lower.includes("not applicable") ||
      lower.includes("n/a") ||
      lower.includes("does not emit runtime incidents") ||
      lower.includes("no runtime incidents"))
  );
}

function isShipyardDistributionRepo(repoName: string, files: RepoFiles): boolean {
  const deployedTo = REPO_CONTEXT[repoName]?.deployedTo?.toLowerCase() ?? "";
  if (deployedTo.includes("shipyard")) return true;

  // Any workspace carrying the private registry name participates in the
  // Shipyard distribution contract, even when it only consumes sibling crates.
  // Consumers still need Cargo attribution but must not own publish credentials.
  const text = [
    ...files.cargoManifests.map((manifest) => manifest.text),
    ...files.cargoConfigs.map((config) => config.text),
  ]
    .join("\n")
    .toLowerCase();

  return (
    text.includes("ssh.shipyard.rs") ||
    text.includes("shipyard.rs") ||
    text.includes('registry = "reflective-labs"') ||
    text.includes("registry = 'reflective-labs'")
  );
}

function hasReflectiveLabsRegistryAttribution(
  cargoManifests: Array<{ path: string; text: string }>
): boolean {
  return cargoManifests.some((manifest) =>
    /registry\s*=\s*["']reflective-labs["']/.test(manifest.text)
  );
}

function privateDistributionSurfaceIsDepotOwned(files: RepoFiles): boolean {
  const workflowText = files.workflows.join("\n").toLowerCase();
  const markdownText = files.markdown.toLowerCase();

  const hasShipyardWorkflow =
    workflowText.includes("shipyard") ||
    workflowText.includes("shipyard_ssh_key") ||
    workflowText.includes("shipyard_token") ||
    workflowText.includes("cargo_registries_reflective_labs_index");
  const hasPrivateRegistryGuide =
    markdownText.includes("ssh.shipyard.rs") ||
    markdownText.includes("shipyard_ssh_key") ||
    markdownText.includes("shipyard_token") ||
    markdownText.includes("cargo_registries_reflective_labs_index");

  return !hasShipyardWorkflow && !hasPrivateRegistryGuide;
}

interface CheckoutState {
  present: boolean;
  detail: string;
}

function readCheckoutState(repoPath: string): CheckoutState {
  const inWorktree = runGit(repoPath, ["rev-parse", "--is-inside-work-tree"]);
  if (inWorktree.status !== 0 || inWorktree.stdout.trim() !== "true") {
    return {
      present: true,
      detail:
        "No Git worktree metadata was available; adoption verdict uses local files only.",
    };
  }

  const upstream = runGit(repoPath, [
    "rev-parse",
    "--abbrev-ref",
    "--symbolic-full-name",
    "@{upstream}",
  ]);
  if (upstream.status !== 0 || upstream.stdout.trim().length === 0) {
    return {
      present: true,
      detail:
        "No upstream branch is configured; adoption verdict uses the local checkout.",
    };
  }

  const upstreamName = upstream.stdout.trim();
  const remoteName = upstreamName.split("/")[0] ?? "origin";
  const fetch = runGit(repoPath, ["fetch", "--quiet", remoteName], FETCH_TIMEOUT_MS);
  if (fetch.status !== 0) {
    return {
      present: false,
      detail: `Could not fetch ${remoteName} to verify checkout freshness; rescan with network access before trusting adoption state.`,
    };
  }

  const counts = runGit(repoPath, ["rev-list", "--left-right", "--count", "HEAD...@{upstream}"]);
  const [aheadRaw, behindRaw] = counts.stdout.trim().split(/\s+/);
  const ahead = Number.parseInt(aheadRaw ?? "", 10);
  const behind = Number.parseInt(behindRaw ?? "", 10);

  if (counts.status !== 0 || Number.isNaN(ahead) || Number.isNaN(behind)) {
    return {
      present: false,
      detail: `Could not compare checkout with ${upstreamName}; fetch and rescan before trusting adoption state.`,
    };
  }

  if (behind > 0) {
    const aheadDetail = ahead > 0 ? ` and ${ahead} commit(s) ahead` : "";
    return {
      present: false,
      detail: `Local checkout is ${behind} commit(s) behind ${upstreamName}${aheadDetail}; update it before trusting adoption state.`,
    };
  }

  if (ahead > 0) {
    return {
      present: true,
      detail: `Local checkout is ${ahead} commit(s) ahead of ${upstreamName} and not behind; scan includes local changes.`,
    };
  }

  return {
    present: true,
    detail: `Local checkout matches ${upstreamName}.`,
  };
}

const FETCH_TIMEOUT_MS = 30_000;

function runGit(
  repoPath: string,
  args: string[],
  timeoutMs?: number
): { status: number; stdout: string } {
  const result = spawnSync("git", ["-C", repoPath, ...args], {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
    timeout: timeoutMs,
  });

  return {
    status: result.status ?? 1,
    stdout: result.stdout,
  };
}

function isKnownProductionRepo(repoName: string): boolean {
  const context = REPO_CONTEXT[repoName];
  return Boolean(context?.deployedTo || context?.sentryProject);
}

function isProductionRuntime(
  repoName: string,
  tier: AdoptionTier,
  repositoryText: string
): boolean {
  if (isKnownProductionRepo(repoName)) return true;
  if (tier === "Tier1") return true;

  const lower = repositoryText.toLowerCase();
  return lower.includes("production") || lower.includes("cloud run");
}

function isTier3Repo(repo: RepoCandidate): boolean {
  const name = repo.name.toLowerCase();
  if (
    name.includes("template") ||
    name.includes("example") ||
    name.includes("archive")
  ) {
    return true;
  }

  const docs = [
    readFile(join(repo.path, "AGENTS.md")),
    readFile(join(repo.path, "README.md")),
  ]
    .join("\n")
    .toLowerCase();
  return docs.includes("archived") || docs.includes("experimental");
}

function adoptionState(tier: AdoptionTier, missingRequiredCount: number): AdoptionState {
  if (tier === "Tier3") return "exempt";
  if (missingRequiredCount === 0) return "adopted";
  if (tier === "Tier0" || tier === "Tier1") return "blocked";
  return "partial";
}

function findExceptionReference(
  text: string,
  keywords: string[]
): string | undefined {
  for (const line of text.split("\n")) {
    const lower = line.toLowerCase();
    const hasKeyword = keywords.some((keyword) =>
      lower.includes(keyword.toLowerCase())
    );
    const hasReference =
      /rfl-\d+/i.test(line) ||
      (lower.includes("accepted risk") && lower.includes("revisit"));

    if (hasKeyword && hasReference) {
      return line.trim().slice(0, 240);
    }
  }

  return undefined;
}

function collectMarkdown(repoPath: string): string {
  const files: string[] = [];

  for (const entry of safeReadDir(repoPath)) {
    if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(join(repoPath, entry.name));
    }
  }

  for (const directory of DOC_DIRECTORIES) {
    collectMarkdownFromDirectory(join(repoPath, directory), files, 0);
  }

  return files.map((filePath) => readFile(filePath)).join("\n");
}

function collectMarkdownFromDirectory(
  directory: string,
  files: string[],
  depth: number
): void {
  if (depth > 5 || !isDirectory(directory)) return;

  for (const entry of safeReadDir(directory)) {
    const entryPath = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (!SKIPPED_DIRECTORIES.has(entry.name)) {
        collectMarkdownFromDirectory(entryPath, files, depth + 1);
      }
      continue;
    }

    if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(entryPath);
    }
  }
}

function collectWorkflowFiles(repoPath: string): string[] {
  const workflowsDir = join(repoPath, ".github", "workflows");
  if (!isDirectory(workflowsDir)) return [];

  return safeReadDir(workflowsDir).flatMap((entry) => {
    if (!entry.isFile() || !/\.ya?ml$/i.test(entry.name)) return [];
    return [readFile(join(workflowsDir, entry.name))];
  });
}

function collectCargoConfigs(repoPath: string): Array<{ path: string; text: string }> {
  return [join(".cargo", "config.toml"), join(".cargo", "config")].flatMap(
    (path) => {
      const text = readFile(join(repoPath, path));
      return text.trim().length > 0 ? [{ path, text }] : [];
    }
  );
}

function collectNamedFiles(
  directory: string,
  fileName: string,
  depth: number
): Array<{ path: string; text: string }> {
  if (depth > 6 || !isDirectory(directory)) return [];

  const files: Array<{ path: string; text: string }> = [];
  for (const entry of safeReadDir(directory)) {
    const entryPath = join(directory, entry.name);
    if (entry.isDirectory()) {
      if (!SKIPPED_DIRECTORIES.has(entry.name)) {
        files.push(...collectNamedFiles(entryPath, fileName, depth + 1));
      }
      continue;
    }

    if (entry.isFile() && entry.name === fileName) {
      files.push({ path: entryPath, text: readFile(entryPath) });
    }
  }

  return files;
}

function readFile(path: string): string {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return "";
  }
}

function isDirectory(path: string): boolean {
  try {
    return statSync(path).isDirectory();
  } catch {
    return false;
  }
}

function safeReadDir(path: string): Dirent<string>[] {
  try {
    return readdirSync(path, { withFileTypes: true });
  } catch {
    return [];
  }
}

function readGitRevision(workspaceRoot: string): string {
  const headPath = join(workspaceRoot, ".git", "HEAD");
  const head = readFile(headPath).trim();
  if (!head) return "unknown";

  if (!head.startsWith("ref:")) return head;

  const ref = head.slice("ref:".length).trim();
  return readFile(join(workspaceRoot, ".git", ref)).trim() || ref;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

export function main(): void {
  const json = process.argv.includes("--json");
  const workspaceArgIndex = process.argv.indexOf("--workspace-root");
  const workspaceRoot =
    workspaceArgIndex >= 0 ? process.argv[workspaceArgIndex + 1] : undefined;

  if (workspaceArgIndex >= 0 && !workspaceRoot) {
    console.error("Usage: bun scripts/factory-adoption-doctor.ts [--json] [--workspace-root PATH]");
    process.exit(2);
  }

  const scan = scanWorkspace(workspaceRoot ? { workspaceRoot } : {});
  console.log(json ? JSON.stringify(scan, null, 2) : formatScan(scan));

  if (scan.blockingFailures > 0) {
    process.exit(1);
  }
}

if (import.meta.main) {
  main();
}
