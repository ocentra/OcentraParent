# WP30 Parent And Child UI/UX Surfaces

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP30 Parent And Child UI/UX Surfaces`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Bounded Completion Status

**DONE for the reviewed product-consumer truthfulness slice.** The real
`PolicyTracking` route consumes the typed Rust tracking snapshot, proof fixtures
remain confined to `ProofPanels`, malformed or missing snapshots fail closed,
and the hosted desktop and mobile routes render an honest unavailable state
without fabricated tracking rows.

This bounded result does not close the broader parent/child product-ready UI
backlog below. Live location, authenticated child delivery, retention mutation,
notification delivery, policy action authority, and end-to-end restart
projection remain owned by their upstream runtime workpacks.

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

Durable manifest:
[`WP30_PARENT_CHILD_UI_UX_SURFACES_PROOF.md`](../../../proof/tracking-plan/WP30_PARENT_CHILD_UI_UX_SURFACES_PROOF.md)

- `11-ui-snapshots/`
- `11-ui-fixture-state-matrix.json`
- `11-ui-snapshots/policy-tracking-parent-fixture.png`
- `11-ui-snapshots/hosted-policy-tracking-live-summary.png`
- `11-ui-snapshots/hosted-policy-tracking-live-summary-mobile.png`
- `11-ui-snapshots/hosted-policy-tracking-citation-detail.png`
- `11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png`
- `11-ui-snapshots/hosted-policy-tracking-child-check-in.png`
- `11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png`
- `11-ui-snapshots/hosted-policy-tracking-family-dashboard-rollup.png`
- `11-ui-snapshots/hosted-policy-tracking-report-export.png`
- `11-ui-snapshots/hosted-policy-tracking-report-policy-consumer.png`
- `11-ui-snapshots/hosted-policy-tracking-retention-settings.png`
- `11-ui-snapshots/hosted-policy-tracking-notification-parent-surface.png`
- `11-ui-snapshots/hosted-policy-tracking-parent-action-readiness.png`
- `11-ui-snapshots/hosted-policy-tracking-missing-device.png`
- `12-playwright-proof.log`
- `13-security-negative-proof.log`
- `16-validation-commands.log`
- `17-hosted-ui-proof.json`
- `18-service-data-ui-proof.json`
- `19-child-runtime-ui-proof.json`
- `20-evidence-drawer-hosted-ui-proof.json`
- `21-report-export-hosted-ui-proof.json`
- `21-hosted-ui-artifact-inventory-proof.json`
- `22-notification-parent-surface-hosted-ui-proof.json`
- `23-parent-action-readiness-hosted-ui-proof.json`
- `24-missing-device-hosted-ui-proof.json`
- `25-report-policy-consumer-hosted-ui-proof.json`
- `26-child-runtime-delivery-boundary-proof.json`
- `27-child-runtime-execution-readiness-proof.json`
- `28-child-runtime-snapshot-requirements-proof.json`
- `29-child-runtime-product-readiness-blocker-proof.json`
- `30-child-runtime-artifact-gate-proof.json`
- `34-full-product-ui-runtime-preflight-proof.json`
- Unsupported/manual platform screenshot:
  `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/19-unsupported-manual-hosted-ui.png`
- Accessibility summary:
  `test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json`
- Pre-device gate:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

## Bounded Completion Checklist

- [x] Mount the Rust-owned tracking snapshot on the real `PolicyTracking` route.
- [x] Keep fixed proof fixtures confined to the `ProofPanels` route.
- [x] Remove the false vendor `unwired` state for the mounted tracking surface.
- [x] Fail closed for missing, unavailable, and malformed tracking snapshots.
- [x] Pass the focused Rust, Portal-domain, Portal, and hosted route behavior tests.
- [x] Retain the generated proof bundle through a checked-in manifest with exact hashes.

## Broader Product-Ready Backlog (Not Closed By This Bounded Slice)

- [ ] Add Playwright coverage for all hosted proof-route screens and badges.
- [ ] Add no-overlap/no-overclaim screenshot proof for the hosted proof route.
- [ ] Add first-target parent route tracking state fixture surface.
- [ ] Capture local rendered parent-route fixture screenshot.
- [ ] Keep UI rows at no-product-claim with P1 fixture status.
- [ ] Ensure deleted history disappears from the P1 parent route fixture.
- [ ] Render local proof artifact references for each parent route fixture row.
- [ ] Feed UI proof gaps into the pre-device proof gate before device work.
- [ ] Render live service-backed read-model citation rows with evidence refs
      and retention tombstone refs.
- [ ] Capture hosted service-backed citation detail screenshot and accessibility
      proof without adding child-device, provider, physical-device, or production
      claims.
- [ ] Capture hosted parent route desktop/mobile screenshot and accessibility
      proof against the real Rust service.
- [ ] Render and screenshot hosted child-safe check-in copy/actions with no
      child-device delivery/runtime claim.
- [ ] Render hosted parent service-data coverage from the parsed
      `trackingReadModel` payload without physical-device, provider, or
      production claims.
- [ ] Render and screenshot hosted child-runtime disclosure, safe/help response,
      location-share consent, and delivery-boundary copy without claiming
      child-device delivery or runtime execution.
- [ ] Carry the hosted child-safe check-in and hosted child-runtime UI readiness
      screenshots into the full-product local artifact capture proof while
      keeping the actual child-device runtime artifact refs missing.
- [ ] Add a child-runtime delivery boundary proof that links hosted child-runtime
      UI proof refs to child check-in timeout rows while recording required
      runtime/device proof refs and keeping actual child-device delivery,
      execution, physical-device, authority, provider delivery, production, and
      product-ready claims false.
- [ ] Add a child-runtime execution readiness proof that consumes the
      delivery-boundary rows and records delivery-envelope, execution-result,
      visible-snapshot, parent-receipt, and runtime-observation requirement refs
      while keeping actual child-device delivery/execution, physical-device,
      authority, provider delivery, production, and product-ready claims false.
- [ ] Add a child-runtime snapshot requirements proof that consumes the
      execution-readiness rows and verifies delivery-envelope,
      execution-result, visible-snapshot, parent-receipt, and
      runtime-observation refs for each child check-in state while keeping
      actual child-device delivery/execution, rendered child-device runtime UI,
      physical-device, authority, provider delivery, production, and
      product-ready claims false.
- [ ] Add a child-runtime product-readiness blocker proof that consumes the
      snapshot requirements rows and the Android emulator readiness bridge,
      records package launch, foreground-service, and local emulator geofence
      prerequisite accounting, preserves exact required/present/missing
      child-runtime artifact refs/counts from the artifact gate, and records the
      remaining actual delivery, execution-result, rendered child UI, parent
      receipt, runtime observation, physical-device, and authority blockers
      without claiming child-device delivery, runtime execution, provider
      delivery, production, or product-ready behavior.
- [ ] Add a child-runtime artifact gate proof that verifies the required real
      child-device delivery/execution artifact names and keeps actual delivery,
      execution, rendered child UI runtime, parent receipt runtime, runtime
      observation, physical-device, authority, provider delivery, production,
      and product-ready claims false until those artifacts exist.
- [ ] Add a full product UI runtime preflight proof that turns the four remaining
      hard product UI runtime artifact refs into manual-required rows: retention
      settings production write-result UI, rendered child-device check-in,
      rendered child-device location consent, and child-device safe/help
      response. The proof keeps full product UI runtime, child-device runtime,
      physical-device, authority, provider delivery, production product UI, and
      product-ready claims false while closure/handoff carry the four required,
      zero present, four missing artifact counts.
- [ ] Render and screenshot hosted family dashboard rollup rows without claiming
      full dashboard UI, child-device delivery, authority, provider delivery,
      physical-device execution, or production readiness.
- [ ] Render and screenshot hosted report/export read-model packet rows without
      claiming raw location payload export, service mutation, platform runtime,
      child-device delivery, provider delivery, notification receipt ingestion,
      physical-device proof, authority, or product readiness.
- [ ] Render and screenshot hosted retention settings read-model rows without
      claiming writable settings, service mutation, platform runtime,
      child-device delivery, provider delivery, authority, physical-device
      execution, or production readiness.
- [ ] Send and render hosted retention settings local service write result from
      the typed service command while proving only local service execution,
      local service state revision, and local durable settings persistence, and
      render the local service-state proof artifact reference, while keeping
      durable production/product-ready settings, platform
      runtime, child-device delivery, provider delivery, authority,
      physical-device execution, and product-ready claims false.
- [ ] Render and screenshot hosted read-only evidence drawer drill-in from the
      selected service-backed citation without claiming policy evaluation,
      action dispatch, child-device delivery, provider delivery, physical-device
      proof, authority, or product readiness.
- [ ] Render and screenshot hosted notification parent-surface
      history/preference-intent rows without claiming writable preference
      mutation, provider delivery, receipt ingestion, child-device delivery,
      physical-device proof, authority, production storage, or product readiness.
- [ ] Render and screenshot hosted parent action readiness rows for
      expected-place alert policy and parent acknowledgement actions without
      claiming live service mutation, alert/provider delivery, receipt
      ingestion, child-device runtime, physical-device proof, authority,
      production workers, adapter dispatch, or product readiness.
- [ ] Render and screenshot hosted missing-device state rows for last-known,
      offline/powered-off, contact-requested, and manual-required states without
      claiming current-location runtime, powered-off tracking, remote sync,
      provider delivery, physical-device proof, OS lost-mode API execution,
      authority, production workers, or product readiness.
- [ ] Render and screenshot hosted report/policy consumer rows for parent report
      summary, policy evidence drill-in, and retention audit export without
      claiming AI execution, product policy mutation, platform runtime,
      child-device delivery, provider delivery, notification receipt ingestion,
      physical-device proof, authority, production, or product readiness.
- [ ] Verify all current hosted screenshot PNG artifacts, evidence drawer proof
      output, unsupported/manual platform proof output, accessibility
      assertions, and no-overlap layout geometry are present and non-empty
      through a dedicated artifact inventory proof, including the child-runtime
      execution readiness artifact, without claiming full parent/child UI or
      device runtime execution.
- [ ] Ensure child copy avoids accusation.
- [ ] Keep portal as authoring/display surface, not evaluator for the hosted
      evidence drawer proof; broader UI/product paths remain separately gated.

## Where We Are

A P1 parent portal proof route renders a first-target tracking state
matrix for tracking off, permission-required, stale, offline, low accuracy,
ambiguous nearby place, policy alert, parent acknowledgement, exception,
child check-in, temporary live, missing device, and retention-deleted states,
including a retention-deleted row that marks deleted history hidden and does
not render the deleted evidence id. Each row also renders the local proof
artifact path that backs the fixture state.
The fixture is implemented in `apps/portal/src/tracking-status-panel.ts` and
`apps/portal/src/TrackingStatusRoutePanel.tsx`, currently attached only to the
`proof-panels` route. The `policy-tracking` product route does not currently
mount that panel, the shared Rust snapshot appends the fixed proof fixtures to
both routes, and the vendor surface still reports that the Rust read model is
unwired. These are open source defects, not proof of a live product surface.
The component is covered by
`apps/portal/tests/unit/tracking-status-panel.test.ts`, and recorded in
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`.
The repeatable `node scripts/test/tracking-plan-runtime-proof.mjs` command now
captures and records the local rendered full-page screenshot at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-parent-fixture.png`.
The Rust bridge already produces a narrow live service summary and
service-backed citation rows for the P2 `trackingReadModel` event, covered by
`apps/portal/tests/unit/tracking-status-panel.test.ts` and the service read-model
proof script. The bounded repair must keep those real rows on the
`policy-tracking` product route while confining fixed `fixture` and
`ui-fixture` rows to `proof-panels`. The service-data coverage panel includes
active/tombstone row counts, latest tombstone metadata, kind coverage,
custody/capability, active evidence references, deleted evidence references,
and `productClaimReady=false`. The repeatable
`npm run test:tracking-plan-service-data-ui-proof` command records this under
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`.
`npm run test:tracking-plan-hosted-ui-proof` now starts the real
Rust service with a seeded temporary ActivityStore SQLite database, drives the
hosted React parent `policy-tracking` route through Playwright, renders the
service-data coverage card beside the service read-model summary, captures
desktop and mobile screenshots, writes accessibility summary output, and records
`productClaimReady=false`. The hosted proof now also marks the live
service-backed citation detail card and captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`.
The hosted route now also renders a child-safe check-in proof card with calm
copy, safe/help/share/call actions, and an explicit "child-device delivery not
proved" boundary. The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-check-in.png`
and records the non-claim in proof output.
The hosted route now also renders a parent action readiness proof card for WP16
expected-place alert policy rows and WP17 parent acknowledgement action rows.
The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-parent-action-readiness.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/23-parent-action-readiness-hosted-ui-proof.json`
without claiming live mutation, alert/provider delivery, receipt ingestion,
child-device runtime, physical-device proof, authority, production workers,
adapter dispatch, or product readiness.
The hosted route now also renders a missing-device state proof card for WP29
last-known-only, offline/powered-off, contact-requested, and manual-required
rows. The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-missing-device.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/24-missing-device-hosted-ui-proof.json`
without claiming current-location runtime, powered-off tracking, remote sync,
provider delivery, physical-device proof, OS lost-mode API execution,
authority, production workers, or product readiness.
The hosted route now also renders a child-runtime UI proof card with tracking
disclosure, safe/help response labels, location-share consent copy, a hosted-only
adapter boundary, and no child-device delivery claim. The repeatable hosted proof
captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json`.
The hosted route now also renders a family dashboard rollup proof card for the
existing active family, child-attention, and retention-audit summary rows. The
repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-family-dashboard-rollup.png`
and records the same no-product-claim boundary in hosted proof output.
The hosted route now also renders a retention settings read-model proof card for
retention window, delete-after-alert, parent export, remote-sync disabled, and
remote-AI disabled rows. The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`
and now also clicks the hosted local write button, renders the typed service
accepted result, command id, setting kind, source writer/read-model refs,
mutation proof refs, applied local retention values, local service state
revision/snapshot ref, local durable settings persistence, and explicit
no-product-claim boundary. It still records no durable production/product-ready
settings, product-ready service mutation execution, platform
runtime, child-device delivery, provider delivery, authority, physical-device,
or product readiness claim.

The child-runtime product-readiness blocker proof now consumes the existing
parent-child local runtime bridge alongside the child runtime snapshot
requirements proof and Android emulator readiness bridge. It records typed local
transport, stored-event count, zero dead letters, child-agent phase coverage,
and parent read-model projection before the final physical child-device runtime
handoff, while keeping child-device delivery/execution, rendered child runtime
UI, parent receipt runtime, physical-device, authority, provider, production,
and product-ready claims false.
The hosted route now also renders a read-only evidence drawer card from the
selected service-backed citation. The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json`
while keeping policy evaluation, action dispatch, child-device delivery,
provider delivery, physical-device proof, authority, and product readiness
unclaimed.
The hosted route now also renders read-only notification parent-surface
history/preference-intent rows derived from the WP26 notification proof stack.
`packages/portal-domain/src/tracking-notification-parent-surface-hosted-ui-proof.ts`
now maps those hosted rows from a structured notification-history read-model
shape instead of a token-only table, and
`packages/portal-domain/tests/unit/tracking-notification-parent-surface-hosted-ui-proof.test.ts`
covers the schema-backed portal consumer plus the explicit invalid-input
fallback.
The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-notification-parent-surface.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/22-notification-parent-surface-hosted-ui-proof.json`
while keeping writable preference mutation, provider delivery, receipt
ingestion runtime, child-device delivery, physical-device proof, authority,
production storage, adapter dispatch, and product readiness unclaimed.
The hosted route now also renders read-only report/policy consumer rows derived
from the WP32 parent report summary, policy evidence drill-in, and retention
audit export consumer proof. The repeatable hosted proof captures it at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-report-policy-consumer.png`
and writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/25-report-policy-consumer-hosted-ui-proof.json`
while keeping AI execution, product policy mutation, platform runtime,
child-device delivery, provider delivery, notification receipt ingestion,
physical-device proof, authority, production, and product readiness unclaimed.
`node scripts/test/tracking-hosted-ui-artifact-inventory-proof.mjs` now verifies
the full current hosted screenshot PNG inventory, hosted proof outputs,
unsupported/manual platform screenshot/proof output, parent overview/devices
shell screenshots around the tracking route, including service-backed tracking
summary and no-product-claim assertions on the normal overview/devices routes,
accessibility assertions, and 11-card no-overlap layout geometry as a separate
inventory gate. It also
verifies both the child-runtime execution readiness artifact and the
child-runtime snapshot requirements artifact so those runtime-readiness refs do
not become orphan proof rows. It writes
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/21-hosted-ui-artifact-inventory-proof.json`
and
`output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/28-hosted-ui-artifact-inventory-proof.json`
without claiming full product parent/child UI, child-device runtime,
physical-device proof, authority, provider delivery, production proof, or
product-ready tracking.
This is not product-complete UI proof: the normal overview/devices route cards
are local hosted summary evidence only, and full dashboard UI beyond the hosted
parent shell, actual child-device delivery/runtime execution, applied
product-ready retention settings execution, physical-device evidence, authority,
provider delivery, and production proof remain pending.
`node scripts/test/tracking-full-product-ui-local-runtime-artifact-capture-proof.mjs`
now also consumes the retention product-settings writable execution derivation
proof and child-runtime artifact gate proof as closure evidence for the local
product UI trace. It copies the hosted retention settings local write-result
screenshot into the product UI artifact root as a local artifact, records one
local retention writable execution row/derivation and the ten missing
child-runtime artifacts, and keeps the missing production write-result runtime
artifact, full product UI runtime, child-device runtime, physical-device,
authority, provider delivery, production UI, and product-ready claims false.
`node scripts/test/tracking-plan-pre-device-proof.mjs` now records those UI
gaps in the aggregate pre-device gate so the next pass can run actual
child-device runtime execution and full parent/child UI proof beyond the hosted
parent route before any product claim.
`node scripts/test/tracking-child-runtime-artifact-gate-proof.mjs` now records
the exact required child-device runtime artifact names under WP30/WP33 and keeps
the row `manual-required` until real delivery envelope, execution result,
rendered child UI snapshot, parent receipt, runtime observation, consent state,
device log, and result summary artifacts exist.
`node scripts/test/tracking-child-runtime-android-emulator-readiness-bridge-proof.mjs`
now links Android emulator package/foreground-service/permission and local
emulator geofence evidence to the child-runtime artifact gate under
WP08/WP30/WP33. The bridge records emulator prerequisites observed while
preserving the missing child-device delivery envelope, execution result,
rendered child UI snapshot, parent receipt, runtime observation, consent state,
device log, and result summary artifacts as manual-required P4 proof.

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
- vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx
- apps/portal/src/portal-route-content.ts
- apps/portal/src/tracking-status-panel.ts
- apps/portal/src/styles/parent-portal-route.css
- apps/portal/tests/unit/tracking-status-panel.test.ts
- apps/portal/tests/e2e/tracking-hosted-ui-proof.spec.ts
- packages/text-domain/src/portal-dev.ts
- packages/portal-domain/src/contracts.ts
- packages/portal-domain/src/details.ts
- packages/portal-domain/src/tracking-status-proof-artifacts.ts
- `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`

## Manual-Required Gaps

- Full dashboard UI beyond the hosted parent route, actual child-device
  delivery/runtime execution, physical-device proof, authority proof, provider
  delivery, and production proof remain manual-required until the assigned proof
  artifacts exist. The hosted overview/devices cards are service-backed summary
  evidence only and do not close those product-runtime blockers.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch:
      `codex/tracking-live-service-citation-proof`.
- [ ] Touched files: portal tracking status renderer/tests, service proof
      script, tracking feature doc, implementation checklist, WP30, WP32,
      WP33, and generated WP32 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`
      and companion WP32 proof files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and WP33 updated; central capability checklist
      row delta queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: full parent/child UI, hosted
      Playwright/accessibility output, Android/iOS physical-device proof,
      authority, provider delivery, notifications, and production proof remain
      proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-hosted-ui-accessibility-proof-v2`.
- [ ] Touched files: hosted Playwright proof spec, hosted proof script, root
      script wiring, parent route tracking CSS, tracking feature doc,
      implementation checklist, WP30, WP33, and generated hosted proof
      artifacts.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-hosted-ui-proof` passed locally.
- [ ] Proof artifacts under
      `test-results/tracking-plan-hosted-ui-proof/`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`, and
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: full service-data UI beyond the hosted
      parent route, actual child-device delivery/runtime execution, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-child-check-in-ui-proof`.
- [ ] Touched files: hosted child check-in proof model, portal tracking status
      renderer/tests, hosted Playwright proof spec, hosted proof script,
      portal/text domain constants, tracking feature doc, implementation
      checklist, WP30, WP33, and generated hosted proof artifacts.
- [ ] Validation commands and results: `npm run test:tracking-plan-hosted-ui-proof`
      passes locally after focused portal/text tests.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-check-in.png`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion WP30/WP33
      hosted proof JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: actual child-device delivery/runtime
      execution, full parent/child UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.

- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: full product UI readiness blocker proof/test, focused
      proof script, product-readiness closure proof model, closure harness,
      owning tracking feature doc, implementation checklist, WP30, WP33, and
      generated blocker/closure proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-full-product-ui-readiness-blocker-proof.mjs`
      passed; `node scripts/test/tracking-product-readiness-closure-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/31-full-product-ui-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/56-full-product-ui-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/tracking-full-product-ui-readiness-blocker-proof/proof.json`,
      `output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json`,
      `test-results/tracking-full-product-ui-readiness-blocker-proof/proof.json`,
      and `test-results/tracking-product-readiness-closure-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP33 updated. Central product capability checklist
      update will be queued through hub doc-deltas; this branch does not edit
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: hosted route evidence is acknowledged,
      but full parent/child product UI beyond the hosted route, rendered
      child-device runtime UI, parent receipt UI, physical-device UI proof,
      authority-gated UI proof, provider-delivery UI proof, production product
      UI, and product-ready tracking UI remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted parent action readiness proof model, portal route
      renderer/tests, hosted Playwright proof spec, hosted proof script,
      portal/text/domain constants, owning tracking feature doc, implementation
      checklist, WP16, WP17, WP30, WP33, and generated hosted proof artifacts.
- [ ] Validation commands and results: pending final hosted proof refresh after
      focused text-domain, portal-domain, and portal tracking-status tests
      passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-parent-action-readiness.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/23-parent-action-readiness-hosted-ui-proof.json`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion WP16/WP17/WP33
      hosted proof JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP16, WP17, WP30, and WP33 updated; central capability row
      delta remains hub-sequenced because E-B owns
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: hosted parent action readiness is
      read-only rendering proof only; actual child-device delivery/runtime
      execution, full parent/child UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications,
      production workers, adapter dispatch, and product-ready behavior remain
      proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted notification parent-surface proof model, portal
      tracking status route renderer, portal hosted Playwright proof spec,
      portal tests, hosted proof script, portal/text/domain constants, owning
      tracking feature doc, implementation checklist, WP26/WP30/WP33 docs, and
      generated hosted proof artifacts.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/text-domain` passed;
      `cmd /c npm run build --workspace @ocentra-parent/portal-domain` passed;
      `cmd /c npm run test --workspace @ocentra-parent/portal --
tracking-status-panel` passed; `cmd /c npm run lint --workspace
@ocentra-parent/portal` passed; `cmd /c npm run
test:tracking-plan-hosted-ui-proof` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-notification-parent-surface.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/22-notification-parent-surface-hosted-ui-proof.json`,
      `output/tracking-plan-proof/26-alert-severity-and-notification-model/27-notification-parent-surface-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/35-notification-parent-surface-hosted-ui-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP26, WP30, and WP33 updated; central capability row delta
      stays hub-sequenced instead of editing
      `docs/product-capability-checklist.md` while E-B owns that lock.
- [ ] Known gaps/manual-required states: hosted notification parent-surface rows
      are read-only rendering proof only; writable preference mutation,
      provider delivery, receipt ingestion runtime, actual child-device
      delivery/runtime execution, Android/iOS physical-device proof, authority,
      production storage, adapter dispatch, full parent/child UI beyond the
      hosted route, and product-ready notification behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted retention settings UI proof model, React route
      panel, DOM tracking status panel, live activity state, event-result
      routing, hosted Playwright proof, portal tests, text-domain tokens,
      portal-domain artifact marker, hosted UI proof harness, tracking feature
      doc, implementation checklist, WP07, WP30, WP32, and regenerated hosted UI
      proof screenshots/results.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/text-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
      passed; `cmd /c npm run test --workspace @ocentra-parent/portal --
tracking-status-panel` passed; `cmd /c npm run build --workspace
@ocentra-parent/portal` passed; `cmd /c npm run format:check` passed;
      `cmd /c npm run test:tracking-plan-hosted-ui-proof` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/18-hosted-ui-accessibility-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, WP30, and WP32 updated; central product capability
      checklist remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: hosted route only proves command/result
      rendering plus local service execution, local service state revision, and
      local durable settings persistence. Durable production/product-ready
      settings, applied product-ready service mutation
      execution, platform runtime, child-device delivery/runtime execution,
      Android/iOS physical proof, authority,
      provider delivery, notification receipts, production workers, and full
      parent/child UI beyond the hosted route remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-evidence-drawer-hosted-ui-proof`.
- [ ] Touched files: hosted evidence drawer proof model, React and DOM tracking
      status renderers, portal/text/domain constants, hosted Playwright proof
      spec, hosted proof script, owning tracking feature doc, implementation
      checklist, WP30, WP32, and generated hosted proof artifacts.
- [ ] Validation commands and results: pending final focused hosted proof refresh
      and guard run before PR-ready report.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-evidence-drawer.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion hosted proof
      JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability checklist row delta
      stays hub-sequenced instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: policy evaluation, action dispatch,
      actual child-device delivery/runtime execution, full parent/child UI
      beyond the hosted route, Android/iOS physical-device proof, authority,
      provider delivery, notifications, and production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-family-dashboard-hosted-ui-proof`.
- [ ] Touched files: hosted parent route renderer, tracking status renderer,
      portal/text/domain constants, hosted Playwright proof spec, hosted proof
      script, owning tracking feature doc, implementation checklist, WP30, and
      generated hosted proof artifacts.
- [ ] Validation commands and results: focused portal/text/domain tests and
      `node scripts/test/tracking-plan-hosted-ui-proof.mjs` pass locally after
      proof refresh.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-family-dashboard-rollup.png`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion WP30/WP33
      hosted proof JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP30 updated. Central capability checklist row delta is
      queued/reported through hub because E-C owns
      `docs/product-capability-checklist.md`; portal README was not edited
      because E-D owns `apps/portal/README.md`.
- [ ] Known gaps/manual-required states: actual child-device delivery/runtime
      execution, full dashboard UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-hosted-citation-detail-proof`.
- [ ] Touched files: hosted parent route renderer, tracking status renderer,
      portal-domain proof marker, hosted Playwright proof spec, owning tracking
      feature doc, implementation checklist, WP30, WP32, portal README, and
      generated hosted proof artifacts.
- [ ] Validation commands and results: pending final local proof refresh and
      guard run before PR-ready report.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-citation-detail.png`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion hosted proof
      JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, WP32, and portal README updated; central capability row
      delta stays hub-sequenced instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: actual child-device delivery/runtime
      execution, full dashboard UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-family-dashboard-rendered-ui-proof`.
- [ ] Touched files: React portal tracking status route renderer, hosted
      Playwright proof spec, service-data UI proof script, tracking feature doc,
      implementation checklist, WP30, and generated hosted/service-data proof
      artifacts.
- [ ] Validation commands and results:
      `cmd /c npm run test --workspace @ocentra-parent/portal -- tracking-status-panel`
      and `cmd /c npm run lint:exec --workspace @ocentra-parent/portal` passed
      before final proof refresh.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
      `test-results/tracking-plan-service-data-ui-proof/proof.json`, and
      `test-results/tracking-plan-hosted-ui-proof/accessibility-summary.json`
      after final proof refresh.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, and WP30 updated; central capability checklist not edited
      because another lane owns that lock.
- [ ] Known gaps/manual-required states: parent-domain family dashboard rollup
      rendering needs the existing rollup contract exported before portal can
      consume it without duplicating rows; actual child-device delivery/runtime
      execution, full parent/child UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-service-data-ui-proof`.
- [ ] Touched files: portal tracking status renderer/tests, portal text token
      source/tests, service-data UI proof script, root script wiring, tracking
      feature doc, implementation checklist, WP30, WP32, and generated WP30/WP32
      service-data UI proof artifacts.
- [ ] Validation commands and results:
      `npm run test:tracking-plan-service-data-ui-proof` passed locally after
      focused text-domain and portal tracking status panel tests.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
      and `test-results/tracking-plan-service-data-ui-proof/proof.json`.
- [ ] Service-backed citation matrix recorded in the same proof artifacts,
      covering the read-model command/event/payload, live citation fields,
      active evidence refs, tombstone deleted-evidence refs, and no-product
      claim boundaries used by the hosted service-data coverage card.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and WP32 updated; central capability row delta queued
      through the hub instead of editing `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: full parent/child UI beyond the hosted
      parent route, actual child-device delivery/runtime execution, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-child-runtime-ui-proof`.
- [ ] Touched files: hosted child runtime UI proof model, portal tracking status
      renderer/tests, hosted Playwright proof spec, hosted proof script,
      portal/text domain constants, tracking feature doc, implementation
      checklist, WP30, portal README, and generated hosted proof artifacts.
- [ ] Validation commands and results: `npm run test:tracking-plan-hosted-ui-proof`
      passes locally after focused portal/text/domain tests.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/19-child-runtime-ui-proof.json`,
      `test-results/tracking-plan-hosted-ui-proof/`, and companion WP30/WP33
      hosted proof JSON files.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP30, and portal README updated; central capability row delta
      queued through the hub instead of editing
      `docs/product-capability-checklist.md`.
- [ ] Known gaps/manual-required states: actual child-device delivery/runtime
      execution, full parent/child UI beyond the hosted route, Android/iOS
      physical-device proof, authority, provider delivery, notifications, and
      production proof remain proof-gated.
