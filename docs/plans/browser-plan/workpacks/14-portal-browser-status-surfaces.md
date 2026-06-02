# 14 Portal Browser Status Surfaces

## Where We Are

Portal renders managed browser status, evidence summary, and intervention
summary. The richer parent browser workflow is not complete.

## Where We Want To Be

Parents can inspect installed browsers, managed sessions, tab evidence,
unmanaged bypass, stale/degraded states, policy preview, and interventions from
service-backed read models.

## Scope

- Inventory dashboard.
- Managed session card.
- Tab evidence list.
- Active-state certainty copy.
- Unmanaged bypass cards.
- Intervention/action results.
- URL/video intelligence explanation states when local AI proof exists.
- Social platform overview, approved accounts, pending approvals, secondary
  account attempts, feed/short-video route states, and messaging-route states
  when social read models exist.
- Browser-game dashboard, cloud-gaming approval, educational-game allow,
  game-account/purchase approval, unblocked-site, and unmanaged game bypass
  states when browser-game read models exist.
- Redacted copy/debug details.
- Empty, stale, degraded, manual-required, unsupported states.

## Touched Paths

- `apps/portal/src/browser-status-panel.ts`
- `apps/portal/src/browser-intervention-panel.ts`
- `apps/portal/src/portal-browser-route-panels.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrowserRulesQuestionnaire.tsx`
- `apps/portal/tests/live-activity-browser-status.test.ts`

## Tests And Proof

- Mocked-backend Playwright fixtures first.
- Real service Playwright proof when service paths exist.
- Malicious URL/title layout and escaping tests.
- AI classification/degraded-state UI tests once intelligence read models exist.
- Social approval/evidence drawer/child hold screen Playwright tests once
  social read models exist.
- Browser-game dashboard/evidence drawer/child checking screen Playwright tests
  once browser-game read models exist.

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

UI cannot claim exact evidence or intervention unless service proof backs it.
