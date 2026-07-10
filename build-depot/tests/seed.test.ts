import { describe, expect, test } from "bun:test";
import { buildSeedRecords, buildSeedRecordsFromInputs } from "../scripts/seed";

describe("seed generation", () => {
  test("includes Build-Depot repository metadata", () => {
    const records = buildSeedRecords();

    expect(records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "build-depot",
        language: "TypeScript",
        layer: "factory",
      }),
    });
  });

  test("loads findings and recurring properties from the factory ledger", () => {
    const records = buildSeedRecords();
    const findingCount = records.filter((record) => record.type === "Finding").length;
    const propertyCount = records.filter(
      (record) => record.type === "RPProperty"
    ).length;

    expect(findingCount).toBeGreaterThan(0);
    expect(propertyCount).toBeGreaterThan(0);
  });

  test("does not emit explicit undefined graph fields", () => {
    const records = buildSeedRecords();

    for (const record of records) {
      expect(JSON.stringify(record)).not.toContain(":undefined");
      const values =
        "data" in record
          ? Object.values(record.data)
          : [
              ...Object.values(record.from.key),
              ...Object.values(record.to.key),
            ];
      for (const value of values) {
        expect(value).not.toBeUndefined();
      }
    }
  });

  test("emits provenance edges from the factory ledger", () => {
    const records = buildSeedRecords();

    expect(records).toContainEqual({
      type: "FindingWorksOnRP",
      from: { type: "Finding", key: { id: "QF-2026-06-08-11" } },
      to: { type: "RPProperty", key: { id: "RP-CI-PARITY" } },
    });
    expect(records).toContainEqual({
      type: "FindingInRepo",
      from: { type: "Finding", key: { id: "QF-2026-07-02-08" } },
      to: { type: "Repository", key: { name: "runtime-runway" } },
    });
  });

  test("emits standards and standard provenance edges when promoted", () => {
    const records = buildSeedRecordsFromInputs(
      [
        "#### QF-2026-07-08-01",
        "",
        "- Date: 2026-07-08",
        "- Bucket: B. Should fix soon",
        "- Area: test",
        "- Evidence: build-depot docs captured the standard.",
        "- Status: Done",
        "- Owner: Codex",
        "- Discovered during: incident",
        "- Properties: RP-CI-PARITY",
        "- Linked PRs / commits: build-depot#42",
        "- Standard promoted: `KB/05-engineering/standards/ci-parity.md`",
        "- ADR: `KB/04-architecture/decisions/2026-07-08-ci-parity.md`",
        "- Risk register entry: `KB/06-operations/risk-register.md#rr-2026-07-08-01`",
      ].join("\n"),
      [
        {
          id: "RP-CI-PARITY",
          property_name: "CI parity",
          enforcement_status: "Aspired",
        },
      ]
    );

    expect(records).toContainEqual({
      type: "Standard",
      data: {
        path: "KB/05-engineering/standards/ci-parity.md",
        title: "Ci Parity",
      },
    });
    expect(records).toContainEqual({
      type: "FindingToStandard",
      from: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
      to: {
        type: "Standard",
        key: { path: "KB/05-engineering/standards/ci-parity.md" },
      },
    });
    expect(records).toContainEqual({
      type: "FindingWorksOnRP",
      from: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
      to: { type: "RPProperty", key: { id: "RP-CI-PARITY" } },
    });
    expect(records).toContainEqual({
      type: "Owner",
      data: {
        id: "codex",
        name: "Codex",
      },
    });
    expect(records).toContainEqual({
      type: "OwnerOwnsFinding",
      from: { type: "Owner", key: { id: "codex" } },
      to: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
    });
    expect(records).toContainEqual({
      type: "PullRequest",
      data: {
        id: "build-depot#42",
        repo: "build-depot",
        number: 42,
      },
    });
    expect(records).toContainEqual({
      type: "PullRequestAddressesFinding",
      from: { type: "PullRequest", key: { id: "build-depot#42" } },
      to: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
    });
    expect(records).toContainEqual({
      type: "ADR",
      data: {
        path: "KB/04-architecture/decisions/2026-07-08-ci-parity.md",
        title: "2026 07 08 Ci Parity",
      },
    });
    expect(records).toContainEqual({
      type: "ADRDecidesFinding",
      from: {
        type: "ADR",
        key: { path: "KB/04-architecture/decisions/2026-07-08-ci-parity.md" },
      },
      to: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
    });
    expect(records).toContainEqual({
      type: "Risk",
      data: {
        id: "KB/06-operations/risk-register.md#rr-2026-07-08-01",
        title: "Risk Register",
      },
    });
    expect(records).toContainEqual({
      type: "RiskTracksFinding",
      from: {
        type: "Risk",
        key: { id: "KB/06-operations/risk-register.md#rr-2026-07-08-01" },
      },
      to: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
    });
    expect(records).toContainEqual({
      type: "FactorySignal",
      data: expect.objectContaining({
        id: "QF-2026-07-08-01:evidence",
        aggregate_key: "QF-2026-07-08-01",
        category: "test",
        source: "quality-backlog",
        kind: "incident",
        finding_id: "QF-2026-07-08-01",
      }),
    });
    expect(records).toContainEqual({
      type: "SignalSupportsFinding",
      from: {
        type: "FactorySignal",
        key: { id: "QF-2026-07-08-01:evidence" },
      },
      to: { type: "Finding", key: { id: "QF-2026-07-08-01" } },
    });
  });
});
