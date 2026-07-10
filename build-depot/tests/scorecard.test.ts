import { describe, expect, test } from "bun:test";
import { buildScorecardExport } from "../scripts/scorecard";
import type { AdoptionScanResult } from "../scripts/factory-adoption-doctor";
import type { GraphRecord } from "../trigger.dev/graph-records";

describe("scorecard export", () => {
  test("summarizes graph-compatible records and adoption scan state", () => {
    const records: GraphRecord[] = [
      {
        type: "Repository",
        data: { name: "build-depot", language: "TypeScript", layer: "factory" },
      },
      {
        type: "Finding",
        data: {
          id: "QF-2026-07-08-01",
          date: "2026-07-08",
          bucket: "B",
          area: "test",
          status: "Open",
          title: "test finding",
        },
      },
      {
        type: "RPProperty",
        data: {
          id: "RP-CI-PARITY",
          property_name: "CI parity",
          enforcement_status: "Aspired",
        },
      },
      {
        type: "FindingWorksOnRP",
        from: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
        to: { type: "RPProperty", key: { id: "RP-CI-PARITY" } },
      },
      {
        type: "FactorySignal",
        data: {
          id: "sentry:123:signal",
          aggregate_key: "sentry:123",
          category: "runtime_telemetry",
          source: "sentry",
          kind: "sentry_issue",
          title: "runtime failure",
          status: "Open",
          severity: "P2",
          observed_at: "2026-07-08T00:00:00.000Z",
          event_count: 12,
          affected_users: 3,
          repo: "build-depot",
          finding_id: "QF-2026-07-08-01",
        },
      },
    ];
    const adoptionScan: AdoptionScanResult = {
      scannedAt: "2026-07-08T00:00:00.000Z",
      sourceRevision: "test",
      workspaceRoot: "/tmp/reflective",
      blockingFailures: 1,
      repositories: [
        {
          name: "build-depot",
          path: "/tmp/reflective/build-depot",
          tier: "Tier0",
          state: "adopted",
          signals: [],
          missingRequiredSignals: [],
        },
        {
          name: "bedrock-consolidated",
          path: "/tmp/reflective/bedrock-consolidated",
          tier: "Tier1",
          state: "blocked",
          signals: [],
          missingRequiredSignals: ["ci-workflow"],
        },
      ],
    };

    const exportData = buildScorecardExport(
      records,
      adoptionScan,
      new Date("2026-07-08T00:00:00.000Z")
    );

    expect(exportData.graph_records).toEqual({
      total: 5,
      nodes: 4,
      edges: 1,
      edge_counts: { FindingWorksOnRP: 1 },
    });
    expect(exportData.findings.by_bucket).toEqual({ B: 1 });
    expect(exportData.repositories.adoption_by_state).toEqual({
      adopted: 1,
      blocked: 1,
    });
    expect(exportData.repositories.adoption_blocking_failures).toBe(1);
    expect(exportData.signals).toEqual({
      total: 1,
      total_events: 12,
      affected_users: 3,
      by_category: { runtime_telemetry: 1 },
      by_source: { sentry: 1 },
      by_status: { Open: 1 },
    });
    expect(exportData.runtime.omnigraph_ingest).toBe("deferred");
  });
});
