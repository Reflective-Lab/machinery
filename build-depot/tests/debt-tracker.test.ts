import { describe, expect, test } from "bun:test";
import { normalizeDebtPayload, recordsToNdjson } from "../trigger.dev/debt-tracker";

const NOW = new Date("2026-07-07T00:00:00.000Z");

describe("debt-tracker normalization", () => {
  test("normalizes GitHub check suites into repository CI status", () => {
    const event = normalizeDebtPayload(
      {
        action: "completed",
        repository: { name: "runtime-runway" },
        check_suite: { status: "completed", conclusion: "success" },
      },
      NOW
    );

    expect(event.source).toBe("github");
    expect(event.records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "runtime-runway",
        language: "Rust",
        ci_status: "Green",
      }),
    });
    expect(event.records).toContainEqual({
      type: "CheckRun",
      data: expect.objectContaining({
        id: "github:runtime-runway:check-suite:2026-07-07T00:00:00.000Z",
        repo: "runtime-runway",
        name: "check suite",
        status: "completed",
        conclusion: "success",
      }),
    });
    expect(event.records).toContainEqual({
      type: "CheckRunInRepo",
      from: {
        type: "CheckRun",
        key: { id: "github:runtime-runway:check-suite:2026-07-07T00:00:00.000Z" },
      },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "github:runtime-runway:check-suite:2026-07-07T00:00:00.000Z:signal",
        category: "quality_gate",
        source: "github",
        kind: "check_suite",
        repo: "runtime-runway",
      }),
    });
  });

  test("links GitHub pull requests to cited factory findings", () => {
    const event = normalizeDebtPayload(
      {
        action: "opened",
        repository: { name: "build-depot" },
        number: 42,
        pull_request: {
          number: 42,
          title: "QF-2026-07-07-01 fix security gate",
          body: "Closes QF-2026-07-07-01",
          html_url: "https://github.com/Reflective-Lab/build-depot/pull/42",
          state: "open",
          head: { sha: "abc123", ref: "fix/QF-2026-07-07-01" },
        },
      },
      NOW
    );

    expect(event.source).toBe("github");
    expect(event.records).toContainEqual({
      type: "PullRequest",
      data: {
        id: "build-depot#42",
        repo: "build-depot",
        number: 42,
        title: "QF-2026-07-07-01 fix security gate",
        url: "https://github.com/Reflective-Lab/build-depot/pull/42",
        state: "open",
        head_sha: "abc123",
      },
    });
    expect(event.records).toContainEqual({
      type: "PullRequestAddressesFinding",
      from: { type: "PullRequest", key: { id: "build-depot#42" } },
      to: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "github:build-depot#42:signal",
        category: "security_scan",
        source: "github",
        kind: "pull_request",
        repo: "build-depot",
        finding_id: "QF-2026-07-07-01",
      }),
    });
  });

  test("captures GitHub releases as deployment facts", () => {
    const event = normalizeDebtPayload(
      {
        action: "published",
        repository: { name: "runtime-runway" },
        release: {
          id: 9,
          tag_name: "v1.2.3",
          html_url: "https://github.com/Reflective-Lab/runtime-runway/releases/tag/v1.2.3",
          published_at: "2026-07-07T01:00:00.000Z",
        },
      },
      NOW
    );

    expect(event.records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "runtime-runway",
        last_release: "v1.2.3",
      }),
    });
    expect(event.records).toContainEqual({
      type: "Deployment",
      data: {
        id: "github:runtime-runway:release:9",
        repo: "runtime-runway",
        environment: "release",
        status: "published",
        version: "v1.2.3",
        deployed_at: "2026-07-07T01:00:00.000Z",
      },
    });
    expect(event.records).toContainEqual({
      type: "DeploymentInRepo",
      from: { type: "Deployment", key: { id: "github:runtime-runway:release:9" } },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "github:runtime-runway:release:9:signal",
        category: "delivery",
        source: "github",
        kind: "release",
        repo: "runtime-runway",
      }),
    });
  });

  test("normalizes Linear factory issues into repository and finding records", () => {
    const event = normalizeDebtPayload(
      {
        type: "Issue",
        action: "update",
        data: {
          id: "lin-1",
          identifier: "RFL-1",
          title: "QF-2026-07-07-01 tighten factory CI",
          createdAt: "2026-07-07T00:00:00.000Z",
          priority: 2,
          labels: [
            { name: "module:build-depot" },
            { name: "type:ci" },
            { name: "effort:S" },
            { name: "confidence:H" },
            { name: "codex-safe" },
          ],
          state: { type: "started", name: "In Progress" },
          assignee: { name: "Codex" },
          description: "Next action: add a strict test command",
        },
      },
      NOW
    );

    expect(event.source).toBe("linear");
    expect(event.records).toHaveLength(8);
    expect(event.records[0]).toEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "build-depot",
        language: "TypeScript",
      }),
    });
    expect(event.records[1]).toEqual({
      type: "Finding",
      data: {
        id: "QF-2026-07-07-01",
        date: "2026-07-07",
        bucket: "B",
        area: "ci",
        status: "InProgress",
        title: "QF-2026-07-07-01 tighten factory CI",
        effort: "S",
        owner: "Codex",
        codex_safe: "Yes",
        confidence: "H",
        next_action: "add a strict test command",
        repo: "build-depot",
      },
    });
    expect(event.records[2]).toEqual({
      type: "FindingInRepo",
      from: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
      to: { type: "Repository", key: { name: "build-depot" } },
    });
    expect(event.records).toContainEqual({
      type: "Owner",
      data: {
        id: "codex",
        name: "Codex",
      },
    });
    expect(event.records).toContainEqual({
      type: "OwnerOwnsFinding",
      from: { type: "Owner", key: { id: "codex" } },
      to: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "linear:RFL-1:signal",
        category: "test",
        source: "linear",
        kind: "issue",
        repo: "build-depot",
        finding_id: "QF-2026-07-07-01",
      }),
    });
    expect(event.records).toContainEqual({
      type: "SignalSupportsFinding",
      from: { type: "FactorySignal", key: { id: "linear:RFL-1:signal" } },
      to: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
    });
  });

  test("normalizes Sentry issues into repository and incident records", () => {
    const event = normalizeDebtPayload(
      {
        action: "created",
        data: {
          issue: {
            id: "123",
            title: "runtime failure QF-2026-07-07-01",
            level: "error",
            status: "unresolved",
            firstSeen: "2026-07-07T00:00:00.000Z",
            lastSeen: "2026-07-07T02:00:00.000Z",
            count: "1,234",
            userCount: 17,
            project: { slug: "runtime-runway" },
          },
        },
      },
      NOW
    );

    expect(event.source).toBe("sentry");
    expect(event.records[0]).toEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "runtime-runway",
        language: "Rust",
      }),
    });
    expect(event.records[1]).toEqual({
      type: "Incident",
      data: {
        id: "123",
        repo: "runtime-runway",
        title: "runtime failure QF-2026-07-07-01",
        severity: "P2",
        status: "Open",
        opened_at: "2026-07-07T00:00:00.000Z",
        linked_finding: "QF-2026-07-07-01",
      },
    });
    expect(event.records[2]).toEqual({
      type: "IncidentInRepo",
      from: { type: "Incident", key: { id: "123" } },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "sentry:123:signal",
        aggregate_key: "sentry:123",
        category: "runtime_telemetry",
        source: "sentry",
        kind: "sentry_issue",
        event_count: 1234,
        affected_users: 17,
        first_seen_at: "2026-07-07T00:00:00.000Z",
        last_seen_at: "2026-07-07T02:00:00.000Z",
        repo: "runtime-runway",
        finding_id: "QF-2026-07-07-01",
      }),
    });
    expect(event.records).toContainEqual({
      type: "SignalSupportsFinding",
      from: { type: "FactorySignal", key: { id: "sentry:123:signal" } },
      to: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
    });
  });

  test("normalizes Runtime deploy summaries into deployment and factory signals", () => {
    const event = normalizeDebtPayload(
      {
        type: "runtime.deploy.summary",
        schema_version: "1",
        source: "runtime-runway",
        repo: "runtime-runway",
        service: "api-server",
        app_id: "quorum-sense",
        environment: "prod",
        region: "us-central1",
        deployment_id: "api-server-prod-20260707-01",
        version: "3.6.0",
        commit_sha: "abc123",
        image_digest: "sha256:feed",
        status: "succeeded",
        occurred_at: "2026-07-07T03:00:00.000Z",
        duration_ms: 42000,
        external_url: "https://runway.example/deployments/api-server-prod-20260707-01",
        evidence_ref: "gcs://runtime-evidence/deployments/api-server-prod-20260707-01.json",
        signals: [
          {
            signal_id: "api-server-prod-20260707-01-p95-latency",
            category: "runtime_telemetry",
            kind: "latency_delta",
            title: "Cloud Run p95 latency increased 40 percent after deploy",
            status: "watching",
            observed_at: "2026-07-07T03:05:00.000Z",
            first_seen_at: "2026-07-07T03:01:00.000Z",
            last_seen_at: "2026-07-07T03:05:00.000Z",
            metric_name: "http.server.duration.p95_delta",
            metric_value: 40,
            unit: "percent",
          },
        ],
      },
      NOW
    );

    expect(event.source).toBe("runtime-runway");
    expect(event.action).toBe("succeeded");
    expect(event.records).toContainEqual({
      type: "Deployment",
      data: {
        id: "runtime:runtime-runway:deployment:api-server-prod-20260707-01",
        repo: "runtime-runway",
        environment: "prod",
        status: "succeeded",
        version: "3.6.0",
        deployed_at: "2026-07-07T03:00:00.000Z",
      },
    });
    expect(event.records).toContainEqual({
      type: "DeploymentInRepo",
      from: {
        type: "Deployment",
        key: {
          id: "runtime:runtime-runway:deployment:api-server-prod-20260707-01",
        },
      },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "runtime:runtime-runway:deployment:api-server-prod-20260707-01:summary-signal",
        aggregate_key: "runtime:runtime-runway:deployment:api-server-prod-20260707-01",
        category: "delivery",
        source: "runtime-runway",
        kind: "deploy_summary",
        repo: "runtime-runway",
        metric_name: "deployment.duration",
        metric_value: 42000,
        unit: "ms",
      }),
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "runtime:runtime-runway:signal:api-server-prod-20260707-01-p95-latency",
        aggregate_key: "runtime:runtime-runway:deployment:api-server-prod-20260707-01",
        category: "runtime_telemetry",
        source: "runtime-runway",
        kind: "latency_delta",
        metric_name: "http.server.duration.p95_delta",
        metric_value: 40,
        unit: "percent",
      }),
    });
  });

  test("normalizes Runtime incident summaries without raw telemetry", () => {
    const event = normalizeDebtPayload(
      {
        type: "runtime.deploy.summary",
        schema_version: "1",
        source: "runtime-runway",
        repo: "runtime-runway",
        service: "api-server",
        environment: "prod",
        deployment_id: "api-server-prod-20260707-02",
        commit_sha: "def456",
        status: "failed",
        occurred_at: "2026-07-07T04:00:00.000Z",
        incident: {
          incident_id: "api-server-prod-20260707-storage",
          title: "Storage preflight missed write failure QF-2026-07-07-02",
          severity: "P2",
          status: "Open",
          opened_at: "2026-07-07T04:02:00.000Z",
          first_seen_at: "2026-07-07T04:02:00.000Z",
          last_seen_at: "2026-07-07T04:07:00.000Z",
          event_count: "42",
          affected_users: 9,
          external_url: "https://sentry.example/issues/runtime-runway-456",
        },
      },
      NOW
    );

    expect(event.records).toContainEqual({
      type: "Incident",
      data: {
        id: "runtime:runtime-runway:incident:api-server-prod-20260707-storage",
        repo: "runtime-runway",
        title: "Storage preflight missed write failure QF-2026-07-07-02",
        severity: "P2",
        status: "Open",
        opened_at: "2026-07-07T04:02:00.000Z",
        linked_finding: "QF-2026-07-07-02",
      },
    });
    expect(event.records).toContainEqual({
      type: "IncidentInRepo",
      from: {
        type: "Incident",
        key: {
          id: "runtime:runtime-runway:incident:api-server-prod-20260707-storage",
        },
      },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
    expect(event.records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "runtime:runtime-runway:incident:api-server-prod-20260707-storage:signal",
        aggregate_key: "runtime:runtime-runway:deployment:api-server-prod-20260707-02",
        category: "runtime_telemetry",
        source: "runtime-runway",
        kind: "incident_summary",
        event_count: 42,
        affected_users: 9,
        repo: "runtime-runway",
        finding_id: "QF-2026-07-07-02",
      }),
    });
  });

  test("rejects Runtime deploy summaries that include raw telemetry fields", () => {
    const event = normalizeDebtPayload(
      {
        type: "runtime.deploy.summary",
        schema_version: "1",
        source: "runtime-runway",
        repo: "runtime-runway",
        service: "api-server",
        environment: "prod",
        deployment_id: "api-server-prod-20260707-03",
        status: "succeeded",
        occurred_at: "2026-07-07T05:00:00.000Z",
        raw_logs: ["do not ingest raw logs"],
      },
      NOW
    );

    expect(event).toEqual({
      source: "unknown",
      records: [],
      skippedReason: "Unsupported webhook payload shape",
    });
  });

  test("skips unsupported payloads explicitly", () => {
    const event = normalizeDebtPayload({ hello: "world" }, NOW);

    expect(event).toEqual({
      source: "unknown",
      records: [],
      skippedReason: "Unsupported webhook payload shape",
    });
  });

  test("serializes graph records as newline-terminated NDJSON", () => {
    expect(
      recordsToNdjson([
        {
          type: "Repository",
          data: { name: "build-depot", language: "TypeScript" },
        },
        {
          type: "FindingInRepo",
          from: { type: "Finding", key: { id: "QF-2026-07-07-01" } },
          to: { type: "Repository", key: { name: "build-depot" } },
        },
      ])
    ).toBe(
      '{"type":"Repository","data":{"name":"build-depot","language":"TypeScript"}}\n{"type":"FindingInRepo","from":{"type":"Finding","key":{"id":"QF-2026-07-07-01"}},"to":{"type":"Repository","key":{"name":"build-depot"}}}\n'
    );
  });
});
