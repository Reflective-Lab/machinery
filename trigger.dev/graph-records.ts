import type { RepositoryNodeData } from "./repositories";

export type FindingStatus =
  | "Open"
  | "InProgress"
  | "Done"
  | "Superseded"
  | "AcceptedRisk"
  | "WontDo";

export type IncidentStatus = "Open" | "Resolved" | "Ignored";
export type IncidentSeverity = "P1" | "P2" | "P3";
export type FactorySignalCategory =
  | "quality_gate"
  | "security_scan"
  | "dependency"
  | "test"
  | "github_lifecycle"
  | "runtime_telemetry"
  | "product_feedback"
  | "architecture_drift"
  | "agent_behavior"
  | "operational_hygiene"
  | "data_durability"
  | "cost_capacity"
  | "delivery"
  | "repository_adoption";

export interface FindingNodeData {
  id: string;
  date: string;
  bucket: string;
  area: string;
  status: FindingStatus;
  title: string;
  effort?: string;
  owner?: string;
  codex_safe?: string;
  confidence?: string;
  cycles_open?: number;
  last_reviewed?: string;
  standard_promoted?: string;
  drift_check?: string;
  next_action?: string;
  repo?: string;
}

export interface RPPropertyNodeData {
  id: string;
  property_name: string;
  enforcement_status: "Enforced" | "Aspired";
  tracked_by?: string;
}

export interface StandardNodeData {
  path: string;
  title: string;
}

export interface IncidentNodeData {
  id: string;
  repo: string;
  title?: string;
  severity: IncidentSeverity;
  status: IncidentStatus;
  opened_at: string;
  linked_finding?: string;
}

export interface PullRequestNodeData {
  id: string;
  repo: string;
  number: number;
  title?: string;
  url?: string;
  state?: string;
  head_sha?: string;
}

export interface CheckRunNodeData {
  id: string;
  repo: string;
  name?: string;
  status?: string;
  conclusion?: string;
  completed_at?: string;
}

export interface DeploymentNodeData {
  id: string;
  repo: string;
  environment?: string;
  status?: string;
  version?: string;
  deployed_at?: string;
}

export interface RiskNodeData {
  id: string;
  title?: string;
  status?: string;
  owner?: string;
  revisit_date?: string;
}

export interface ADRNodeData {
  path: string;
  title: string;
  status?: string;
}

export interface OwnerNodeData {
  id: string;
  name: string;
  email?: string;
}

export interface FactorySignalNodeData {
  id: string;
  aggregate_key?: string;
  category: FactorySignalCategory;
  source: string;
  kind: string;
  title: string;
  status?: string;
  severity?: string;
  observed_at: string;
  first_seen_at?: string;
  last_seen_at?: string;
  event_count?: number;
  affected_users?: number;
  repo?: string;
  finding_id?: string;
  external_url?: string;
  evidence_ref?: string;
  metric_name?: string;
  metric_value?: number;
  unit?: string;
}

export type NodeGraphRecord =
  | { type: "Repository"; data: RepositoryNodeData }
  | { type: "RPProperty"; data: RPPropertyNodeData }
  | { type: "Standard"; data: StandardNodeData }
  | { type: "Finding"; data: FindingNodeData }
  | { type: "Incident"; data: IncidentNodeData }
  | { type: "PullRequest"; data: PullRequestNodeData }
  | { type: "CheckRun"; data: CheckRunNodeData }
  | { type: "Deployment"; data: DeploymentNodeData }
  | { type: "Risk"; data: RiskNodeData }
  | { type: "ADR"; data: ADRNodeData }
  | { type: "Owner"; data: OwnerNodeData }
  | { type: "FactorySignal"; data: FactorySignalNodeData };

export type FindingReference = {
  type: "Finding";
  key: { id: string };
};

export type RepositoryReference = {
  type: "Repository";
  key: { name: string };
};

export type RPPropertyReference = {
  type: "RPProperty";
  key: { id: string };
};

export type StandardReference = {
  type: "Standard";
  key: { path: string };
};

export type IncidentReference = {
  type: "Incident";
  key: { id: string };
};

export type PullRequestReference = {
  type: "PullRequest";
  key: { id: string };
};

export type CheckRunReference = {
  type: "CheckRun";
  key: { id: string };
};

export type DeploymentReference = {
  type: "Deployment";
  key: { id: string };
};

export type RiskReference = {
  type: "Risk";
  key: { id: string };
};

export type ADRReference = {
  type: "ADR";
  key: { path: string };
};

export type OwnerReference = {
  type: "Owner";
  key: { id: string };
};

export type FactorySignalReference = {
  type: "FactorySignal";
  key: { id: string };
};

export type EdgeGraphRecord =
  | {
      type: "FindingInRepo";
      from: FindingReference;
      to: RepositoryReference;
    }
  | {
      type: "IncidentInRepo";
      from: IncidentReference;
      to: RepositoryReference;
    }
  | {
      type: "FindingWorksOnRP";
      from: FindingReference;
      to: RPPropertyReference;
    }
  | {
      type: "FindingToStandard";
      from: FindingReference;
      to: StandardReference;
    }
  | {
      type: "FindingSupersedes";
      from: FindingReference;
      to: FindingReference;
    }
  | {
      type: "PullRequestAddressesFinding";
      from: PullRequestReference;
      to: FindingReference;
    }
  | {
      type: "CheckRunInRepo";
      from: CheckRunReference;
      to: RepositoryReference;
    }
  | {
      type: "DeploymentInRepo";
      from: DeploymentReference;
      to: RepositoryReference;
    }
  | {
      type: "RiskTracksFinding";
      from: RiskReference;
      to: FindingReference;
    }
  | {
      type: "ADRDecidesFinding";
      from: ADRReference;
      to: FindingReference;
    }
  | {
      type: "OwnerOwnsFinding";
      from: OwnerReference;
      to: FindingReference;
    }
  | {
      type: "SignalInRepo";
      from: FactorySignalReference;
      to: RepositoryReference;
    }
  | {
      type: "SignalSupportsFinding";
      from: FactorySignalReference;
      to: FindingReference;
    };

export type GraphRecord = NodeGraphRecord | EdgeGraphRecord;

export function recordsToNdjson(records: GraphRecord[]): string {
  return `${records.map((record) => JSON.stringify(record)).join("\n")}\n`;
}

export function findingRef(id: string): FindingReference {
  return { type: "Finding", key: { id } };
}

export function repositoryRef(name: string): RepositoryReference {
  return { type: "Repository", key: { name } };
}

export function incidentRef(id: string): IncidentReference {
  return { type: "Incident", key: { id } };
}

export function rpPropertyRef(id: string): RPPropertyReference {
  return { type: "RPProperty", key: { id } };
}

export function standardRef(path: string): StandardReference {
  return { type: "Standard", key: { path } };
}

export function pullRequestRef(id: string): PullRequestReference {
  return { type: "PullRequest", key: { id } };
}

export function checkRunRef(id: string): CheckRunReference {
  return { type: "CheckRun", key: { id } };
}

export function deploymentRef(id: string): DeploymentReference {
  return { type: "Deployment", key: { id } };
}

export function riskRef(id: string): RiskReference {
  return { type: "Risk", key: { id } };
}

export function adrRef(path: string): ADRReference {
  return { type: "ADR", key: { path } };
}

export function ownerRef(id: string): OwnerReference {
  return { type: "Owner", key: { id } };
}

export function factorySignalRef(id: string): FactorySignalReference {
  return { type: "FactorySignal", key: { id } };
}
