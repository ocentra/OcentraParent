# logging-domain-parity

## Normalized Header

- plan/thread name: `logging-domain-parity`
- source thread label: dedicated Codex plan thread for `docs/plans/logging-domain-parity`
- source thread id: `019ed329-a1a4-7b90-93db-083e6a041adb`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: WP07/WP10 proof-inventory slice completed locally; full plan not done
- claimed source files/crates/packages: `scripts/dev/lib/log-query-service.mjs`; plan docs under `docs/plans/logging-domain-parity/*`; proof roots under `output/logging-domain-parity-proof/*`; test roots under `test-results/logging-domain-parity-*`
- claimed tests: `packages/logging-domain/tests/integration/mcp-query-interface.test.ts`; `apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts`; `apps/portal/tests/logging/portal-proof-trace.test.ts`; `apps/portal/tests/logging/portal-dev-log-route.test.ts`
- claimed proof commands/artifacts: `node scripts/dev/agent-query.mjs latest-failures`; `node scripts/dev/mcp-logging-server.mjs --smoke latest-failures`; `node scripts/dev/mcp-logging-server.mjs --smoke run-diagnostics`; `node scripts/dev/mcp-logging-server.mjs --smoke artifact-slice`; `node --import tsx scripts/dev/logging-proof-trace-smoke.mjs --root=...test-results/logging-domain-parity-proof-trace --keep-root`; `node scripts/dev/mcp-logging-server.mjs --smoke proof-trace --smoke-root ...test-results/logging-domain-parity-proof-trace`; `output/logging-domain-parity-proof/07-mcp-query-interface/*`; `output/logging-domain-parity-proof/10-proof-trace-pipeline/*`
- claimed blockers: no blocker remained for the WP07/WP10 proof-root slice after the fresh-root path fix; full-plan closure still depends on remaining workpacks/proof inventory being reconciled
- claimed next actions: truthful WP03 portal/dev-log consumer closeout first, then WP06 checker/enforcement hardening
- obvious missing evidence fields: no archival paste here proves full-plan completion, remaining workpack proof roots, or coordinator verification of the self-report
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

WP07/WP10 proof roots are now real on `codex/tracking-plan-full-continuation-a`, and the logging plan docs are synced to those generated artifacts.

**Changed surfaces**
- Code fix: [scripts/dev/lib/log-query-service.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/dev/lib/log-query-service.mjs)
- Synced plan docs:
  - [docs/plans/logging-domain-parity/PLAN_STATE.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/PLAN_STATE.md)
  - [docs/plans/logging-domain-parity/NEXT_ACTIONS.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/NEXT_ACTIONS.md)
  - [docs/plans/logging-domain-parity/WORKPACK_INDEX.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/WORKPACK_INDEX.md)
  - [docs/plans/logging-domain-parity/PLAN_HEALTH.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/PLAN_HEALTH.md)
  - [docs/plans/logging-domain-parity/workpacks/07-mcp-query-interface.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/workpacks/07-mcp-query-interface.md)
  - [docs/plans/logging-domain-parity/workpacks/10-proof-trace-pipeline.md](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/docs/plans/logging-domain-parity/workpacks/10-proof-trace-pipeline.md)
- Canonical proof roots generated:
  - [output/logging-domain-parity-proof/07-mcp-query-interface](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/logging-domain-parity-proof/07-mcp-query-interface)
  - [output/logging-domain-parity-proof/10-proof-trace-pipeline](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/output/logging-domain-parity-proof/10-proof-trace-pipeline)
- Canonical test/proof data roots generated:
  - [test-results/logging-domain-parity-mcp](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/test-results/logging-domain-parity-mcp)
  - [test-results/logging-domain-parity-proof-trace](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/test-results/logging-domain-parity-proof-trace)

**What was fixed**
- The minimal blocking bug was in [scripts/dev/lib/log-query-service.mjs](C:/Users/sujan/.codex/worktrees/ocentra-parent-codex-a/OcentraParent/scripts/dev/lib/log-query-service.mjs): agent-evidence DB/NDJSON/artifact allowlisting still resolved against `<workspace>/.logs` instead of the configured log root. That made fresh-root MCP proof non-deterministic.
- After switching those paths to `getLogRoot()`, fresh-root MCP `latest-failures`, `run-diagnostics`, `artifact-slice`, and `proof-trace` all worked against seeded local roots.

**Scoped validation run**
- `node scripts/dev/agent-query.mjs latest-failures`
- `node scripts/dev/mcp-logging-server.mjs --smoke latest-failures`
- `node scripts/dev/mcp-logging-server.mjs --smoke run-diagnostics`
- `node scripts/dev/mcp-logging-server.mjs --smoke artifact-slice`
- `cmd /c npx vitest run packages/logging-domain/tests/integration/mcp-query-interface.test.ts`
- `cmd /c npx vitest run apps/portal/tests/logging/portal-proof-trace-pipeline.test.ts apps/portal/tests/logging/portal-proof-trace.test.ts apps/portal/tests/logging/portal-dev-log-route.test.ts`
- `node --import tsx scripts/dev/logging-proof-trace-smoke.mjs --root=...test-results/logging-domain-parity-proof-trace --keep-root`
- `node scripts/dev/mcp-logging-server.mjs --smoke proof-trace --smoke-root ...test-results/logging-domain-parity-proof-trace`
- `npm run lint:architecture -- --files scripts/dev/lib/log-query-service.mjs scripts/dev/mcp-logging-server.mjs scripts/dev/logging-proof-trace-smoke.mjs packages/logging-domain/tests/integration/mcp-query-interface.test.ts`
- `git diff --check` on the touched code/docs/proof slice

Ambient old logs are no longer required for the WP07/WP10 proof surfaces. The regenerated proof roots are seeded from the harnesses themselves and the docs now describe that truth instead of the old ambient-log assumption.

The next slice should be `portal/dev-log consumer follow-through`, not checker hardening. Concretely: close WP03 truthfully against the now-honest proof surface, then do WP06 checker/enforcement hardening after the consumer-side proof claims are aligned.

## Optional Addendum

- Earlier audit passes also established that the full plan is still incomplete beyond this slice: only WP07 and WP10 canonical proof roots were restored in this checkout; the remaining claimed roots under `output/logging-domain-parity-proof/*` and `test-results/logging-domain-parity-*` are still absent or need claim reduction.
- Earlier audit passes also called out stale plan-state overclaiming outside the restored slice, especially WP03 parent architecture/routing and WP06 validation/enforcement. The recommended completion order from the thread was: WP03 truthful closeout, then WP06 honest enforcement/proof-inventory hardening, then remaining proof-root regeneration or claim reduction.
- This archival file preserves the thread's own report and addendum only. It does not assert `DONE`, `PR_READY`, or coordinator-verified truth for the plan.
