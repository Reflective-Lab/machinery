import { logger, task } from "@trigger.dev/sdk";
import { z } from "zod";
import {
  repoFromLinearLabels,
  repoFromSentryProject,
  repositoryData,
  type RepositoryNodeData,
} from "./repositories";
import {
  checkRunRef,
  deploymentRef,
  factorySignalRef,
  findingRef,
  incidentRef,
  ownerRef,
  pullRequestRef,
  recordsToNdjson as serializeRecordsToNdjson,
  repositoryRef,
  type CheckRunNodeData,
  type DeploymentNodeData,
  type FactorySignalCategory,
  type FactorySignalNodeData,
  type FindingNodeData,
  type FindingStatus,
  type GraphRecord,
  type IncidentNodeData,
  type IncidentSeverity,
  type IncidentStatus,
  type OwnerNodeData,
  type PullRequestNodeData,
} from "./graph-records";

export { recordsToNdjson } from "./graph-records";

export interface NormalizedDebtEvent {
  source: "github" | "linear" | "sentry" | "runtime-runway" | "unknown";
  action?: string;
  records: GraphRecord[];
  skippedReason?: string;
}

interface DeliveryResult {
  delivered: boolean;
  reason?: string;
  status?: number;
}

const NonEmptyString = z.string().min(1);
const NullableString = z.string().nullable().optional();
const DateString = z.string().datetime({ offset: true }).or(z.string());
const MaybeDateString = DateString.optional();
const MaybeNumber = z.union([z.number(), z.string()]).optional();

const FactorySignalCategorySchema = z.enum([
  "quality_gate",
  "security_scan",
  "dependency",
  "test",
  "github_lifecycle",
  "runtime_telemetry",
  "product_feedback",
  "architecture_drift",
  "agent_behavior",
  "operational_hygiene",
  "data_durability",
  "cost_capacity",
  "delivery",
  "repository_adoption",
]);

const GitHubRepositorySchema = z
  .object({
    name: NonEmptyString,
    full_name: z.string().optional(),
    owner: z
      .object({
        login: z.string().optional(),
      })
      .passthrough()
      .optional(),
  })
  .passthrough();

const GitHubCheckRunSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    name: NullableString,
    status: NullableString,
    conclusion: NullableString,
    html_url: NullableString,
    details_url: NullableString,
    completed_at: MaybeDateString,
    started_at: MaybeDateString,
    head_sha: NullableString,
  })
  .passthrough();

const GitHubCheckSuiteSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    status: NullableString,
    conclusion: NullableString,
    html_url: NullableString,
    check_runs_url: NullableString,
    updated_at: MaybeDateString,
    head_sha: NullableString,
  })
  .passthrough();

const GitHubReleaseSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    tag_name: NullableString,
    name: NullableString,
    html_url: NullableString,
    published_at: MaybeDateString,
    created_at: MaybeDateString,
  })
  .passthrough();

const GitHubPullRequestSchema = z
  .object({
    number: z.number().optional(),
    title: NullableString,
    body: NullableString,
    html_url: NullableString,
    state: NullableString,
    head: z
      .object({
        sha: NullableString,
        ref: NullableString,
      })
      .passthrough()
      .optional(),
  })
  .passthrough();

const GitHubDeploymentSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    environment: NullableString,
    ref: NullableString,
    sha: NullableString,
    task: NullableString,
    created_at: MaybeDateString,
    updated_at: MaybeDateString,
    statuses_url: NullableString,
  })
  .passthrough();

const GitHubDeploymentStatusSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    state: NullableString,
    environment: NullableString,
    target_url: NullableString,
    created_at: MaybeDateString,
    updated_at: MaybeDateString,
  })
  .passthrough();

const GitHubPayloadSchema = z
  .object({
    action: z.string().optional(),
    number: z.number().optional(),
    repository: GitHubRepositorySchema,
    check_run: GitHubCheckRunSchema.optional(),
    check_suite: GitHubCheckSuiteSchema.optional(),
    release: GitHubReleaseSchema.optional(),
    pull_request: GitHubPullRequestSchema.optional(),
    deployment: GitHubDeploymentSchema.optional(),
    deployment_status: GitHubDeploymentStatusSchema.optional(),
    commits: z.unknown().optional(),
  })
  .passthrough();

const LinearLabelSchema = z.union([
  z.string(),
  z.object({ name: z.string() }).passthrough(),
]);

const LinearIssueSchema = z
  .object({
    id: NonEmptyString,
    identifier: z.string().optional(),
    title: z.string().optional(),
    description: z.string().nullable().optional(),
    url: z.string().optional(),
    createdAt: MaybeDateString,
    updatedAt: MaybeDateString,
    priority: z.unknown().optional(),
    labels: z.array(LinearLabelSchema).optional(),
    state: z
      .object({
        name: z.string().optional(),
        type: z.string().optional(),
      })
      .passthrough()
      .nullable()
      .optional(),
    assignee: z
      .object({
        id: z.string().optional(),
        name: z.string().optional(),
        email: z.string().optional(),
      })
      .passthrough()
      .nullable()
      .optional(),
  })
  .passthrough();

const LinearPayloadSchema = z
  .object({
    action: z.string().optional(),
    type: z.string().optional(),
    data: LinearIssueSchema,
  })
  .passthrough();

const SentryProjectSchema = z.union([
  z.string(),
  z.object({ slug: z.string().optional(), name: z.string().optional() }).passthrough(),
]);

const SentryIssueSchema = z
  .object({
    id: z.union([z.string(), z.number()]).optional(),
    shortId: z.string().optional(),
    short_id: z.string().optional(),
    title: z.string().optional(),
    culprit: z.string().optional(),
    level: z.string().optional(),
    status: z.string().optional(),
    firstSeen: MaybeDateString,
    first_seen: MaybeDateString,
    lastSeen: MaybeDateString,
    last_seen: MaybeDateString,
    dateCreated: MaybeDateString,
    count: MaybeNumber,
    eventCount: MaybeNumber,
    userCount: MaybeNumber,
    user_count: MaybeNumber,
    permalink: NullableString,
    web_url: NullableString,
    project: SentryProjectSchema.optional(),
  })
  .passthrough();

const SentryPayloadSchema = z
  .object({
    action: z.string().optional(),
    data: z
      .object({
        issue: SentryIssueSchema.optional(),
        event: z.unknown().optional(),
      })
      .passthrough()
      .optional(),
    issue: SentryIssueSchema.optional(),
    project: SentryProjectSchema.optional(),
  })
  .passthrough();

const RuntimeIncidentSummarySchema = z
  .object({
    incident_id: NonEmptyString.optional(),
    title: NonEmptyString.optional(),
    severity: z.enum(["P1", "P2", "P3"]).optional(),
    status: z.enum(["Open", "Resolved", "Ignored"]).optional(),
    opened_at: MaybeDateString,
    linked_finding: NonEmptyString.optional(),
    first_seen_at: MaybeDateString,
    last_seen_at: MaybeDateString,
    event_count: MaybeNumber,
    affected_users: MaybeNumber,
    external_url: z.string().url().optional(),
    evidence_ref: NonEmptyString.optional(),
  })
  .strict();

const RuntimeFactorySignalSummarySchema = z
  .object({
    signal_id: NonEmptyString.optional(),
    aggregate_key: NonEmptyString.optional(),
    category: FactorySignalCategorySchema.optional(),
    kind: NonEmptyString,
    title: NonEmptyString,
    status: z.string().optional(),
    severity: z.string().optional(),
    observed_at: MaybeDateString,
    first_seen_at: MaybeDateString,
    last_seen_at: MaybeDateString,
    event_count: MaybeNumber,
    affected_users: MaybeNumber,
    finding_id: NonEmptyString.optional(),
    external_url: z.string().url().optional(),
    evidence_ref: NonEmptyString.optional(),
    metric_name: NonEmptyString.optional(),
    metric_value: MaybeNumber,
    unit: NonEmptyString.optional(),
  })
  .strict();

const RuntimeDeploySummarySchema = z
  .object({
    type: z.literal("runtime.deploy.summary"),
    schema_version: z.literal("1"),
    source: z.literal("runtime-runway"),
    repo: NonEmptyString,
    service: NonEmptyString,
    app_id: NonEmptyString.optional(),
    environment: NonEmptyString,
    region: NonEmptyString.optional(),
    deployment_id: NonEmptyString.optional(),
    version: NonEmptyString.optional(),
    commit_sha: NonEmptyString.optional(),
    image_digest: NonEmptyString.optional(),
    status: NonEmptyString,
    occurred_at: DateString,
    duration_ms: MaybeNumber,
    external_url: z.string().url().optional(),
    evidence_ref: NonEmptyString.optional(),
    title: NonEmptyString.optional(),
    incident: RuntimeIncidentSummarySchema.optional(),
    signals: z.array(RuntimeFactorySignalSummarySchema).max(50).optional(),
  })
  .strict();

const SinkEnvSchema = z.object({
  OMNIGRAPH_INGEST_URL: z.string().url().optional(),
  OMNIGRAPH_INGEST_TOKEN: z.string().optional(),
  OMNIGRAPH_GRAPH: z.string().default("build-depot"),
});

export const debtTrackerTask = task({
  id: "debt-tracker",
  maxDuration: 300,
  run: async (payload: unknown) => {
    const normalized = normalizeDebtPayload(payload);

    if (normalized.records.length === 0) {
      logger.log("debt-tracker: skipped", {
        source: normalized.source,
        reason: normalized.skippedReason,
      });
      return { ...normalized, skipped: true };
    }

    const delivery = await deliverGraphRecords(normalized.records);
    logger.log("debt-tracker: normalized", {
      source: normalized.source,
      action: normalized.action,
      records: normalized.records.length,
      delivered: delivery.delivered,
      deliveryReason: delivery.reason,
    });

    return {
      ...normalized,
      recordCount: normalized.records.length,
      delivery,
    };
  },
});

export function normalizeDebtPayload(
  payload: unknown,
  now: Date = new Date()
): NormalizedDebtEvent {
  const github = GitHubPayloadSchema.safeParse(payload);
  if (github.success) {
    return normalizeGitHub(github.data, now);
  }

  const linear = LinearPayloadSchema.safeParse(payload);
  if (linear.success && linear.data.type === "Issue") {
    return normalizeLinear(linear.data, now);
  }

  const sentry = SentryPayloadSchema.safeParse(payload);
  if (sentry.success && hasSentryIssue(sentry.data)) {
    return normalizeSentry(sentry.data, now);
  }

  const runtime = RuntimeDeploySummarySchema.safeParse(payload);
  if (runtime.success) {
    return normalizeRuntimeDeploySummary(runtime.data, now);
  }

  return normalizedEvent({
    source: "unknown",
    records: [],
    skippedReason: "Unsupported webhook payload shape",
  });
}

export async function deliverGraphRecords(
  records: GraphRecord[],
  env: NodeJS.ProcessEnv = process.env
): Promise<DeliveryResult> {
  const config = SinkEnvSchema.parse(env);

  if (!config.OMNIGRAPH_INGEST_URL) {
    return {
      delivered: false,
      reason: "OMNIGRAPH_INGEST_URL not configured",
    };
  }

  const headers = new Headers({
    "Content-Type": "application/x-ndjson",
    "X-Omnigraph-Graph": config.OMNIGRAPH_GRAPH,
  });
  if (config.OMNIGRAPH_INGEST_TOKEN) {
    headers.set("Authorization", `Bearer ${config.OMNIGRAPH_INGEST_TOKEN}`);
  }

  const response = await fetch(config.OMNIGRAPH_INGEST_URL, {
    method: "POST",
    headers,
    body: serializeRecordsToNdjson(records),
  });

  if (!response.ok) {
    const body = await response.text();
    throw new Error(
      `Omnigraph ingest failed ${response.status}: ${body.slice(0, 500)}`
    );
  }

  return { delivered: true, status: response.status };
}

function normalizeGitHub(
  payload: z.infer<typeof GitHubPayloadSchema>,
  now: Date
): NormalizedDebtEvent {
  const patch: Partial<RepositoryNodeData> = {};
  const repo = payload.repository.name;
  const records: GraphRecord[] = [];

  if (payload.check_run) {
    patch.ci_status = ciStatus(payload.check_run.status, payload.check_run.conclusion);
  } else if (payload.check_suite) {
    patch.ci_status = ciStatus(
      payload.check_suite.status,
      payload.check_suite.conclusion
    );
  } else if (payload.release?.tag_name) {
    patch.last_release = payload.release.tag_name;
  }

  records.push(repositoryRecord(repo, patch));

  if (payload.check_run) {
    const checkRun = checkRunFromGitHub(repo, "check-run", payload.check_run, now);
    records.push(
      { type: "CheckRun", data: checkRun },
      {
        type: "CheckRunInRepo",
        from: checkRunRef(checkRun.id),
        to: repositoryRef(repo),
      }
    );
    appendSignal(records, githubCheckSignal(repo, checkRun, payload.check_run, now));
  }

  if (payload.check_suite) {
    const checkRun = checkRunFromGitHub(repo, "check-suite", payload.check_suite, now);
    records.push(
      { type: "CheckRun", data: checkRun },
      {
        type: "CheckRunInRepo",
        from: checkRunRef(checkRun.id),
        to: repositoryRef(repo),
      }
    );
    appendSignal(records, githubCheckSignal(repo, checkRun, payload.check_suite, now));
  }

  if (payload.release?.tag_name) {
    const deployment = deploymentFromRelease(repo, payload.release, payload.action, now);
    records.push(
      { type: "Deployment", data: deployment },
      {
        type: "DeploymentInRepo",
        from: deploymentRef(deployment.id),
        to: repositoryRef(repo),
      }
    );
    appendSignal(records, githubDeploymentSignal(repo, deployment, payload.release, now));
  }

  if (payload.deployment || payload.deployment_status) {
    const deployment = deploymentFromGitHub(
      repo,
      payload.deployment,
      payload.deployment_status,
      payload.action,
      now
    );
    records.push(
      { type: "Deployment", data: deployment },
      {
        type: "DeploymentInRepo",
        from: deploymentRef(deployment.id),
        to: repositoryRef(repo),
      }
    );
    appendSignal(
      records,
      githubDeploymentStatusSignal(repo, deployment, payload.deployment_status, now)
    );
  }

  if (payload.pull_request) {
    const pullRequest = pullRequestFromGitHub(repo, payload.pull_request, payload.number);
    if (pullRequest) {
      const linkedFinding = extractQfId(
        payload.pull_request.title,
        payload.pull_request.body,
        payload.pull_request.head?.ref
      );
      records.push({ type: "PullRequest", data: pullRequest });
      if (linkedFinding) {
        records.push({
          type: "PullRequestAddressesFinding",
          from: pullRequestRef(pullRequest.id),
          to: findingRef(linkedFinding),
        });
      }
      appendSignal(
        records,
        githubPullRequestSignal(repo, pullRequest, payload.pull_request, linkedFinding, now)
      );
    }
  }

  return normalizedEvent({
    source: "github",
    action: payload.action ?? githubPayloadKind(payload),
    records,
  });
}

function normalizeLinear(
  payload: z.infer<typeof LinearPayloadSchema>,
  now: Date
): NormalizedDebtEvent {
  const issue = payload.data;
  const labels = linearLabels(issue.labels ?? []);
  const qfId = extractQfId(issue.identifier, issue.title, issue.description);

  if (!qfId && !isFactoryFinding(labels, issue.title, issue.description)) {
    return normalizedEvent({
      source: "linear",
      action: payload.action,
      records: [],
      skippedReason: "Linear issue is not labeled as factory debt",
    });
  }

  const repo = repoFromLinearLabels(labels);
  const findingId = qfId ?? issue.identifier ?? issue.id;
  const owner = ownerFromLinearAssignee(issue.assignee);
  const records: GraphRecord[] = [
    {
      type: "Finding",
      data: compactFinding({
        id: findingId,
        date: dateOnly(issue.createdAt, now),
        bucket: bucketFromLinear(issue.priority, labels),
        area: areaFromLabels(labels),
        status: statusFromLinear(issue.state?.type, issue.state?.name),
        title: truncate(issue.title ?? issue.identifier ?? issue.id, 200),
        effort: effortFromLabels(labels),
        owner: issue.assignee?.name,
        codex_safe: codexSafeFromLabels(labels),
        confidence: confidenceFromLabels(labels),
        next_action: nextAction(issue.description),
        repo,
      }),
    },
  ];

  if (repo) {
    records.unshift(repositoryRecord(repo));
    records.push({
      type: "FindingInRepo",
      from: findingRef(findingId),
      to: repositoryRef(repo),
    });
  }

  if (owner) {
    records.push(
      { type: "Owner", data: owner },
      {
        type: "OwnerOwnsFinding",
        from: ownerRef(owner.id),
        to: findingRef(findingId),
      }
    );
  }

  appendSignal(
    records,
    linearIssueSignal(issue, labels, findingId, repo, payload.action, now)
  );

  return normalizedEvent({
    source: "linear",
    action: payload.action,
    records,
  });
}

function normalizeSentry(
  payload: z.infer<typeof SentryPayloadSchema>,
  now: Date
): NormalizedDebtEvent {
  const issue = payload.data?.issue ?? payload.issue;
  if (!issue) {
    return normalizedEvent({
      source: "sentry",
      action: payload.action,
      records: [],
      skippedReason: "Sentry payload did not include an issue",
    });
  }

  const project = projectSlug(issue.project ?? payload.project);
  const repo = project ? repoFromSentryProject(project) ?? project : undefined;
  const id = String(issue.id ?? issue.shortId ?? issue.short_id ?? "");

  if (!repo || !id) {
    return normalizedEvent({
      source: "sentry",
      action: payload.action,
      records: [],
      skippedReason: "Sentry issue is missing repo or id",
    });
  }

  const title = issue.title ?? issue.culprit;
  const linkedFinding = extractQfId(title, issue.culprit);
  const incident = compactIncident({
    id,
    repo,
    title: title ? truncate(title, 200) : undefined,
    severity: severityFromSentry(issue.level),
    status: statusFromSentry(issue.status),
    opened_at: issue.firstSeen ?? issue.first_seen ?? issue.dateCreated ?? now.toISOString(),
    linked_finding: linkedFinding,
  });
  const records: GraphRecord[] = [
    repositoryRecord(repo),
    {
      type: "Incident",
      data: incident,
    },
    {
      type: "IncidentInRepo",
      from: incidentRef(id),
      to: repositoryRef(repo),
    },
  ];
  appendSignal(records, sentryIssueSignal(issue, incident, linkedFinding, now));

  return normalizedEvent({
    source: "sentry",
    action: payload.action,
    records,
  });
}

function normalizeRuntimeDeploySummary(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  now: Date
): NormalizedDebtEvent {
  const repo = payload.repo;
  const deployment = deploymentFromRuntimeSummary(payload, now);
  const records: GraphRecord[] = [
    repositoryRecord(repo),
    {
      type: "Deployment",
      data: deployment,
    },
    {
      type: "DeploymentInRepo",
      from: deploymentRef(deployment.id),
      to: repositoryRef(repo),
    },
  ];

  appendSignal(records, runtimeDeploymentSignal(payload, deployment, now));

  if (payload.incident) {
    const incident = incidentFromRuntimeSummary(payload, deployment, now);
    records.push(
      {
        type: "Incident",
        data: incident,
      },
      {
        type: "IncidentInRepo",
        from: incidentRef(incident.id),
        to: repositoryRef(repo),
      }
    );
    appendSignal(
      records,
      runtimeIncidentSignal(payload, payload.incident, incident, deployment, now)
    );
  }

  for (const [index, signal] of (payload.signals ?? []).entries()) {
    appendSignal(records, runtimeFactorySignal(payload, signal, deployment, index, now));
  }

  return normalizedEvent({
    source: "runtime-runway",
    action: payload.status,
    records,
  });
}

function normalizedEvent(input: {
  source: NormalizedDebtEvent["source"];
  action?: string | undefined;
  records: GraphRecord[];
  skippedReason?: string | undefined;
}): NormalizedDebtEvent {
  const event: NormalizedDebtEvent = {
    source: input.source,
    records: input.records,
  };

  if (input.action) event.action = input.action;
  if (input.skippedReason) event.skippedReason = input.skippedReason;

  return event;
}

function repositoryRecord(
  repo: string,
  patch: Partial<RepositoryNodeData> = {}
): GraphRecord {
  return { type: "Repository", data: repositoryData(repo, patch) };
}

function checkRunFromGitHub(
  repo: string,
  kind: "check-run" | "check-suite",
  check: z.infer<typeof GitHubCheckRunSchema> | z.infer<typeof GitHubCheckSuiteSchema>,
  now: Date
): CheckRunNodeData {
  const providerId = String(check.id ?? check.head_sha ?? now.toISOString());
  const name =
    optionalStringFromUnknown("name" in check ? check.name : undefined) ??
    (kind === "check-suite" ? "check suite" : "check run");
  return compactCheckRun({
    id: `github:${repo}:${kind}:${providerId}`,
    repo,
    name,
    status: optionalStringFromUnknown(check.status),
    conclusion: optionalStringFromUnknown(check.conclusion),
    completed_at:
      optionalStringFromUnknown("completed_at" in check ? check.completed_at : undefined) ??
      optionalStringFromUnknown("updated_at" in check ? check.updated_at : undefined) ??
      now.toISOString(),
  });
}

function deploymentFromRelease(
  repo: string,
  release: z.infer<typeof GitHubReleaseSchema>,
  action: string | undefined,
  now: Date
): DeploymentNodeData {
  const version = optionalString(release.tag_name) ?? "unknown";
  return compactDeployment({
    id: `github:${repo}:release:${String(release.id ?? version)}`,
    repo,
    environment: "release",
    status: action ?? "published",
    version,
    deployed_at: release.published_at ?? release.created_at ?? now.toISOString(),
  });
}

function deploymentFromGitHub(
  repo: string,
  deployment: z.infer<typeof GitHubDeploymentSchema> | undefined,
  deploymentStatus: z.infer<typeof GitHubDeploymentStatusSchema> | undefined,
  action: string | undefined,
  now: Date
): DeploymentNodeData {
  const providerId = String(
    deployment?.id ??
      deploymentStatus?.id ??
      deployment?.sha ??
      deployment?.ref ??
      now.toISOString()
  );
  const environment =
    optionalString(deploymentStatus?.environment) ??
    optionalString(deployment?.environment);
  return compactDeployment({
    id: `github:${repo}:deployment:${providerId}`,
    repo,
    environment,
    status: optionalString(deploymentStatus?.state) ?? action,
    version: optionalString(deployment?.sha) ?? optionalString(deployment?.ref),
    deployed_at:
      deploymentStatus?.updated_at ??
      deploymentStatus?.created_at ??
      deployment?.updated_at ??
      deployment?.created_at ??
      now.toISOString(),
  });
}

function deploymentFromRuntimeSummary(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  now: Date
): DeploymentNodeData {
  return compactDeployment({
    id: runtimeDeploymentGraphId(payload, now),
    repo: payload.repo,
    environment: payload.environment,
    status: payload.status,
    version: payload.version ?? payload.commit_sha ?? payload.image_digest,
    deployed_at: payload.occurred_at || now.toISOString(),
  });
}

function runtimeDeploymentGraphId(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  now: Date
): string {
  const providerId =
    payload.deployment_id ??
    stableId(
      [
        payload.service,
        payload.environment,
        payload.version ?? payload.commit_sha ?? payload.image_digest,
        payload.occurred_at,
      ]
        .filter(Boolean)
        .join(":")
    ) ??
    now.toISOString();

  return `runtime:${payload.repo}:deployment:${providerId}`;
}

function incidentFromRuntimeSummary(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  deployment: DeploymentNodeData,
  now: Date
): IncidentNodeData {
  const summary = payload.incident;
  if (!summary) {
    throw new Error("Runtime incident summary is missing");
  }

  const incidentProviderId =
    summary.incident_id ??
    stableId(`${payload.service}:${payload.environment}:${payload.occurred_at}`);
  const linkedFinding =
    summary.linked_finding ?? extractQfId(summary.title, payload.title, payload.evidence_ref);

  return compactIncident({
    id: `runtime:${payload.repo}:incident:${incidentProviderId || deployment.id}`,
    repo: payload.repo,
    title: summary.title ?? `Runtime incident for ${payload.service}`,
    severity: summary.severity ?? "P3",
    status: summary.status ?? "Open",
    opened_at:
      summary.opened_at ??
      summary.first_seen_at ??
      payload.occurred_at ??
      deployment.deployed_at ??
      now.toISOString(),
    linked_finding: linkedFinding,
  });
}

function pullRequestFromGitHub(
  repo: string,
  pullRequest: z.infer<typeof GitHubPullRequestSchema>,
  payloadNumber: number | undefined
): PullRequestNodeData | undefined {
  const number = pullRequest.number ?? payloadNumber;
  if (number === undefined) return undefined;

  return compactPullRequest({
    id: `${repo}#${number}`,
    repo,
    number,
    title: optionalString(pullRequest.title),
    url: optionalString(pullRequest.html_url),
    state: optionalString(pullRequest.state),
    head_sha: optionalString(pullRequest.head?.sha),
  });
}

function githubCheckSignal(
  repo: string,
  checkRun: CheckRunNodeData,
  check: z.infer<typeof GitHubCheckRunSchema> | z.infer<typeof GitHubCheckSuiteSchema>,
  now: Date
): FactorySignalNodeData {
  const status = checkRun.conclusion ?? checkRun.status;
  return compactFactorySignal({
    id: `${checkRun.id}:signal`,
    aggregate_key: checkRun.id,
    category: signalCategoryFromText(`${checkRun.name ?? ""} ${status ?? ""}`),
    source: "github",
    kind: checkRun.id.includes(":check-suite:") ? "check_suite" : "check_run",
    title: `${checkRun.name ?? "GitHub check"} ${status ?? "updated"}`,
    status,
    observed_at: checkRun.completed_at ?? now.toISOString(),
    repo,
    external_url:
      optionalStringFromUnknown("html_url" in check ? check.html_url : undefined) ??
      optionalStringFromUnknown("details_url" in check ? check.details_url : undefined) ??
      optionalStringFromUnknown("check_runs_url" in check ? check.check_runs_url : undefined),
  });
}

function githubDeploymentSignal(
  repo: string,
  deployment: DeploymentNodeData,
  release: z.infer<typeof GitHubReleaseSchema>,
  now: Date
): FactorySignalNodeData {
  return compactFactorySignal({
    id: `${deployment.id}:signal`,
    aggregate_key: deployment.id,
    category: "delivery",
    source: "github",
    kind: "release",
    title: `Release ${deployment.version ?? "updated"}`,
    status: deployment.status,
    observed_at: deployment.deployed_at ?? now.toISOString(),
    repo,
    external_url: optionalString(release.html_url),
  });
}

function githubDeploymentStatusSignal(
  repo: string,
  deployment: DeploymentNodeData,
  deploymentStatus: z.infer<typeof GitHubDeploymentStatusSchema> | undefined,
  now: Date
): FactorySignalNodeData {
  return compactFactorySignal({
    id: `${deployment.id}:status-signal`,
    aggregate_key: deployment.id,
    category: "delivery",
    source: "github",
    kind: "deployment_status",
    title: `Deployment ${deployment.environment ?? "default"} ${deployment.status ?? "updated"}`,
    status: deployment.status,
    observed_at: deployment.deployed_at ?? now.toISOString(),
    repo,
    external_url: optionalString(deploymentStatus?.target_url),
  });
}

function runtimeDeploymentSignal(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  deployment: DeploymentNodeData,
  now: Date
): FactorySignalNodeData {
  const durationMs = finiteNumberFromValue(payload.duration_ms);

  return compactFactorySignal({
    id: `${deployment.id}:summary-signal`,
    aggregate_key: deployment.id,
    category: "delivery",
    source: "runtime-runway",
    kind: "deploy_summary",
    title:
      payload.title ??
      `Runtime deploy ${payload.service} ${payload.environment} ${payload.status}`,
    status: payload.status,
    observed_at: payload.occurred_at || now.toISOString(),
    repo: payload.repo,
    external_url: payload.external_url,
    evidence_ref: payload.evidence_ref,
    metric_name: durationMs === undefined ? undefined : "deployment.duration",
    metric_value: durationMs,
    unit: durationMs === undefined ? undefined : "ms",
  });
}

function runtimeIncidentSignal(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  summary: z.infer<typeof RuntimeIncidentSummarySchema>,
  incident: IncidentNodeData,
  deployment: DeploymentNodeData,
  now: Date
): FactorySignalNodeData {
  return compactFactorySignal({
    id: `${incident.id}:signal`,
    aggregate_key: deployment.id,
    category: "runtime_telemetry",
    source: "runtime-runway",
    kind: "incident_summary",
    title: incident.title ?? `Runtime incident for ${payload.service}`,
    status: incident.status,
    severity: incident.severity,
    observed_at:
      summary.last_seen_at ??
      summary.opened_at ??
      summary.first_seen_at ??
      payload.occurred_at ??
      now.toISOString(),
    first_seen_at: summary.first_seen_at ?? summary.opened_at,
    last_seen_at: summary.last_seen_at,
    event_count: numberFromValue(summary.event_count),
    affected_users: numberFromValue(summary.affected_users),
    repo: payload.repo,
    finding_id: incident.linked_finding,
    external_url: summary.external_url ?? payload.external_url,
    evidence_ref: summary.evidence_ref ?? payload.evidence_ref,
  });
}

function runtimeFactorySignal(
  payload: z.infer<typeof RuntimeDeploySummarySchema>,
  signal: z.infer<typeof RuntimeFactorySignalSummarySchema>,
  deployment: DeploymentNodeData,
  index: number,
  now: Date
): FactorySignalNodeData {
  const fallbackId = stableId(`${index}:${signal.kind}:${signal.title}`);
  const signalId = signal.signal_id
    ? `runtime:${payload.repo}:signal:${signal.signal_id}`
    : `${deployment.id}:signal:${fallbackId || index}`;

  return compactFactorySignal({
    id: signalId,
    aggregate_key: signal.aggregate_key ?? deployment.id,
    category: signal.category ?? signalCategoryFromText(`${signal.kind} ${signal.title}`),
    source: "runtime-runway",
    kind: signal.kind,
    title: signal.title,
    status: signal.status ?? payload.status,
    severity: signal.severity,
    observed_at: signal.observed_at ?? payload.occurred_at ?? now.toISOString(),
    first_seen_at: signal.first_seen_at,
    last_seen_at: signal.last_seen_at,
    event_count: numberFromValue(signal.event_count),
    affected_users: numberFromValue(signal.affected_users),
    repo: payload.repo,
    finding_id: signal.finding_id,
    external_url: signal.external_url ?? payload.external_url,
    evidence_ref: signal.evidence_ref ?? payload.evidence_ref,
    metric_name: signal.metric_name,
    metric_value: finiteNumberFromValue(signal.metric_value),
    unit: signal.unit,
  });
}

function githubPullRequestSignal(
  repo: string,
  pullRequest: PullRequestNodeData,
  rawPullRequest: z.infer<typeof GitHubPullRequestSchema>,
  linkedFinding: string | undefined,
  now: Date
): FactorySignalNodeData {
  return compactFactorySignal({
    id: `github:${pullRequest.id}:signal`,
    aggregate_key: pullRequest.id,
    category: signalCategoryFromText(
      `${rawPullRequest.title ?? ""} ${rawPullRequest.body ?? ""}`
    ),
    source: "github",
    kind: "pull_request",
    title: pullRequest.title ?? `Pull request ${pullRequest.id}`,
    status: pullRequest.state,
    observed_at: now.toISOString(),
    repo,
    finding_id: linkedFinding,
    external_url: pullRequest.url,
  });
}

function appendSignal(records: GraphRecord[], signal: FactorySignalNodeData): void {
  records.push({ type: "FactorySignal", data: signal });
  if (signal.repo) {
    records.push({
      type: "SignalInRepo",
      from: factorySignalRef(signal.id),
      to: repositoryRef(signal.repo),
    });
  }
  if (signal.finding_id) {
    records.push({
      type: "SignalSupportsFinding",
      from: factorySignalRef(signal.id),
      to: findingRef(signal.finding_id),
    });
  }
}

function ciStatus(status?: string | null, conclusion?: string | null): string {
  if (status && status !== "completed") {
    return "Unknown";
  }

  switch (conclusion) {
    case "success":
      return "Green";
    case "failure":
    case "timed_out":
    case "action_required":
    case "cancelled":
    case "startup_failure":
      return "Red";
    case "neutral":
    case "skipped":
    case undefined:
    case null:
      return "Unknown";
    default:
      return "Unknown";
  }
}

function githubPayloadKind(payload: z.infer<typeof GitHubPayloadSchema>): string {
  if (payload.check_run) return "check_run";
  if (payload.check_suite) return "check_suite";
  if (payload.release) return "release";
  if (payload.deployment_status) return "deployment_status";
  if (payload.deployment) return "deployment";
  if (payload.pull_request) return "pull_request";
  if (payload.commits) return "push";
  return "repository";
}

function linearLabels(labels: z.infer<typeof LinearLabelSchema>[]): string[] {
  return labels
    .map((label) => (typeof label === "string" ? label : label.name))
    .map((label) => label.trim())
    .filter((label) => label.length > 0);
}

function isFactoryFinding(
  labels: string[],
  title?: string,
  description?: string | null
): boolean {
  const text = `${labels.join(" ")} ${title ?? ""} ${description ?? ""}`.toLowerCase();
  return [
    "quality",
    "factory",
    "debt",
    "security",
    "ci",
    "reliability",
    "test",
    "drift",
    "bug",
  ].some((needle) => text.includes(needle));
}

function bucketFromLinear(priority: unknown, labels: string[]): string {
  const explicit = prefixedLabel(labels, "bucket:");
  if (explicit && /^[a-d]$/i.test(explicit)) {
    return explicit.toUpperCase();
  }

  const number = linearPriority(priority);
  if (number === 1) return "A";
  if (number === 2) return "B";
  if (number === 3) return "C";
  return "D";
}

function linearPriority(priority: unknown): number | undefined {
  if (typeof priority === "number") {
    return priority;
  }

  if (isRecord(priority) && typeof priority.value === "number") {
    return priority.value;
  }

  return undefined;
}

function statusFromLinear(type?: string, name?: string): FindingStatus {
  const value = `${type ?? ""} ${name ?? ""}`.toLowerCase();
  if (value.includes("completed") || value.includes("done")) return "Done";
  if (value.includes("canceled") || value.includes("cancelled")) return "WontDo";
  if (value.includes("started") || value.includes("progress")) return "InProgress";
  return "Open";
}

function statusFromSentry(status?: string): IncidentStatus {
  switch (status?.toLowerCase()) {
    case "resolved":
      return "Resolved";
    case "ignored":
      return "Ignored";
    default:
      return "Open";
  }
}

function severityFromSentry(level?: string): IncidentSeverity {
  switch (level?.toLowerCase()) {
    case "fatal":
      return "P1";
    case "error":
      return "P2";
    default:
      return "P3";
  }
}

function areaFromLabels(labels: string[]): string {
  return (
    prefixedLabel(labels, "area:") ??
    prefixedLabel(labels, "type:") ??
    labels.find((label) => !label.toLowerCase().startsWith("module:")) ??
    "factory debt"
  ).slice(0, 120);
}

function effortFromLabels(labels: string[]): string | undefined {
  const effort = prefixedLabel(labels, "effort:");
  return effort && /^[sml]$/i.test(effort) ? effort.toUpperCase() : undefined;
}

function confidenceFromLabels(labels: string[]): string | undefined {
  const confidence = prefixedLabel(labels, "confidence:");
  return confidence && /^[hml]$/i.test(confidence)
    ? confidence.toUpperCase()
    : undefined;
}

function codexSafeFromLabels(labels: string[]): string | undefined {
  const normalized = labels.map((label) => label.toLowerCase());
  if (normalized.includes("codex-safe") || normalized.includes("codex_safe")) {
    return "Yes";
  }
  if (normalized.includes("codex-unsafe") || normalized.includes("human-review")) {
    return "No";
  }
  return undefined;
}

function prefixedLabel(labels: string[], prefix: string): string | undefined {
  const label = labels.find((candidate) =>
    candidate.toLowerCase().startsWith(prefix)
  );
  return label?.slice(prefix.length).trim();
}

function nextAction(description?: string | null): string | undefined {
  if (!description) return undefined;

  const explicit = description
    .split("\n")
    .map((line) => line.trim())
    .find((line) => /^next action:/i.test(line));

  if (explicit) {
    return truncate(explicit.replace(/^next action:\s*/i, ""), 200);
  }

  const checklist = description
    .split("\n")
    .map((line) => line.trim())
    .find((line) => /^[-*]\s+\[[ x-]\]/i.test(line));

  return checklist ? truncate(checklist.replace(/^[-*]\s+\[[ x-]\]\s*/i, ""), 200) : undefined;
}

function extractQfId(...values: Array<string | null | undefined>): string | undefined {
  for (const value of values) {
    const match = value?.match(/\bQF-\d{4}-\d{2}-\d{2}-\d+\b/);
    if (match?.[0]) return match[0];
  }
  return undefined;
}

function ownerFromLinearAssignee(
  assignee: z.infer<typeof LinearIssueSchema>["assignee"]
): OwnerNodeData | undefined {
  if (!assignee) return undefined;

  const name = optionalString(assignee.name);
  const email = optionalString(assignee.email)?.toLowerCase();
  const id = optionalString(assignee.id) ?? email ?? (name ? stableId(name) : undefined);
  if (!id || !name && !email) return undefined;

  return compactOwner({
    id,
    name: name ?? email ?? id,
    email,
  });
}

function linearIssueSignal(
  issue: z.infer<typeof LinearIssueSchema>,
  labels: string[],
  findingId: string,
  repo: string | undefined,
  action: string | undefined,
  now: Date
): FactorySignalNodeData {
  const status = issue.state?.name ?? issue.state?.type ?? action;
  return compactFactorySignal({
    id: `linear:${issue.identifier ?? issue.id}:signal`,
    aggregate_key: issue.identifier ?? issue.id,
    category: signalCategoryFromText(
      `${labels.join(" ")} ${issue.title ?? ""} ${issue.description ?? ""}`
    ),
    source: "linear",
    kind: "issue",
    title: issue.title ?? issue.identifier ?? issue.id,
    status,
    severity: bucketFromLinear(issue.priority, labels),
    observed_at: issue.updatedAt ?? issue.createdAt ?? now.toISOString(),
    repo,
    finding_id: findingId,
    external_url: issue.url,
  });
}

function sentryIssueSignal(
  issue: z.infer<typeof SentryIssueSchema>,
  incident: IncidentNodeData,
  linkedFinding: string | undefined,
  now: Date
): FactorySignalNodeData {
  return compactFactorySignal({
    id: `sentry:${incident.id}:signal`,
    aggregate_key: `sentry:${incident.id}`,
    category: "runtime_telemetry",
    source: "sentry",
    kind: "sentry_issue",
    title: incident.title ?? issue.culprit ?? incident.id,
    status: incident.status,
    severity: incident.severity,
    observed_at: incident.opened_at || now.toISOString(),
    first_seen_at: incident.opened_at,
    last_seen_at: issue.lastSeen ?? issue.last_seen,
    event_count: numberFromValue(issue.count ?? issue.eventCount),
    affected_users: numberFromValue(issue.userCount ?? issue.user_count),
    repo: incident.repo,
    finding_id: linkedFinding,
    external_url: optionalString(issue.permalink) ?? optionalString(issue.web_url),
  });
}

function signalCategoryFromText(text: string): FactorySignalCategory {
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
      "codeql",
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
  if (matches(["pull request", "pr review", "branch", "commit", "github"])) {
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

function dateOnly(value: string | undefined, fallback: Date): string {
  return value?.slice(0, 10) ?? fallback.toISOString().slice(0, 10);
}

function projectSlug(
  project: z.infer<typeof SentryProjectSchema> | undefined
): string | undefined {
  if (!project) return undefined;
  if (typeof project === "string") return project;
  return project.slug ?? project.name;
}

function hasSentryIssue(payload: z.infer<typeof SentryPayloadSchema>): boolean {
  return Boolean(payload.data?.issue ?? payload.issue);
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

function compactIncident(data: {
  id: string;
  repo: string;
  title?: string | undefined;
  severity: IncidentSeverity;
  status: IncidentStatus;
  opened_at: string;
  linked_finding?: string | undefined;
}): IncidentNodeData {
  const incident: IncidentNodeData = {
    id: data.id,
    repo: data.repo,
    severity: data.severity,
    status: data.status,
    opened_at: data.opened_at,
  };

  if (data.title) incident.title = data.title;
  if (data.linked_finding) incident.linked_finding = data.linked_finding;

  return incident;
}

function compactCheckRun(data: {
  id: string;
  repo: string;
  name?: string | undefined;
  status?: string | undefined;
  conclusion?: string | undefined;
  completed_at?: string | undefined;
}): CheckRunNodeData {
  const checkRun: CheckRunNodeData = {
    id: data.id,
    repo: data.repo,
  };

  if (data.name) checkRun.name = data.name;
  if (data.status) checkRun.status = data.status;
  if (data.conclusion) checkRun.conclusion = data.conclusion;
  if (data.completed_at) checkRun.completed_at = data.completed_at;

  return checkRun;
}

function compactDeployment(data: {
  id: string;
  repo: string;
  environment?: string | undefined;
  status?: string | undefined;
  version?: string | undefined;
  deployed_at?: string | undefined;
}): DeploymentNodeData {
  const deployment: DeploymentNodeData = {
    id: data.id,
    repo: data.repo,
  };

  if (data.environment) deployment.environment = data.environment;
  if (data.status) deployment.status = data.status;
  if (data.version) deployment.version = data.version;
  if (data.deployed_at) deployment.deployed_at = data.deployed_at;

  return deployment;
}

function compactPullRequest(data: {
  id: string;
  repo: string;
  number: number;
  title?: string | undefined;
  url?: string | undefined;
  state?: string | undefined;
  head_sha?: string | undefined;
}): PullRequestNodeData {
  const pullRequest: PullRequestNodeData = {
    id: data.id,
    repo: data.repo,
    number: data.number,
  };

  if (data.title) pullRequest.title = data.title;
  if (data.url) pullRequest.url = data.url;
  if (data.state) pullRequest.state = data.state;
  if (data.head_sha) pullRequest.head_sha = data.head_sha;

  return pullRequest;
}

function compactOwner(data: {
  id: string;
  name: string;
  email?: string | undefined;
}): OwnerNodeData {
  const owner: OwnerNodeData = {
    id: data.id,
    name: data.name,
  };

  if (data.email) owner.email = data.email;
  return owner;
}

function compactFactorySignal(data: {
  id: string;
  aggregate_key?: string | undefined;
  category: FactorySignalCategory;
  source: string;
  kind: string;
  title: string;
  status?: string | undefined;
  severity?: string | undefined;
  observed_at: string;
  first_seen_at?: string | undefined;
  last_seen_at?: string | undefined;
  event_count?: number | undefined;
  affected_users?: number | undefined;
  repo?: string | undefined;
  finding_id?: string | undefined;
  external_url?: string | undefined;
  evidence_ref?: string | undefined;
  metric_name?: string | undefined;
  metric_value?: number | undefined;
  unit?: string | undefined;
}): FactorySignalNodeData {
  const signal: FactorySignalNodeData = {
    id: data.id,
    category: data.category,
    source: data.source,
    kind: data.kind,
    title: truncate(data.title, 200),
    observed_at: data.observed_at,
  };

  if (data.aggregate_key) signal.aggregate_key = data.aggregate_key;
  if (data.status) signal.status = data.status;
  if (data.severity) signal.severity = data.severity;
  if (data.first_seen_at) signal.first_seen_at = data.first_seen_at;
  if (data.last_seen_at) signal.last_seen_at = data.last_seen_at;
  if (data.event_count !== undefined) signal.event_count = data.event_count;
  if (data.affected_users !== undefined) signal.affected_users = data.affected_users;
  if (data.repo) signal.repo = data.repo;
  if (data.finding_id) signal.finding_id = data.finding_id;
  if (data.external_url) signal.external_url = data.external_url;
  if (data.evidence_ref) signal.evidence_ref = data.evidence_ref;
  if (data.metric_name) signal.metric_name = data.metric_name;
  if (data.metric_value !== undefined) signal.metric_value = data.metric_value;
  if (data.unit) signal.unit = data.unit;

  return signal;
}

function truncate(value: string, max: number): string {
  return value.length > max ? value.slice(0, max) : value;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function optionalString(value: string | null | undefined): string | undefined {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

function optionalStringFromUnknown(value: unknown): string | undefined {
  return typeof value === "string" ? optionalString(value) : undefined;
}

function numberFromValue(value: string | number | undefined): number | undefined {
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (typeof value !== "string") return undefined;

  const parsed = Number.parseInt(value.replaceAll(",", ""), 10);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function finiteNumberFromValue(value: string | number | undefined): number | undefined {
  if (typeof value === "number" && Number.isFinite(value)) return value;
  if (typeof value !== "string") return undefined;

  const parsed = Number(value.replaceAll(",", ""));
  return Number.isFinite(parsed) ? parsed : undefined;
}

function stableId(value: string): string {
  return value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}
