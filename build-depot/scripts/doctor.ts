import { existsSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { z } from "zod";

type Category = "quality" | "security" | "delivery";

type Check = {
  id: string;
  title: string;
  pass: boolean;
  detail: string;
};

const root = process.cwd();

const PackageJsonSchema = z
  .object({
    packageManager: z.string().optional(),
    scripts: z.record(z.string()).optional(),
  })
  .passthrough();

const TsconfigSchema = z
  .object({
    compilerOptions: z
      .object({
        strict: z.boolean().optional(),
        noUncheckedIndexedAccess: z.boolean().optional(),
        exactOptionalPropertyTypes: z.boolean().optional(),
      })
      .passthrough(),
  })
  .passthrough();

function path(...parts: string[]): string {
  return join(root, ...parts);
}

function exists(relativePath: string): boolean {
  return existsSync(path(relativePath));
}

function isDirectory(relativePath: string): boolean {
  try {
    return statSync(path(relativePath)).isDirectory();
  } catch {
    return false;
  }
}

function read(relativePath: string): string {
  try {
    return readFileSync(path(relativePath), "utf8");
  } catch {
    return "";
  }
}

function parseJson<T>(relativePath: string, schema: z.ZodType<T>): T | undefined {
  const contents = read(relativePath);
  if (!contents) {
    return undefined;
  }

  try {
    return schema.parse(JSON.parse(contents));
  } catch {
    return undefined;
  }
}

function hasRecipe(justfile: string, recipe: string): boolean {
  return new RegExp(`^${recipe.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}:`, "m").test(
    justfile
  );
}

function check(id: string, title: string, pass: boolean, detail: string): Check {
  return { id, title, pass, detail };
}

function qualityChecks(): Check[] {
  const packageJson = parseJson("package.json", PackageJsonSchema);
  const scripts = packageJson?.scripts ?? {};
  const tsconfig = parseJson("tsconfig.json", TsconfigSchema);
  const compilerOptions = tsconfig?.compilerOptions;
  const justfile = read("Justfile");

  return [
    check(
      "Q1",
      "Bun lockfile is authoritative",
      exists("bun.lock") &&
        !exists("package-lock.json") &&
        !exists("pnpm-lock.yaml") &&
        !exists("yarn.lock"),
      "Expected bun.lock and no npm/pnpm/yarn lockfile."
    ),
    check(
      "Q2",
      "package.json exposes Bun quality scripts",
      packageJson?.packageManager?.startsWith("bun@") === true &&
        typeof scripts.check === "string" &&
        typeof scripts.test === "string" &&
        typeof scripts.ci === "string",
      "Expected packageManager bun@... plus check, test, and ci scripts."
    ),
    check(
      "Q3",
      "TypeScript strictness is enabled",
      compilerOptions?.strict === true &&
        compilerOptions.noUncheckedIndexedAccess === true &&
        compilerOptions.exactOptionalPropertyTypes === true,
      "Expected strict, noUncheckedIndexedAccess, and exactOptionalPropertyTypes."
    ),
    check(
      "Q4",
      "Justfile exposes the quality gate surface",
      hasRecipe(justfile, "check") &&
        hasRecipe(justfile, "test") &&
        hasRecipe(justfile, "ci") &&
        hasRecipe(justfile, "quality-doctor") &&
        hasRecipe(justfile, "factory-adoption-doctor"),
      "Expected check, test, ci, quality-doctor, and factory-adoption-doctor recipes."
    ),
    check(
      "Q5",
      "Normalizer and seed tests exist",
      exists("tests/debt-tracker.test.ts") && exists("tests/seed.test.ts"),
      "Expected debt-tracker and seed test files."
    ),
    check(
      "Q6",
      "Architecture and operations docs exist",
      exists("docs/architecture/software-factory-build-depot.md") &&
        exists("docs/operations/software-factory-quality-system.md") &&
        exists("docs/operations/quality-gates.md") &&
        exists("docs/operations/quality.md") &&
        exists("docs/operations/security.md") &&
        exists("docs/operations/reliable-delivery.md") &&
        exists("docs/operations/factory-scorecard.md") &&
        exists("docs/operations/repository-adoption.md"),
      "Expected canonical architecture and operations Markdown docs."
    ),
  ];
}

function securityChecks(): Check[] {
  const gitignore = read(".gitignore");
  const justfile = read("Justfile");
  const packageJson = parseJson("package.json", PackageJsonSchema);
  const scripts = packageJson?.scripts ?? {};
  const securityWorkflow = read(".github/workflows/security.yml");
  const securityDocs = read("docs/operations/security.md");

  return [
    check(
      "S1",
      "Environment files are ignored",
      /^\.env$/m.test(gitignore) && /^\.env\.\*$/m.test(gitignore),
      "Expected .env and .env.* in .gitignore."
    ),
    check(
      "S2",
      "Secret manager Terraform exists",
      exists("terraform/secrets.tf") && read("terraform/secrets.tf").includes("build-depot"),
      "Expected Terraform secret slots for Build-Depot."
    ),
    check(
      "S3",
      "Security recipes are operator-visible",
      hasRecipe(justfile, "secrets-scan") &&
        hasRecipe(justfile, "security-audit") &&
        hasRecipe(justfile, "security-doctor"),
      "Expected secrets-scan, security-audit, and security-doctor recipes."
    ),
    check(
      "S4",
      "Secret scanning is implemented in TypeScript",
      exists("scripts/secrets-scan.ts") &&
        exists("scripts/security-audit.ts") &&
        typeof scripts["security:secrets"] === "string" &&
        typeof scripts["security:audit"] === "string",
      "Expected TypeScript security scripts and package security commands."
    ),
    check(
      "S5",
      "Security workflow calls the Just recipe",
      securityWorkflow.includes("permissions:") &&
        securityWorkflow.includes("contents: read") &&
        securityWorkflow.includes("just security-audit"),
      "Expected .github/workflows/security.yml with read-only contents permission."
    ),
    check(
      "S6",
      "Security runbook documents secret handling",
      securityDocs.includes("## Secret Handling") &&
        securityDocs.includes("Scheduled Security"),
      "Expected secret handling and scheduled security sections."
    ),
  ];
}

function deliveryChecks(): Check[] {
  const justfile = read("Justfile");
  const ciWorkflow = read(".github/workflows/ci.yml");
  const deliveryWorkflow = read(".github/workflows/delivery.yml");

  return [
    check(
      "D1",
      "CI workflow is a thin Just runner",
      ciWorkflow.includes("just ci") && ciWorkflow.includes("bun install --frozen-lockfile"),
      "Expected CI workflow to install dependencies and call just ci."
    ),
    check(
      "D2",
      "Delivery workflow gates manual deploys",
      deliveryWorkflow.includes("workflow_dispatch") &&
        deliveryWorkflow.includes("just delivery-preflight") &&
        deliveryWorkflow.includes("just deploy"),
      "Expected manual delivery workflow with preflight before deploy."
    ),
    check(
      "D3",
      "Delivery recipes are operator-visible",
      hasRecipe(justfile, "delivery-preflight") &&
        hasRecipe(justfile, "delivery-doctor") &&
        hasRecipe(justfile, "deploy"),
      "Expected delivery-preflight, delivery-doctor, and deploy recipes."
    ),
    check(
      "D4",
      "Terraform deployment surface exists",
      exists("terraform/main.tf") &&
        exists("terraform/iam.tf") &&
        exists("terraform/storage.tf") &&
        exists("terraform/secrets.tf") &&
        exists("terraform/webhooks.tf"),
      "Expected main, IAM, storage, secrets, and webhook Terraform files."
    ),
    check(
      "D5",
      "Trigger.dev runtime files exist",
      exists("trigger.config.ts") &&
        isDirectory("trigger.dev") &&
        exists("trigger.dev/debt-tracker.ts") &&
        exists("trigger.dev/pr-gate.ts"),
      "Expected Trigger config and task files."
    ),
    check(
      "D6",
      "Graph schema and queries exist",
      exists("build-depot.pg") &&
        isDirectory("queries") &&
        exists("queries/repo_health.gq") &&
        exists("queries/open_incidents.gq") &&
        exists("queries/repository_adoption.gq"),
      "Expected graph schema and core query files."
    ),
  ];
}

const categories: Record<Category, () => Check[]> = {
  quality: qualityChecks,
  security: securityChecks,
  delivery: deliveryChecks,
};

function requestedCategories(): Category[] {
  const arg = process.argv[2] ?? "all";
  if (arg === "all" || arg === "doctor") {
    return ["quality", "security", "delivery"];
  }

  if (arg === "quality" || arg === "security" || arg === "delivery") {
    return [arg];
  }

  console.error("Usage: bun scripts/doctor.ts [all|quality|security|delivery]");
  process.exit(2);
}

let failures = 0;

for (const category of requestedCategories()) {
  console.log(`-- ${category}-doctor --`);

  for (const result of categories[category]()) {
    const mark = result.pass ? "OK" : "FAIL";
    console.log(`${mark} ${result.id} ${result.title}`);
    if (!result.pass) {
      console.log(`   ${result.detail}`);
      failures += 1;
    }
  }
}

if (failures > 0) {
  console.log(`-- FAIL doctor: ${failures} check(s) failed --`);
  process.exit(1);
}

console.log("-- OK doctor: all checks passed --");
