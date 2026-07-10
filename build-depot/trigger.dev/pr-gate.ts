import { logger, schemaTask } from "@trigger.dev/sdk";
import Anthropic from "@anthropic-ai/sdk";
import { z } from "zod";
import { REPO_CONTEXT } from "./repositories";

// ---------------------------------------------------------------------------
// Schemas
// ---------------------------------------------------------------------------

const GitHubPRPayloadSchema = z
  .object({
    action: z.string(),
    number: z.number().int().positive(),
    repository: z
      .object({
        name: z.string().min(1),
        full_name: z.string().min(1),
        owner: z.object({ login: z.string().min(1) }).passthrough(),
      })
      .passthrough(),
    pull_request: z
      .object({
        title: z.string(),
        body: z.string().nullable(),
        head: z.object({ sha: z.string() }).passthrough(),
        base: z.object({ ref: z.string() }).passthrough(),
        additions: z.number().int().nonnegative(),
        deletions: z.number().int().nonnegative(),
        changed_files: z.number().int().nonnegative(),
        html_url: z.string().url(),
      })
      .passthrough(),
  })
  .passthrough();

const PRFileSchema = z
  .object({
    filename: z.string(),
    status: z.string(),
    additions: z.number().int().nonnegative(),
    deletions: z.number().int().nonnegative(),
    patch: z.string().optional(),
  })
  .passthrough();

const FindingSchema = z.object({
  severity: z.enum(["critical", "major", "minor"]),
  area: z.enum(["correctness", "security", "test-coverage", "architecture"]),
  description: z.string(),
});

const AssessmentSchema = z.object({
  verdict: z.enum(["approve", "request-changes", "block"]),
  confidence: z.enum(["H", "M", "L"]),
  summary: z.string(),
  findings: z.array(FindingSchema),
});

type GitHubPRPayload = z.infer<typeof GitHubPRPayloadSchema>;
type PRFile = z.infer<typeof PRFileSchema>;
type Assessment = z.infer<typeof AssessmentSchema>;

// ---------------------------------------------------------------------------
const VERDICT_BADGE: Record<Assessment["verdict"], string> = {
  approve: "✅ Approve",
  "request-changes": "⚠️ Request Changes",
  block: "🚫 Block",
};

const CONFIDENCE_LABEL: Record<Assessment["confidence"], string> = {
  H: "High",
  M: "Medium",
  L: "Low",
};

// Large PRs: cap diff sent to Claude to keep tokens reasonable
const MAX_DIFF_CHARS = 6000;

// ---------------------------------------------------------------------------
// GitHub API helpers
// ---------------------------------------------------------------------------

function githubHeaders(): Record<string, string> {
  return {
    Authorization: `Bearer ${process.env.GITHUB_TOKEN}`,
    Accept: "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28",
  };
}

async function fetchPRFiles(
  owner: string,
  repo: string,
  prNumber: number
): Promise<PRFile[]> {
  const res = await fetch(
    `https://api.github.com/repos/${owner}/${repo}/pulls/${prNumber}/files?per_page=50`,
    { headers: githubHeaders() }
  );
  if (!res.ok) {
    throw new Error(`GitHub files API ${res.status}: ${await res.text()}`);
  }
  const body: unknown = await res.json();
  return z.array(PRFileSchema).parse(body);
}

async function postPRComment(
  owner: string,
  repo: string,
  prNumber: number,
  body: string
): Promise<void> {
  const res = await fetch(
    `https://api.github.com/repos/${owner}/${repo}/issues/${prNumber}/comments`,
    {
      method: "POST",
      headers: { ...githubHeaders(), "Content-Type": "application/json" },
      body: JSON.stringify({ body }),
    }
  );
  if (!res.ok) {
    throw new Error(`GitHub comment API ${res.status}: ${await res.text()}`);
  }
}

// ---------------------------------------------------------------------------
// Diff assembly
// ---------------------------------------------------------------------------

function buildDiffContext(files: PRFile[]): string {
  const sections: string[] = [];
  let chars = 0;

  for (const file of files) {
    const patch = file.patch ?? "(binary or no diff available)";
    const block = `\n### ${file.filename} (+${file.additions}/-${file.deletions})\n\`\`\`\n${patch}\n\`\`\`\n`;

    if (chars + block.length > MAX_DIFF_CHARS) {
      sections.push(`\n### ${file.filename} — diff omitted (size limit reached)\n`);
      break;
    }
    sections.push(block);
    chars += block.length;
  }

  return sections.join("");
}

// ---------------------------------------------------------------------------
// Analysis
// ---------------------------------------------------------------------------

async function analyze(
  payload: GitHubPRPayload,
  files: PRFile[]
): Promise<Assessment> {
  const repoName = payload.repository.name;
  const pr = payload.pull_request;
  const ctx = REPO_CONTEXT[repoName] ?? { layer: "unknown", purpose: "unknown" };
  const diffContext = buildDiffContext(files);

  const prompt = `You are the pr-gate quality engine for the Reflective Software Factory.

Repository: ${repoName} (${ctx.layer} layer — ${ctx.purpose})
PR #${payload.number}: ${pr.title}
Base branch: ${pr.base.ref}
Scale: ${pr.changed_files} files changed, +${pr.additions}/-${pr.deletions} lines

PR Description:
${pr.body?.trim() || "(no description provided)"}

Changed files and diffs:
${diffContext}

Assess this PR across four dimensions:
1. Correctness — logic errors, off-by-one, type mismatches, incorrect assumptions
2. Security — injection risks, secret exposure, auth bypass, unsafe deserialization
3. Test coverage — are changes covered? are edge cases tested?
4. Architecture — respects module boundaries and upstream-only dependency direction?

Return ONLY valid JSON, no markdown fences, no explanation:
{
  "verdict": "approve" | "request-changes" | "block",
  "confidence": "H" | "M" | "L",
  "summary": "two to three sentences for the PR author explaining the verdict",
  "findings": [
    {
      "severity": "critical" | "major" | "minor",
      "area": "correctness" | "security" | "test-coverage" | "architecture",
      "description": "specific, actionable — cite filename and context"
    }
  ]
}

Verdict rules:
- "block": any critical finding (security vulnerability, data loss risk, broken invariant)
- "request-changes": major findings, or significant logic without test coverage
- "approve": no findings, or only minor findings`;

  const client = new Anthropic();
  const message = await client.messages.create({
    model: "claude-sonnet-4-6",
    max_tokens: 1024,
    messages: [{ role: "user", content: prompt }],
  });

  const textBlock = message.content.find((block) => block.type === "text");
  if (!textBlock || textBlock.type !== "text") {
    throw new Error("Claude response did not include a text block");
  }

  // Strip markdown fences if the model wrapped output despite instructions
  const json = textBlock.text
    .replace(/^```(?:json)?\n?/, "")
    .replace(/\n?```$/, "")
    .trim();

  return AssessmentSchema.parse(JSON.parse(json));
}

// ---------------------------------------------------------------------------
// Comment formatting
// ---------------------------------------------------------------------------

function formatComment(assessment: Assessment): string {
  const badge = VERDICT_BADGE[assessment.verdict];
  const confidenceLabel = CONFIDENCE_LABEL[assessment.confidence];

  const findingsTable =
    assessment.findings.length > 0
      ? "\n\n| Severity | Area | Finding |\n|----------|------|---------|\n" +
        assessment.findings
          .map(
            (f) =>
              `| ${f.severity} | ${f.area} | ${markdownTableCell(f.description)} |`
          )
          .join("\n")
      : "";

  return `## Build-Depot PR Gate — ${badge}

${assessment.summary}${findingsTable}

<sub>Confidence: ${confidenceLabel} · powered by [Build-Depot](https://cloud.trigger.dev/projects/v3/proj_idrnrbkbldpxamvcvhqr)</sub>`;
}

function markdownTableCell(value: string): string {
  return value.replace(/\n/g, " ").replace(/\|/g, "\\|");
}

// ---------------------------------------------------------------------------
// Task
// ---------------------------------------------------------------------------

export const prGateTask = schemaTask({
  id: "pr-gate",
  schema: GitHubPRPayloadSchema,
  maxDuration: 120,
  retry: {
    maxAttempts: 2,
    minTimeoutInMs: 5000,
    maxTimeoutInMs: 30000,
    factor: 2,
  },
  run: async (payload) => {
    const { name: repoName, owner } = payload.repository;
    const prNumber = payload.number;

    if (!["opened", "synchronize", "reopened"].includes(payload.action)) {
      logger.log("pr-gate: skipping", { action: payload.action });
      return { skipped: true, reason: `action=${payload.action}` };
    }

    logger.log("pr-gate: starting", {
      repo: repoName,
      pr: prNumber,
      action: payload.action,
    });

    const files = await fetchPRFiles(owner.login, repoName, prNumber);
    logger.log("pr-gate: files fetched", { count: files.length });

    const assessment = await analyze(payload, files);
    logger.log("pr-gate: assessed", {
      verdict: assessment.verdict,
      confidence: assessment.confidence,
      findings: assessment.findings.length,
    });

    await postPRComment(owner.login, repoName, prNumber, formatComment(assessment));
    logger.log("pr-gate: comment posted");

    return {
      repo: repoName,
      pr: prNumber,
      verdict: assessment.verdict,
      confidence: assessment.confidence,
      findingCount: assessment.findings.length,
    };
  },
});
