# Portal UX Household Surfaces Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `portal-ux-household-surfaces-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for coherent parent-facing household UX across setup, devices, policy, activity, LAN state, assistant/report surfaces, degraded states, and proof.

## Current ownership interpretation

```text
apps/portal:
  React/Tauri parent portal runtime, route composition, local interaction state, and focused UI/e2e proof.

portal-domain:
  Public portal route, DOM, panel, projection, proof-artifact, and presentation contract package.

schema-domain and domain packages:
  Canonical source/read-model contracts consumed by portal; they own source truth, not portal presentation.

agent-protocol-domain and agent-protocol/service surfaces:
  Service/read-model seams consumed by portal when selected.

policy-control-plane-plan:
  Policy source truth, compiler, delivery, approval, and ask-parent semantics. Portal renders selected policy states only.

Sibling domain/runtime plans:
  Setup, account, device-trust, LAN, browser, app/game, network, screen, tracking, AI, payment, custody, notification, and enforcement plans own their domain truth and runtime behavior.
```

## Current proof interpretation

```text
output/portal-ux-household-surfaces-plan-proof/<workpack>/ is the deterministic proof root.
docs/proof/portal-ux-household-surfaces-plan/ remains compatibility/reference for old slice proof names.
Checked workpacks prove selected UX slices only and do not close open policy/assistant/reports/degraded/accessibility/screenshot/mobile/manual-review work.
Screenshot proof supports review; it does not replace service-backed validation.
Portal projection proof is not domain runtime proof.
```

## Current coupling risks

```text
- Portal route existence is not product readiness.
- Visual screenshot proof is not runtime/service proof.
- Service parser proof is not rendered UX proof.
- Fixture/demo data is not runtime state.
- Portal-local replacement read models are not product truth.
- UI preview is not applied policy.
- Delivered/acknowledged status is not active enforcement.
- Assistant output is not parent-approved action.
- Portal projection is not capture, transport, billing, custody, policy, or enforcement proof.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- No `current-*.md` snapshot exists; use README/implementation indexes until one is added.

## What is already present / proved

- `portal-domain` is a real public contract package with many route, panel, presentation, and proof exports.
- `apps/portal` is the actual React/Tauri portal runtime surface and consumes domain packages as projection inputs.
- Checked workpacks currently cover selected shell/navigation, first-run/profiles, device inventory/source states, selected-device context, LAN pairing consumption, and the bounded no-fake-data projection slice.
- WP05 has substantial policy preview/read-model/current-seam proof recorded. The portal now has a bounded typed staged-draft/parent-confirmation relay through the Rust facade; downstream policy mutation, request-envelope projection into a complete domain writer, co-parent authZ, rollback execution, unsupported target handling, delivery, and enforcement remain open.

## Production reachability audit (2026-08-16)

This audit is against consolidated root `1988a85cd`. The shipped portal path is
`apps/portal/src/main.ts` -> `PortalApp` -> `ParentPortalRoute` -> typed
`host-bridge` route snapshot/action calls. A surface counts only when that path
reaches a real Rust/service read model or action; static cards, proof/status
panels, fixtures, generic JSON, and route existence do not count as product
truth. This pass corrected one stale shipped authority message; it did not add
new domain/runtime authority because the remaining gaps belong to named sibling
owners or deferred validation/proof, not a missing portal-only substitute.

| WP | Actual portal/backend reachability | Remaining production gap |
| --- | --- | --- |
| WP01 | Shell/navigation is shipped through `ParentPortalRoute` and Rust `parent_ui_bridge` snapshots via Tauri/dev-web bridge. | Runtime composition/auth/session authority and full service-backed route validation remain open. |
| WP02 | First-run/profile panel consumes a Rust route snapshot, with explicit unavailable wording when live state is absent. | No live account/setup/trust source is supplied by the portal; onboarding truth remains sibling-owned. |
| WP03 | Device inventory/source panels consume Rust parent-runtime LAN/read-model snapshots; canonical `b6d8d12ea` includes the accepted `activity-ui-intent.ts` absorbing-revocation projection fix. | Focused projection coverage for both revocation update directions and true identity/slot changes, the named proof root, and physical/provider pairing, freshness, and device-authority validation remain open; UI cannot promote LAN evidence. |
| WP04 | Selected-device context is held by typed portal state and route context. | Selection is not device authority and has no portal-owned mutation path. |
| WP05 | `PolicyPreviewRoutePanel` stages/cancels/requests confirmation through typed actions; Rust `parent_ui_bridge/policy_preview/authoring.rs` validates and creates the typed command from the staged trusted preview. | Downstream policy mutation, co-parent authZ, rollback, unsupported targets, delivery, and enforcement remain open; this existing bounded path needs no duplicate portal seam. |
| WP06 | Schedule route/nav entries exist. | No real schedule/time-budget read model or action reaches the shipped portal. |
| WP07 | Timer/policy surfaces and typed parent-resolution actions are present. | Parent request/approval authority and resulting domain mutation remain unavailable/manual-required. |
| WP08 | Network evidence/diagnostic panels can consume Rust snapshots/events. | Durable diagnostic custody/export and complete activity source coverage remain open. |
| WP09 | Browser/app/network panels are routed and some consume typed Rust read models. | Browser/provider/action authority and unsupported-target runtime ownership remain open; status panels are not product enforcement. |
| WP10 | LAN pairing state is consumed through Rust parent-runtime snapshots and replay/read-model handoffs. | Physical pairing and cross-device acceptance remain provider/manual-required. |
| WP11 | Assistant UI and typed preview/confirm action seams exist. | Real assistant provider, parent-approved action authority, and domain execution remain open. |
| WP12 | Notification/report surfaces are mounted. | No complete durable notification/report/custody source backs the rendered product state. |
| WP13 | Connection, empty, stale, and unavailable states derive from bridge/service state. | Domain-specific degraded/error semantics remain incomplete. |
| WP14 | Diagnostics/debug panels can display bridge events. | No durable product audit-history read model or custody path. |
| WP15 | Portal styles/layout provide the accessibility/responsive surface. | Accessibility/manual review and full interaction proof remain open; no backend slice is implicated. |
| WP16 | Typed Rust parent-host distribution state reaches the generated bridge and rendered route; fixture, invalid, missing, read-only, compact-layout, and Remote Access honesty cases have retained proof. | Installer, updater, rollback, signing, store, transport, and authenticated remote-session execution remain sibling-owner non-claims. |
| WP17 | Tests-only workpack; no production implementation. | Screenshot/manual proof remains open. |
| WP18 | Mobile directories/workflow scaffolds exist. | No shipped parent-mobile portal runtime reaches these surfaces. |
| WP19 | No production source. | Documentation/checklist synchronization only. |
| WP20 | No production source. | Manual user review only. |

The 2026-08-16 audit above predates the current graph refresh. The 2026-08-25
WP03 source packet ran `npm run --silent graph:validate` successfully against
the current canonical checkout, reporting 713 nodes and 1,276 edges. WP03 is
still validation-only because implementation, focused tests, proof, and
checklist evidence are not all reviewed; the source packet does not imply
READY, PR readiness, or DONE.

## Current Portal WP03 Source Truth (2026-08-25)

- Canonical source is `b6d8d12ead5965cea80625431fdc67e518702403`; accepted commits `aa5f4579e` and `adba0c9c1` are both in `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`.
- The production flow is Rust LAN read model -> `ParentPortalRoute`/vendor surface caller -> `ParentPortalSvgSurface` -> Portal LAN slot projection. Collection order is canonical -> discovered -> trusted -> pairing -> selected. The selected `preferState` update cannot clear a revoked state for a matching slot, while an actual identity/slot change remains distinct.
- Existing service-backed route/E2E and live-activity surfaces are present, but the focused projection test for absorbing revocation and identity semantics is absent. The expected WP03 proof root is also absent; no test, proof, CI, or pre-commit command was run for this source-only packet.
- This is a code-derived source truth record only. The remaining test/proof gaps and external pairing/freshness/device-authority boundary keep WP03 in validation and do not authorize READY or DONE.

## Current Portal WP16 Completion (2026-09-05)

- The shipped Platforms and Install Updates route renders the typed Rust parent-host distribution snapshot through the generated bridge; explicit fixture data cannot enter the runtime path.
- Missing and invalid payloads remain visible, execution controls remain unavailable, and Remote Access remains unavailable without an authenticated session.
- Portal and Portal-domain builds and full unit suites, Rust bridge contracts and parent-route projection tests, focused real Platforms/Install Updates and Remote Access E2E scenarios, and the scoped architecture gate passed.
- The broader Portal route E2E still has owner-gated and stale sibling-route failures. WP16 closure is bounded and does not claim plan-wide or PR readiness.
- Durable proof: [WP16 No-Fake-Data Contract Adapter Proof](../../proof/portal-ux-household-surfaces-plan/WP16_NO_FAKE_DATA_CONTRACT_ADAPTER_PROOF.md).

## Open gaps / missing product runtime

- No concise current snapshot file exists.
- Policy authoring, schedules, approvals, activity diagnostics, browser/app/network surfaces, assistant action preview, reports/notifications/custody, degraded states, audit/debug, accessibility, screenshot, mobile shell, docs sync, and manual review workpacks remain open.
- The portal must not claim domain/runtime completion for any sibling plan from UI projection alone.

## Checklist summary

- No `implementation-checklist.md` exists in this plan; use the 20-step/test-blueprint files listed in `DOC_INDEX.md` and `ARCHIVE_INDEX.md` only when assigned.
- Checkbox rows detected: 0 total, 0 checked, 0 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 20.
- Workpacks with open checkboxes: 14.
- Workpacks with all detected boxes checked: 6.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [05 Policy Authoring Control Center](workpacks/05-policy-authoring-control-center.md) - 0/5 checked, 5 open.
- [06 Schedules, Time Budgets, And Templates](workpacks/06-schedules-time-budgets-and-templates.md) - 0/5 checked, 5 open.
- [07 Parent Requests And Approvals](workpacks/07-parent-requests-and-approvals.md) - 0/5 checked, 5 open.
- [08 Activity Evidence And Diagnostics](workpacks/08-activity-evidence-and-diagnostics.md) - 0/5 checked, 5 open.
- [09 Browser, App, And Network Surfaces](workpacks/09-browser-app-and-network-surfaces.md) - 0/5 checked, 5 open.
- [11 Assistant Action Preview Flow](workpacks/11-assistant-action-preview-flow.md) - 0/5 checked, 5 open.
- [12 Reports, Notifications, And Custody](workpacks/12-reports-notifications-and-custody.md) - 0/5 checked, 5 open.
- [13 Degraded, Empty, Stale, And Error States](workpacks/13-degraded-empty-stale-and-error-states.md) - 0/5 checked, 5 open.
- [14 Audit History And Copy/Debug](workpacks/14-audit-history-and-copy-debug.md) - 0/5 checked, 5 open.
- [15 Accessibility, Responsive, And Keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md) - 0/5 checked, 5 open.
- [17 Playwright Screenshot Proof](workpacks/17-playwright-screenshot-proof.md) - 0/5 checked, 5 open.
- [18 Parent Mobile Shell Readiness](workpacks/18-parent-mobile-shell-readiness.md) - 1/6 checked, 5 open.
- [19 Product Docs And Checklist Sync](workpacks/19-product-docs-and-checklist-sync.md) - 0/5 checked, 5 open.
- [20 Manual User Review Gate](workpacks/20-manual-user-review-gate.md) - 0/5 checked, 5 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full 20-step/test-blueprint files unless `DOC_INDEX.md` or the hub assignment names them.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log or explicit known blocker from the assigned implementation boundary,
  - a proof manifest under `output/portal-ux-household-surfaces-plan-proof/<workpack>/`.
- Required proof must include commands, pass/fail, negative cases, and manual-required notes.
- Failure rule: no PR-ready claim until authZ/replay, stale/error/degraded, no-fake-data, screenshot/manual-review, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
