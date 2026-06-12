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

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/01-contract-boundary-and-effect-schemas/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity updated only after contracts exist. Rust protocol parity was added; service and portal were not touched because this slice does not expose a runtime read model or UI.
- [x] Raw evidence artifacts captured where applicable: contract-only slice marked runtime, journal, SQLite, and action artifacts N/A in the proof pack.
- [x] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured where applicable: unmanaged inventory rows cannot claim exact managed URL support in TypeScript or Rust proof.
- [x] Manual platform proof captured for real browser/OS claims; no real browser/OS claim was made, so manual platform proof is N/A.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No runtime/browser/platform claim is created by contracts alone.

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
