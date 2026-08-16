# 18 Unmanaged Browser Detection

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `18 Unmanaged Browser Detection`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

The service reports unmanaged browser processes with process-only evidence:
process id/name, redacted executable path ref when available, optional
signature/hash refs, family/channel guess, process kind, confidence, detection
reason, custody, and query visibility. Contract tests reject exact URL/tab,
social account/route/feed/video, and browser-game/cloud-gaming exact fields for
unmanaged evidence.

2026-08-16 browser-code-pass: the existing typed `BrowserInventoryReadModel`
service event is now carried through a JSON payload field and loaded into the
Rust parent bridge on the Browser route. The projection remains process-only;
it does not add exact URL, active-tab, page-title, or enforcement authority.
This is code-drafted, unvalidated, and tests/proof/checklist-deferred.

## Where We Want To Be

Browser-like processes outside current managed sessions are recorded as
unmanaged/possible-bypass evidence with no exact URL fields.

## Scope

- Supported browser outside managed session.
- Unsupported browser.
- Portable browser.
- Tor/privacy browser.
- Packaged browser.
- Embedded/unknown browser-like process.
- Social platform opened outside managed session as possible bypass.
- Browser-game portal or cloud-gaming surface opened outside managed session as
  possible bypass.
- Process id, name, path/signature/hash refs, family guess, confidence, reason,
  timestamp.

## Touched Paths

- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime_status.rs`
- `crates/agent-service/src/browser_payload.rs`
- `crates/parent-runtime-core/src/agent_service_client/snapshots_browser.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot/dependencies/load.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/browser.rs`
- `crates/schema/src/parent_ui_bridge.rs`
- `apps/portal/generated/parent-ui-bridge.ts`
- `packages/activity-domain/src/browser*.ts`
- `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`

## Tests And Proof

- [ ] Fake process snapshot tests.
- [ ] Contract tests forbidding exact URL/tab fields.
- [ ] Contract tests forbidding exact social account, route, feed, or video fields
      on unmanaged browser evidence.
- [ ] Contract tests forbidding exact browser-game URL, runtime signal, game title,
      account, purchase, or cloud-title fields on unmanaged browser evidence.
- [ ] Portal unmanaged bypass fixture tests.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/18-unmanaged-browser-detection/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: unmanaged process rows and service read-model rows.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no unmanaged exact URL claim and no exact social/game fields on unmanaged evidence.
- [ ] Manual platform proof captured for real browser/OS claims, including manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Unmanaged detection is evidence of bypass/process use, not the page visited.
It can report possible social bypass, not social account creation proof.
It can report possible browser-game/cloud-gaming bypass, not exact game proof.
