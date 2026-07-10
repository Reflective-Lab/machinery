import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  REPO_CONTEXT,
  repositoryData,
  type RepositoryNodeData,
} from "../trigger.dev/repositories";
import {
  adrRef,
  factorySignalRef,
  findingRef,
  ownerRef,
  pullRequestRef,
  repositoryRef,
  rpPropertyRef,
  riskRef,
  standardRef,
  type ADRNodeData,
  type FactorySignalCategory,
  type FactorySignalNodeData,
  type FindingNodeData,
  type FindingStatus,
  type GraphRecord,
  type OwnerNodeData,
  type PullRequestNodeData,
  type RPPropertyNodeData,
  type RiskNodeData,
  type StandardNodeData,
} from "../trigger.dev/graph-records";

interface RecurringPropertiesFile {
  properties?: Array<{
    id?: unknown;
    property?: unknown;
    status?: unknown;
    tracked_by?: unknown;
  }>;
}

interface ParsedFinding {
  data: FindingNodeData;
  repoNames: string[];
  rpIds: string[];
  standardPaths: string[];
  pullRequests: PullRequestNodeData[];
  riskEntries: RiskNodeData[];
  adrRecords: ADRNodeData[];
  owner?: OwnerNodeData;
  signal: FactorySignalNodeData;
  supersedes: string[];
  supersededBy: string[];
}

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(SCRIPT_DIR, "..");
const WORKSPACE_ROOT = resolve(PROJECT_ROOT, "..");
const QUALITY_BACKLOG = resolveInputPath("QUALITY_BACKLOG.md", [
  join(PROJECT_ROOT, "QUALITY_BACKLOG.md"),
  join(WORKSPACE_ROOT, "QUALITY_BACKLOG.md"),
]);
const RP_JSON = resolveInputPath(
  "KB/05-engineering/standards/recurring-properties.json",
  [
    join(
      PROJECT_ROOT,
      "KB",
      "05-engineering",
      "standards",
      "recurring-properties.json"
    ),
    join(
      WORKSPACE_ROOT,
      "KB",
      "05-engineering",
      "standards",
      "recurring-properties.json"
    ),
  ]
);
const OUTPUT = join(PROJECT_ROOT, "seed", "seed.jsonl");

export function buildSeedRecords(): GraphRecord[] {
  const content = readFileSync(QUALITY_BACKLOG, "utf8");
  return buildSeedRecordsFromInputs(content, parseRpProperties());
}

export function buildSeedRecordsFromInputs(
  content: string,
  rpProperties: RPPropertyNodeData[] = parseRpProperties()
): GraphRecord[] {
  const parsedFindings = parseFindings(content);
  const findings = parsedFindings.map((finding) => finding.data);
  const standards = standardRecordsFromFindings(parsedFindings);
  const owners = ownerRecordsFromFindings(parsedFindings);
  const pullRequests = pullRequestRecordsFromFindings(parsedFindings);
  const risks = riskRecordsFromFindings(parsedFindings);
  const adrs = adrRecordsFromFindings(parsedFindings);
  const signals = signalRecordsFromFindings(parsedFindings);
  const relationRecords = relationRecordsFromFindings(
    parsedFindings,
    rpProperties
  );
  const repositories = Object.keys(REPO_CONTEXT).map((name) =>
    repositoryData(name, { ci_status: "Unknown" })
  );

  return [
    ...repositories.map((data) => ({ type: "Repository" as const, data })),
    ...rpProperties.map((data) => ({ type: "RPProperty" as const, data })),
    ...standards.map((data) => ({ type: "Standard" as const, data })),
    ...owners.map((data) => ({ type: "Owner" as const, data })),
    ...pullRequests.map((data) => ({ type: "PullRequest" as const, data })),
    ...risks.map((data) => ({ type: "Risk" as const, data })),
    ...adrs.map((data) => ({ type: "ADR" as const, data })),
    ...signals.map((data) => ({ type: "FactorySignal" as const, data })),
    ...findings.map((data) => ({ type: "Finding" as const, data })),
    ...relationRecords,
  ];
}

export function writeSeed(records: GraphRecord[] = buildSeedRecords()): void {
  mkdirSync(dirname(OUTPUT), { recursive: true });
  writeFileSync(
    OUTPUT,
    records.map((record) => JSON.stringify(record)).join("\n") + "\n"
  );
}

export function main(): void {
  const records = buildSeedRecords();
  writeSeed(records);

  const findingCount = records.filter((record) => record.type === "Finding").length;
  const rpCount = records.filter((record) => record.type === "RPProperty").length;
  const repoCount = records.filter((record) => record.type === "Repository").length;
  const standardCount = records.filter((record) => record.type === "Standard")
    .length;
  const edgeCount = records.filter((record) => "from" in record && "to" in record)
    .length;
  const openCount = records.filter(
    (record) => record.type === "Finding" && record.data.status === "Open"
  ).length;

  console.log(`Wrote ${records.length} records -> ${OUTPUT}`);
  console.log(`  ${repoCount} repositories`);
  console.log(
    `  ${findingCount} findings (${openCount} open, ${findingCount - openCount} closed)`
  );
  console.log(`  ${rpCount} RP-* properties`);
  console.log(`  ${standardCount} standards`);
  console.log(`  ${edgeCount} edges`);
}

function parseFindings(content: string): ParsedFinding[] {
  const parts = content.split(/^\#\#\#\# (QF-\d{4}-\d{2}-\d{2}-\d+)/m);
  const findings: ParsedFinding[] = [];

  for (let i = 1; i < parts.length; i += 2) {
    const qfId = parts[i];
    const block = parts[i + 1] ?? "";
    if (!qfId) continue;

    const finding = parseFindingBlock(qfId, block);
    if (finding) findings.push(finding);
  }

  return findings;
}

function parseFindingBlock(
  qfId: string,
  blockText: string
): ParsedFinding | undefined {
  const fields = new Map<string, string[]>();
  let currentKey: string | undefined;

  for (const line of blockText.split("\n")) {
    const field = line.match(/^- ([^:]+):\s*(.*)/);
    if (field) {
      const key = normalizeFieldKey(field[1] ?? "");
      const value = (field[2] ?? "").trim();
      currentKey = key;
      if (!fields.has(key)) fields.set(key, []);
      if (value) fields.get(key)?.push(value);
      continue;
    }

    if (line.startsWith("  ") && currentKey) {
      fields.get(currentKey)?.push(line.trim());
      continue;
    }

    if (/^#{2,}/.test(line)) {
      break;
    }
  }

  const get = (key: string): string => (fields.get(key) ?? []).join(" ").trim();
  const date = get("date");
  if (!date) return undefined;

  const area = firstLine(get("area"));
  const evidence = firstLine(get("evidence"));
  const title = evidence || area;
  const codexRaw = get("codex_safe_now") || get("codex_safe");
  const repoNames = parseRepoNames(blockText);
  const standardPaths = parseStandardPaths(get("standard_promoted"));
  const owner = ownerFromText(get("owner"));
  const pullRequests = parsePullRequests(get("linked_prs_commits"));
  const riskEntries = parseRiskEntries(
    get("risk_register_entry") || get("risk_register")
  );
  const adrRecords = parseAdrRecords(get("adr"));
  const supersession = parseSupersession(
    qfId,
    get("supersedes"),
    get("superseded_by"),
    get("supersedes_superseded_by")
  );
  const findingData = compactFinding({
    id: qfId,
    date,
    bucket: parseBucket(get("bucket")),
    area: area.slice(0, 120),
    status: parseStatus(get("status")),
    title: title.slice(0, 200),
    effort: parseEffort(get("effort")),
    owner: owner?.name ?? (firstLine(get("owner"), 100) || undefined),
    codex_safe: parseCodexSafe(codexRaw),
    confidence: parseConfidence(get("confidence")),
    cycles_open: parseCyclesOpen(get("cycles_open")),
    last_reviewed: firstLine(get("last_reviewed")).split("(")[0]?.trim().slice(0, 20),
    standard_promoted: standardPaths[0],
    drift_check: firstLine(get("drift_check"), 200) || undefined,
    next_action: firstLine(get("next_action"), 200) || undefined,
    repo: repoNames.length === 1 ? repoNames[0] : undefined,
  });

  const parsed: ParsedFinding = {
    data: findingData,
    repoNames,
    rpIds: parseRpIds(get("properties")),
    standardPaths,
    pullRequests,
    riskEntries,
    adrRecords,
    signal: signalFromFinding(qfId, findingData, {
      discoveredDuring: get("discovered_during"),
      evidence: get("evidence"),
      area: get("area"),
      repoNames,
    }),
    supersedes: supersession.supersedes,
    supersededBy: supersession.supersededBy,
  };

  if (owner) parsed.owner = owner;
  return parsed;
}

function parseRpProperties(): RPPropertyNodeData[] {
  const raw = JSON.parse(readFileSync(RP_JSON, "utf8")) as RecurringPropertiesFile;

  return (raw.properties ?? []).flatMap((property) => {
    if (typeof property.id !== "string") return [];

    const statusText = typeof property.status === "string" ? property.status : "";
    const trackedBy =
      typeof property.tracked_by === "string" ? property.tracked_by.trim() : "";

    return [
      compactRpProperty({
        id: property.id,
        property_name:
          typeof property.property === "string"
            ? property.property.slice(0, 200)
            : "",
        enforcement_status: statusText.startsWith("Enforced")
          ? "Enforced"
          : "Aspired",
        tracked_by: ["-", "--", "—", ""].includes(trackedBy) ? undefined : trackedBy,
      }),
    ];
  });
}

function resolveInputPath(label: string, candidates: string[]): string {
  const match = candidates.find((candidate) => existsSync(candidate));
  if (match) return match;

  throw new Error(
    `Missing seed input ${label}. Checked:\n${candidates
      .map((candidate) => `- ${candidate}`)
      .join("\n")}`
  );
}

function standardRecordsFromFindings(
  findings: ParsedFinding[]
): StandardNodeData[] {
  const standards = new Map<string, StandardNodeData>();

  for (const finding of findings) {
    for (const path of finding.standardPaths) {
      standards.set(path, {
        path,
        title: titleFromPath(path),
      });
    }
  }

  return [...standards.values()].sort((left, right) =>
    left.path.localeCompare(right.path)
  );
}

function ownerRecordsFromFindings(findings: ParsedFinding[]): OwnerNodeData[] {
  const owners = new Map<string, OwnerNodeData>();

  for (const finding of findings) {
    if (finding.owner) owners.set(finding.owner.id, finding.owner);
  }

  return [...owners.values()].sort((left, right) => left.id.localeCompare(right.id));
}

function pullRequestRecordsFromFindings(
  findings: ParsedFinding[]
): PullRequestNodeData[] {
  const pullRequests = new Map<string, PullRequestNodeData>();

  for (const finding of findings) {
    for (const pullRequest of finding.pullRequests) {
      pullRequests.set(pullRequest.id, pullRequest);
    }
  }

  return [...pullRequests.values()].sort((left, right) =>
    left.id.localeCompare(right.id)
  );
}

function riskRecordsFromFindings(findings: ParsedFinding[]): RiskNodeData[] {
  const risks = new Map<string, RiskNodeData>();

  for (const finding of findings) {
    for (const risk of finding.riskEntries) {
      risks.set(risk.id, risk);
    }
  }

  return [...risks.values()].sort((left, right) => left.id.localeCompare(right.id));
}

function adrRecordsFromFindings(findings: ParsedFinding[]): ADRNodeData[] {
  const adrs = new Map<string, ADRNodeData>();

  for (const finding of findings) {
    for (const adr of finding.adrRecords) {
      adrs.set(adr.path, adr);
    }
  }

  return [...adrs.values()].sort((left, right) =>
    left.path.localeCompare(right.path)
  );
}

function signalRecordsFromFindings(
  findings: ParsedFinding[]
): FactorySignalNodeData[] {
  return findings
    .map((finding) => finding.signal)
    .sort((left, right) => left.id.localeCompare(right.id));
}

function relationRecordsFromFindings(
  findings: ParsedFinding[],
  rpProperties: RPPropertyNodeData[]
): GraphRecord[] {
  const knownRpIds = new Set(rpProperties.map((property) => property.id));
  const edges: GraphRecord[] = [];
  const seen = new Set<string>();

  const push = (edge: GraphRecord): void => {
    if (!("from" in edge)) return;
    const key = JSON.stringify(edge);
    if (seen.has(key)) return;
    seen.add(key);
    edges.push(edge);
  };

  for (const finding of findings) {
    for (const repoName of finding.repoNames) {
      push({
        type: "FindingInRepo",
        from: findingRef(finding.data.id),
        to: repositoryRef(repoName),
      });
      push({
        type: "SignalInRepo",
        from: factorySignalRef(finding.signal.id),
        to: repositoryRef(repoName),
      });
    }

    for (const rpId of finding.rpIds.filter((id) => knownRpIds.has(id))) {
      push({
        type: "FindingWorksOnRP",
        from: findingRef(finding.data.id),
        to: rpPropertyRef(rpId),
      });
    }

    for (const path of finding.standardPaths) {
      push({
        type: "FindingToStandard",
        from: findingRef(finding.data.id),
        to: standardRef(path),
      });
    }

    if (finding.owner) {
      push({
        type: "OwnerOwnsFinding",
        from: ownerRef(finding.owner.id),
        to: findingRef(finding.data.id),
      });
    }

    for (const pullRequest of finding.pullRequests) {
      push({
        type: "PullRequestAddressesFinding",
        from: pullRequestRef(pullRequest.id),
        to: findingRef(finding.data.id),
      });
    }

    for (const risk of finding.riskEntries) {
      push({
        type: "RiskTracksFinding",
        from: riskRef(risk.id),
        to: findingRef(finding.data.id),
      });
    }

    for (const adr of finding.adrRecords) {
      push({
        type: "ADRDecidesFinding",
        from: adrRef(adr.path),
        to: findingRef(finding.data.id),
      });
    }

    push({
      type: "SignalSupportsFinding",
      from: factorySignalRef(finding.signal.id),
      to: findingRef(finding.data.id),
    });

    for (const target of finding.supersedes) {
      push({
        type: "FindingSupersedes",
        from: findingRef(finding.data.id),
        to: findingRef(target),
      });
    }

    for (const source of finding.supersededBy) {
      push({
        type: "FindingSupersedes",
        from: findingRef(source),
        to: findingRef(finding.data.id),
      });
    }
  }

  for (const property of rpProperties) {
    for (const findingId of parseQfIds(property.tracked_by ?? "")) {
      push({
        type: "FindingWorksOnRP",
        from: findingRef(findingId),
        to: rpPropertyRef(property.id),
      });
    }
  }

  return edges;
}

function normalizeFieldKey(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "");
}

function parseBucket(text: string): string {
  const trimmed = text.trim();
  return trimmed.match(/^([A-D])[.\s]/)?.[1] ?? trimmed.slice(0, 1);
}

function parseStatus(text: string): FindingStatus {
  const lower = text.trim().toLowerCase();
  if (lower.startsWith("done")) return "Done";
  if (lower.startsWith("in progress")) return "InProgress";
  if (lower.startsWith("superseded")) return "Superseded";
  if (lower.startsWith("accepted risk")) return "AcceptedRisk";
  if (lower.startsWith("won't do") || lower.startsWith("wont do")) return "WontDo";
  return "Open";
}

function parseEffort(text: string): string | undefined {
  const effort = text.trim().slice(0, 1).toUpperCase();
  return ["S", "M", "L"].includes(effort) ? effort : undefined;
}

function parseCodexSafe(text: string): string | undefined {
  const lower = text.trim().toLowerCase();
  if (lower.startsWith("yes")) return "Yes";
  if (lower.startsWith("no")) return "No";
  return undefined;
}

function parseConfidence(text: string): string | undefined {
  const confidence = text.trim().slice(0, 1).toUpperCase();
  return ["H", "M", "L"].includes(confidence) ? confidence : undefined;
}

function parseCyclesOpen(text: string): number | undefined {
  const value = Number.parseInt(text.trim(), 10);
  return Number.isFinite(value) ? value : undefined;
}

function parseRepoNames(text: string): string[] {
  return Object.keys(REPO_CONTEXT).filter((repoName) =>
    new RegExp(
      `(^|[^A-Za-z0-9_-])${escapeRegExp(repoName)}([^A-Za-z0-9_-]|$)`,
      "i"
    ).test(text)
  );
}

function ownerFromText(text: string): OwnerNodeData | undefined {
  const value = firstLine(text, 120)
    .replace(/\([^)]*\)/g, "")
    .replace(/\s+/g, " ")
    .trim();
  const lower = value.toLowerCase();
  if (
    !value ||
    lower === "tbd" ||
    lower === "n/a" ||
    lower === "none" ||
    value.startsWith("<")
  ) {
    return undefined;
  }

  const email = value.match(/[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}/i)?.[0];
  const name = value.replace(email ?? "", "").replace(/[;,].*$/, "").trim();
  const id = email?.toLowerCase() ?? stableId(name || value);
  if (!id) return undefined;

  const owner: OwnerNodeData = { id, name: name || email || value };
  if (email) owner.email = email.toLowerCase();
  return owner;
}

function parsePullRequests(text: string): PullRequestNodeData[] {
  if (isEmptyReference(text)) return [];

  const pullRequests = new Map<string, PullRequestNodeData>();
  const add = (repo: string, numberText: string, url?: string): void => {
    const number = Number.parseInt(numberText, 10);
    if (!repo || !Number.isFinite(number)) return;

    const data: PullRequestNodeData = {
      id: `${repo}#${number}`,
      repo,
      number,
    };
    if (url) data.url = url;
    pullRequests.set(data.id, data);
  };

  const githubUrlPattern =
    /https:\/\/github\.com\/[^/\s)]+\/([A-Za-z0-9._-]+)\/pull\/(\d+)/g;
  for (const match of text.matchAll(githubUrlPattern)) {
    const repo = match[1];
    const number = match[2];
    if (repo && number) add(repo, number, match[0]);
  }

  const shorthandPattern = /\b(?:[A-Za-z0-9._-]+\/)?([A-Za-z0-9._-]+)#(\d+)\b/g;
  for (const match of text.matchAll(shorthandPattern)) {
    const repo = match[1];
    const number = match[2];
    if (repo && number) add(repo, number);
  }

  const spacedPattern = /\b([A-Za-z0-9._-]+)\s+#(\d+)\b/g;
  for (const match of text.matchAll(spacedPattern)) {
    const repo = match[1];
    const number = match[2];
    if (repo && number) add(repo, number);
  }

  return [...pullRequests.values()].sort((left, right) =>
    left.id.localeCompare(right.id)
  );
}

function parseRiskEntries(text: string): RiskNodeData[] {
  if (isEmptyReference(text)) return [];

  const risks = new Map<string, RiskNodeData>();
  for (const path of parseMarkdownRefs(text)) {
    const risk: RiskNodeData = {
      id: path,
      title: titleFromPath(path.split("#")[0] ?? path),
    };
    const status = statusFromReferenceText(text);
    if (status) risk.status = status;
    risks.set(risk.id, risk);
  }

  for (const match of text.matchAll(/\bRR-\d{4}-\d{2}-\d{2}-\d+\b/g)) {
    const id = `KB/06-operations/risk-register.md#${match[0].toLowerCase()}`;
    const risk: RiskNodeData = {
      id,
      title: match[0],
    };
    const status = statusFromReferenceText(text);
    if (status) risk.status = status;
    risks.set(id, risk);
  }

  return [...risks.values()].sort((left, right) => left.id.localeCompare(right.id));
}

function parseAdrRecords(text: string): ADRNodeData[] {
  if (isEmptyReference(text)) return [];

  return parseMarkdownRefs(text)
    .map((path) => {
      const adr: ADRNodeData = {
        path,
        title: titleFromPath(path.split("#")[0] ?? path),
      };
      const status = statusFromReferenceText(text);
      if (status) adr.status = status;
      return adr;
    })
    .sort((left, right) => left.path.localeCompare(right.path));
}

function signalFromFinding(
  qfId: string,
  finding: FindingNodeData,
  input: {
    discoveredDuring: string;
    evidence: string;
    area: string;
    repoNames: string[];
  }
): FactorySignalNodeData {
  const source = "quality-backlog";
  const kind = input.discoveredDuring
    ? stableId(input.discoveredDuring).replaceAll("-", "_")
    : "finding_evidence";
  const text = `${input.area} ${input.discoveredDuring} ${input.evidence} ${finding.title}`;
  const signal: FactorySignalNodeData = {
    id: `${qfId}:evidence`,
    aggregate_key: qfId,
    category: signalCategory(text),
    source,
    kind,
    title: firstLine(input.evidence, 200) || finding.title,
    status: finding.status,
    severity: finding.bucket,
    observed_at: finding.date,
    finding_id: qfId,
    evidence_ref: `QUALITY_BACKLOG.md#${qfId.toLowerCase()}`,
  };
  const repo = input.repoNames[0];
  if (input.repoNames.length === 1 && repo) signal.repo = repo;
  return signal;
}

function parseRpIds(text: string): string[] {
  return unique(text.match(/\bRP-[A-Z0-9-]+\b/g) ?? []);
}

function parseQfIds(text: string): string[] {
  return unique(text.match(/\bQF-\d{4}-\d{2}-\d{2}-\d+\b/g) ?? []);
}

function parseStandardPaths(text: string): string[] {
  const lower = text.toLowerCase();
  if (
    lower.includes("n/a") ||
    lower.includes("pending") ||
    lower.includes("propose")
  ) {
    return [];
  }

  return parseMarkdownRefs(text);
}

function parseMarkdownRefs(text: string): string[] {
  if (isEmptyReference(text)) return [];

  const paths: string[] = [];
  const pattern =
    /`([^`]+\.md(?:#[A-Za-z0-9._-]+)?)`|((?:KB|docs|AGENTS|QUALITY_BACKLOG|README)[^\s),;*]+\.md(?:#[A-Za-z0-9._-]+)?)/g;
  for (const match of text.matchAll(pattern)) {
    const path = (match[1] ?? match[2])
      ?.trim()
      .replace(/[).,;:]+$/, "");
    if (path) paths.push(path);
  }

  return unique(paths);
}

function isEmptyReference(text: string): boolean {
  const lower = text.trim().toLowerCase();
  return (
    !lower ||
    lower === "n/a" ||
    lower === "na" ||
    lower === "none" ||
    lower === "optional" ||
    lower.startsWith("<optional") ||
    lower.includes("pending") ||
    lower.includes("created on first use")
  );
}

function statusFromReferenceText(text: string): string | undefined {
  const lower = text.toLowerCase();
  if (lower.includes("accepted")) return "Accepted";
  if (lower.includes("resolved") || lower.includes("closed")) return "Resolved";
  if (lower.includes("open")) return "Open";
  return undefined;
}

function signalCategory(text: string): FactorySignalCategory {
  const lower = text.toLowerCase();
  const matches = (needles: string[]): boolean =>
    needles.some((needle) => lower.includes(needle));

  if (
    matches([
      "dependabot",
      "dependency",
      "dependencies",
      "sbom",
      "license",
      "cargo audit",
      "bun audit",
      "advisory",
      "ghsa",
      "cve",
    ])
  ) {
    return "dependency";
  }
  if (
    matches([
      "security",
      "secret",
      "secrets",
      "sast",
      "vulnerability",
      "vulnerable",
      "pii",
      "audit-ignore",
      "deny.toml",
    ])
  ) {
    return "security_scan";
  }
  if (
    matches([
      "test",
      "tests",
      "flake",
      "flaky",
      "coverage",
      "hermetic",
      "snapshot",
      "trybuild",
      "insta",
    ])
  ) {
    return "test";
  }
  if (
    matches([
      "sentry",
      "incident",
      "crash",
      "latency",
      "error rate",
      "runtime",
      "log",
      "trace",
      "observability",
      "queue depth",
    ])
  ) {
    return "runtime_telemetry";
  }
  if (
    matches(["release", "deploy", "deployment", "rollback", "hotfix", "semver", "publish"])
  ) {
    return "delivery";
  }
  if (
    matches([
      "github",
      "pull request",
      "pr review",
      "branch",
      "commit",
      "stale branch",
      "workflow",
      "check suite",
    ])
  ) {
    return "github_lifecycle";
  }
  if (
    matches([
      "architecture",
      "adr",
      "boundary",
      "layering",
      "dependency direction",
      "ownership split",
    ])
  ) {
    return "architecture_drift";
  }
  if (
    matches([
      "agent",
      "codex",
      "claude",
      "gemini",
      "ai-factory",
      "ai factory",
      "unsupported claim",
    ])
  ) {
    return "agent_behavior";
  }
  if (
    matches([
      "support",
      "customer",
      "feedback",
      "funnel",
      "feature non-use",
      "product",
    ])
  ) {
    return "product_feedback";
  }
  if (matches(["backup", "restore", "migration", "retention", "privacy"])) {
    return "data_durability";
  }
  if (
    matches([
      "cost",
      "capacity",
      "ci minutes",
      "cloud spend",
      "runner-hour",
      "runner hour",
      "artifact size",
      "size budget",
    ])
  ) {
    return "cost_capacity";
  }
  if (
    matches([
      "adoption",
      "stale issue",
      "missing owner",
      "doctor",
      "hygiene",
      "ignored alert",
      "exception",
    ])
  ) {
    return "operational_hygiene";
  }

  return "quality_gate";
}

function parseSupersession(
  currentId: string,
  supersedesText: string,
  supersededByText: string,
  combinedText: string
): { supersedes: string[]; supersededBy: string[] } {
  const supersedes = parseQfIds(supersedesText).filter((id) => id !== currentId);
  const supersededBy = parseQfIds(supersededByText).filter(
    (id) => id !== currentId
  );
  const lower = combinedText.toLowerCase();

  if (lower.includes("superseded by")) {
    supersededBy.push(
      ...parseQfIds(combinedText).filter((id) => id !== currentId)
    );
  } else if (lower.includes("supersedes")) {
    supersedes.push(...parseQfIds(combinedText).filter((id) => id !== currentId));
  }

  return {
    supersedes: unique(supersedes),
    supersededBy: unique(supersededBy),
  };
}

function titleFromPath(path: string): string {
  return basename(path, ".md")
    .replaceAll("-", " ")
    .replace(/\b[a-z]/g, (letter) => letter.toUpperCase());
}

function unique(values: string[]): string[] {
  return [...new Set(values)];
}

function stableId(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

function firstLine(text: string, maxLength = 150): string {
  const line = text.split("\n")[0]?.trim() ?? "";
  return line.length > maxLength ? line.slice(0, maxLength) : line;
}

function compactFinding(data: {
  id: string;
  date: string;
  bucket: string;
  area: string;
  status: FindingStatus;
  title: string;
  effort?: string | undefined;
  owner?: string | undefined;
  codex_safe?: string | undefined;
  confidence?: string | undefined;
  cycles_open?: number | undefined;
  last_reviewed?: string | undefined;
  standard_promoted?: string | undefined;
  drift_check?: string | undefined;
  next_action?: string | undefined;
  repo?: string | undefined;
}): FindingNodeData {
  const finding: FindingNodeData = {
    id: data.id,
    date: data.date,
    bucket: data.bucket,
    area: data.area,
    status: data.status,
    title: data.title,
  };

  if (data.effort) finding.effort = data.effort;
  if (data.owner) finding.owner = data.owner;
  if (data.codex_safe) finding.codex_safe = data.codex_safe;
  if (data.confidence) finding.confidence = data.confidence;
  if (data.cycles_open !== undefined) finding.cycles_open = data.cycles_open;
  if (data.last_reviewed) finding.last_reviewed = data.last_reviewed;
  if (data.standard_promoted) finding.standard_promoted = data.standard_promoted;
  if (data.drift_check) finding.drift_check = data.drift_check;
  if (data.next_action) finding.next_action = data.next_action;
  if (data.repo) finding.repo = data.repo;

  return finding;
}

function compactRpProperty(data: {
  id: string;
  property_name: string;
  enforcement_status: "Enforced" | "Aspired";
  tracked_by?: string | undefined;
}): RPPropertyNodeData {
  const property: RPPropertyNodeData = {
    id: data.id,
    property_name: data.property_name,
    enforcement_status: data.enforcement_status,
  };

  if (data.tracked_by) property.tracked_by = data.tracked_by;

  return property;
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

if (import.meta.main) {
  main();
}
