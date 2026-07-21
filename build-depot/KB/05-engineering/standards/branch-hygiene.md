# Standard: Branch hygiene — no stranded work

**Property:** `RP-BRANCH-HYGIENE`.
**Enforcement:** `just project-doctor` check 8 (root `Justfile`), wired
into `.github/workflows/doctor.yml`'s `project-doctor` job.
**Origin:** `QF-2026-07-02-06`. On 2026-07-03 Karl ruled all four fleet
stashes (converge kernel-surface WIP, runtime-runway builder.rs WIP,
mobile-apps refiner WIP ×2) stale and wasted work. That write-off is the
cost this standard exists to prevent.

## The loop

Every change rides one traceable loop, and the loop is *tight* — days,
not weeks:

1. **Linear issue** — the intent, with a real description and checklist.
2. **Branch** — `e{N}/lin-XX-slug` (AGENTS.md § Branch and worktree
   convention). The name ties the tree to the issue.
3. **Commits** — including `wip:` commits. Unfinished work is *committed
   to the issue branch*, never stashed. A commit on `e5/lin-67-...`
   carries its intent in the ref name; a stash carries nothing.
4. **PR** — body links the issue (`Fixes: <linear-url>`); merge closes it.
5. **Delete the branch.**

## The rules

1. **No stashes across sessions.** `git stash` is a within-session tool
   (e.g. clearing the tree for a pull). By session end every stash is
   either applied and committed to an issue branch, or dropped. A stash
   that survives a session is stranded work by definition — the doctor
   fails on it.
2. **WIP is commits on the issue branch.** "Not ready" is what `wip:`
   commit prefixes and squash-on-merge are for. Committed WIP is pushed,
   named, and diffable; stashed WIP is single-machine, anonymous, and
   rots.
3. **One route to main.** A change reaches `main` either by direct push
   (docs-only exemption) or by riding its branch through a PR — never
   both. Cherry-picking a `next`/branch commit onto `main` plants
   patch-identical twins that make every later merge suspect (seen
   2026-07-03: root `4f080d8`/`4ef2768`).
4. **End-of-session invariant.** Zero stashes; tracked tree clean or the
   dirt belongs to an open issue branch you'll resume; no local-only
   commits sitting on `main`/`next` unpushed. Anything that can't meet
   the invariant gets a Linear issue *the same day* — debts surface when
   they're created, not at the next health review.

## Enforcement

`project-doctor` check 8 sweeps every git repo in the workspace (depth ≤ 3):

- **Any stash anywhere → check fails.** Stashes have no legitimate
  resting state in this fleet.
- **Dirty tracked files → warning**, listed per repo. Dirt is normal
  mid-session and CI checkouts are always clean, so this is a local
  nudge, not a gate — but it prints on every doctor run, which is the
  tight loop doing its job.

## Cross-references

- `AGENTS.md` § Project Tracking / Branch and worktree convention — the
  loop this standard hardens.
- `KB/05-engineering/standards/repo-layering.md` — sibling standard;
  same doctrine of "make the debt explicit or don't create it".
- `QUALITY_BACKLOG.md` — `QF-2026-07-02-06` (the finding), RP row.
