# 17 Managed Intervention And Block Page

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `17 Managed Intervention And Block Page`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

`scripts/test/managed-browser-intervention-proof.mjs` now proves managed
block, warning, approval-hold, and checking-hold pages through real Chrome,
Firefox, and Edge harnesses. The typed decision/action/audit/evidence/delivery
fields also round-trip through activity-domain contracts, Rust protocol,
journal/read-model replay, service payload flattening, and portal parsing.
After PR399, the composited blocker proof uses the shared
`renderBrowserChildInterventionPage` renderer and the Rust child-agent
`/api/browser/intervention/page` endpoint instead of a one-off preview route.

## Where We Want To Be

Managed warn/block/redirect actions have policy decision refs, evidence refs,
target refs, audit refs, child-facing delivery proof, and portal proof.

## Production-code pass (2026-08-16)

The browser runtime now derives action-intent outbox and handoff references from
the actual action-intent identity instead of test-only constants. The service
does not synthesize child acceptance from a handoff that lacks trusted parent
profile, device, and observation context; child delivery remains unavailable /
manual-required until that typed authority is supplied. This is a code-drafted,
tests/proof/checklist-deferred slice and does not claim intervention execution,
child delivery, or portal proof.

Owning production paths:

- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff.rs`
- `crates/agent-service/src/browser_runtime_stream_api.rs`

## Scope

- Observe navigation.
- Match policy target.
- Dry-run result.
- Warn child.
- Redirect/block page.
- Account creation approval hold screen.
- Feed/short-video route warning or block screen.
- Browser-game checking/hold screen.
- Game account/purchase approval gate.
- Cloud-gaming approval gate.
- Journal policy decision and action.
- Portal intervention row.
- YouTube/video limitations and SPA route caveats.

## Touched Paths

- `crates/agent-core/src/browser_intervention_event*.rs`
- `crates/agent-core/src/activity_store_browser_intervention*.rs`
- `crates/agent-service/src/activity_api/browser_intervention_*.rs`
- `apps/portal/src/browser-intervention-panel.ts`
- `scripts/test/managed-browser-intervention-proof.mjs`

## Tests And Proof

- Managed intervention dry-run tests.
- Managed block-page E2E proof.
- Managed social signup approval hold proof.
- Managed feed/short-video route intervention proof.
- Managed browser-game checking/hold proof.
- Managed cloud-gaming and game-purchase approval proof.
- Journal/read-model replay tests.
- Portal screenshot proof.

## Implementation Proof

- Proof pack:
  `output/browser-plan-proof/17-managed-intervention-and-block-page/`
- Latest real browser proof:
  `test-results/managed-browser-intervention-proof/2026-06-06T03-40-32-849Z.json`
- Latest endpoint-backed composited blocker proof:
  `test-results/managed-browser-composited-block-proof/2026-06-06T03-39-49-777Z.json`
- Latest child-agent endpoint proof:
  `test-results/child-agent-browser-intervention-page-proof/2026-06-06T03-39-01-991Z.json`
- Endpoint-backed blocker screenshot:
  `test-results/managed-browser-composited-block-proof/2026-06-06T03-39-49-777Z-screenshots/chrome-stable-composited-block-youtube.png`
- Screenshot directory:
  `test-results/managed-browser-intervention-proof/2026-06-06T03-40-32-849Z-screenshots`
- Focused validation:
  `cmd /c npm run build:contracts`,
  `cmd /c npm run test --workspace @ocentra-parent/activity-domain -- browser-intervention.test.ts`,
  `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- contracts.test.ts`,
  `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- contracts.test.ts`,
  `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-browser-status.test.ts`,
  `cargo test -p ocentra-parent-agent-protocol browser_intervention`,
  `cargo test -p ocentra-parent-agent-core activity_store_browser_intervention`,
  `cargo test -p ocentra-parent-agent-service browser_intervention`,
  `node --check scripts/test/managed-browser-intervention-proof.mjs`, and
  `cmd /c npm run test:managed-browser-intervention`,
  `node --check scripts/test/managed-browser-composited-block-proof.mjs`, and
  `cmd /c npm run test:managed-browser-composited-block`.

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

Managed intervention proof is not broad OS browser blocking and does not cover
unmanaged browsers. It also does not prove native-app per-reel or per-message
blocking, native-game scene control, or cloud-streamed frame analysis.
The endpoint-backed composited proof proves the rendered child blocker page is
served by the local child-agent endpoint after a policy-acceptable live YouTube
capture; it does not claim a final policy evaluator, broad browser interception,
or native/mobile blocking.
