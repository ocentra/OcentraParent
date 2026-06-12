# Codex Cloud Handoff: Doc Routing / Plan-Gap Audit

Use this as your single source of context before resuming.

Last handoff point: continuation of objective "some plans have good structure, some don't, some we don't have proper plans, and nothing should be marked done without evidence."

Repository: `E:\OcentraParent`  
Branch: `codex/doc-docs-gap-audit-pass2`  
Last commit: `784b925ef`

## Why this branch exists

This branch is the active branch for this doc-audit objective. I removed old duplicate cleanup branches to avoid split-brain state.

Superseded branches removed:
- `codex/doc-routing-gaps-pass1` (deleted local + remote)
- `codex/doc-update-agent-routing-v3` (deleted local + remote)

Current local branches:
- `codex/doc-docs-gap-audit-pass2`
- `codex/local-ai-activity-memory-graph`
- `codex/network-flow-v4`
- `main`
- `refactor-codex-worktree-structure`

Current remote branches:
- `origin/codex/doc-docs-gap-audit-pass2`
- `origin/codex/tracking-plan-full-continuation-a`
- `origin/main`
- `origin/production`

## What was done before this handoff

1) Branch hygiene and cloud readiness
- `codex/doc-docs-gap-audit-pass2` tracks `origin/codex/doc-docs-gap-audit-pass2`.
- Working tree is clean.
- Remote old cleanup branches were removed from GitHub.

2) Current docs structure baseline
- `docs/plans` contains **19** plans.
- Each plan folder currently has required per-plan files (AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, PLAN_HEALTH.md, WORKPACK_INDEX.md, CHECKLIST_INDEX.md, PROOF_INDEX.md, ARCHIVE_INDEX.md, ROUTE_INDEX.md, DOC_INDEX.md).
- No blanket "missing plan folder" condition remains.

3) Mapping / health baseline
- `docs/FEATURE_ROUTE_INDEX.md` maps all 18 feature docs to plan owners.
- `docs/PLAN_HEALTH_INDEX.md` identifies explicit first-pass plans:
  - `setup-install-provisioning-plan`
  - `account-identity-family-plan`
  - `data-custody-storage-plan`
  - `payment-subscription-plan`
  - `policy-control-plane-plan`
  - `remote-access-plan`

4) Status of objective
- The real issue is not folder existence, but false-completion claims.
- First-pass plans still must not be treated as done.
- We need evidence-first verification before changing feature/plan status.

## Mistakes to avoid (important)

1. Do not treat plan existence as completion.
2. Do not treat first-pass docs as implementation-ready.
3. Do not use broad "read everything" scanning as default.
4. Do not return to deleted cleanup branches.
5. Do not ignore hook/script drift. Current hooks use:
   - `scripts/dev/ocentra-ledger-guard.mjs`
   - `scripts/dev/ocentra-ledger-compat.mjs`

## Mandatory startup sequence in cloud (copy/paste)

```powershell
cd E:\OcentraParent
git status --short --branch
git fetch origin
git checkout doc-docs-gap-audit-pass2
git merge origin/main
npm run ledger:install
npm run hub:guard
```

Then continue per lane route in repo `AGENTS.md`.

If `hub:guard` fails after merge + install, only then use any documented guard bypass workflow.

Helpful repo commands:
```powershell
npm run ledger:doctor
npm run hub:inbox
npm run hub:heartbeats
npm run ledger:workers
npm run ledger:tasks
npm run hub:message -- --lane <lane-id> --subject "..." --body "..."
npm run hub:ack -- --lane <lane-id> --message-id <id>
npm run hub:lock -- --paths "docs/**" --reason "..."
```

## Where we started

- Your target was to avoid token waste and false "done" claims.
- You wanted one branch, no duplicate plan folder noise, and high-density plan routing for future agents.
- You rejected handwave outputs and asked for explicit, checkable readiness states.

## Where we are now

- Branches are consolidated.
- Route+gap indexes are stable.
- Audit is still incomplete in the sense that each feature/plan still needs explicit evidence of execution-readiness.

## What remains

1. Produce final evidence-backed completion matrix:
   - Cross-check `PLAN_AUDIT_PASS1.md`, `PLAN_INDEX.md`, `PLAN_HEALTH_INDEX.md`, `FEATURE_ROUTE_INDEX.md`, and each owning plan's `PLAN_STATE.md`.
2. For any plan/feature not truly complete, update status language and next actions in owning route docs only.
3. Keep updates minimal and only in files required by the route.

## Open references to read first

- `docs/CLOUD_HANDOFF.md` (this file)
- `AGENTS.md` (repo root)
- `docs/PLAN_AUDIT_PASS1.md`
- `docs/PLAN_INDEX.md`
- `docs/PLAN_HEALTH_INDEX.md`
- `docs/FEATURE_ROUTE_INDEX.md`

## First cloud message template

Continue on `E:\OcentraParent` branch `codex/doc-docs-gap-audit-pass2` (upstream same). Objective: finish the evidence-based doc completeness audit and prevent false completion claims. I cleaned duplicate branches and kept one working branch. Treat `setup-install-provisioning`, `account-identity-family`, `data-custody-storage`, `payment-subscription`, `policy-control-plane`, and `remote-access` as first-pass research-only until explicitly closed. Read `docs/CLOUD_HANDOFF.md`, then root `AGENTS.md`, then route by feature via `docs/FEATURE_ROUTE_INDEX.md` and `docs/PLAN_INDEX.md`. Open only the owning plan docs and work only through the assigned route flow.
