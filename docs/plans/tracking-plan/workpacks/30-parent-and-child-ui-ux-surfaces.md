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
- [x] Add hosted Playwright coverage for the narrow service-backed parent route
      summary and rendered citation labels.
- [x] Capture desktop/mobile hosted parent-route screenshots and accessibility
      summary output for the service-backed summary.
- [x] Add first-target parent route tracking state fixture surface.
- [x] Capture local rendered parent-route fixture screenshot.
- [x] Keep UI rows at no-product-claim with P1 fixture status.
- [x] Ensure deleted history disappears from the P1 parent route fixture.
- [x] Render local proof artifact references for each parent route fixture row.
- [x] Feed UI proof gaps into the pre-device proof gate before device work.
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
The same route now has a narrow live service summary for the P2
`trackingReadModel` event, covered by `apps/portal/tests/tracking-status-panel.test.ts`
and the service read-model proof script. This is not product-complete UI proof:
child-device UI, hosted Playwright/accessibility output, richer
service-backed citations, and physical device evidence remain pending.
`npm run test:tracking-plan-hosted-ui-proof` now starts the real Rust service
against a seeded temporary ActivityStore SQLite database, starts the Vite
portal, runs Playwright on `#/policy-tracking`, proves the service-backed
summary labels/values, records no-product-claim state, and captures desktop and
mobile screenshots plus accessibility summary output. It writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`,
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/12-playwright-proof.log`,
and `test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json`.
This closes the narrow hosted parent-route proof gap only. `node
scripts/test/tracking-plan-pre-device-proof.mjs` still records the broader UI
gaps in the aggregate pre-device gate so future passes can cover child UI,
richer live service-backed citations, and full product accessibility before any
product claim.

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

- Full service-data UI beyond the narrow hosted parent summary, child-device UI,
  broader accessibility, richer service-backed citations, and physical-device
  proof remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `30-parent-and-child-ui-ux-surfaces`,
      `codex/tracking-hosted-ui-accessibility-proof`.
- [x] Touched files: hosted proof script, hosted Playwright spec, package script
      wiring, tracking feature doc, tracking README/checklist, this workpack,
      WP32, WP33, and hosted proof artifacts.
- [x] Validation commands and results: `node --check scripts/test/tracking-plan-hosted-ui-proof.mjs`
      passed; `npx prettier --check ...` passed for touched files;
      `git diff --check` passed;
      `npm run test:tracking-plan-hosted-ui-proof` passed locally;
      `npm run format:check` passed; `npm run lint:schema-boundaries` passed
      with existing source-shape advisory warnings only; `npm run lanes:guard`
      passed; `npm run hub:guard` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`,
      including `17-hosted-ui-proof.json`, `12-playwright-proof.log`, and
      hosted desktop/mobile screenshots.
- [x] Product doc/checklist updates: owning feature doc and tracking checklist
      updated; central product capability checklist update is queued through the
      hub DOC_DELTA queue because this worker must not edit that file.
- [x] Known gaps/manual-required states: full child/parent UI beyond the narrow
      hosted parent route, broader accessibility, richer live citations,
      Android/iOS physical behavior, authority, provider delivery, and
      production proof remain proof-gated.
