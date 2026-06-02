# 20 Windows AppLocker And App Control Proof

## Where We Are

Docs correctly say unmanaged browser prevention needs OS application control,
not URL/tab inference. Current proof includes scoped process termination
guardrails, not production AppLocker/App Control deployment.

## Where We Want To Be

Windows can represent app-control readiness, audit-only, enforced, unavailable,
manual-required, and failed states before claiming unmanaged browser prevention.

## Scope

- AppLocker/App Control capability detection.
- Audit-only state.
- Enforced state.
- Policy creation/update status.
- Rule target identity by publisher/path/hash/package.
- Admin/permission requirement.
- Rollback/failure/audit events.
- Parent-visible manual setup state.

## Touched Paths

- `packages/parent-domain/src/browser-control-policy.ts`
- `crates/agent-service/src/browser_policy_runtime*.rs`
- `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`
- Windows platform docs when implementation starts.

## Tests And Proof

- Model tests for capability states.
- Adapter tests only where safe.
- Real/manual Windows proof before claim upgrade.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No production AppLocker/App Control claim until real device artifacts exist.
