# WP30 Parent And Child UI/UX Surfaces

## Purpose

Render parent map/list/status, alert cards, evidence drawer, exception editor,
child check-in, live tracking, missing-device, retention, and capability states.

## Source Inputs

- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`
- `docs/expectations/portal.md`
- `docs/expectations/location-geofence.md`

## Target State

UI shows source, freshness, accuracy, custody, retention, permission,
capability, ambiguity, deleted state, and safe copy across parent and child
surfaces.

## Tests And Proof

Proof root: `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`

- `11-ui-snapshots/`
- `11-ui-fixture-state-matrix.json`
- `11-ui-snapshots/policy-tracking-parent-fixture.png`
- `11-ui-snapshots/hosted-policy-tracking-live-summary.png`
- `11-ui-snapshots/hosted-policy-tracking-live-summary-mobile.png`
- `12-playwright-proof.log`
- `13-security-negative-proof.log`
- `16-validation-commands.log`
- `17-hosted-ui-proof.json`
- Accessibility summary:
  `test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json`
- Pre-device gate:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

## AI Worker Checklist

- [ ] Add Playwright coverage for all required screens and badges.
- [ ] Add no-overlap/no-overclaim screenshot proof.
- [x] Add first-target parent route tracking state fixture surface.
- [x] Capture local rendered parent-route fixture screenshot.
- [x] Keep UI rows at no-product-claim with P1 fixture status.
- [x] Ensure deleted history disappears from the P1 parent route fixture.
- [x] Render local proof artifact references for each parent route fixture row.
- [x] Feed UI proof gaps into the pre-device proof gate before device work.
- [x] Render live service-backed read-model citation rows with evidence refs
      and retention tombstone refs.
- [x] Capture hosted parent route desktop/mobile screenshot and accessibility
      proof against the real Rust service.
- [ ] Ensure child copy avoids accusation.
- [ ] Keep portal as authoring/display surface, not evaluator.

## Where We Are

A P1 parent portal fixture route now renders a first-target tracking state
matrix for tracking off, permission-required, stale, offline, low accuracy,
ambiguous nearby place, policy alert, parent acknowledgement, exception,
child check-in, temporary live, missing device, and retention-deleted states,
including a retention-deleted row that marks deleted history hidden and does
not render the deleted evidence id. Each row also renders the local proof
artifact path that backs the fixture state.
The fixture is implemented in `apps/portal/src/tracking-status-panel.ts` and
`apps/portal/src/TrackingStatusRoutePanel.tsx`, attached to the live
`policy-tracking` product route in `apps/portal/src/ParentPortalRoute.tsx`,
covered by `apps/portal/tests/tracking-status-panel.test.ts`, and recorded in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`.
The repeatable `node scripts/test/tracking-plan-runtime-proof.mjs` command now
captures and records the local rendered screenshot at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-parent-fixture.png`.
The same route now has a narrow live service summary and service-backed
citation rows for the P2 `trackingReadModel` event, covered by
`apps/portal/tests/tracking-status-panel.test.ts` and the service read-model
proof script. `npm run test:tracking-plan-hosted-ui-proof` now starts the real
Rust service with a seeded temporary ActivityStore SQLite database, drives the
hosted parent `policy-tracking` route through Playwright, captures desktop and
mobile screenshots, writes accessibility summary output, and records
`productClaimReady=false`.
This is not product-complete UI proof: full service-data UI beyond the hosted
parent route, child-device UI, physical-device evidence, authority, provider
delivery, and production proof remain pending.
`node scripts/test/tracking-plan-pre-device-proof.mjs` now records those UI
gaps in the aggregate pre-device gate so the next pass can run child UI and full
parent/child UI proof beyond the hosted parent route before any product claim.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md
- docs/plans/tracking-plan/implementation-checklist.md
- apps/portal/src/ParentPortalRoute.tsx
- apps/portal/src/TrackingStatusRoutePanel.tsx
- apps/portal/src/portal-route-content.ts
- apps/portal/src/tracking-status-panel.ts
- apps/portal/src/styles/parent-portal-route.css
- apps/portal/tests/tracking-status-panel.test.ts
- packages/text-domain/src/portal-dev.ts
- packages/portal-domain/src/contracts.ts
- packages/portal-domain/src/details.ts
- packages/portal-domain/src/tracking-status-proof-artifacts.ts
- `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`

## Manual-Required Gaps

- Full service-data UI beyond the hosted parent route, child-device UI,
  physical-device proof, authority proof, provider delivery, and production
  proof remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-live-service-citation-proof`.
- [x] Touched files: portal tracking status renderer/tests, service proof
      script, tracking feature doc, implementation checklist, WP30, WP32,
      WP33, and generated WP32 proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`
      and companion WP32 proof files.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and WP33 updated; central capability checklist
      row delta queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: full parent/child UI, hosted
      Playwright/accessibility output, Android/iOS physical-device proof,
      authority, provider delivery, notifications, and production proof remain
      proof-gated.
- [x] Workpack id and branch:
      `codex/tracking-hosted-ui-accessibility-proof-v2`.
- [x] Touched files: hosted Playwright proof spec, hosted proof script, root
      script wiring, parent route tracking CSS, tracking feature doc,
      implementation checklist, WP30, WP33, and generated hosted proof
      artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-hosted-ui-proof` passed locally.
- [x] Proof artifacts under
      `test-results/tracking-plan-hosted-ui-proof/`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`, and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: full service-data UI beyond the hosted
      parent route, child UI, Android/iOS physical-device proof, authority,
      provider delivery, notifications, and production proof remain proof-gated.
