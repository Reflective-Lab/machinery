type AuditStep = {
  name: string;
  command: string[];
};

const steps: AuditStep[] = [
  { name: "dependency audit", command: ["bun", "audit"] },
  { name: "secret scan", command: ["bun", "scripts/secrets-scan.ts"] },
];

let failures = 0;

for (const step of steps) {
  console.log(`-- ${step.name} --`);
  const result = Bun.spawnSync(step.command, {
    stdout: "inherit",
    stderr: "inherit",
  });

  if (result.exitCode !== 0) {
    failures += 1;
  }
}

if (failures > 0) {
  console.error(`security audit failed: ${failures} step(s) failed`);
  process.exit(1);
}

console.log("security audit passed");

export {};
