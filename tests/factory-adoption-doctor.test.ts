import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { spawnSync } from "node:child_process";
import { describe, expect, test } from "bun:test";
import {
  adoptionRecordsFromScan,
  evaluateRepository,
  scanWorkspace,
  type AdoptionSignalResult,
} from "../scripts/factory-adoption-doctor";

describe("factory adoption doctor", () => {
  test("marks Build-Depot adopted when the control-plane contract is present", () => {
    const workspace = fixtureWorkspace();
    writeFileSync(join(workspace, "release-train.yaml"), "version: 1\nprojects: []\n");
    const repo = repoFixture(workspace, "build-depot", {
      agents:
        "Build-Depot is the canonical software-factory policy source. module:build-depot\n",
      justfile: [
        "ci:",
        "    bun run ci",
        "security-audit:",
        "    bun run security:audit",
        "delivery-preflight:",
        "    bun run delivery:preflight",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
    });

    const result = evaluateRepository(repo);

    expect(result.tier).toBe("Tier0");
    expect(result.state).toBe("adopted");
    expect(result.missingRequiredSignals).toEqual([]);
  });

  test("blocks Tier 1 repos with untracked required gaps", () => {
    const workspace = fixtureWorkspace();
    writeFileSync(
      join(workspace, "release-train.yaml"),
      "version: 1\nprojects:\n  - name: runway\n    dir: runtime-runway\n"
    );
    repoFixture(workspace, "runtime-runway", {
      agents:
        "This repo points to Build-Depot for software-factory policy. module:runtime-runway\n",
      justfile: "ci:\n    cargo test\n",
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Sentry project: runtime-runway\n",
    });

    const scan = scanWorkspace({
      workspaceRoot: workspace,
      now: new Date("2026-07-08T00:00:00.000Z"),
      sourceRevision: "test",
    });
    const repo = scan.repositories.find((candidate) => candidate.name === "runtime-runway");

    expect(repo?.tier).toBe("Tier1");
    expect(repo?.state).toBe("blocked");
    expect(repo?.missingRequiredSignals).toContain("security");
    expect(repo?.missingRequiredSignals).toContain("delivery");
    expect(scan.blockingFailures).toBe(1);
  });

  test("blocks active repo adoption verdicts when the checkout is behind upstream", () => {
    const workspace = fixtureWorkspace();
    const repoPath = join(workspace, "runtime-runway");
    const remotePath = join(workspace, "runtime-runway.git");
    const writerPath = join(workspace, "writer");
    mkdirSync(repoPath, { recursive: true });
    writeAdoptionFiles(repoPath, {
      agents:
        "This repo points to Build-Depot for software-factory policy. module:runtime-runway\n",
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo test",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
    });

    git(repoPath, ["init"]);
    git(repoPath, ["checkout", "-b", "main"]);
    git(repoPath, ["config", "user.email", "factory@example.test"]);
    git(repoPath, ["config", "user.name", "Factory Test"]);
    git(repoPath, ["add", "."]);
    git(repoPath, ["commit", "-m", "initial"]);
    git(workspace, ["init", "--bare", remotePath]);
    git(repoPath, ["remote", "add", "origin", remotePath]);
    git(repoPath, ["push", "-u", "origin", "main"]);

    git(workspace, ["clone", remotePath, writerPath]);
    git(writerPath, ["config", "user.email", "factory@example.test"]);
    git(writerPath, ["config", "user.name", "Factory Test"]);
    writeFileSync(join(writerPath, "README.md"), "remote update\n");
    git(writerPath, ["add", "README.md"]);
    git(writerPath, ["commit", "-m", "remote update"]);
    git(writerPath, ["push", "origin", "main"]);
    git(repoPath, ["fetch", "origin"]);

    const result = evaluateRepository({ name: "runtime-runway", path: repoPath });

    expect(result.state).toBe("blocked");
    expect(result.missingRequiredSignals).toEqual(["checkout-current"]);
    expect(signal(result.signals, "checkout-current").detail).toContain(
      "behind origin/main"
    );
  });

  test("blocks the adoption verdict when the upstream cannot be fetched", () => {
    const workspace = fixtureWorkspace();
    const repoPath = join(workspace, "runtime-runway");
    const remotePath = join(workspace, "runtime-runway.git");
    mkdirSync(repoPath, { recursive: true });
    writeAdoptionFiles(repoPath, {
      agents:
        "This repo points to Build-Depot for software-factory policy. module:runtime-runway\n",
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo test",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
    });

    git(repoPath, ["init"]);
    git(repoPath, ["checkout", "-b", "main"]);
    git(repoPath, ["config", "user.email", "factory@example.test"]);
    git(repoPath, ["config", "user.name", "Factory Test"]);
    git(repoPath, ["add", "."]);
    git(repoPath, ["commit", "-m", "initial"]);
    git(workspace, ["init", "--bare", remotePath]);
    git(repoPath, ["remote", "add", "origin", remotePath]);
    git(repoPath, ["push", "-u", "origin", "main"]);

    rmSync(remotePath, { recursive: true, force: true });

    const result = evaluateRepository({ name: "runtime-runway", path: repoPath });

    expect(result.state).toBe("blocked");
    expect(result.missingRequiredSignals).toEqual(["checkout-current"]);
    expect(signal(result.signals, "checkout-current").detail).toContain(
      "Could not fetch origin"
    );
  });

  test("treats accepted-risk or Linear references as tracked exceptions", () => {
    const workspace = fixtureWorkspace();
    const repo = repoFixture(workspace, "runtime-runway", {
      agents: [
        "This repo points to Build-Depot for software-factory policy.",
        "Security audit gap RFL-166 accepted risk revisit 2026-08-01.",
        "module:runtime-runway",
      ].join("\n"),
      justfile: [
        "ci:",
        "    cargo test",
        "release-preflight:",
        "    cargo test",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Sentry project: runtime-runway\n",
    });

    const result = evaluateRepository(repo, new Set(["runtime-runway"]));
    const security = signal(result.signals, "security");

    expect(result.state).toBe("adopted");
    expect(security.status).toBe("exception");
    expect(security.reference).toContain("RFL-166");
  });

  test("keeps template repositories exempt from active gate requirements", () => {
    const workspace = fixtureWorkspace();
    repoFixture(workspace, "forge-templates", {
      agents: "Experimental template repo.\n",
      justfile: "",
      workflow: "",
      docs: "Template examples.\n",
    });

    const scan = scanWorkspace({
      workspaceRoot: workspace,
      now: new Date("2026-07-08T00:00:00.000Z"),
      sourceRevision: "test",
    });
    const repo = scan.repositories.find((candidate) => candidate.name === "forge-templates");

    expect(repo?.tier).toBe("Tier3");
    expect(repo?.state).toBe("exempt");
    expect(scan.blockingFailures).toBe(0);
  });

  test("converts scan output into repository adoption graph records", () => {
    const workspace = fixtureWorkspace();
    writeFileSync(
      join(workspace, "release-train.yaml"),
      "version: 1\nprojects:\n  - name: runway\n    dir: runtime-runway\n"
    );
    repoFixture(workspace, "build-depot", {
      agents:
        "Build-Depot is the canonical software-factory policy source. module:build-depot\n",
      justfile: [
        "ci:",
        "    bun run ci",
        "security-audit:",
        "    bun run security:audit",
        "delivery-preflight:",
        "    bun run delivery:preflight",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
    });
    repoFixture(workspace, "runtime-runway", {
      agents:
        "This repo points to Build-Depot for software-factory policy. module:runtime-runway\n",
      justfile: "ci:\n    cargo test\n",
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Sentry project: runtime-runway\n",
    });

    const records = adoptionRecordsFromScan(
      scanWorkspace({
        workspaceRoot: workspace,
        now: new Date("2026-07-08T00:00:00.000Z"),
        sourceRevision: "test",
      })
    );

    expect(records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "build-depot",
        adoption_tier: "Tier0",
        adoption_state: "adopted",
        adoption_last_scanned: "2026-07-08T00:00:00.000Z",
        adoption_scan_revision: "test",
      }),
    });
    expect(records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "runtime-runway",
        adoption_tier: "Tier1",
        adoption_state: "blocked",
        adoption_missing_required: "security,delivery",
      }),
    });
  });

  test("uses factory cohort manifest as a virtual wrapper", () => {
    const workspace = fixtureWorkspace();
    writeFileSync(
      join(workspace, "factory-cohorts.json"),
      JSON.stringify({
        version: 1,
        cohorts: [
          {
            id: "A",
            name: "Factory proof cohort",
            wrapper: "virtual",
            members: [
              { repo: "forge-templates", tier: "Tier2" },
              { repo: "runtime-runway", tier: "Tier1" },
            ],
          },
        ],
      })
    );
    repoFixture(workspace, "forge-templates", {
      agents:
        "This repo points to Build-Depot for software-factory policy. module:forge-templates\n",
      justfile: "ci:\n    bun test\nsecurity-audit:\n    bun audit\n",
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Template repo follows Build-Depot factory docs.\n",
    });

    const scan = scanWorkspace({
      workspaceRoot: workspace,
      now: new Date("2026-07-08T00:00:00.000Z"),
      sourceRevision: "test",
    });
    const repo = scan.repositories.find((candidate) => candidate.name === "forge-templates");
    const records = adoptionRecordsFromScan(scan);

    expect(repo?.cohort).toBe("A");
    expect(repo?.tier).toBe("Tier2");
    expect(repo?.state).toBe("adopted");
    expect(records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "forge-templates",
        adoption_cohort: "A",
        adoption_tier: "Tier2",
        adoption_state: "adopted",
      }),
    });
  });

  test("marks Bedrock adopted when Shipyard Cargo facts stay and private ops are depot-owned", () => {
    const workspace = fixtureWorkspace();
    writeFileSync(
      join(workspace, "factory-cohorts.json"),
      JSON.stringify({
        version: 1,
        cohorts: [
          {
            id: "C",
            name: "Bedrock consolidation transition",
            wrapper: "bedrock-consolidated",
            members: [{ repo: "bedrock-consolidated", tier: "Tier1" }],
          },
        ],
      })
    );
    repoFixture(workspace, "bedrock-consolidated", {
      agents: [
        "This repo points to Build-Depot for software-factory policy.",
        "module:bedrock-consolidated",
        "Sentry: not applicable because this is a library workspace and emits no runtime incidents.",
      ].join("\n"),
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo publish --dry-run -p converge-model",
        "publish-dry:",
        "    cargo publish --dry-run -p converge-model",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy and private distribution operations.\n",
      cargoManifest:
        '[workspace.dependencies]\nconverge-model = { path = "foundation/converge/crates/model", version = "4.0.0", registry = "reflective-labs" }\n',
    });

    const scan = scanWorkspace({
      workspaceRoot: workspace,
      now: new Date("2026-07-08T00:00:00.000Z"),
      sourceRevision: "test",
    });
    const repo = scan.repositories.find(
      (candidate) => candidate.name === "bedrock-consolidated"
    );
    const records = adoptionRecordsFromScan(scan);

    expect(repo?.cohort).toBe("C");
    expect(repo?.tier).toBe("Tier1");
    expect(repo?.state).toBe("adopted");
    expect(signal(repo?.signals ?? [], "shipyard-registry").status).toBe("present");
    expect(signal(repo?.signals ?? [], "depot-distribution").status).toBe("present");
    expect(records).toContainEqual({
      type: "Repository",
      data: expect.objectContaining({
        name: "bedrock-consolidated",
        language: "Rust",
        layer: "platform",
        linear_label: "module:bedrock-consolidated",
        deployed_to: "shipyard.rs/reflective-labs",
        adoption_cohort: "C",
        adoption_tier: "Tier1",
        adoption_state: "adopted",
      }),
    });
  });

  test("blocks Bedrock when Shipyard Cargo registry attribution is missing", () => {
    const workspace = fixtureWorkspace();
    const repo = repoFixture(workspace, "bedrock-consolidated", {
      agents: [
        "This repo points to Build-Depot for software-factory policy.",
        "module:bedrock-consolidated",
        "Sentry: not applicable because this is a library workspace and emits no runtime incidents.",
      ].join("\n"),
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo publish --dry-run -p converge-model",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
      cargoManifest:
        '[workspace.dependencies]\nconverge-model = { path = "foundation/converge/crates/model", version = "4.0.0" }\n',
    });

    const result = evaluateRepository(repo);

    expect(result.tier).toBe("Tier1");
    expect(result.state).toBe("blocked");
    expect(result.missingRequiredSignals).toContain("shipyard-registry");
  });

  test("allows Bedrock to keep non-secret Shipyard registry index config for Cargo metadata", () => {
    const workspace = fixtureWorkspace();
    const repo = repoFixture(workspace, "bedrock-consolidated", {
      agents: [
        "This repo points to Build-Depot for software-factory policy.",
        "module:bedrock-consolidated",
        "Sentry: not applicable because this is a library workspace and emits no runtime incidents.",
      ].join("\n"),
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo publish --dry-run -p converge-model",
      ].join("\n"),
      workflow: "jobs:\n  ci:\n    steps:\n      - run: just ci\n",
      docs: "Build-Depot owns software-factory policy.\n",
      cargoManifest:
        '[workspace.dependencies]\nconverge-model = { path = "foundation/converge/crates/model", version = "4.0.0", registry = "reflective-labs" }\n',
      cargoConfig:
        '[registries.reflective-labs]\nindex = "ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git"\n',
    });

    const result = evaluateRepository(repo);

    expect(result.tier).toBe("Tier1");
    expect(result.state).toBe("adopted");
    expect(result.missingRequiredSignals).toEqual([]);
    expect(signal(result.signals, "depot-distribution").status).toBe("present");
  });

  test("blocks Bedrock when Shipyard publish secrets are wired in repo workflows", () => {
    const workspace = fixtureWorkspace();
    const repo = repoFixture(workspace, "bedrock-consolidated", {
      agents: [
        "This repo points to Build-Depot for software-factory policy.",
        "module:bedrock-consolidated",
        "Sentry: not applicable because this is a library workspace and emits no runtime incidents.",
      ].join("\n"),
      justfile: [
        "ci:",
        "    cargo test",
        "security-audit:",
        "    cargo audit",
        "delivery-preflight:",
        "    cargo publish --dry-run -p converge-model",
      ].join("\n"),
      workflow: [
        "jobs:",
        "  publish:",
        "    steps:",
        "      - run: cargo publish --registry reflective-labs",
        "        env:",
        "          SHIPYARD_TOKEN: ${{ secrets.SHIPYARD_TOKEN }}",
      ].join("\n"),
      docs:
        "Build-Depot owns software-factory policy. Shipyard transition tracked by RFL-174.\n",
      cargoManifest:
        '[workspace.dependencies]\nconverge-model = { path = "foundation/converge/crates/model", version = "4.0.0", registry = "reflective-labs" }\n',
      cargoConfig:
        '[registries.reflective-labs]\nindex = "ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git"\n',
    });

    const result = evaluateRepository(repo);

    expect(result.tier).toBe("Tier1");
    expect(result.state).toBe("blocked");
    expect(signal(result.signals, "depot-distribution").status).toBe("missing");
    expect(result.missingRequiredSignals).toContain("depot-distribution");
  });
});

function fixtureWorkspace(): string {
  return mkdtempSync(join(tmpdir(), "build-depot-adoption-"));
}

function repoFixture(
  workspace: string,
  name: string,
  files: {
    agents: string;
    justfile: string;
    workflow: string;
    docs: string;
    cargoManifest?: string;
    cargoConfig?: string;
  }
): { name: string; path: string } {
  const repoPath = join(workspace, name);
  mkdirSync(join(repoPath, ".git"), { recursive: true });
  writeFileSync(join(repoPath, ".git", "HEAD"), "test\n");
  writeAdoptionFiles(repoPath, files);
  return { name, path: repoPath };
}

function writeAdoptionFiles(
  repoPath: string,
  files: {
    agents: string;
    justfile: string;
    workflow: string;
    docs: string;
    cargoManifest?: string;
    cargoConfig?: string;
  }
): void {
  mkdirSync(join(repoPath, ".github", "workflows"), { recursive: true });
  mkdirSync(join(repoPath, "docs"), { recursive: true });
  writeFileSync(join(repoPath, "AGENTS.md"), files.agents);
  writeFileSync(join(repoPath, "Justfile"), files.justfile);
  writeFileSync(join(repoPath, ".github", "workflows", "ci.yml"), files.workflow);
  writeFileSync(join(repoPath, "docs", "factory.md"), files.docs);
  if (files.cargoManifest) {
    writeFileSync(join(repoPath, "Cargo.toml"), files.cargoManifest);
  }
  if (files.cargoConfig) {
    mkdirSync(join(repoPath, ".cargo"), { recursive: true });
    writeFileSync(join(repoPath, ".cargo", "config.toml"), files.cargoConfig);
  }
}

function git(cwd: string, args: string[]): void {
  const result = spawnSync("git", args, {
    cwd,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });

  if (result.status !== 0) {
    throw new Error(
      `git ${args.join(" ")} failed\nstdout: ${result.stdout}\nstderr: ${result.stderr}`
    );
  }
}

function signal(signals: AdoptionSignalResult[], id: string): AdoptionSignalResult {
  const result = signals.find((candidate) => candidate.id === id);
  if (!result) throw new Error(`Missing signal ${id}`);
  return result;
}
