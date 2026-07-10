type SecretPattern = {
  name: string;
  pattern: RegExp;
};

type Finding = {
  path: string;
  line: number;
  pattern: string;
};

const patterns: SecretPattern[] = [
  { name: "private-key", pattern: /-----BEGIN [A-Z ]*PRIVATE KEY-----/ },
  { name: "github-token", pattern: /\b(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9_]{30,}\b/ },
  { name: "github-pat", pattern: /\bgithub_pat_[A-Za-z0-9_]{40,}\b/ },
  { name: "anthropic-key", pattern: /\bsk-ant-[A-Za-z0-9_-]{20,}\b/ },
  { name: "openai-key", pattern: /\bsk-[A-Za-z0-9]{32,}\b/ },
  { name: "linear-key", pattern: /\blin_api_[A-Za-z0-9]{20,}\b/ },
  { name: "aws-access-key", pattern: /\bAKIA[0-9A-Z]{16}\b/ },
  { name: "slack-token", pattern: /\bxox[baprs]-[A-Za-z0-9-]{20,}\b/ },
];

const skippedPaths = new Set(["bun.lock"]);

function trackedFiles(): string[] {
  const result = Bun.spawnSync(
    ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
    {
    stdout: "pipe",
    stderr: "pipe",
    }
  );

  if (result.exitCode !== 0) {
    const stderr = new TextDecoder().decode(result.stderr);
    throw new Error(`git ls-files failed: ${stderr.trim()}`);
  }

  return new TextDecoder()
    .decode(result.stdout)
    .split("\0")
    .filter((file) => file.length > 0 && !skippedPaths.has(file));
}

async function scanFile(file: string): Promise<Finding[]> {
  const text = Bun.file(file);

  try {
    const contents = await text.text();
    const findings: Finding[] = [];
    const lines = contents.split(/\r?\n/);

    for (const [index, line] of lines.entries()) {
      for (const secretPattern of patterns) {
        if (secretPattern.pattern.test(line)) {
          findings.push({
            path: file,
            line: index + 1,
            pattern: secretPattern.name,
          });
        }
      }
    }

    return findings;
  } catch {
    return [];
  }
}

const findings = (await Promise.all(trackedFiles().map(scanFile))).flat();

if (findings.length === 0) {
  console.log("No obvious secrets detected in tracked or untracked non-ignored files.");
  process.exit(0);
}

console.error("Potential secrets detected:");
for (const finding of findings) {
  console.error(`${finding.path}:${finding.line} ${finding.pattern}`);
}

process.exit(1);

export {};
