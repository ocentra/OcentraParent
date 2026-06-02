# 01 Contract Boundary And Effect Schemas

Sources: [folder README](../README.md), [source index](../source-index.md),
[full scope plan](../v0-5-managed-browser-full-scope-plan.md), and
[test blueprint](../v0-5-managed-browser-test-blueprint.md). Browser URL/video
intelligence contracts are scoped by
[V0.5 Browser URL And Video AI Intelligence Plan](../v0-5-browser-url-video-ai-intelligence-plan.md).
Social platform/account/feed gating contracts are scoped by
[V0.5 Social Platform Account Feed And Gating Plan](../v0-5-social-platform-account-feed-gating-plan.md).
Browser-game/cloud-gaming contracts are scoped by
[V0.5 Browser Games Cloud Gaming And Game Portal Gating Plan](../v0-5-browser-games-cloud-gaming-gating-plan.md).

## Where We Are

`packages/activity-domain` already owns browser tab evidence, managed session
status, read models, and browser intervention schemas. `packages/parent-domain`
owns browser policy authoring/catalog/update contracts. `packages/agent-protocol-domain`
owns browser policy command/event adapter contracts.

## Where We Want To Be

Every browser workpack must start from typed Effect Schema contracts in the
owning domain package before Rust protocol or service code claims support.

## Scope

- Inventory and support matrix contracts.
- Managed profile and managed session contracts.
- Browser bridge and tab evidence contracts.
- Active-tab certainty contracts.
- Unmanaged browser evidence contracts.
- Browser policy/action/intervention contracts.
- URL shape, metadata evidence, AI result, provider route, memory hit, and
  policy handoff contracts when browser intelligence starts.
- Social platform, route kind, account flow, account identity, approval request,
  social AI, risk signal, feed/short-video, messaging route, and social policy
  target contracts when social gating starts.
- Browser game URL shape, runtime signals, metadata, AI analysis, game policy
  target, game approval request, game memory, and cloud/UGC/manual-required
  contracts when browser-game gating starts.
- Capability, degraded, stale, custody, and manual-required states.

## Touched Paths

- `packages/activity-domain/src/browser*.ts`
- `packages/parent-domain/src/browser-control-*.ts`
- `packages/agent-protocol-domain/src/browser-policy-adapter.ts`
- `crates/agent-protocol/src/browser*.rs`

## Tests And Proof

- Contract tests for every schema and invalid-state rejection.
- Rust protocol parity tests after TypeScript contracts exist.
- No manual brands, no raw `string` annotations in runtime/app source.

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

No runtime/browser/platform claim is created by contracts alone.
