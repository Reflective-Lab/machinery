import { spawn, spawnSync } from "node:child_process";
import { createConnection } from "node:net";
import { dirname, resolve } from "node:path";
import { setTimeout as sleep } from "node:timers/promises";
import { fileURLToPath } from "node:url";
import { main as seedMain } from "./seed";

const SCRIPT_DIR = dirname(fileURLToPath(import.meta.url));
const PROJECT_ROOT = resolve(SCRIPT_DIR, "..");
const SERVER = "http://localhost:8080";
const GRAPH = "build-depot";

async function main(): Promise<void> {
  if (process.argv.includes("--help") || process.argv.includes("-h")) {
    printHelp();
    return;
  }

  step("Generating seed data from QUALITY_BACKLOG.md");
  seedMain();

  step("Validating cluster config");
  run("omnigraph", ["cluster", "validate", "--config", "."]);

  step("Applying cluster (creates graph + registers queries)");
  run("omnigraph", ["cluster", "plan", "--config", "."]);
  run("omnigraph", ["cluster", "apply", "--config", "."]);

  step("Starting omnigraph-server on localhost:8080");
  if (await isPortOpen(8080)) {
    console.log("Port 8080 already in use - assuming server is running");
  } else {
    const server = spawn(
      "omnigraph-server",
      ["--cluster", PROJECT_ROOT, "--bind", "localhost:8080", "--unauthenticated"],
      {
        cwd: PROJECT_ROOT,
        detached: true,
        stdio: "ignore",
      }
    );
    server.unref();
    console.log(`Server PID ${server.pid ?? "unknown"}`);
    await sleep(2_000);
  }

  step("Loading seed data into the server");
  run("omnigraph", [
    "load",
    "--data",
    "seed/seed.jsonl",
    "--mode",
    "merge",
    "--server",
    SERVER,
    "--graph",
    GRAPH,
  ]);

  step("Smoke test: rp_status query");
  run("omnigraph", ["query", "rp_status", "--server", SERVER, "--graph", GRAPH]);

  console.log("");
  console.log("=== Setup complete ===");
  console.log("");
  console.log(`Server:      ${SERVER}`);
  console.log(`${"MCP route:"}   ${SERVER}/graphs/${GRAPH}/mcp`);
  console.log("");
  console.log("Restart Claude Code (or /mcp reset) to activate the MCP server.");
  console.log("");
  console.log("Quick queries:");
  console.log(`  omnigraph query open_findings --server ${SERVER} --graph ${GRAPH}`);
  console.log(`  omnigraph query bucket_a      --server ${SERVER} --graph ${GRAPH}`);
  console.log(`  omnigraph query codex_safe    --server ${SERVER} --graph ${GRAPH}`);
  console.log(`  omnigraph query repo_health   --server ${SERVER} --graph ${GRAPH}`);
  console.log(`  omnigraph query open_incidents --server ${SERVER} --graph ${GRAPH}`);
  console.log(
    `  omnigraph query open_by_bucket --params '{"bucket":"B"}' --server ${SERVER} --graph ${GRAPH}`
  );
}

function step(message: string): void {
  console.log("");
  console.log(`--- ${message} ---`);
}

function printHelp(): void {
  console.log("Build-Depot Omnigraph setup");
  console.log("");
  console.log("Usage:");
  console.log("  bun run setup");
  console.log("  just setup");
  console.log("");
  console.log("Requires omnigraph and omnigraph-server in PATH.");
}

function run(command: string, args: string[]): void {
  const result = spawnSync(command, args, {
    cwd: PROJECT_ROOT,
    stdio: "inherit",
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} exited with ${result.status ?? "signal"}`);
  }
}

async function isPortOpen(port: number): Promise<boolean> {
  return await new Promise((resolvePort) => {
    const socket = createConnection({ host: "127.0.0.1", port });
    socket.once("connect", () => {
      socket.destroy();
      resolvePort(true);
    });
    socket.once("error", () => {
      socket.destroy();
      resolvePort(false);
    });
  });
}

if (import.meta.main) {
  await main();
}
