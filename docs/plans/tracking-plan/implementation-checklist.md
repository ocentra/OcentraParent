# Tracking Plan Implementation Checklist

A checkbox may be marked `[x]` only after the referenced proof pack exists
under:

```text
output/tracking-plan-proof/<workpack-id>/
```

Every checked item must cite one or more proof artifacts.

## Fill Rules

- Leave a checkbox unchecked until proof exists.
- Add the proof artifact path next to the item in the assigned workpack before
  reporting DONE or PR-ready.
- Every proof item must list required proof tier, current proof tier, status,
  artifact path, and missing proof reason.
- Do not require P4/P5 proof in normal CI. CI passing P0/P1/P2 can make the
  covered code boundary ready, but it cannot prove physical-device,
  authority-enrolled, or production-pilot product claims.
- Do not fail a docs/checklist task because physical proof is unavailable in
  GitHub CI. Generate the manual proof command/artifact expectation and mark the
  product claim `manual_required` or `authority_required`.
- Do not mark platform behavior complete from docs, mocks, screenshots of
  settings only, or simulator-only evidence unless the workpack explicitly
  allows simulator proof.
- Do not mark UI complete without screenshot states and accessibility proof.
- Do not mark product docs complete unless the feature doc and capability
  checklist status/gap text were updated or explicitly justified as unchanged.

## Main Execution Gates

- [x] No precise location is inferred from LAN/IP/pairing. Contract proof:
      `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`.
- [x] Every location sample has source, timestamp, accuracy/hint quality,
      freshness, custody, retention, permission state, confidence, and reason
      codes. Contract proof:
      `output/tracking-plan-proof/04-location-evidence-model/`.
- [x] Geofence transitions cite location evidence and geofence rule refs.
      Contract proof:
      `output/tracking-plan-proof/15-geofence-transition-engine/`.
- [x] Expected-place decisions cite schedule and expected-place rule refs.
      Contract proof:
      `output/tracking-plan-proof/16-expected-place-schedule-engine/`.
- [x] Nearby-place evidence includes query radius, distance, provider,
      category, confidence, and ambiguity state. Contract proof:
      `output/tracking-plan-proof/19-nearby-place-provider-abstraction/`.
- [x] AI location safety result is evidence only and cannot alert/escalate
      directly. Contract proof:
      `output/tracking-plan-proof/23-ai-location-safety-analysis-contracts/`.
- [x] Parent policy is the only authority for notification/action/escalation.
      Contract proof:
      `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`.
- [x] Parent acknowledgement and exceptions can suppress or modify alerts
      according to rules. Contract proof:
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`.
- [x] Critical alerts cannot be suppressed by generic holiday/exception unless
      explicitly configured. Contract proof:
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`.
- [x] Retention/delete/export behavior is implemented and tested at P1
      fixture tier. Retention delete, parent-owned export, and UI-visible
      deleted-history hiding have proof in
      `output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json`;
      `output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json`;
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`.
      Product live service-backed retention settings remain pending.
- [x] Remote sync and remote AI are disabled by default. Contract proof:
      `output/tracking-plan-proof/07-retention-and-custody-model/`,
      `output/tracking-plan-proof/24-ai-provider-routing/`.
- [ ] Android background claims have real device permission/background proof.
- [ ] iOS background/region claims have real device permission/background
      proof.
- [x] Desktop LAN/IP/Wi-Fi presence is labelled hint-only unless OS location
      proof exists. Contract proof:
      `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`.
- [x] Pre-device gap-closure gate reruns tracking P0/P1/P2 proofs, mobile
      scaffold proofs, Android debug package artifact proof, and emits Android
      Studio/iOS simulator/WSL/physical-device proof plans before device work.
      Aggregate proof:
      `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`.
- [x] Android emulator package launch, foreground-service scaffold, UI tree,
      screenshot, logcat, battery, and connectivity dumps are captured without
      claiming foreground location, background location, geofence transitions,
      notification delivery, physical-device, or authority behavior. Emulator
      proof: `test-results/tracking-plan-android-emulator-proof/proof.json`.

## Proof Tier Gates

Use [Tracking Proof Tiers](proof-tiers.md) for every checklist item.

| Gate                                                                      | Required proof tier          | Current proof tier    | Current status     | Artifact path                                                                                                                                                                                                                                                                                             | Missing proof reason                                                                                                                                               |
| ------------------------------------------------------------------------- | ---------------------------- | --------------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| No precise location inferred from LAN/IP/pairing                          | P0_CONTRACT                  | P0_CONTRACT           | proved             | `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`                                                                                                                                                                                                                                 | none                                                                                                                                                               |
| Location evidence source/time/accuracy/freshness/custody/retention schema | P0_CONTRACT                  | P0_CONTRACT           | proved             | `output/tracking-plan-proof/04-location-evidence-model/`                                                                                                                                                                                                                                                  | none for schema; platform ingest remains separate                                                                                                                  |
| Geofence transitions cite location evidence and rule refs                 | P1_FIXTURE_SIMULATION        | P1_FIXTURE_SIMULATION | simulated          | `output/tracking-plan-proof/15-geofence-transition-engine/05-geofence-transition-proof.json`                                                                                                                                                                                                              | Android/iOS physical geofence proof remains P4 manual-required                                                                                                     |
| Expected-place decisions cite schedule and rule refs                      | P1_FIXTURE_SIMULATION        | P1_FIXTURE_SIMULATION | simulated          | `output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json`                                                                                                                                                                                                               | UI and alert policy integration remain pending                                                                                                                     |
| Nearby-place evidence carries provider/radius/category/ambiguity          | P1_FIXTURE_SIMULATION        | P0_CONTRACT           | simulated          | `output/tracking-plan-proof/19-nearby-place-provider-abstraction/`                                                                                                                                                                                                                                        | provider adapter proof remains pending                                                                                                                             |
| Local parent-defined place CRUD/export/delete store                       | P1_FIXTURE_SIMULATION        | P1_FIXTURE_SIMULATION | simulated          | `output/tracking-plan-proof/22-local-parent-defined-place-database/07-nearby-place-proof.json`; `output/tracking-plan-proof/22-local-parent-defined-place-database/14-retention-delete-proof.json`; `output/tracking-plan-proof/22-local-parent-defined-place-database/17-parent-owned-export-proof.json` | platform adapters, provider delivery, live UI, hosted accessibility, physical-device background location, and production persistence remain unclaimed              |
| AI cannot directly alert/escalate                                         | P0_CONTRACT                  | P0_CONTRACT           | proved             | `output/tracking-plan-proof/23-ai-location-safety-analysis-contracts/`                                                                                                                                                                                                                                    | none                                                                                                                                                               |
| Parent policy is alert/action authority                                   | P1_FIXTURE_SIMULATION        | P0_CONTRACT           | simulated          | `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`                                                                                                                                                                                                                                       | runtime compiler/evaluator proof remains pending                                                                                                                   |
| Retention/delete/export behavior                                          | P1_FIXTURE_SIMULATION        | P1_FIXTURE_SIMULATION | simulated          | `output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json`; `output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json`; `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`                   | product live service-backed retention settings remain pending                                                                                                      |
| Tracking service read-model command                                       | P2_HOSTED_CI                 | P2_HOSTED_CI          | proved             | `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`                                                                                                                                                                                                      | narrow portal summary consumption proved; richer product read models, deletion/tombstone replay, hosted UI/accessibility, and platform replay proof remain pending |
| Tracking pre-device proof gate                                            | P2_HOSTED_CI                 | P2_HOSTED_CI          | proved             | `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`                                                                                                                                                                                                                                    | Android Studio/emulator, iOS simulator, WSL/local replay, physical-device, authority, hosted full UI accessibility, and production-pilot artifacts remain separate |
| Android emulator package/service/status scaffold                          | P3_LOCAL_DEV_MACHINE         | P3_LOCAL_DEV_MACHINE  | scaffold_observed  | `test-results/tracking-plan-android-emulator-proof/proof.json`; `output/tracking-plan-proof/08-android-foreground-location-adapter/`; `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`; `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/` | foreground location sample, background/geofence transitions, notification delivery, physical-device, and authority proof remain missing                            |
| Android background behavior                                               | P4_PHYSICAL_DEVICE           | P3_LOCAL_DEV_MACHINE  | manual_required    | `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json`                                                                                                                                                                                        | physical Android background geofence artifact missing; emulator proof records no background/geofence transition claim                                              |
| iOS background/region behavior                                            | P4_PHYSICAL_DEVICE           | P2_HOSTED_CI          | manual_required    | `output/tracking-plan-proof/ios-region-monitoring/`                                                                                                                                                                                                                                                       | physical iOS region/background artifact missing                                                                                                                    |
| Authority hard-control behavior                                           | P5_AUTHORITY_ENROLLED_DEVICE | P0_CONTRACT           | authority_required | no artifact yet                                                                                                                                                                                                                                                                                           | Device Owner, supervised/MDM, AppLocker/App Control, or equivalent authority proof missing                                                                         |

## Base Workpacks

| Step | Workpack                                         | Status                             | Required tier         | Current tier          | Current proof status | Proof artifact path                                                                                                                                                                                                                                                                           | Missing proof reason                                                                                                                                                |
| ---- | ------------------------------------------------ | ---------------------------------- | --------------------- | --------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 01   | Source index and repo reconciliation             | [x] Planning folder created        | P0_CONTRACT           | P0_CONTRACT           | proved               | `docs/plans/tracking-plan/source-index.md`                                                                                                                                                                                                                                                    | none                                                                                                                                                                |
| 02   | Current tracking snapshot and gap map            | [x] Planning folder created        | P0_CONTRACT           | P0_CONTRACT           | proved               | `docs/plans/tracking-plan/current-tracking-snapshot.md`                                                                                                                                                                                                                                       | none                                                                                                                                                                |
| 03   | Contract boundary and Effect schemas             | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/03-contract-boundary-and-effect-schemas/`                                                                                                                                                                                                                         | none                                                                                                                                                                |
| 04   | Location evidence model                          | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/04-location-evidence-model/`                                                                                                                                                                                                                                      | runtime location ingest remains separate                                                                                                                            |
| 05   | Device status model                              | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/05-device-status-model/`                                                                                                                                                                                                                                          | runtime device adapter remains separate                                                                                                                             |
| 06   | Permission and capability status model           | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/06-permission-and-capability-status-model/`                                                                                                                                                                                                                       | platform permission artifact remains P3/P4                                                                                                                          |
| 07   | Retention and custody model                      | [x] Fixture proof complete         | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json`; `output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json`; `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`       | product live service-backed retention settings remain pending                                                                                                       |
| 08   | Android foreground location adapter              | [ ] Emulator scaffold proof only   | P3_LOCAL_DEV_MACHINE  | P3_LOCAL_DEV_MACHINE  | manual_required      | `output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json`                                                                                                                                                                                         | Android package/service proof exists; foreground location permission and sample adapter remain missing                                                              |
| 09   | Android background location and geofence adapter | [ ] Emulator scaffold proof only   | P4_PHYSICAL_DEVICE    | P3_LOCAL_DEV_MACHINE  | manual_required      | `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json`                                                                                                                                                                            | physical Android background geofence artifact missing; current package has no background/geofence transition proof                                                  |
| 10   | Android battery connectivity and status adapter  | [ ] Emulator status scaffold proof | P4_PHYSICAL_DEVICE    | P3_LOCAL_DEV_MACHINE  | scaffold_observed    | `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/04-device-status-proof.json`                                                                                                                                                                                   | emulator battery/connectivity and foreground service observed; offline, killed/restarted, pending-upload, and physical-device proof remain missing                  |
| 11   | iOS Core Location foreground adapter             | [ ] Not started                    | P4_PHYSICAL_DEVICE    | P2_HOSTED_CI          | manual_required      | `output/tracking-plan-proof/ios-core-location-foreground/`                                                                                                                                                                                                                                    | physical iOS Core Location proof missing                                                                                                                            |
| 12   | iOS background region significant-change adapter | [ ] Not started                    | P4_PHYSICAL_DEVICE    | P2_HOSTED_CI          | manual_required      | `output/tracking-plan-proof/ios-region-monitoring/`                                                                                                                                                                                                                                           | physical iOS background/region artifact missing                                                                                                                     |
| 13   | Desktop location and presence hint model         | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`                                                                                                                                                                                                                     | desktop OS precise location proof remains separate                                                                                                                  |
| 14   | Geofence rule model                              | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/14-geofence-rule-model/`                                                                                                                                                                                                                                          | none                                                                                                                                                                |
| 15   | Geofence transition engine                       | [x] Fixture proof complete         | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/15-geofence-transition-engine/05-geofence-transition-proof.json`                                                                                                                                                                                                  | Android/iOS physical geofence proof remains P4 manual-required                                                                                                      |
| 16   | Expected-place schedule engine                   | [x] Fixture proof complete         | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/16-expected-place-schedule-engine/06-expected-place-proof.json`                                                                                                                                                                                                   | UI and alert policy integration remain pending                                                                                                                      |
| 17   | Parent acknowledgement and exception model       | [x] Fixture proof complete         | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/09-policy-alert-proof.json`                                                                                                                                                                                         | alert delivery and portal acknowledgement UI remain pending                                                                                                         |
| 18   | Child check-in flow                              | [x] Fixture proof complete         | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/18-child-check-in-flow/09-policy-alert-proof.json`                                                                                                                                                                                                                | child-device UI, delivery, timeout escalation wiring, and screenshots remain pending                                                                                |
| 19   | Nearby-place provider abstraction                | [x] Contract proof complete        | P1_FIXTURE_SIMULATION | P0_CONTRACT           | simulated            | `output/tracking-plan-proof/19-nearby-place-provider-abstraction/`                                                                                                                                                                                                                            | provider adapter proof remains pending                                                                                                                              |
| 20   | Google Places and POI provider adapter           | [ ] Not started                    | P1_FIXTURE_SIMULATION | P0_CONTRACT           | not_claimed          | `output/tracking-plan-proof/20-google-places-and-poi-provider-adapter/`                                                                                                                                                                                                                       | adapter implementation, field-mask proof, and provider-failure proof missing                                                                                        |
| 21   | Place-risk taxonomy and ambiguity model          | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/21-place-category-taxonomy-and-ambiguity-model/`                                                                                                                                                                                                                  | none                                                                                                                                                                |
| 22   | Local parent-defined place database              | [x] P1 local store proof complete  | P1_FIXTURE_SIMULATION | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/22-local-parent-defined-place-database/07-nearby-place-proof.json`; `output/tracking-plan-proof/22-local-parent-defined-place-database/14-retention-delete-proof.json`; `output/tracking-plan-proof/22-local-parent-defined-place-database/17-parent-owned-export-proof.json` | platform adapters, provider delivery, live UI, hosted accessibility, physical-device background location, and production persistence remain unclaimed               |
| 23   | AI location safety analysis contracts            | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/23-ai-location-safety-analysis-contracts/`                                                                                                                                                                                                                        | runtime/provider routing proof remains separate                                                                                                                     |
| 24   | AI provider routing                              | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/24-ai-provider-routing/`                                                                                                                                                                                                                                          | remote provider adapter remains not claimed                                                                                                                         |
| 25   | Policy compiler for tracking rules               | [ ] Contract proof partial         | P1_FIXTURE_SIMULATION | P0_CONTRACT           | simulated            | `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`                                                                                                                                                                                                                           | runtime compiler/evaluator proof remains pending                                                                                                                    |
| 26   | Alert severity and notification model            | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/26-alert-severity-and-notification-model/`                                                                                                                                                                                                                        | provider delivery remains not claimed                                                                                                                               |
| 27   | Escalation engine                                | [ ] Contract proof partial         | P1_FIXTURE_SIMULATION | P0_CONTRACT           | simulated            | `output/tracking-plan-proof/27-escalation-engine/`                                                                                                                                                                                                                                            | runtime execution proof remains pending                                                                                                                             |
| 28   | Temporary live tracking mode                     | [ ] Contract proof partial         | P3_LOCAL_DEV_MACHINE  | P0_CONTRACT           | manual_required      | `output/tracking-plan-proof/28-temporary-live-tracking-mode/`                                                                                                                                                                                                                                 | live runtime/UI and battery proof missing                                                                                                                           |
| 29   | Missing-device mode                              | [ ] Contract proof partial         | P3_LOCAL_DEV_MACHINE  | P0_CONTRACT           | manual_required      | `output/tracking-plan-proof/29-missing-device-mode/`                                                                                                                                                                                                                                          | runtime/UI and offline proof missing                                                                                                                                |
| 30   | Parent and child UI/UX surfaces                  | [ ] Fixture proof partial          | P2_HOSTED_CI          | P1_FIXTURE_SIMULATION | simulated            | `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-fixture-state-matrix.json`; `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-parent-fixture.png`                                                                           | full service-data UI, child-device UI, hosted Playwright/accessibility, richer live service-backed citations, and physical-device proof remain pending              |
| 31   | Platform extension checklists and proof routing  | [x] Contract proof complete        | P0_CONTRACT           | P0_CONTRACT           | proved               | `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`                                                                                                                                                                                                              | none                                                                                                                                                                |
| 32   | Journal SQLite and read-model proof              | [ ] P2 service proof partial       | P2_HOSTED_CI          | P2_HOSTED_CI          | proved               | `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/10-journal-sqlite-proof.json`; `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`                                                                                        | narrow portal summary consumption proved; deletion/tombstone replay, richer read models, full UI, hosted UI/accessibility, and platform replay proof remain pending |
| 33   | Proof gates fixtures rollout and PR gate         | [x] Pre-device proof gate complete | P2_HOSTED_CI          | P2_HOSTED_CI          | proved               | `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/proof-summary.json`; generated runtime artifact `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/00-run-metadata.json`; `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json` | hosted full UI/a11y, live UI, platform-local, physical-device, authority, and production proof remain pending                                                       |

## Proof Pack Requirements

Each applicable proof pack must include:

- [ ] `00-source-snapshot.md`
- [ ] `01-contract-proof.log`
- [ ] `02-platform-permission-proof.md`
- [ ] `03-runtime-location-evidence.json`
- [ ] `04-device-status-proof.json`
- [ ] `05-geofence-transition-proof.json`
- [ ] `06-expected-place-proof.json`
- [ ] `07-nearby-place-proof.json`
- [ ] `08-ai-analysis-proof.json`
- [ ] `09-policy-alert-proof.json`
- [ ] `10-journal-sqlite-proof.json`
- [ ] `11-ui-snapshots/`
- [ ] `12-playwright-proof.log`
- [ ] `13-security-negative-proof.log`
- [ ] `14-retention-delete-proof.json`
- [ ] `15-manual-platform-proof.md`
- [ ] `16-validation-commands.log`

Focused contract proof roots generated by
`scripts/test/tracking-plan-contract-proof.mjs` currently include
`00-source-snapshot.md`, `01-contract-proof.log`,
`13-security-negative-proof.log`, `15-manual-platform-proof.md`,
`16-validation-commands.log`, and `proof-summary.json`. Full platform,
runtime, hosted Playwright, and journal proof files remain unchecked until
those implementations exist. WP30 additionally has P1 parent portal
fixture evidence in `11-ui-fixture-state-matrix.json`; the repeatable
`node scripts/test/tracking-plan-runtime-proof.mjs` command also captures a
local rendered route screenshot at
`11-ui-snapshots/policy-tracking-parent-fixture.png`; accessibility, child UI,
richer service-data UI, and hosted Playwright proof remain pending.
WP32 additionally has P2 service-command plus narrow portal summary-consumption proof from
`node scripts/test/tracking-plan-service-read-model-proof.mjs`, written to
`18-service-read-model-proof.json`; richer product read models and hosted
portal proof remain pending.
WP33 `proof-summary.json` records a tracked `minimumSeriousMvpAuditSummary`.
The runtime proof command also records the full `minimumSeriousMvpAudit` in
generated `00-run-metadata.json`; both are first-checkpoint P1
reconciliations, not product-complete, PR-ready, or full-scope claims.
WP33 additionally has pre-device gap-closure proof from
`node scripts/test/tracking-plan-pre-device-proof.mjs`, written to
`output/tracking-plan-proof/pre-device-gap-closure/`. That aggregate proof
passes the tracking P0/P1/P2 stack and mobile scaffold proof stack, then emits
Android Studio, iOS simulator, WSL/local, physical-device, and authority proof
plans. It is a pre-device gate, not Android/iOS physical proof.
Android emulator package/service/status proof is generated by
`npm run test:tracking-plan-android-emulator-proof`, writes raw adb evidence to
`test-results/tracking-plan-android-emulator-proof/`, and writes WP08/WP09/WP10
proof artifacts under their assigned proof roots. It proves emulator package
mechanics and status scaffold behavior only; foreground location samples,
background/geofence transitions, notification delivery, physical-device, and
authority claims remain manual-required.

## Documentation Update Rule

Every implementation workpack must update, or explicitly justify not updating:

- `docs/features/location-geofence-device-status.md`;
- `docs/product-capability-checklist.md`;
- the assigned workpack doc;
- this checklist;
- expectation docs if acceptance contracts change;
- roadmap only if milestone scope, order, or completion changes.

## Progress Reconciliation

- [x] Feature doc exists.
- [x] Expectation doc exists.
- [x] Capability guide exists.
- [x] Schema proposal exists.
- [x] Raw tracking settings inventory exists.
- [x] Tracking plan folder exists.
- [x] Location evidence contracts have focused contract proof; runtime and UI
      remain not product-complete.
- [x] Geofence transition deterministic runtime proof exists at P1 fixture
      tier; Android/iOS physical geofence proof remains manual-required.
- [x] Expected-place schedule engine deterministic runtime proof exists at P1
      fixture tier; UI and alert policy integration remain pending.
- [x] Nearby-place/AI safety analysis contracts have focused contract proof;
      provider runtime and UI remain not product-complete.
- [x] Local parent-defined place database has P1 CRUD/import/export/delete
      store proof with parent-device-local default storage, remote sync
      disabled, parent-owned export, deletion tombstone, and safe/restricted
      policy signals; platform adapters, provider delivery, live UI, hosted
      accessibility, physical-device behavior, and production persistence
      remain unclaimed.
- [x] Parent acknowledgement/exception fixture proof exists at P1 tier;
      alert delivery and portal acknowledgement UI remain pending.
- [x] Child check-in fixture proof exists at P1 tier; child-device UI,
      delivery, timeout escalation wiring, and screenshots remain pending.
- [x] Android emulator package launch, foreground-service scaffold, battery,
      and connectivity status proof exists at P3 local-dev tier; foreground
      location and background/geofence behavior are not claimed by it.
- [ ] Android background permission proof is not complete. The emulator proof
      records this as manual-required instead of product-ready.
- [ ] iOS background/region proof is not complete.
- [x] Retention/delete/export P1 checkpoint proof exists. Retention delete has
      P1 read-model proof, parent-owned export has P1 snapshot proof, and
      UI-visible deleted-history hiding has P1 route fixture proof; product
      live-service retention settings remain pending.
- [x] Tracking service read-model command has P2 proof for SQLite tracking rows,
      citation IDs through `trackingReadModel`, and narrow parent portal summary
      consumption; deletion/tombstone replay, richer read models, hosted
      UI/accessibility, and platform replay proof remain pending.
- [ ] Tracking UI/UX is not product-complete. A P1 parent portal tracking-state
      fixture and local parent-route screenshot exist; live parent/child UI,
      accessibility, service-data, hosted proof, and physical-device proof
      remain pending.
- [x] Minimum Serious MVP first-checkpoint audit exists in tracked
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/proof-summary.json`
      and generated
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/00-run-metadata.json`;
      it explicitly blocks product-complete, PR-ready, and full-scope claims
      until the remaining live UI, platform, hosted, and authority
      proof gaps are closed.
- [x] Pre-device proof gate exists in
      `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`
      and was generated by
      `node scripts/test/tracking-plan-pre-device-proof.mjs`; it proves the
      current tracking P0/P1/P2 stack and mobile scaffold/package proof stack
      while preserving Android Studio/emulator, iOS simulator, WSL/local,
      physical-device, authority, hosted full UI accessibility, and production
      proof as separate gates.
- [x] Product capability checklist row update is queued through the hub
      DOC_DELTA queue instead of this worker branch; the queued row records the
      pre-device proof gate plus Android emulator scaffold proof, while
      preserving remaining Android foreground/background location, iOS
      simulator, WSL/local, physical-device, authority, hosted UI, and
      production gaps.

## UI Snapshot Gates

- [x] Parent route fixture covers tracking off, permission-required, stale,
      offline, low accuracy, ambiguous nearby place, alert, acknowledgement,
      exception, child check-in, temporary live, missing device, and
      retention-deleted states at P1 with local proof artifact references. The
      runtime proof command captures a local rendered parent-route screenshot;
      live-data screenshots, hosted Playwright/a11y proof, and child UI remain
      pending.
- [ ] Child snapshots cover check-in, disclosure, safe/help responses, and
      location-share consent.
- [ ] Screenshots are stored under the assigned proof root. The runtime proof
      command captures the local parent fixture screenshot at
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/policy-tracking-parent-fixture.png`;
      child, live-data, and hosted screenshots remain pending.
- [ ] Accessibility output is stored with the UI proof.

## Evidence Quality Gates

- [ ] Every location-derived UI or alert cites evidence refs.
- [ ] Every geofence transition cites rule refs and source evidence refs.
- [ ] Every nearby-place result carries radius, provider, category, distance,
      confidence, and ambiguity state.
- [ ] Every AI result carries source refs and no final action.
- [ ] Every alert carries policy decision refs.
- [ ] Every retention/delete/export claim has before/after proof.

## Fixture And Manual Gates

- [ ] Fixtures cover fresh, stale, offline, denied, low accuracy, ambiguous,
      exception, acknowledged, check-in, temporary-live-expired,
      missing-device, retention-deleted, remote-sync-disabled, and
      remote-AI-disabled states.
- [ ] Android/iOS/manual desktop claims include real-device or explicitly
      approved manual proof.
- [x] Pre-device proof records unsupported or not-yet-proved platform states as
      `manual_required` or `authority_required` instead of fake capability.
- [ ] Unsupported platforms render manual-required state instead of fake
      capability.

## Explicit Merge-Blocking Checklist

Block PR-ready or merge-ready status when any of these are true:

- precise location is inferred from LAN/IP/pairing;
- stale data is displayed as live;
- nearby POI is displayed as exact child place without proof;
- AI triggers notification/escalation without policy decision;
- remote sync or remote AI runs by default;
- Android/iOS background behavior is claimed without manual proof;
- retention delete leaves visible cached history;
- workpack proof root or validation log is missing.

## Worker Report Template

```text
DONE <workpack-id> <branch> <commit>
Touched files:
Validation:
Proof artifacts:
Product docs/checklist updated:
Known gaps/manual-required:
```
