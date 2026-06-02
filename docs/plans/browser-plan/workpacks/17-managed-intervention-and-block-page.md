# 17 Managed Intervention And Block Page

## Where We Are

`scripts/test/managed-browser-intervention-proof.mjs` proves managed block-page
behavior through real browser harnesses, but product-level integration still
needs typed decision/action/audit/journal/portal wiring.

## Where We Want To Be

Managed warn/block/redirect actions have policy decision refs, evidence refs,
target refs, audit refs, child-facing delivery proof, and portal proof.

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
