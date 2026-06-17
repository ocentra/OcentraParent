# 01 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `01 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

The audited TypeScript owner for the active browser-plan contract surface is
`packages/browser-domain`, which now carries browser AI, social, browser-game,
policy compiler, and contract-only proof read-model schemas. `packages/agent-protocol-domain`
still owns the browser policy command/event adapter boundary, and Rust protocol
parity still routes through `crates/agent-protocol`.

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

- `packages/browser-domain/src/browser-*.ts`
- `packages/browser-domain/src/social-*.ts`
- `packages/browser-domain/src/browser-game-*.ts`
- `packages/browser-domain/tests/unit/*.test.ts`
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
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist. Rust protocol parity was added; service and portal were not touched because this slice does not expose a runtime read model or UI.
- [ ] Raw evidence artifacts captured where applicable: contract-only slice marked runtime, journal, SQLite, and action artifacts N/A in the proof pack.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: unmanaged inventory rows cannot claim exact managed URL support in TypeScript or Rust proof.
- [ ] Manual platform proof captured for real browser/OS claims; no real browser/OS claim was made, so manual platform proof is N/A.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No runtime/browser/platform claim is created by contracts alone.

## Audit Addendum - 2026-06-16

This workpack previously described a legacy `packages/activity-domain` and
`packages/parent-domain` split that no longer matches the current browser-plan
source surface. The 2026-06-16 repair slice under this workpack instead landed
in `packages/browser-domain`.

Validated repair scope:

- `packages/browser-domain/src/social-applied-schedule-time-budget-proof.ts`
  now includes the required `compilerCapabilityState` field for the ready and
  manual-required social decision candidate fixtures.
- `packages/browser-domain/tests/unit/browser-package-exports.test.ts` and
  `packages/browser-domain/tests/unit/browser-plan-package-exports.test.ts` now
  resolve the package root correctly and assert the package wildcard export
  contract plus source-module presence instead of reading
  `packages/browser-domain/tests/package.json`.
- Focused validation passed with targeted `browser-domain` tests, full
  `browser-domain` tests, package type-check, and the touched-file architecture
  gate. Package-wide `npm run lint:architecture -- --files packages/browser-domain`
  still fails on pre-existing re-export debt outside this slice and remains an
  open blocker for true WP01 completion.

## Completion Note - 2026-06-02

Base browser inventory/support matrix contracts now exist in
`packages/activity-domain/src/browser-inventory-schemas.ts`, re-exported through
`packages/activity-domain/src/browser.ts`, with Rust protocol parity in
`crates/agent-protocol`. The proof keeps managed target-list evidence separate
from active-tab proof and keeps unmanaged browsers as process/bypass evidence
only. URL/video AI, social, and browser-game contract families remain deferred
until those enhancement workpacks start; this slice does not claim runtime
browser inventory, managed launch, journal/SQLite replay, portal UI, or platform
enforcement support.
