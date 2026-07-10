import { buildSeedRecords } from "./seed";
import {
  scanWorkspace,
  type AdoptionScanResult,
} from "./factory-adoption-doctor";
import type { GraphRecord } from "../trigger.dev/graph-records";

interface ScorecardExport {
  generated_at: string;
  source: "build-depot";
  runtime: {
    omnigraph_ingest: "deferred";
    note: string;
  };
  graph_records: {
    total: number;
    nodes: number;
    edges: number;
    edge_counts: Record<string, number>;
  };
  findings: {
    total: number;
    open: number;
    by_bucket: Record<string, number>;
    by_status: Record<string, number>;
  };
  repositories: {
    total: number;
    by_layer: Record<string, number>;
    adoption_by_state: Record<string, number>;
    adoption_by_tier: Record<string, number>;
    adoption_blocking_failures: number;
  };
  incidents: {
    total: number;
    open: number;
    by_severity: Record<string, number>;
  };
  signals: {
    total: number;
    total_events: number;
    affected_users: number;
    by_category: Record<string, number>;
    by_source: Record<string, number>;
    by_status: Record<string, number>;
  };
  pull_requests: {
    total: number;
  };
  check_runs: {
    total: number;
    by_status: Record<string, number>;
    by_conclusion: Record<string, number>;
  };
  deployments: {
    total: number;
    by_environment: Record<string, number>;
    by_status: Record<string, number>;
  };
  risks: {
    total: number;
    by_status: Record<string, number>;
  };
  adrs: {
    total: number;
    by_status: Record<string, number>;
  };
  owners: {
    total: number;
  };
  recurring_properties: {
    total: number;
    by_enforcement_status: Record<string, number>;
  };
  standards: {
    total: number;
  };
}

export function buildScorecardExport(
  records: GraphRecord[] = buildSeedRecords(),
  adoptionScan: AdoptionScanResult = scanWorkspace(),
  now: Date = new Date()
): ScorecardExport {
  const repositories = records.filter((record) => record.type === "Repository");
  const findings = records.filter((record) => record.type === "Finding");
  const incidents = records.filter((record) => record.type === "Incident");
  const signals = records.filter((record) => record.type === "FactorySignal");
  const pullRequests = records.filter((record) => record.type === "PullRequest");
  const checkRuns = records.filter((record) => record.type === "CheckRun");
  const deployments = records.filter((record) => record.type === "Deployment");
  const risks = records.filter((record) => record.type === "Risk");
  const adrs = records.filter((record) => record.type === "ADR");
  const owners = records.filter((record) => record.type === "Owner");
  const properties = records.filter((record) => record.type === "RPProperty");
  const standards = records.filter((record) => record.type === "Standard");
  const edges = records.filter((record) => "from" in record && "to" in record);

  return {
    generated_at: now.toISOString(),
    source: "build-depot",
    runtime: {
      omnigraph_ingest: "deferred",
      note:
        "Omnigraph runtime is intentionally deferred; this export uses graph-compatible local records.",
    },
    graph_records: {
      total: records.length,
      nodes: records.length - edges.length,
      edges: edges.length,
      edge_counts: countBy(edges.map((edge) => edge.type)),
    },
    findings: {
      total: findings.length,
      open: findings.filter((finding) => finding.data.status === "Open").length,
      by_bucket: countBy(findings.map((finding) => finding.data.bucket)),
      by_status: countBy(findings.map((finding) => finding.data.status)),
    },
    repositories: {
      total: repositories.length,
      by_layer: countBy(
        repositories.map((repository) => repository.data.layer ?? "unknown")
      ),
      adoption_by_state: countBy(
        adoptionScan.repositories.map((repository) => repository.state)
      ),
      adoption_by_tier: countBy(
        adoptionScan.repositories.map((repository) => repository.tier)
      ),
      adoption_blocking_failures: adoptionScan.blockingFailures,
    },
    incidents: {
      total: incidents.length,
      open: incidents.filter((incident) => incident.data.status === "Open").length,
      by_severity: countBy(incidents.map((incident) => incident.data.severity)),
    },
    signals: {
      total: signals.length,
      total_events: signals.reduce(
        (total, signal) => total + (signal.data.event_count ?? 0),
        0
      ),
      affected_users: signals.reduce(
        (total, signal) => total + (signal.data.affected_users ?? 0),
        0
      ),
      by_category: countBy(signals.map((signal) => signal.data.category)),
      by_source: countBy(signals.map((signal) => signal.data.source)),
      by_status: countBy(signals.map((signal) => signal.data.status ?? "unknown")),
    },
    pull_requests: {
      total: pullRequests.length,
    },
    check_runs: {
      total: checkRuns.length,
      by_status: countBy(checkRuns.map((checkRun) => checkRun.data.status ?? "unknown")),
      by_conclusion: countBy(
        checkRuns.map((checkRun) => checkRun.data.conclusion ?? "unknown")
      ),
    },
    deployments: {
      total: deployments.length,
      by_environment: countBy(
        deployments.map((deployment) => deployment.data.environment ?? "unknown")
      ),
      by_status: countBy(
        deployments.map((deployment) => deployment.data.status ?? "unknown")
      ),
    },
    risks: {
      total: risks.length,
      by_status: countBy(risks.map((risk) => risk.data.status ?? "unknown")),
    },
    adrs: {
      total: adrs.length,
      by_status: countBy(adrs.map((adr) => adr.data.status ?? "unknown")),
    },
    owners: {
      total: owners.length,
    },
    recurring_properties: {
      total: properties.length,
      by_enforcement_status: countBy(
        properties.map((property) => property.data.enforcement_status)
      ),
    },
    standards: {
      total: standards.length,
    },
  };
}

export function main(): void {
  console.log(JSON.stringify(buildScorecardExport(), null, 2));
}

function countBy(values: string[]): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const value of values) {
    counts[value] = (counts[value] ?? 0) + 1;
  }
  return counts;
}

if (import.meta.main) {
  main();
}
