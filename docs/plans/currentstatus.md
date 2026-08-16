# Current Plan Status Matrix

This file is hand-written from inspection of the current worktree source plus the owning `WORKPACK_INDEX.md` files. It is a working implementation inventory, not a validation or proof artifact.

## Working Process

- This file is the execution checklist for the current coding round.
- Primary objective for this round: finish **production/core code + corresponding tests** plan by plan.
- First pass is **strictly code and test writing**:
  - do **not** run the full test suite yet
  - do **not** run full validation yet
  - do **not** collect proof artifacts yet
- Reorganization strategy first:
  - many code/tests already exist but may be spread across older locations
  - first move files into the right crate/domain, rewire imports/exports, and reuse existing code/tests where possible
  - only write new code/tests when the required behavior does not already exist in reusable form
- When starting work on a specific plan:
  - read that plan folder once before coding
  - at minimum read the local `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and `WORKPACK_INDEX.md`
  - then read the detailed workpack(s) you are about to implement
  - also read the owning `docs/features/*.md` and relevant `docs/expectations/*.md` linked by that plan so the feature intent and final expected product behavior are both explicit
  - use that one-time read to understand where the plan is now, where it needs to end, and what code + tests need to be written so we do not drift
- Architectural rule for this round:
  - keep the runtime/event flow **event-driven**
  - keep business/runtime logic in Rust crates
  - keep TS focused on UI, contracts, read models, and schema boundaries
  - avoid duplicate truth, duplicate shapes, and same-schema-different-name drift
- True integration/e2e tests must wait until their dependent plans are actually ready. When that happens, note the dependency explicitly in the plan section instead of churning on fake green tests.
- After each meaningful coding chunk:
  - update `Previous Write Chunk`
  - update the affected plan sections in this file
  - then update `Next Write Order`

## Legend

- `plan status`: my current repo-state read for that plan on this branch.
- `status`: the workpack/index status currently written in the plan docs.
- `code`:
  - `done` = there is already an obvious dedicated implementation path for this row and the plan index marks it checked.
  - `in-progress` = active branch-local work exists now.
  - `partial` = owner crates/domains exist and related code exists, but this workpack is still open/planned.
  - `missing` = I do not see a dedicated implementation path yet.
- `test`:
  - `covered` = checked/indexed row with dedicated test ownership already present.
  - `in-progress` = active branch-local test-writing exists now.
  - `partial` = related tests exist at crate/domain level, but this workpack is not closed.
  - `missing` = I do not see dedicated test ownership yet.

Grouped rows are used only where the same owner and the same current repo state apply across that workpack slice.

## Shared/Common Ownership Map

### Shared Rust crates

| crate                      | role                                                               |
| -------------------------- | ------------------------------------------------------------------ |
| `ocentra-eventing`         | named events, publish/subscribe flow, awaitable event coordination |
| `agent-protocol`           | shared Rust wire/protocol contracts                                |
| `agent-service`            | service/runtime boundary glue around protocol and eventing         |
| `ocentra-evidence`         | shared evidence identity and evidence persistence primitives       |
| `ocentra-network-evidence` | shared network evidence capture/query primitives                   |

### Shared TS domains

| domain                  | role                                                   |
| ----------------------- | ------------------------------------------------------ |
| `schema-domain`         | branded/effect-schema primitives and decode boundaries |
| `event-domain`          | named event identifiers and TS event contracts         |
| `agent-protocol-domain` | TS protocol shapes mirroring Rust protocol ownership   |
| `evidence-domain`       | evidence refs, metadata, and read-model shapes         |
| `capability-domain`     | cross-plan capability/status vocabulary                |
| `endpoint-domain`       | local/remote endpoint contract vocabulary              |
| `family-domain`         | household, device, and authority primitives            |
| `parent-domain`         | parent-facing read-model/control-surface contracts     |
| `portal-domain`         | portal-only view-model and surface contracts           |

## Plan Directory

This file currently covers these **19** plans:

1. Tracking Plan
2. Screen Plan
3. Browser Plan
4. Eventing Plan
5. Setup Install Provisioning Plan
6. Account Identity Family Plan
7. Data Custody Storage Plan
8. LAN Plan
9. Network Plan
10. Payment Subscription Plan
11. Policy Control Plane Plan
12. Portal UX Household Surfaces Plan
13. Remote Access Plan
14. Screen AI Pipeline Plan
15. App + Game Plan
16. App Plan
17. Parent Desktop Runtime Package Plan
18. AI Plan
19. V0.8 Enforcement Control Plan

## Per-Plan Matrices

### Tracking Plan

- plan status: **in-progress**
- primary Rust crates: `tracking-core`, `child-ai-core`, `storage-custody-core`, `child-notification-core`
- primary TS domains/apps: `tracking-domain`, `event-domain`, `evidence-domain`, `setup-domain`, `apps/portal`
- read if working this plan: [AGENTS](tracking-plan/AGENTS.md), [PLAN_STATE](tracking-plan/PLAN_STATE.md), [NEXT_ACTIONS](tracking-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](tracking-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `tracking-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                             | status  | code        | test        | location crate                             | location domain/app                               |
| ------------------------------------------------------- | ------- | ----------- | ----------- | ------------------------------------------ | ------------------------------------------------- |
| WP01-WP03 source/gap/contracts                          | open    | in-progress | in-progress | `tracking-core`                            | `tracking-domain`, `event-domain`                 |
| WP04-WP07 location/device/permission/custody models     | open    | partial     | partial     | `tracking-core`, `storage-custody-core`    | `tracking-domain`, `evidence-domain`              |
| WP08-WP13 platform adapters and desktop presence        | open    | partial     | partial     | `tracking-core`                            | `tracking-domain`, `capability-domain`            |
| WP14-WP18 geofence/schedule/ack/check-in logic          | open    | in-progress | in-progress | `tracking-core`                            | `tracking-domain`, `parent-domain`                |
| WP19-WP24 place providers and AI boundary               | open    | partial     | partial     | `tracking-core`, `child-ai-core`           | `tracking-domain`, `ai-domain`                    |
| WP25 policy compiler for tracking rules                 | checked | done        | covered     | `tracking-core`, `policy-control-core`     | `tracking-domain`, `policy-domain`                |
| WP26 alert severity and notification model              | open    | in-progress | in-progress | `tracking-core`, `child-notification-core` | `tracking-domain`, `notification-domain`          |
| WP27-WP29 escalation/live/missing-device                | checked | done        | covered     | `tracking-core`, `child-notification-core` | `tracking-domain`, `notification-domain`          |
| WP30 parent and child UI/UX surfaces                    | open    | partial     | partial     | `tracking-core`                            | `tracking-domain`, `portal-domain`, `apps/portal` |
| WP31 platform extension checklists and proof routing    | open    | partial     | partial     | `tracking-core`                            | `tracking-domain`                                 |
| WP32 journal/sqlite/read-model proof                    | open    | partial     | partial     | `tracking-core`, `storage-custody-core`    | `tracking-domain`, `evidence-domain`              |
| WP33 proof gates/fixtures/PR gate                       | checked | done        | covered     | `tracking-core`                            | `tracking-domain`                                 |
| capability guide / schema proposal / settings inventory | open    | partial     | partial     | `tracking-core`                            | `tracking-domain`                                 |

**Goal**

- Finish tracking runtime end-to-end: evidence intake, status/geofence/expected-place reasoning, AI boundary handoff, policy evidence handoff, notification/escalation handoff, and parent/child read models.

**Code Written**

- `tracking-core` exists and already owns meaningful runtime slices.
- Policy compiler, escalation engine, temporary live tracking mode, and missing-device mode are already in the checked bucket.
- `tracking-domain` now enforces the WP03 temporary-live companion-setting contract in the control-catalog policy/effective-policy path instead of leaving that invalid state implicit.
- `tracking-domain` now mirrors the Rust expected-place branch order for WP16 in the TypeScript runtime helper, including `missed-arrival -> late-arrival`, manual-review degradation, and grace-window suppression.
- `tracking-domain` expected-place schedule/decision contracts now carry an explicit active exception state and audit ref, and the runtime suppresses holiday-mode/trip-exception schedules before alert-ready outcomes instead of leaving exception handling to proof-only policy harness rows.
- `tracking-domain` expected-place schedule/decision contracts now also preserve explicit `ruleId`, `distanceToleranceMeters`, `lateGraceSeconds`, and `earlyExitGraceSeconds` citations on the decision payload, so the TS owner seam no longer drops schedule-rule/tolerance metadata before parent/read-model consumers see the WP16 outcome.
- `tracking-core` geofence transition evaluation now has direct crate-owned coverage for the explicit grace-period suppression path plus the default geofence rule/evidence citation path instead of leaving those WP15 guarantees implicit behind the broader ordering tests.
- `tracking-core` now mirrors that WP16 holiday/trip exception suppression in the Rust owner crate with dedicated reason codes instead of leaving schedule-engine exception handling unowned outside the TS helper and higher-level proof fixtures.
- `tracking-core` and `agent-protocol` now also keep explicit WP16 expected-place event citations on the Rust side: the evaluated event preserves `expectedPlaceRef`, `distanceToleranceMeters`, `lateGraceSeconds`, `earlyExitGraceSeconds`, and `exceptionState` instead of collapsing the runtime contract down to schedule id plus reasons/evidence only.
- `agent-protocol` now owns a typed parent-requested `tracking.child-check-in.requested` request/receipt contract with explicit request state, delivery state, timeout, source/target handler constants, and duplicate/stale/unsupported reason codes.
- `child-runtime` now subscribes to parent-requested child check-ins and records the request metadata, receipt state, and request completion outcome instead of leaving `request-child-check-in` as a proof-only parent action.
- `parent-runtime-core` now owns a concrete parent-requested child check-in event flow that turns a dispatch decision into a live child-runtime request/receipt path instead of leaving the new request helper test-only.
- `agent-protocol` now owns a concrete `tracking.alert.evaluated` runtime contract with typed parent-notification states plus Rust-side `info/watch/warning/urgent/critical` tracking alert severity constants instead of leaving the alert-evaluated event type as a string-only placeholder.
- `tracking-core` alert evaluation now separates duplicate and missing-evidence suppression states from the alert severity ladder, preserves severity during duplicate suppression, downgrades evidence-free evaluations to info-only, and maps review/warning/urgent/critical policy severities into the tracking alert model.
- `tracking-core` tracking read-model selection now includes `activity.tracking.alert.evaluated` and `activity.tracking.parent-notification.requested` rows, so WP26 alert/notification state reaches the service-backed tracking projection instead of stopping at proof-only contracts.
- `portal-domain` notification parent-surface hosted UI proof now consumes a structured tracking notification history read-model shape instead of a token-only row table, so WP30 portal rendering derives policy/evidence/provider/receipt/manual-proof refs from the same source model that owns the notification parent-surface boundary.
- `portal-domain` tracking status-panel consumers now prefer `latestActive*` summary metadata, keep tombstone coverage on the service-data card, consume additive active kind/device/capability count buckets when present, and stop reintroducing deleted-history evidence refs into the narrow live summary.
- Retention/custody and AI boundary ownership exist but are still partial.

**Test List Done**

- Existing coverage is already present around the checked slices: compiler, escalation, temporary live mode, and missing-device handling.
- `packages/tracking-domain/tests/contract/tracking-control-catalog.test.ts` now covers the repaired plan-owned doc route plus temporary-live companion-setting rejection/acceptance.
- `packages/tracking-domain/tests/unit/tracking.test.ts` now covers the WP16 parity cases for late arrival, manual-required degradation, and late/early grace suppression.
- `packages/tracking-domain/tests/unit/tracking.test.ts` now also covers schedule-engine holiday-mode and trip-exception suppression, including surfaced exception state/audit refs on the expected-place decision contract.
- `packages/tracking-domain/tests/unit/tracking.test.ts` now also covers expected-place decision rule/tolerance/grace citations, the low-accuracy ambiguous path that avoids late/exit accusation, and a DST-spanning absolute-window case in the package-owned WP16 unit bucket.
- `packages/tracking-domain/tests/unit/tracking.test.ts` now also covers distinct school, activity, and calendar-backed expected-place rule cases plus a direct expected-place tolerance-boundary case at the geofence edge.
- `crates/tracking-core/tests/unit/geofence.rs` now covers WP15 grace-period ambiguity suppression plus the default geofence rule/evidence citation path for transition events.
- `crates/tracking-core/tests/contract/runtime_events.rs` now serializes the WP15 geofence transition event with explicit `geofenceRuleRef` and `evidenceRefs` assertions instead of only checking the contract event type.
- `crates/tracking-core/tests/unit/expected_place.rs` now covers WP16 missed-arrival -> late-arrival, late-grace suppression, early-exit grace suppression, and schedule-disabled manual-required outcomes in the Rust owner crate instead of leaving those branches covered only in TypeScript.
- `crates/tracking-core/tests/unit/expected_place.rs` now also covers holiday-mode and trip-exception suppression in the Rust owner crate so the schedule engine does not accuse during explicit parent exceptions.
- `crates/tracking-core/tests/unit/expected_place.rs` now also covers distinct school, activity, and calendar-backed expected-place cases so the broader WP16 schedule matrix is explicit in the Rust owner bucket.
- `crates/tracking-core/tests/contract/runtime_events.rs` now also asserts the expected-place event contract carries the schedule id, expected-place ref, distance tolerance, grace seconds, null exception state, evidence refs, and required parent-action state for a late-arrival decision.
- `crates/agent-protocol/tests/contract/tracking_expected_place_state_evaluated_event.rs` now covers the expected-place evaluated event contract type/idempotency plus serialized grace/tolerance/exception citations in the protocol owner crate.
- `crates/agent-protocol/tests/contract/tracking_retention_settings_write_command.rs` now covers child check-in request/receipt contract serialization, request-id ownership, and duplicate-reason receipt wiring.
- `crates/agent-protocol/tests/contract/tracking_alert_evaluated_event.rs` now covers the new alert-evaluated contract event type, idempotency key, and suppressed-missing-evidence serialization state.
- `crates/parent-runtime-core/tests/unit/runtime_dispatch.rs` now covers awaited, fire-and-forget, and duplicate parent-requested child check-in dispatch behavior.
- `crates/parent-runtime-core/tests/unit/tracking_child_check_in_request_flow.rs` now covers the real parent-runtime flow surface for awaited, fire-and-forget, and blocked child-check-in dispatch.
- `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs` now covers duplicate, stale, and unsupported parent-requested child check-in delivery receipts.
- `crates/tracking-core/tests/unit/alerting.rs` is now present for the WP26 Rust slice and covers review -> watch mapping, warning/urgent/critical severity mapping, duplicate suppression, and missing-evidence info suppression.
- `crates/tracking-core/tests/observability/alert_decision.rs` now expects duplicate rate-limiting to suppress parent notification state without flattening the underlying alert severity.
- `crates/tracking-core/tests/unit/read_model.rs` now covers WP26 service-backed read-model inclusion for `activity.tracking.alert.evaluated` and `activity.tracking.parent-notification.requested`, plus the negative case that non-tracking activity rows stay out of the tracking projection.
- `crates/agent-protocol/tests/contract/tracking_read_model.rs` now serializes the new WP26 tracking alert and parent-notification activity kinds through the tracking read-model boundary instead of leaving those row kinds unexercised at the protocol layer.
- `crates/agent-service/src/tracking_read_model_service_tests.rs` now seeds alert-evaluated and parent-notification-requested activity rows and expects them in the service-backed tracking read model counts/latest-active event surface.
- `packages/portal-domain/tests/unit/tracking-notification-parent-surface-hosted-ui-proof.test.ts` now covers the WP30 portal-domain consumer for schema-backed notification parent-surface rows, including the manual-action quiet-hours branch and the invalid-input empty fallback.
- `apps/portal/tests/tracking-status-panel.test.ts` now asserts the WP32 portal consumer prefers active summary metadata over the tombstone-latest surface, keeps tombstone citation rows separate from active summary evidence refs, consumes active kind/capability counts when present, and falls back to legacy row-derived behavior when the additive fields are absent.
- `packages/portal-domain/tests/unit/tracking-status-panel.test.ts` now gives the shared WP32 status-panel owner a direct package-local unit test for active summary metadata, active device/kind/capability counts, and the legacy additive-field-absent fallback without relying on the blocked `apps/portal` workspace suite.

**Test List Required**

- Contract/model tests for WP03-WP07.
- Platform adapter tests for WP08-WP13.
- Remaining engine tests for the non-WP16 WP14-WP18 seams.
- Provider/AI-boundary tests for WP19-WP24.
- Notification/read-model/UI tests for WP26/WP30/WP32.
- Cross-plan integration/e2e only after AI, policy, notification, and custody paths are ready.

**Reason / Blocker / Deferred**

- True end-to-end tests are deferred until AI, policy-control, notification, and storage-custody ownership are less partial.
- `cmd /c npm run lint:architecture -- crates/tracking-core/src/geofence.rs crates/tracking-core/tests/unit/geofence.rs crates/tracking-core/tests/contract/runtime_events.rs` passed for the touched WP15 files.
- `cmd /c npm run lint:architecture -- crates/tracking-core/tests/unit/expected_place.rs crates/tracking-core/tests/contract/runtime_events.rs` passed for the touched WP16 files.
- `cmd /c npx vitest run tests/unit/tracking.test.ts` passed in `packages/tracking-domain` for the focused WP16 decision-citation, low-accuracy ambiguous, DST-window, and earlier holiday/trip exception suppression slice.
- `cargo test -p ocentra-parent-agent-protocol --test contract expected_place_event_uses_tracking_contract_and_idempotency -- --test-threads=1` passed for the new WP16 protocol contract test.
- `cargo test -p ocentra-parent-agent-protocol --test contract expected_place_event_serializes_grace_tolerance_and_exception_citations -- --test-threads=1` passed for the new WP16 protocol serialization slice.
- `cmd /c npm run lint:architecture -- ...` passed for the touched WP26 Rust files, and `cargo test -p ocentra-parent-agent-protocol --test contract tracking_alert_evaluated_event` passed.
- Focused WP26 read-model work now adds real code/test ownership in `agent-protocol`, `tracking-core`, and the service-backed tracking read-model test surface instead of leaving alert/notification rows outside the live tracking projection.
- Focused WP30 portal-domain consumer work now adds real package-owned code/test coverage for the hosted notification parent-surface mapping; `cmd /c npx vitest run tests/unit/tracking-notification-parent-surface-hosted-ui-proof.test.ts` passed in `packages/portal-domain`.
- `cmd /c npm run lint:architecture -- packages/portal-domain/src/tracking-status-panel.ts apps/portal/tests/tracking-status-panel.test.ts` passed for the focused WP32 portal active-summary consumer slice.
- `cmd /c npx vitest run tests/unit/tracking-status-panel.test.ts` passed in `packages/portal-domain` for the focused WP32 active summary/device count consumer slice.
- Focused `apps/portal` confirmation is still blocked in this checkout by the same pre-existing workspace package-resolution issue that breaks `tests/tracking-status-panel.test.ts` before it reaches the tracking assertions (`@ocentra-parent/social-domain/social-alert-report-intent` missing through `agent-protocol-domain` dist imports).
- The earlier `policy-control-core` unresolved-import blocker is cleared in this checkout: `cargo test -p ocentra-tracking-core --test unit -- --test-threads=1`, `cargo test -p ocentra-tracking-core --test unit geofence_ -- --test-threads=1`, `cargo test -p ocentra-tracking-core --test contract tracking_expected_place_event_carries_schedule_evidence_and_parent_action -- --test-threads=1`, and `cargo test -p ocentra-tracking-core --test unit expected_place_ -- --test-threads=1` now all execute and pass for the focused Rust owner seams.

### Screen Plan

- plan status: **in-progress**
- primary Rust crates: `screen-core`, `screen-capture-adapter`, `screen-live-view-core`
- primary TS domains/apps: `screen-domain`, `evidence-domain`, `capability-domain`, `apps/portal`
- read if working this plan: [AGENTS](screen-plan/AGENTS.md), [PLAN_STATE](screen-plan/PLAN_STATE.md), [NEXT_ACTIONS](screen-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](screen-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `screen-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                            | status  | code        | test        | location crate                                           | location domain/app                                 |
| ---------------------------------------------------------------------- | ------- | ----------- | ----------- | -------------------------------------------------------- | --------------------------------------------------- |
| WP01-WP03 source/gap/contracts                                         | open    | in-progress | in-progress | `screen-core`                                            | `screen-domain`, `event-domain`                     |
| WP04 parent opt-in settings contract                                   | checked | done        | covered     | `screen-core`                                            | `screen-domain`, `parent-domain`                    |
| WP05-WP09 capability/capture/platform abstraction/windows proof        | open    | partial     | partial     | `screen-core`, `screen-capture-adapter`                  | `screen-domain`, `capability-domain`                |
| WP10-WP11 macOS/Linux capture proof                                    | checked | done        | covered     | `screen-capture-adapter`                                 | `screen-domain`                                     |
| WP12-WP16 Android/iOS/protected-surface/queue internals                | open    | partial     | partial     | `screen-capture-adapter`, `screen-core`                  | `screen-domain`                                     |
| WP17 local OCR vision runtime model                                    | checked | done        | covered     | `screen-core`, `screen-ai-core`                          | `screen-domain`, `ai-domain`                        |
| WP18 screen analysis result schema                                     | open    | in-progress | in-progress | `screen-core`, `screen-ai-core`                          | `screen-domain`, `ai-domain`                        |
| WP19 sensitive text and redaction model                                | checked | done        | covered     | `screen-core`                                            | `screen-domain`, `evidence-domain`                  |
| WP20-WP23 validator/journal/retention/policy compiler                  | open    | partial     | partial     | `screen-core`, `ocentra-evidence`                        | `screen-domain`, `evidence-domain`, `policy-domain` |
| WP24 enforcement handoff guard                                         | checked | done        | covered     | `screen-core`, `child-enforcement-core`                  | `screen-domain`, `enforcement-domain`               |
| WP25-WP27 portal/disclosure/retention mode                             | checked | done        | covered     | `screen-core`                                            | `screen-domain`, `portal-domain`, `apps/portal`     |
| WP28-WP30 live view/proof packs/playwright gate                        | open    | partial     | partial     | `screen-live-view-core`, `screen-core`                   | `screen-domain`, `portal-domain`, `apps/portal`     |
| WP31-WP38 screen intelligence, browser capture path, OCR/VLM, queueing | checked | done        | covered     | `screen-core`, `screen-ai-core`, `screen-live-view-core` | `screen-domain`, `ai-domain`, `browser-domain`      |
| WP39 redacted-summary-only remote boundary                             | open    | partial     | partial     | `screen-live-view-core`, `remote-access-core`            | `screen-domain`, `remote-access-domain`             |
| WP40 detector prompt packs and schema tests                            | checked | done        | covered     | `screen-ai-core`                                         | `screen-domain`, `ai-domain`                        |
| control/settings/capability/schema auxiliary docs                      | open    | partial     | partial     | `screen-core`                                            | `screen-domain`                                     |

**Goal**

- Finish screen capture and derived evidence lifecycle without fake capture paths: capture scope/trigger, platform adapters, analysis result validation, custody/retention, policy handoff, and parent/child surfaces.

**Code Written**

- Opt-in settings, OCR/VLM routing, sensitive-text redaction, enforcement handoff, portal summary, retention mode, browser structured extraction, and several intelligence lanes already exist.
- `screen-domain` now owns a canonical policy-evidence and deletion-reason chain contract instead of leaving `activity-domain` to define those fields locally.
- `screen-evidence-result.ts` and `screen-evidence-read-model.ts` now surface that shared chain for screen result/read-model consumers, and `activity-domain` reuses the shared defaulted field contract inside the Activity Screen read model.

**Test List Done**

- Observed existing coverage for checked slices: opt-in, redaction, intelligence routing, portal/disclosure surfaces, retention mode, browser structured extraction, OCR/VLM evaluation lanes.
- `packages/screen-domain/tests/unit/screen-evidence.test.ts` now covers the shared policy-evidence chain defaults, malformed no-decision policy payload rejection, and recent-summary policy-ref validation.
- `packages/activity-domain/tests/unit/activity-surface.test.ts` now carries malformed screen policy chain rejection cases for Activity Screen rows, although executing that package test remains blocked in this checkout by a pre-existing unresolved `@ocentra-parent/app-game-domain/app-game-primitives` import path.

**Test List Required**

- Platform adapter tests for Windows/Android/iOS and capture abstraction.
- Queue/debouncer/encrypted-temporary-image tests.
- Result schema and invalid-output handling tests.
- Journal/retention/policy compiler tests.
- Live-view and remote-boundary tests.
- Real screen e2e only after capture/runtime/eventing integration is stable.

**Reason / Blocker / Deferred**

- Live proof and remote/e2e coverage depend on remote-access, eventing, and policy paths being more stable.

### Browser Plan

- plan status: **in-progress**
- primary Rust crates: `browser-core`
- primary TS domains/apps: `browser-domain`, `portal-domain`, `capability-domain`, `apps/portal`
- read if working this plan: [AGENTS](browser-plan/AGENTS.md), [PLAN_STATE](browser-plan/PLAN_STATE.md), [NEXT_ACTIONS](browser-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](browser-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `browser-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                                                                | status  | code    | test    | location crate | location domain/app                                                   |
| -------------------------------------------------------------------------------------------------------------------------- | ------- | ------- | ------- | -------------- | --------------------------------------------------------------------- |
| WP01-WP24 browser inventory/managed profile/CDP/evidence/policy/intervention/perf/rollout chain                            | checked | done    | covered | `browser-core` | `browser-domain`, `evidence-domain`, `portal-domain`, `policy-domain` |
| settings inventory / coverage matrix / schema proposal / questionnaire / settings catalog / managed-unmanaged browser refs | open    | partial | partial | `browser-core` | `browser-domain`, `portal-domain`                                     |

**Goal**

- Keep browser control as a fully owned, canonical slice: managed/unmanaged browser inventory, CDP capture/control, policy targeting, intervention, and parent surfaces.

**Code Written**

- `browser-core` is already one of the strongest runtime-complete areas in the repo.
- Main runtime/policy/intervention/read-model chain is already in the checked bucket.

**Test List Done**

- Existing checked-row coverage appears to already back the main browser runtime and rollout chain.
- `packages/browser-domain/tests/unit/browser-control-coverage-matrix.test.ts` now gives the auxiliary browser reference seam its own owner-bucket coverage: every candidate MVP item and major catalog-section summary appears exactly once, direct-control/capability/docs-only statuses stay honest, questionnaire source coverage still spans the full catalog, and managed-vs-unmanaged platform references stay aligned instead of depending on doc inspection alone.

**Test List Required**

- Remaining inventory/schema/settings-catalog consistency tests.
- Any missing parent-surface/e2e smoke that ties the reference docs back to the real runtime.

**Reason / Blocker / Deferred**

- Remaining work is more catalog/reference alignment than missing core runtime.

### Eventing Plan

- plan status: **in-progress**
- primary Rust crates: `ocentra-eventing`, `agent-protocol`, `agent-core`, `agent-service`
- primary TS domains/apps: `event-domain`, `agent-protocol-domain`, `endpoint-domain`
- read if working this plan: [AGENTS](eventing-plan/AGENTS.md), [PLAN_STATE](eventing-plan/PLAN_STATE.md), [NEXT_ACTIONS](eventing-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](eventing-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `eventing-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                           | status | code    | test    | location crate                                        | location domain/app                                        |
| ----------------------------------------------------- | ------ | ------- | ------- | ----------------------------------------------------- | ---------------------------------------------------------- |
| 01 source boundary and semantics audit                | open   | partial | partial | `ocentra-eventing`, `agent-protocol`                  | `event-domain`, `agent-protocol-domain`                    |
| 02 crate contract and type boundary                   | open   | done    | covered | `ocentra-eventing`, `agent-protocol`                  | `event-domain`, `agent-protocol-domain`                    |
| 03-06 runtime/queue/request-response/journal replay   | open   | done    | covered | `ocentra-eventing`, `agent-service`                   | `event-domain`, `endpoint-domain`                          |
| 07-08 protocol/runtime integration                       | open   | partial | partial | `ocentra-eventing`, `agent-protocol`, `agent-service` | `event-domain`, `agent-protocol-domain`, `endpoint-domain` |
| 09 network consumer event chain                          | ready  | drafted | deferred | `agent-protocol`, `agent-core`, `agent-service`       | `event-domain`, `agent-protocol-domain`                    |
| 10 LAN household mesh consumer                           | open   | partial | partial | `ocentra-eventing`, `agent-protocol`, `agent-service` | `event-domain`, `agent-protocol-domain`, `endpoint-domain` |
| 11 type safety and ownership hardening                | open   | done    | covered | `ocentra-eventing`, `agent-protocol`                  | `event-domain`, `schema-domain`                            |
| 12 rollout proof and PR gate                          | open   | partial | partial | `ocentra-eventing`                                    | `event-domain`                                             |

**Goal**

- Finalize canonical event bus, request/response, queue, and journal ownership so downstream plans stop inventing divergent event flow patterns.

**Code Written**

- `ocentra-eventing` is already a substantial reusable crate with real bus, queue, request/response, journaling, replay, contract registry, topology, delivery-route, compatibility, and testkit ownership rather than placeholder scaffolding.
- Workpack `02` now has a stronger public contract boundary: `SchemaVersion` fails closed on serde deserialize instead of only through manual constructors, and the strong wrapper IDs now reject whitespace while still accepting the dotted subscriber and target lineage already used by repo constants and crate fixtures.
- Workpack `11` now carries `expected_schema_version` and `received_schema_version` through stored-envelope decode contract mismatches, so downstream drift reports stay explicit instead of generic.
- Workpacks `03-06` already have concrete runtime/request/journal mechanics in the crate; the remaining open state is no longer “missing core code”, it is mostly consumer-handoff and proof reconciliation.
- WP09 is the single legal READY code packet for the missing production network foundation: agent-core capture must publish once at ingestion with deterministic identity/idempotency, agent-core/agent-service must own durable network journaling and startup replay before readiness, and read-time APIs must not republish. Nested fixture/prove/`TEST_*` runtime files are not shipped production behavior. AI, policy, enforcement, audit, and portal consumers remain downstream blocked/fail-closed until their owning plans provide real authority and handoffs. The current `OnceCell`/`EventBus::new` read-time spine does not establish this boundary.

**Test List Done**

- `ocentra-eventing` now has real crate tests across internal runtime behavior plus public bucketed tests in `tests/unit`, `tests/integration`, and `tests/version-skew`.
- Actual contract-boundary coverage now exists for event type grammar, schema-version validation, live-vs-stored envelope serialization, request/response behavior, queue/idempotency, journal/replay, lifecycle, topology, and compatibility semantics.
- New public tests now cover the WP02 gap that was still weak from the matrix view: `tests/unit/ids.rs`, `tests/unit/envelope.rs`, and `tests/version-skew/roundtrip.rs` prove strong wrapper acceptance/rejection, zero-version serde rejection, and version-skew fail-closed behavior.
- `crates/ocentra-eventing/tests/unit/ids.rs` now proves representative event, request, journal, subscriber, target, source-service/component, and runtime-instance wrappers accept real repo values while rejecting whitespace without inventing incompatible slug grammar.
- `crates/ocentra-eventing/tests/unit/envelope.rs`, `crates/ocentra-eventing/tests/version-skew/roundtrip.rs`, and `packages/event-domain/tests/unit/eventing.test.ts` now prove stored-envelope mismatch strings keep expected/received schema-version context and that the shared TS stored-header boundary preserves valid versions while rejecting `0`.
- `cargo test -p ocentra-eventing` and `npm test` in `packages/event-domain` pass, but the reusable-eventing proof packs still fail on the workspace clippy gate because `ocentra-eventing` uses denied `expect_used`, `clone_on_ref_ptr`, and `needless_pass_by_value` patterns in both library and test targets.
- WP09 production tests and retained proof are deliberately deferred to its later validation/proof phase; the existing contract/test references do not claim a shipped network journal or startup-recovery path.

**Test List Required**

- Consumer integration tests for parent, LAN, network, and other downstream adopters.
- Remaining proof-pack and checklist reconciliation for source-boundary, hardening, and rollout rows that are still open in the plan docs.

**Reason / Blocker / Deferred**

- Remaining eventing open state is now mostly proof/doc/consumer-boundary reconciliation, not absence of core event-bus code or crate tests.
- WP09 remains open after the READY routing decision: no completion, live-capture, enforcement, Network WP04 unblock, review, or merge claim is made.
- Deep e2e and product claims should still wait for the downstream consumer plans to finish their own protocol/runtime proofs.

### Setup Install Provisioning Plan

- plan status: **in-progress**
- primary Rust crates: `provisioning-core`, `child-runtime`, `family-identity-core`, `parent-runtime-core`
- primary TS domains/apps: `setup-domain`, `family-domain`, `parent-domain`, `production-domain`
- read if working this plan: [AGENTS](setup-install-provisioning-plan/AGENTS.md), [PLAN_STATE](setup-install-provisioning-plan/PLAN_STATE.md), [NEXT_ACTIONS](setup-install-provisioning-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](setup-install-provisioning-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `setup-install-provisioning-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                         | status  | code        | test        | location crate                                               | location domain/app                              |
| ----------------------------------- | ------- | ----------- | ----------- | ------------------------------------------------------------ | ------------------------------------------------ |
| 01 family web info site             | Planned | in-progress | in-progress | `no Rust owner yet`                                          | `production-domain`                              |
| 02 registration/login entry         | Planned | in-progress | in-progress | `family-identity-core`                                       | `family-domain`, `parent-domain`, `setup-domain` |
| 03 parent install journey           | Planned | partial     | partial     | `parent-runtime-core`, `agent-updater`                       | `production-domain`, `parent-domain`             |
| 04 child install/permission journey | Planned | in-progress | in-progress | `child-runtime`, `provisioning-core`                         | `setup-domain`, `capability-domain`              |
| 05 pairing/readiness/recovery       | Planned | in-progress | in-progress | `provisioning-core`, `child-runtime`, `family-identity-core` | `setup-domain`, `family-domain`, `parent-domain` |
| 06 rollout proof and route gate     | Planned | partial     | partial     | `provisioning-core`, `parent-runtime-core`                   | `setup-domain`, `production-domain`              |

**Goal**

- Own the first-run family journey end-to-end: information site, auth entry, parent install, child install/permissions, pairing, readiness, degraded recovery, and rollout gate.

**Code Written**

- Family authority/setup lifecycle groundwork already exists.
- `production-domain` now has a first source-contract slice for the public family web surface: required route/link map, four-mode data-collection matrix, `family.ocentra.ca` deployment shape, privacy/support/status boundaries, and registration/login handoff metadata.
- `setup-domain` now owns a dedicated registration/login entry contract covering the six auth entry routes, the six-state setup matrix, website-to-identity handoff fields, and degraded/rejected invite/recovery/cross-family branches instead of leaving that slice implied by broader setup primitives.
- `setup-domain` now also separates canonical child install state from child service state and derives a coarse public child-install journey contract (`InstallRequired`, `Installed`, `Permissioned`, `Paired`) while keeping service start, trust, and policy-readiness detail in the readiness report/checklist instead of promoting them to top-level public journey stages.
- Pairing/readiness/recovery now has concrete branch-local contract and runtime work in `setup-domain`, `provisioning-core`, and `child-runtime`.
- `setup-domain` now owns a richer pairing lifecycle contract, typed readiness report/checklist contract, and typed recovery operation contract for stale-code, replay, wrong-household, revoked, offline, and manual-recovery states.
- `setup-domain` now also consumes the concrete `family-domain` invite, household-authority, and recovery-operation decisions through `src/family-setup-bridge.ts` instead of leaving setup readiness as an isolated contract-only shape.
- `provisioning-core` now also consumes the concrete `family-identity-core` invite, session-token, household-authority, and recovery-operation decisions through a family-context bridge that projects canonical provisioning readiness/action inputs instead of relying only on hand-filled raw enums.
- `provisioning-core` now owns explicit readiness blocker, degraded-vs-blocked overall state, and recovery-action mapping across account, parent app, child app, permissions, pairing, policy baseline, custody sync, and network reachability.
- `child-runtime` preflight coverage now consumes the richer provisioning readiness shape instead of the earlier minimal parent-device/pairing gate.
- `packages/child-runtime-domain/src/child-runtime-gates.ts` now mirrors that setup owner seam on the TypeScript side: the public preflight contract keeps top-level `provisioningReadiness` for compatibility but also carries a nested `provisioningDecision` with canonical child install state, child service state, overall readiness state, and explicit blocker reasons for not-installed, not-started, offline, and reinstall-required cases.

**Test List Done**

- Family authority/setup lifecycle tests already exist.
- `packages/production-domain/tests/unit/family-web-route-map.test.ts` now covers required public pages, link targets, no-child-activity collection, privacy no-overclaim copy, and registration handoff drift.
- `packages/setup-domain/tests/unit/registration-entry.test.ts` now covers the six auth entry routes, the six-state setup matrix, expired/revoked invite rejection, cross-family rejection, and the no-child-data-before-household boundary.
- Actual unit coverage now exists in `packages/setup-domain/tests/unit/pairing-intent.test.ts` and `packages/setup-domain/tests/unit/readiness.test.ts`, including explicit child install state, child service state, coarse journey-stage derivation, degraded child state, and readiness-checklist coverage.
- Actual setup-domain consumer-wiring coverage now exists in `packages/setup-domain/tests/unit/family-setup-bridge.test.ts` for trusted, replayed, wrong-account, wrong-household, offline-child, support-assisted recovery-blocked, and canonical `Paired` journey-stage mapping states.
- `crates/provisioning-core/tests/unit/readiness_flow.rs` now covers family-context projection for trusted pairing, replay rejection, wrong-household pairing, support-assisted custody-handoff recovery blocking, installed-but-not-started child service, offline-child degradation, and reinstall-required recovery through the Rust bridge.
- `crates/provisioning-core/tests/unit/readiness.rs` and `crates/provisioning-core/tests/unit/readiness_flow.rs` now cover replay rejection, wrong-household rejection, installed-but-not-started recovery, offline/degraded child state, reinstall-required recovery, permission regrant, policy baseline staleness, direct-entry recovery, custody sync degradation, and recovered pairing.
- `crates/child-runtime/tests/unit/runtime_gate.rs` now covers the richer provisioning shape with an explicit installed-but-not-started blocker that stays distinct from the offline-child degraded/manual-review path.
- `packages/child-runtime-domain/tests/unit/child-runtime-gates.test.ts` now proves the TypeScript preflight seam accepts coherent ready, installed-not-started, offline/degraded, and reinstall-required shapes while rejecting mismatches between top-level `provisioningReadiness`, nested `provisioningDecision`, and allowed runtime start.

**Test List Required**

- Additional registration/login integration and consumer-handoff tests.
- Parent install/update journey tests.
- Remaining child install/permission consumer and integration tests that carry the canonical install/service/readiness fields through capability, runtime-startup, and rollout consumers.
- Remaining pairing/recovery integration tests plus any downstream consumers outside the landed setup/provisioning/runtime seams.
- Route/rollout gate tests after the flow is complete.

**Reason / Blocker / Deferred**

- Depends on account-identity, LAN pairing, and parent runtime package ownership becoming less partial.

### Account Identity Family Plan

- plan status: **in-progress**
- primary Rust crates: `family-identity-core`, `provisioning-core`
- primary TS domains/apps: `family-domain`, `setup-domain`, `parent-domain`
- read if working this plan: [AGENTS](account-identity-family-plan/AGENTS.md), [PLAN_STATE](account-identity-family-plan/PLAN_STATE.md), [NEXT_ACTIONS](account-identity-family-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](account-identity-family-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `account-identity-family-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                      | status  | code        | test        | location crate                              | location domain/app              |
| -------------------------------- | ------- | ----------- | ----------- | ------------------------------------------- | -------------------------------- |
| 01 auth provider decision        | Planned | partial     | partial     | `family-identity-core`                      | `family-domain`, `parent-domain` |
| 02 identity/household role model | Planned | in-progress | in-progress | `family-identity-core`                      | `family-domain`                  |
| 03 session/token lifecycle       | Planned | in-progress | in-progress | `family-identity-core`                      | `family-domain`, `parent-domain` |
| 04 invites/recovery lifecycle    | Planned | in-progress | in-progress | `family-identity-core`, `provisioning-core` | `family-domain`, `setup-domain`  |
| 05 device ownership/authz        | Planned | in-progress | in-progress | `family-identity-core`, `provisioning-core` | `family-domain`, `setup-domain`  |
| 06 security proof and route gate | Planned | partial     | partial     | `family-identity-core`                      | `family-domain`                  |

**Goal**

- Make household identity, roles, session lifecycle, recovery, and device ownership explicit and canonical for the rest of the repo.

**Code Written**

- `family-identity-core` and `family-domain` already exist, and some household/setup primitives are already written.
- `family-domain` now has a first-class `ChildProfile` contract and direct export instead of only a child-profile reference shape.
- `family-domain` now has explicit same-family active-member authorization guards for device actions instead of relying only on role-level checks.
- `family-domain` now has an explicit trusted-child-agent-to-child-profile binding check so a child profile is not treated as interchangeable with a child device.
- `family-identity-core` now has a canonical `session_lifecycle` ownership slice for browser sessions, device credentials, invite/pairing/recovery tokens, remote session grants, replay rejection, logout/global revoke, and clock-skew handling.
- `family-domain` now has a richer setup lifecycle contract for recovery membership state, identity-proof state, support channel, delete/export custody handoff, single-purpose invite checks, and typed helper decisions for custody routing and child-evidence access.
- `family-identity-core` now has a concrete invite/recovery decision surface for single-use invite enforcement, invite replay rejection, owner-approved lost-device/household-transfer recovery, support-assisted audited recovery, child-evidence access blocking on support paths, and explicit data-custody handoff states.
- `family-domain` now has a typed household action authority matrix that mirrors the Rust device-authz boundary across family match, membership state, account state, child-profile binding, device scope, device trust, session freshness, capability grant, audit requirement, and elevated confirmation.
- `family-identity-core` household authority coverage now explicitly rejects child-device-agent attempts to use parent-controller authority for remote view or policy change.
- `family-domain` household authority now also requires an explicit active controller lease for remote-sensitive parent-controller actions instead of treating remote view/control as authorized once the base matrix passes.
- `family-identity-core` household authority now mirrors that controller-lease gate in Rust, so remote-sensitive parent-controller actions reject missing, expired, and revoked leases before authorization succeeds.

**Test List Done**

- Observed family authority/setup lifecycle tests exist from the earlier branch checkpoint.
- `packages/family-domain/tests/unit/child-profile.test.ts` now covers first-class child-profile parsing, child-profile-to-device matching, and setup-domain staying on child-profile references instead of inventing a second model.
- `packages/family-domain/tests/unit/household-authority.test.ts` now covers cross-family denial, revoked-member denial, support-admin default deny, and trusted child-agent binding requirements inside the existing `tests/unit` bucket.
- `packages/family-domain/tests/unit/household-authority.test.ts` now also covers the typed device-authz matrix for billing, remote capability grant, stale session rejection, revoked-device denial, wrong-family denial, wrong-device-scope denial, and child-device-agent parent-authority rejection.
- `packages/family-domain/tests/unit/household-authority.test.ts` now also covers active, missing, expired, and revoked controller-lease outcomes for remote-sensitive actions in the existing `tests/unit` bucket.
- `packages/family-domain/tests/unit/setup-lifecycle.test.ts` now covers single-purpose invite checks, lost-parent-device owner approval, delete/export custody handoff routing, support-assisted audited recovery, and child-evidence access gating inside the existing `tests/unit` bucket.
- `crates/family-identity-core/tests/unit/session_lifecycle.rs` now covers expiry boundary, revoked refresh, logout invalidation, replay rejection, clock-skew rejection, device-token-vs-user-session separation, and stale remote-session grant rejection.
- `crates/family-identity-core/tests/unit/setup_lifecycle.rs` now covers invite replay and single-use rejection, wrong-role rejection, lost-device owner approval, delete/export custody handoff, support-assisted audited recovery, and household-transfer identity-proof rejection inside the existing `tests/unit` bucket.
- `crates/family-identity-core/tests/unit/household_authority.rs` now explicitly covers child-device-agent rejection on parent-controller-only action families in addition to billing, remote capability, stale session, revoked device, and wrong-scope denial.
- `crates/family-identity-core/tests/unit/household_authority.rs` now also covers active, missing, expired, and revoked controller-lease outcomes for remote-sensitive actions in the existing Rust `tests/unit` bucket.
- `cmd /c npm run type-check` and `cmd /c npm test` now pass in `packages/family-domain` with 19 tests across household authority and setup lifecycle.
- `cargo test -p ocentra-family-identity-core household_authority -- --test-threads=1`, `cargo test -p ocentra-family-identity-core setup_lifecycle -- --test-threads=1`, and `cargo test -p ocentra-family-identity-core -- --test-threads=1` now pass with 34 tests across device scope, household authority, session lifecycle, and setup lifecycle.

**Test List Required**

- Role/household model tests.
- Session/token lifecycle tests.
- Remaining broader device-ownership/authz tests beyond the controller-lease parity slice.
- Security/route-gate tests after the core ownership pass is finished.

**Reason / Blocker / Deferred**

- Foundational plan; should be closed before leaning harder on setup, LAN, portal, and policy.

### Data Custody Storage Plan

- plan status: **in-progress**
- primary Rust crates: `storage-custody-core`, `ocentra-evidence`, `ocentra-network-evidence`
- primary TS domains/apps: `data-custody-domain`, `evidence-domain`
- read if working this plan: [AGENTS](data-custody-storage-plan/AGENTS.md), [PLAN_STATE](data-custody-storage-plan/PLAN_STATE.md), [NEXT_ACTIONS](data-custody-storage-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](data-custody-storage-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `data-custody-storage-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                     | status  | code        | test        | location crate                                                         | location domain/app                      |
| ------------------------------------------------------------------------------- | ------- | ----------- | ----------- | ---------------------------------------------------------------------- | ---------------------------------------- |
| 01-06 source-of-truth / key custody / sync / retention / export / query custody | Planned | in-progress | in-progress | `storage-custody-core`, `ocentra-evidence`, `ocentra-network-evidence` | `data-custody-domain`, `evidence-domain` |
| 07 rollout proof and route gate                                                 | Planned | partial     | partial     | `storage-custody-core`                                                 | `data-custody-domain`                    |
| data and AI UI plan                                                             | unknown | partial     | partial     | `storage-custody-core`                                                 | `data-custody-domain`, `portal-domain`   |

**Goal**

- Canonicalize storage, encryption, sync, retention, export/import, and custody-query ownership so evidence never drifts into ad hoc storage paths.

**Code Written**

- `storage-custody-core`, `ocentra-evidence`, and `ocentra-network-evidence` already exist and are used by downstream slices.
- `data-custody-domain` now owns the first canonical WP01 source-of-truth slice: eight seeded custody class ids, explicit source-of-truth shapes, default storage locations, custody authority, hosting policy, and a canonical matrix for journal/query/rule/approval/device-registry/notification/audit/generated-summary rows.

**Test List Done**

- Partial downstream tests already touch custody concerns, but dedicated plan-owned coverage is still incomplete.
- `packages/data-custody-domain/tests/unit/custody-boundary.test.ts` now covers canonical and ambiguous source-of-truth parsing.
- `packages/data-custody-domain/tests/unit/data-custody-matrix.test.ts` now covers seeded-row completeness, missing-class and empty-id negatives, forbidden hosted defaults, and no-raw-evidence defaults.

**Test List Required**

- Source-of-truth tests.
- Key custody/encryption tests.
- Sync/export/import tests.
- Retention/tombstone tests.
- Query/report custody tests.
- UI-facing custody boundary tests where applicable.

**Reason / Blocker / Deferred**

- Downstream plans consume custody already; this plan still needs its own direct Rust/storage/sync/retention closure, and package-level `data-custody-domain` test/lint execution still needs its own package-specific follow-up in this checkout; the old repo-root `tsconfig.base.json` blocker is no longer the cause.

### LAN Plan

- plan status: **in-progress**
- primary Rust crates: `lan-core`, `provisioning-core`, `remote-access-core`
- primary TS domains/apps: `lan-domain`, `setup-domain`, `endpoint-domain`, `apps/portal`
- read if working this plan: [AGENTS](lan-plan/AGENTS.md), [PLAN_STATE](lan-plan/PLAN_STATE.md), [NEXT_ACTIONS](lan-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](lan-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `lan-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                             | status | code    | test    | location crate                  | location domain/app                                          |
| ----------------------------------------------------------------------- | ------ | ------- | ------- | ------------------------------- | ------------------------------------------------------------ |
| 01-02 contracts/evidence/device record                                  | open   | partial | partial | `lan-core`                      | `lan-domain`, `event-domain`                                 |
| 03-12 discovery/detection/vendor/classification inputs                  | open   | partial | partial | `lan-core`                      | `lan-domain`, `endpoint-domain`                              |
| 13-20 merge/classify/store/events/mesh/revocation/rollout               | open   | partial | partial | `lan-core`, `provisioning-core` | `lan-domain`, `setup-domain`, `parent-domain`                |
| 21-25 second-pass contracts/gap-map/pairing/portal handoff/rollout gate | open   | partial | partial | `lan-core`, `provisioning-core` | `lan-domain`, `setup-domain`, `portal-domain`, `apps/portal` |

**Goal**

- Finish local network discovery, household device assignment, pairing state, and mesh/event handoff without duplicate truth across LAN/setup/portal.

**Code Written**

- `lan-core` exists and downstream portal pairing consumption already has one checked slice.

**Test List Done**

- Checked downstream coverage exists for LAN pairing state consumption, but core LAN plan tests are still not closed.

**Test List Required**

- Discovery adapter tests.
- Classification/dedup/store tests.
- Heartbeat/assignment/revocation tests.
- Portal handoff tests.
- Rollout/route gate tests.

**Reason / Blocker / Deferred**

- Depends on provisioning/account identity/eventing ownership being firmer.

### Network Plan

- plan status: **in-progress**
- primary Rust crates: `network-core`, `agent-protocol`, `agent-core`, `agent-service`, `ocentra-network-evidence`
- primary TS domains/apps: `portal-domain`, `apps/portal` (projection/read-model consumers only)
- read if working this plan: [AGENTS](network-plan/AGENTS.md), [PLAN_STATE](network-plan/PLAN_STATE.md), [NEXT_ACTIONS](network-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](network-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `network-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                       | status | code        | test        | location crate                             | location domain/app                                  |
| ----------------------------------------------------------------- | ------ | ----------- | ----------- | ------------------------------------------ | ---------------------------------------------------- |
| 01-03 contracts/capture/classification                              | open    | in-progress | in-progress | `network-core`, `agent-protocol`, `agent-core`, `agent-service`, `ocentra-network-evidence` | `portal-domain` |
| 04 cross-slice cascade and parent surface                            | blocked | incomplete  | deferred    | `agent-service`, `agent-core`, `ocentra-network-evidence` | `portal-domain`, `apps/portal` |
| 05 intervention adapter proof gates                               | open   | in-progress | in-progress | `ocentra-network-evidence`, `agent-protocol`, `agent-service` | `agent-protocol-domain`, `portal-domain`, `apps/portal` |
| 06 analyzer AI audit and risk budget                              | open   | partial     | partial     | `network-core`, `child-ai-core`            | `network-domain`, `ai-domain`                        |
| 07 performance/security/rollout                                   | open   | partial     | partial     | `network-core`                             | `network-domain`                                     |
| 08 control catalog reference routing                              | open   | partial     | partial     | `network-core`                             | `network-domain`, `capability-domain`                |

**Goal**

- Finish network evidence capture/classification/correlation and safe intervention/reporting boundaries.

**Code Written**

- `network-core` now acts as a compatibility wrapper over the protocol-owned child-domain event chain instead of keeping a second private runtime decision contract.
- The runtime seam now carries explicit observation, AI handoff, and policy handoff semantics and downgrades degraded inputs to observe-only so it stays aligned with the existing owner path.
- The 2026-08-16 production audit found that the former product-path bridge fabricated analyzer, AI, policy, adapter, custody, export, and portal refs from one observation. The shipped caller was first disabled, then the bridge, payload fields, and disconnected evidence pipeline were deleted in `9e9f9ac51`.
- The portal drawer remains a real service-backed projection of stored network observations and runtime-delivery state. It must render unavailable/not-reported for downstream facets until authoritative AI, policy, notification, adapter, and custody owners supply real records.
- A shipped-call audit found no typed durable `NetworkCascadeObligation`, durable cascade table, or composition owner: the apparent cascade is `NetworkRuntimeDelivery`/`NetworkRuntimeSpine`, read-time republish, and manufactured phase refs. WP04 is blocked behind direct Eventing, AI, Policy, Custody, and Portal owner handoffs.
- `ocentra-network-evidence` Android VpnService and Apple Network Extension proof-gate planners are now surfaced through new `agent-protocol` command/event/status contracts, `agent-service` websocket bridges, and `agent-protocol-domain` parser/default seams instead of stopping below the service boundary.
- `portal-domain` live activity state, command surfaces, and diagnostics export now carry the Android VpnService and Apple Network Extension gate-status results alongside the existing Windows/Linux network gate statuses, while keeping the scope to the existing developer/live-activity seams and not widening into new drawer UI.
- `network-core` and `ocentra-network-evidence` still leave most of the full plan-owned chain partial.

**Test List Done**

- Partial crate-level/downstream coverage exists, but not enough to call the network plan closed.
- `crates/network-core/tests/unit/network_flow.rs` and `crates/network-core/tests/unit/runtime_flow.rs` now cover the protocol-owned event chain seam, degraded-input downgrade behavior, and wrapper-helper parity with the full network runtime chain.
- Tests that imported or asserted the deleted network product-path bridge/payload/pipeline are invalidated debt. They must be deleted or rewritten against shipped authoritative owners in the test phase and do not count as coverage now.
- WP04 tests remain deferred; this status refresh adds no production/test code, test pass, proof, CI, or completion claim.
- Any Rust/TS contract test that requires the orphaned network product-path field constants is invalidated with the removed producer and must be deleted or rewritten with the dead contract cleanup.
- Portal tests may retain real observation/runtime-delivery projection assertions, but any case that injects product-path refs without a shipped authoritative producer must be rewritten; static refs and fallback precedence are not product-path proof.
- `crates/agent-protocol/src/network_android_vpn_service_gate_status_tests.rs`, `crates/agent-protocol/src/network_apple_network_extension_gate_status_tests.rs`, and `crates/agent-protocol/src/tests.rs` now cover the new Rust protocol status shapes plus Android/Apple command/event serialization.
- `crates/agent-service/src/network_android_vpn_service_gate_status_bridge_tests.rs` and `crates/agent-service/src/network_apple_network_extension_gate_status_bridge_tests.rs` now cover the service payload builders and websocket routing for the Android/Apple proof-gate status commands.
- `packages/agent-protocol-domain/tests/unit/network-android-vpnservice-gate-status.test.ts`, `packages/agent-protocol-domain/tests/unit/network-apple-network-extension-gate-status.test.ts`, `packages/portal-domain/tests/unit/contracts.test.ts`, and `apps/portal/tests/live-activity-state.test.ts` now cover the TS contract mirror, parser failure modes, command wiring, diagnostics fields, and live-activity event capture for those two new gate-status seams.

**Test List Required**

- Passive capture/parsing tests.
- Classification/correlation tests.
- Broader parent surface/read-model tests beyond the landed drawer summary and route-panel seam.
- Intervention adapter tests.
- AI audit/risk-budget coverage only after a shipped authoritative owner produces those records; the deleted citation seam is not a baseline.
- Performance/security/rollout tests.

**Reason / Blocker / Deferred**

- Depends on eventing, enforcement, and AI boundaries.

### Payment Subscription Plan

- plan status: **in-progress**
- primary Rust crates: `billing-core`, `entitlement-core`
- primary TS domains/apps: `billing-domain`, `family-domain`
- read if working this plan: [AGENTS](payment-subscription-plan/AGENTS.md), [PLAN_STATE](payment-subscription-plan/PLAN_STATE.md), [NEXT_ACTIONS](payment-subscription-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](payment-subscription-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `payment-subscription-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                    | status  | code        | test        | location crate                     | location domain/app                |
| -------------------------------------------------------------- | ------- | ----------- | ----------- | ---------------------------------- | ---------------------------------- |
| 01-05 pricing/checkout/webhooks/entitlements/invoice lifecycle | Planned | in-progress | in-progress | `billing-core`, `entitlement-core` | `billing-domain`, `family-domain`  |
| 06 security/privacy/observability                              | Planned | partial     | partial     | `billing-core`                     | `billing-domain`, `logging-domain` |
| 07 rollout proof and route gate                                | Planned | partial     | partial     | `billing-core`                     | `billing-domain`                   |

**Goal**

- Own pricing, checkout, webhook lifecycle, entitlements, billing edge cases, and billing observability in one place.

**Code Written**

- `billing-core`, `entitlement-core`, and `billing-domain` exist, but the plan is still mostly a shell/partial ownership pass.
- `billing-domain` now has a first TS-only pricing slice in `billing-pricing-matrix.ts` and `billing-pricing-matrix-proof.ts`, composing existing entitlement, feature, failure, device-limit, snapshot, and sync contracts into explicit tier-matrix, trial/grace boundary, safety-critical free boundary, and entitlement-source-owner sections.

**Test List Done**

- `packages/billing-domain/tests/unit/billing-pricing-matrix.test.ts` now exists for the pricing tier matrix, trial/grace boundary, safety-critical free boundary, and entitlement-source-owner assertions.
- Focused architecture lint and ESLint passed for the pricing slice, but Vitest execution is still blocked by the pre-existing billing-domain `TSCONFIG_ERROR` in this checkout.

**Test List Required**

- Pricing/product matrix tests.
- Checkout/billing portal tests.
- Webhook/idempotency tests.
- Entitlement delivery/gate tests.
- Invoice/refund/dispute/grace tests.
- Security/privacy/observability tests.

**Reason / Blocker / Deferred**

- Deferred behind more foundational household/eventing/policy/setup work, and the current billing-domain workspace still has a pre-existing test/bootstrap blocker (`TSCONFIG_ERROR` plus an existing initialization cycle in unrelated billing-domain code).

### Policy Control Plane Plan

- plan status: **in-progress**
- primary Rust crates: `policy-control-core`, `child-policy-core`, `child-notification-core`, `child-runtime`, `parent-runtime-core`
- primary TS domains/apps: `policy-domain`, `notification-domain`, `event-domain`, `apps/portal`
- read if working this plan: [AGENTS](policy-control-plane-plan/AGENTS.md), [PLAN_STATE](policy-control-plane-plan/PLAN_STATE.md), [NEXT_ACTIONS](policy-control-plane-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](policy-control-plane-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `policy-control-plane-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                      | status  | code        | test        | location crate                                                                                                | location domain/app                                                              |
| -------------------------------- | ------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| 01 policy source of truth        | Checked | done | covered | `policy-control-core`, `child-policy-core`                                                                    | `policy-domain`                                                                  |
| 02 parent authoring preview      | Checked | done | covered | `policy-control-core`                                                                                         | `policy-domain`, `portal-domain`, `apps/portal`                                  |
| 03 domain policy compilers       | Checked | done | covered | `agent-protocol`, `policy-control-core`, `child-policy-core`, `child-runtime`                                | `policy-domain`, `event-domain`                                                  |
| 07 schedule/time-budget/conflict | Checked | done | covered | `policy-control-core`                                                                                         | `policy-domain`                                                                  |
| 08 event families/idempotency/replay/audit linkage | Checked | done | covered | `policy-control-core`                                                                                         | `policy-domain`, `event-domain`                                                  |
| 04 delivery/ack/audit            | Checked  | done        | covered     | `policy-control-core`, `child-policy-core`, `child-notification-core`, `child-runtime`, `parent-runtime-core` | `policy-domain`, `notification-domain`, `agent-protocol-domain`                  |
| 05 ask-parent overrides          | Checked  | done        | covered     | `policy-control-core`, `child-policy-core`, `child-notification-core`, `child-runtime`, `parent-runtime-core` | `policy-domain`, `notification-domain`, `agent-protocol-domain`, `portal-domain` |
| 06 rollout proof and route gate  | Checked | done        | covered     | `policy-control-core`                                                                                         | `policy-domain`                                                                  |

**Goal**

- Make policy the canonical authority layer: source-of-truth, authoring/preview, compilers, delivery/ack, and parent override flows.

**Code Written**

- `policy-control-core` now has dedicated plan-owned modules for source-of-truth, compiler, conflict, preview, delivery, and ask-parent request/override lifecycles.
- `policy-control-core` source/compiler/conflict/preview/request modules now import their owner types from `policy_source` and `policy_authority` instead of relying on missing crate-root reexports, so the real owner crate compiles again without adding new root export debt.
- `policy-domain` now carries schedule boundary/DST validation plus approval and override lifecycle contracts.
- `policy-domain` now also carries an explicit WP07 time-budget owner contract: authoring schedules require reset/carryover/grace/effective-window/clock-source/offline-recovery semantics, schedule boundaries can carry runtime budget state plus bonus-time expiry/offline-recovery state, and bonus-time approvals now require schedule budget context instead of raw minutes alone.
- `agent-protocol-domain` now carries parent-visible delivery read-model and audit-redaction contracts for policy-control delivery states.
- `notification-domain` now carries approval notification/audit boundary coverage for preview-only, pending-parent-review, approved, denied, modified, expired-request, and replay-rejected queue states without claiming provider delivery or auto-mutation.
- `child-policy-core` now validates child-domain policy evaluation inputs, owns canonical request-builder helpers plus dedicated policy-control request/delivery handoff modules, and is no longer just a thin tracking-only leaf under this plan.
- `child-notification-core` now has a dedicated `policy_control_notification` helper that turns request/override/delivery state into parent-visible notification handoff records with preserved audit refs.
- `child-runtime` now keeps `policy_control_runtime_flow` as a thin notification-projection wrapper over `child-policy-core` request/delivery handoff modules rather than owning those state transitions directly.
- `parent-runtime-core` now owns a dedicated `policy_control_dispatch` and `policy_control_update_flow` slice for dispatch gating, child publish decisions, delivery progression, blocked/manual-required routing, and parent-visible degraded/applied state transitions.
- `@ocentra-parent/agent-protocol-domain` package exports now include the new `policy-control-delivery-read-model` and `policy-control-audit-redaction` adapters instead of leaving the files stranded off the package boundary.
- `policy-control-core` source lifecycle now has a dedicated supersede transition for WP01 that requires a strictly newer replacement policy version plus a fresh audit ref before a source document can move into `superseded`, instead of leaving `policy-source.policy-version-supersede` as a plan-only state name.
- `policy-control-core` source lifecycle now also has a typed rollback reference and dedicated rollback transition for WP01, so a source document cannot move into `rolledBack` unless it cites an older restored policy version, a distinct restored document, and a fresh audit ref.
- `policy-control-core` compiler seam now blocks `draft` and `preview` source documents before domain compilation, so compiler artifacts cannot be produced from pre-confirmation portal state and the lifecycle rule "confirmed source policy exists before compile" is enforced in code instead of only in plan prose.
- `policy-control-core` source and compiler artifacts now also preserve `audit_reference_ids`, `superseded_by_policy_version`, and `rollback_ref`, so lifecycle metadata survives into compiled policy artifacts instead of disappearing between WP01 source truth and WP03 compiler ownership.
- `policy-control-core` compiler artifacts now also own an explicit `support_matrix` plus per-rule `capability_state`, with default domain matrices and an override-capable compiler input seam so supported/manual-required/unsupported ownership is compiler data instead of only hard-coded target matching.
- `policy-control-core` compiler artifacts now also preserve explicit evidence/custody requirements from the source document, instead of dropping retention/export/delete/sync ownership at the compiler boundary.
- `policy-control-core` compiler seam now also rejects `domain-cache` source-surface documents, so compiled artifacts cannot be reintroduced as canonical source truth through the same owner path they are supposed to feed.
- `policy-control-core` conflict detection now treats timezone-mismatched overlapping schedules as an explicit blocking/manualRequired `TimezoneBoundary` conflict instead of silently dropping them as non-overlapping, and the same conflict records now preserve source document version, audit refs, and rollback refs for rollback-aware review.
- `child-policy-core`, `child-notification-core`, `child-runtime`, and `parent-runtime-core` now consume that persisted lifecycle metadata through their real policy-control delivery/notification/runtime seams, so the downstream handoff path no longer relies on stale `ParentPolicySourceDocument` fixtures or missing manifest/runtime glue.
- `policy-control-core` delivery records now also preserve `source_audit_reference_ids`, `source_superseded_by_policy_version`, and `source_rollback_ref` separately from delivery-transition audit/state fields, so WP03 source/compiler provenance survives the first queued-delivery boundary.
- `policy-control-core` delivery records now also cover acknowledged, offline, and redacted-log behavior in focused unit/version-skew tests, and the WP04 proof artifact now captures the per-device/domain delivery and audit evidence for that slice.
- `policy-control-core` ask-parent request and override lifecycles now also cover replay rejection, parent confirmation, assistant preview-only gating, audited overrides, and notification handoff in focused unit/version-skew and downstream handoff tests, and the WP05 proof artifact now captures that evidence in `docs/proof/policy-control-plane-plan/05-ask-parent-overrides-proof.md`.
- `policy-control-plane-plan` WP01 source-of-truth proof bundle now ties the compatibility, schema, version-skew, duplicate-truth, AI-preview-not-write, authz, and custody slices into `docs/proof/policy-control-plane-plan/01-*.md`, with `PLAN_PROOF_MANIFEST.md` and the plan-local route docs updated to match.
- `policy-control-plane-plan` WP02 parent-authoring-preview proof bundle now ties the authoring, conflict-visible, unsupported-target, no-fake-green, and assistant-draft preview-only slices into `docs/proof/policy-control-plane-plan/02-*.md`, with `PLAN_PROOF_MANIFEST.md` and the plan-local route docs updated to match.
- `policy-control-plane-plan` WP06 route gate now ties the closed source, preview, schedule, compiler, delivery, override, and event proof slices into the plan-local proof manifest and no-overclaim bundle, with `docs/proof/policy-control-plane-plan/06-*.md` and a checked WP06 index entry.
- `policy-control-plane-plan` WP07 schedule/time-budget/conflict proof bundle now ties the timezone, DST, budget reset, conflict precedence, and offline recovery slices into `docs/proof/policy-control-plane-plan/07-*.md`, with `PLAN_PROOF_MANIFEST.md` and a checked WP07 index entry.
- `policy-domain` now also owns a shared `policy-compiler.ts` contract for compiled artifact ids, domains, capability states, support matrices, rule statuses, delivery targets, evidence-custody flags, no-claim labels, and rollback/supersede metadata without introducing a barrel-style export seam.
- `policy-control-core` and `policy-domain` now also own the WP08 policy-event model: family registry, idempotency keys, replay ordering, rollback linkage, dead-letter/manual-required handling, and redacted-summary helpers, with focused Rust and TS tests passing in this checkout.
- Portal authoring/preview surfaces and rollout-proof artifacts remain the clearest plan-owned gaps after the new child/parent runtime handoff slices.

**Test List Done**

- `crates/policy-control-core/tests/unit/policy_source.rs`
- `crates/policy-control-core/tests/unit/policy_compiler.rs`
- `crates/policy-control-core/tests/unit/policy_conflict.rs`
- `crates/policy-control-core/tests/unit/policy_preview.rs`
- `crates/policy-control-core/tests/unit/policy_delivery.rs`
- `crates/policy-control-core/tests/unit/policy_request.rs`
- `crates/policy-control-core/tests/version-skew/policy_source.rs`
- `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
- `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
- `crates/policy-control-core/tests/version-skew/policy_preview.rs`
- `crates/policy-control-core/tests/version-skew/policy_request.rs`
- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
- `packages/policy-domain/tests/unit/policy-approval-override.test.ts`
- `packages/policy-domain/tests/unit/policy.test.ts` now covers the new explicit schedule time-budget contract, including required reset-day and capped-carryover negatives.
- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts` now also covers runtime budget-state acceptance, active bonus-time expiry enforcement, and offline timer recovery-state negatives in the existing real unit bucket.
- `packages/policy-domain/tests/unit/policy-approval-override.test.ts` now also proves bonus-time approvals require schedule budget context instead of accepting a request that only carries minutes.
- `crates/policy-control-core/tests/unit/policy_event.rs`, `crates/policy-control-core/tests/version-skew/policy_event.rs`, and `packages/policy-domain/tests/unit/policy-event.test.ts` now cover the WP08 policy-event family registry, idempotency, replay ordering, rollback linkage, and redaction slices.
- `packages/agent-protocol-domain/tests/unit/policy-control-delivery-read-model.test.ts`
- `packages/agent-protocol-domain/tests/unit/policy-control-audit-redaction.test.ts`
- `packages/notification-domain/tests/unit/policy-control-approval-notification-boundary.test.ts`
- `crates/child-policy-core/tests/unit/child_domain_policy.rs`
- `crates/child-policy-core/tests/unit/policy_control_request.rs`
- `crates/child-policy-core/tests/unit/policy_control_request_handoff.rs`
- `crates/child-policy-core/tests/unit/policy_control_delivery_handoff.rs`
- `crates/child-notification-core/tests/unit/policy_control_notification.rs`
- `crates/child-runtime/tests/integration/policy_control_runtime_flow_intent.rs`
- `crates/parent-runtime-core/tests/unit/policy_control_dispatch.rs`
- `crates/parent-runtime-core/tests/unit/policy_control_update_flow.rs`
- `crates/policy-control-core/tests/unit/policy_source.rs` now also covers the new WP01 supersede transition, including the negative same-version replacement and duplicate-audit-ref rejection cases.
- `crates/policy-control-core/tests/version-skew/policy_source.rs` now also proves the source supersede helper rejects non-newer replacement versions as part of the versioning boundary.
- `crates/policy-control-core/tests/unit/policy_source.rs` now also covers mismatched-actor and mismatched-role authority rejection plus the new WP01 rollback transition, including prior-version and duplicate-audit-ref negatives.
- `crates/policy-control-core/tests/version-skew/policy_source.rs` now also proves the rollback helper rejects non-older restored policy versions as part of the WP01 lifecycle/versioning boundary.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also covers the new WP03 pre-confirmation gate, proving `draft` and `preview` source documents are rejected before compiler artifacts are produced.
- `crates/policy-control-core/tests/unit/policy_source.rs` now also proves the older `compile_domain_policy_artifact(...)` helper preserves audit refs, leaves lifecycle refs empty on confirmed source documents, and rejects `draft`/`preview` source documents instead of letting the older seam bypass the new compiler lifecycle rule.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also proves compiled artifacts preserve audit refs plus supersede/rollback lifecycle refs from source documents rather than dropping them at the domain compiler boundary.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also covers `domain-cache` source-surface rejection, explicit screen compiler status mapping, broader tracking/location-geofence target coverage, a cross-domain determinism matrix, and evidence/custody preservation on compiled artifacts.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also covers explicit support-matrix override, per-rule capability-state mapping, and malformed support-matrix rejection cases for the WP03 compiler owner seam.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also covers the WP03 domain-override semantics where `enforcement` and `notification-ask-parent` keep a supported support matrix while selected rules still downgrade to `manual-required` with the correct reason-code split.
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs` now also serializes the new evidence/custody requirements field and screen-domain status strings instead of keeping the version-skew bucket browser-only.
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs` now also round-trips `support_matrix` rows plus rule `capability_state` payloads so the explicit WP03 compiler contract survives serialization.
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs` now also proves those `enforcement` and `notification-ask-parent` override semantics survive JSON payload shape instead of only in-memory structs.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` now also preserves the WP07 time-boundary schedule matrix verbatim across DST spring-forward, DST fall-back, child-device clock-source, and manual-required clock-source fixtures instead of leaving that compiler breadth implicit.
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs` now also round-trips the WP07 time-boundary schedule payload so the serialized artifact keeps the same DST/clock-source/offline-recovery schedule shapes across the version-skew seam.
- `crates/policy-control-core/tests/unit/policy_delivery.rs` and `crates/policy-control-core/tests/version-skew/policy_delivery.rs` now also assert queued delivery keeps source audit/supersede/rollback provenance separate from delivery-transition state.
- `crates/child-policy-core/tests/unit/policy_control_delivery_handoff.rs` and `crates/child-runtime/tests/integration/policy_control_runtime_flow_intent.rs` now also assert queued delivery handoff keeps the preserved source compiler provenance visible to the first downstream consumers.
- `packages/policy-domain/tests/unit/policy-compiler.test.ts` now covers deterministic parse behavior, capability-state/status mismatch rejection, reason-code enforcement for manual-required and unsupported rules, no-claim-set integrity, support-matrix ownership, and rollback-vs-supersede exclusivity for the shared TS compiler contract owner.
- `crates/child-notification-core/tests/unit/policy_control_notification.rs`, `crates/parent-runtime-core/tests/unit/policy_control_dispatch.rs`, and `crates/parent-runtime-core/tests/unit/policy_control_update_flow.rs` already cover the broader downstream runtime handoff seams that sit above this WP03 artifact ownership slice.
- `crates/policy-control-core/tests/unit/policy_conflict.rs` now proves nonexistent-local-time, ambiguous-local-time, and manual clock-skew stay explicit/manualRequired and that conflict records preserve source document version, audit refs, and rollback refs instead of losing rollback-aware context.
- `crates/policy-control-core/tests/unit/policy_preview.rs` now proves the parent preview surface blocks nonexistent-local-time, ambiguous-local-time, and manual clock-skew before save while leaving plain `ChildDevice` clock sources free of synthetic clock-skew findings.

**Test List Required**

- Parent authoring/preview tests that hit real portal-facing preview/conflict states rather than only shared domain/runtime contracts.
- Remaining WP07 gaps are now outside this explicit preview/conflict owner seam: broader schedule-budget parity, downstream parent-surface consumers, and route/proof closure rather than missing Rust-side ambiguous/nonexistent/clock-skew coverage.
- Remaining WP03 compiler gaps are now above the owner seam: downstream adoption of the explicit support-matrix/capability-state contract in existing TS compiler consumers, broader parent-surface consumers, and route/proof closure rather than missing basic artifact ownership.
- Rollout proof manifests and route-gate artifacts for workpack 06.
- End-to-end parent surface consumption tests once the portal/runtime policy rows stop being partial.

**Reason / Blocker / Deferred**

- Eventing and account/setup authority still need to settle so policy stops depending on partial upstream ownership.
- The newest WP03 support-matrix, delivery-provenance, and shared TS compiler-contract additions are recorded here as code plus tests written only; no focused cargo or npm command execution was run for that incremental slice yet.
- `cargo check -p ocentra-policy-control-core`, `cargo test -p ocentra-policy-control-core --test unit -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew -- --test-threads=1` now pass after the owner-import wiring fix in the `policy_source` / `policy_authority` consumers.
- `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_source.rs crates/policy-control-core/tests/unit/policy_source.rs crates/policy-control-core/tests/version-skew/policy_source.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_source -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew policy_source -- --test-threads=1` now pass for the focused WP01 supersede slice.
- The same focused validation set now also passes for the rollback-ref slice, with `policy_source` unit coverage at 13 passing tests and version-skew coverage at 7 relevant passing tests in this checkout.
- `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_compiler.rs crates/policy-control-core/tests/unit/policy_compiler.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_compiler -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew policy_compiler -- --test-threads=1` now pass for the focused WP03 pre-confirmation compile gate.
- `cmd /c npm run lint:architecture -- crates/policy-control-core/src/policy_source.rs crates/policy-control-core/src/policy_compiler.rs crates/policy-control-core/tests/unit/policy_source.rs crates/policy-control-core/tests/unit/policy_compiler.rs crates/policy-control-core/tests/unit/policy_preview.rs crates/policy-control-core/tests/unit/policy_delivery.rs crates/policy-control-core/tests/unit/policy_conflict.rs crates/policy-control-core/tests/version-skew/policy_source.rs crates/policy-control-core/tests/version-skew/policy_source_migration.rs crates/policy-control-core/tests/version-skew/policy_preview.rs crates/policy-control-core/tests/version-skew/policy_delivery.rs crates/policy-control-core/tests/version-skew/policy_compiler.rs crates/child-policy-core/tests/unit/policy_control_delivery_handoff.rs crates/child-notification-core/tests/unit/policy_control_notification.rs crates/child-runtime/tests/integration/policy_control_runtime_flow_intent.rs crates/parent-runtime-core/tests/unit/policy_control_update_flow.rs crates/parent-runtime-core/tests/unit/policy_control_dispatch.rs` now passes for the lifecycle-ref propagation and downstream handoff slice.
- `cargo test -p ocentra-child-policy-core --test unit policy_control_delivery_handoff -- --test-threads=1`, `cargo test -p ocentra-child-notification-core --test unit policy_control_notification -- --test-threads=1`, `cargo test -p ocentra-child-runtime --test integration policy_control_runtime_flow_intent -- --test-threads=1`, and `cargo test -p ocentra-parent-runtime-core --test unit policy_control_ -- --test-threads=1` now pass for the focused WP04/WP05 downstream runtime seam.
- `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_conflict.rs crates/policy-control-core/src/policy_preview.rs crates/policy-control-core/tests/unit/policy_conflict.rs crates/policy-control-core/tests/unit/policy_preview.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_conflict -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test unit policy_preview -- --test-threads=1` now pass for the focused WP07 timezone-boundary/manualRequired conflict slice.
- `cmd /c npm run lint:architecture -- packages/policy-domain/src/policy.ts packages/policy-domain/src/authority.ts packages/policy-domain/tests/unit/policy.test.ts packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts packages/policy-domain/tests/unit/policy-approval-override.test.ts` and `cmd /c npx prettier --check packages/policy-domain/src/policy.ts packages/policy-domain/src/authority.ts packages/policy-domain/tests/unit/policy.test.ts packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts packages/policy-domain/tests/unit/policy-approval-override.test.ts` now pass for the focused TS owner WP07 time-budget slice.
  - `cmd /c npm run type-check --workspace @ocentra-parent/policy-domain`, `cmd /c npm run build --workspace @ocentra-parent/policy-domain`, and `cmd /c npm run test --workspace @ocentra-parent/policy-domain` now pass in this checkout after the repo-root `tsconfig.base.json` landed, so the policy-domain owner slice is runnable again.
- `node --test scripts/test/placeholder-implementation-guard.test.mjs` now covers the Rust-attribute `temporary-override` false-positive path in `scripts/check-no-placeholder-implementation.mjs`, so the focused architecture gate no longer misclassifies `#[serde(...)]` lines as placeholder comments.
- Focused `lint:architecture` for the touched `policy-control-core` owner files now passes end-to-end after moving runtime strings into the `agent-protocol` owner constants, so the next live policy-control gap is workpack closure on source-of-truth, schedule/conflict, authoring/preview, delivery, and override flows rather than crate architecture debt.

### Portal UX Household Surfaces Plan

- plan status: **in-progress**
- primary Rust crates: `parent-runtime-core`
- primary TS domains/apps: `portal-domain`, `parent-domain`, `family-domain`, `notification-domain`, `policy-domain`, `apps/portal`
- read if working this plan: [AGENTS](portal-ux-household-surfaces-plan/AGENTS.md), [PLAN_STATE](portal-ux-household-surfaces-plan/PLAN_STATE.md), [NEXT_ACTIONS](portal-ux-household-surfaces-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](portal-ux-household-surfaces-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `portal-ux-household-surfaces-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                                         | status  | code    | test    | location crate        | location domain/app                                                               |
| --------------------------------------------------------------------------------------------------- | ------- | ------- | ------- | --------------------- | --------------------------------------------------------------------------------- |
| 01-09 shell/household/device/policy/schedule/approvals/evidence/surfaces                            | open    | partial | partial | `parent-runtime-core` | `portal-domain`, `parent-domain`, `family-domain`, `policy-domain`, `apps/portal` |
| 10 LAN pairing state consumption                                                                    | checked | done    | covered | `parent-runtime-core` | `portal-domain`, `lan-domain`, `apps/portal`                                      |
| 11-20 assistant/reports/error-states/audit/accessibility/no-fake-data/playwright/mobile/docs/review | open    | partial | partial | `parent-runtime-core` | `portal-domain`, `parent-domain`, `notification-domain`, `apps/portal`            |

**Goal**

- Make the parent portal consume only canonical service/runtime data and expose the full household workflow without fake placeholder truth.

**Code Written**

- `apps/portal`, `portal-domain`, and `parent-runtime-core` already exist with partial real surfaces.
- LAN pairing surface is already in the checked bucket.

**Test List Done**

- Observed checked LAN pairing surface coverage.

**Test List Required**

- Shell/navigation tests.
- First-run and profile tests.
- Device/source-state and selected-device tests.
- Policy/schedule/approval/reporting surface tests.
- Degraded/error/no-fake-data tests.
- Playwright screenshot/mobile-shell tests.

**Reason / Blocker / Deferred**

- UI completion depends heavily on service read models from LAN/policy/tracking/browser/app/network.

### Remote Access Plan

- plan status: **in-progress**
- primary Rust crates: `remote-access-core`, `screen-live-view-core`
- primary TS domains/apps: `remote-access-domain`, `screen-domain`, `endpoint-domain`
- read if working this plan: [AGENTS](remote-access-plan/AGENTS.md), [PLAN_STATE](remote-access-plan/PLAN_STATE.md), [NEXT_ACTIONS](remote-access-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](remote-access-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `remote-access-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                         | status  | code    | test    | location crate                                | location domain/app                                        |
| ------------------------------------------------------------------- | ------- | ------- | ------- | --------------------------------------------- | ---------------------------------------------------------- |
| 01-05 capability fabric/live relay/input authority/consent/security | Planned | partial | partial | `remote-access-core`, `screen-live-view-core` | `remote-access-domain`, `screen-domain`, `endpoint-domain` |
| 06 rollout proof and route gate                                     | Planned | partial | partial | `remote-access-core`                          | `remote-access-domain`                                     |

**Goal**

- Own remote capability, live relay, input authority, consent/grant lifecycle, and abuse/security boundary in one remote-access path.

**Code Written**

- `remote-access-core` and `screen-live-view-core` exist, but the plan is still partial.

**Test List Done**

- No strong dedicated plan-owned test closure yet.

**Test List Required**

- Capability fabric tests.
- Relay/session tests.
- Input authority/consent tests.
- Abuse/security tests.
- Rollout/route gate tests.

**Reason / Blocker / Deferred**

- Depends on screen capture/live view and eventing/policy security boundaries.

### Screen AI Pipeline Plan

- plan status: **in-progress**
- primary Rust crates: `screen-ai-core`, `child-ai-core`, `ocentra-evidence`
- primary TS domains/apps: `screen-domain`, `ai-domain`, `evidence-domain`
- read if working this plan: [AGENTS](screen-ai-pipeline-plan/AGENTS.md), [PLAN_STATE](screen-ai-pipeline-plan/PLAN_STATE.md), [NEXT_ACTIONS](screen-ai-pipeline-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](screen-ai-pipeline-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `screen-ai-pipeline-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                               | status  | code    | test    | location crate                                      | location domain/app                             |
| --------------------------------------------------------- | ------- | ------- | ------- | --------------------------------------------------- | ----------------------------------------------- |
| 01 prerequisite merge and branch gate                     | checked | done    | covered | `screen-ai-core`                                    | `screen-domain`, `ai-domain`                    |
| 02 real trigger to capture gate                           | open    | partial | partial | `screen-ai-core`, `screen-core`                     | `screen-domain`, `ai-domain`                    |
| 03-09 capture/AI/policy/journal/retention/live/perf gates | checked | done    | covered | `screen-ai-core`, `screen-core`, `ocentra-evidence` | `screen-domain`, `ai-domain`, `evidence-domain` |
| 10 final rollout and PR gate                              | open    | partial | partial | `screen-ai-core`                                    | `screen-domain`, `ai-domain`                    |

**Goal**

- Finish the capture -> AI -> policy -> journal -> portal path as a safe, deterministic screen-analysis pipeline.

**Code Written**

- Most screen AI gates already exist and are checked.

**Test List Done**

- Gate coverage is already present for AI-to-policy, dry-run, journal/read-model, deletion/retention, live operator, and backpressure lanes.

**Test List Required**

- Real trigger-to-capture integration tests.
- Final rollout/PR gate tests.

**Reason / Blocker / Deferred**

- Remaining end-to-end work depends on the underlying screen plan capture/runtime path being fully ready.

### App + Game Plan

- plan status: **in-progress**
- primary Rust crates: `app-game-core`, `policy-control-core`
- primary TS domains/apps: `app-game-domain`, `evidence-domain`, `portal-domain`, `notification-domain`, `apps/portal`
- read if working this plan: [AGENTS](app-game-plan/AGENTS.md), [PLAN_STATE](app-game-plan/PLAN_STATE.md), [NEXT_ACTIONS](app-game-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](app-game-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `app-game-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                                                    | status             | code    | test    | location crate                                                    | location domain/app                                                      |
| -------------------------------------------------------------------------------------------------------------- | ------------------ | ------- | ------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------ |
| 10 / 13 / 14 / 17 / 24 / 25 / 27 / 29-31 core evidence, journal, unknown-approval, classifier, protocol parity | checked            | done    | covered | `app-game-core`, `policy-control-core`                            | `app-game-domain`, `evidence-domain`, `policy-domain`                    |
| 52-70 policy readiness + notification intent/read-model/parent-surface chain                                   | checked            | done    | covered | `app-game-core`, `child-notification-core`, `policy-control-core` | `app-game-domain`, `notification-domain`, `portal-domain`, `apps/portal` |
| 78-107 source-gated policy preview timer chain                                                                 | checked            | done    | covered | `app-game-core`, `policy-control-core`                            | `app-game-domain`, `policy-domain`, `portal-domain`, `apps/portal`       |
| 36-51 / 63 / 73-76 runtime freshness, policy routing, proof-pack readiness reference lanes                     | reference/no boxes | partial | partial | `app-game-core`, `policy-control-core`                            | `app-game-domain`, `evidence-domain`, `portal-domain`                    |
| 186-222 Android/Linux/child-runtime proof and receipt/reference lanes                                          | reference/no boxes | partial | partial | `app-game-core`, `child-runtime`                                  | `app-game-domain`, `notification-domain`, `portal-domain`                |
| capability guides / schema proposals / settings inventories                                                    | open               | partial | partial | `app-game-core`                                                   | `app-game-domain`                                                        |

**Goal**

- Finish app/game inventory, session/runtime evidence, policy preview, notification handoff, and platform/runtime delivery as one canonical slice.

**Code Written**

- `app-game-core` already owns a substantial runtime/read-model/timer/notification chain.

**Test List Done**

- Strong observed coverage exists for the checked runtime, journal, classifier, timer, and notification chains.

**Test List Required**

- Promote remaining reference/proof lanes into real runtime/test ownership where required.
- Android/Linux child-runtime delivery/receipt tests.
- Capability/schema/settings consistency tests.

**Reason / Blocker / Deferred**

- Some remaining tails are proof/reference heavy and depend on child-runtime/platform readiness.

### App Plan

- plan status: **in-progress**
- primary Rust crates: `app-core`, `child-runtime`, `parent-runtime-core`
- primary TS domains/apps: `capability-domain`, `parent-domain`, `portal-domain`, `notification-domain`, `apps/portal`
- read if working this plan: [AGENTS](app-plan/AGENTS.md), [PLAN_STATE](app-plan/PLAN_STATE.md), [NEXT_ACTIONS](app-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](app-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `app-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                  | status       | code    | test    | location crate                                               | location domain/app                                     |
| ---------------------------------------------------------------------------- | ------------ | ------- | ------- | ------------------------------------------------------------ | ------------------------------------------------------- |
| 01-03 contract/reconciliation/current snapshot                               | Open/unknown | partial | partial | `app-core`                                                   | `capability-domain`, `parent-domain`                    |
| 04-05 app identity and installed inventory models                            | Open/unknown | partial | partial | `app-core`                                                   | `capability-domain`, `parent-domain`                    |
| 06-10 Windows inventory/runtime/foreground/authority                         | Open/unknown | partial | partial | `app-core`, `child-runtime`                                  | `capability-domain`, `evidence-domain`                  |
| 11-19 taxonomy/session/journal/read-models/policy/time-budget                | Open/unknown | partial | partial | `app-core`, `policy-control-core`                            | `capability-domain`, `policy-domain`, `evidence-domain` |
| 20-28 child UX / proof gates / AI digest / perf / rollout                    | Open/unknown | partial | partial | `app-core`, `child-ai-core`                                  | `capability-domain`, `portal-domain`, `apps/portal`     |
| 29-49 protocol parity / live sources / service bridges / freshness / routing | Open/unknown | partial | partial | `app-core`, `child-runtime`, `parent-runtime-core`           | `capability-domain`, `evidence-domain`, `portal-domain` |
| 53-67 notification + policy-readiness parent-surface chain                   | Open/unknown | partial | partial | `app-core`, `child-notification-core`, `policy-control-core` | `notification-domain`, `portal-domain`, `apps/portal`   |
| 74-108 source freshness + policy preview + timer chains                      | Open/unknown | partial | partial | `app-core`, `policy-control-core`, `parent-runtime-core`     | `portal-domain`, `policy-domain`, `apps/portal`         |

**Goal**

- Separate app-only runtime ownership cleanly from app-game overlap and finish canonical app inventory, session, policy, and portal consumption paths.

**Code Written**

- `app-core` and related notification/policy preview/runtime chains already exist, but plan-owned closure is still open across most rows.

**Test List Done**

- Indirect/downstream coverage exists, but this plan still needs its own explicit closure.

**Test List Required**

- Inventory/runtime adapter tests.
- Taxonomy/session/journal/read-model tests.
- Policy/time-budget tests.
- AI digest/perf/rollout tests.
- Source-freshness/policy-preview/timer chain tests.

**Reason / Blocker / Deferred**

- Needs reconciliation with app-game ownership so the two plans do not drift into duplicate truth.

### Parent Desktop Runtime Package Plan

- plan status: **in-progress**
- primary Rust crates: `parent-runtime-core`, `agent-updater`
- primary TS domains/apps: `production-domain`, `parent-domain`, `apps/parent-desktop`
- read if working this plan: [AGENTS](parent-desktop-runtime-package-plan/AGENTS.md), [PLAN_STATE](parent-desktop-runtime-package-plan/PLAN_STATE.md), [NEXT_ACTIONS](parent-desktop-runtime-package-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](parent-desktop-runtime-package-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `parent-desktop-runtime-package-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                     | status  | code    | test    | location crate                         | location domain/app                                       |
| ------------------------------------------------------------------------------- | ------- | ------- | ------- | -------------------------------------- | --------------------------------------------------------- |
| 01-03 Tauri shell / local service connection / LAN route controller             | open    | partial | partial | `parent-runtime-core`                  | `parent-domain`, `endpoint-domain`, `apps/parent-desktop` |
| 04 parent observer read-only state                                              | checked | done    | covered | `parent-runtime-core`                  | `parent-domain`, `apps/parent-desktop`                    |
| 05 custody and source labels                                                    | open    | partial | partial | `parent-runtime-core`                  | `parent-domain`, `evidence-domain`, `apps/parent-desktop` |
| 06 parent mobile bridge boundary                                                | checked | done    | covered | `parent-runtime-core`                  | `parent-domain`, `apps/parent-desktop`                    |
| 07-08 installer and package preview                                             | open    | partial | partial | `parent-runtime-core`, `agent-updater` | `production-domain`, `apps/parent-desktop`                |
| 09-12 update/signing/diagnostics/privacy-release docs                           | checked | done    | covered | `parent-runtime-core`, `agent-updater` | `production-domain`, `parent-domain`                      |
| 13-14 desktop launch smoke / build-dev scripts                                  | open    | partial | partial | `parent-runtime-core`                  | `production-domain`, `apps/parent-desktop`                |
| 15-18 capability matrix / release boundary / GH artifact proof / manual runbook | checked | done    | covered | `parent-runtime-core`, `agent-updater` | `production-domain`, `parent-domain`                      |
| 19-20 checklist sync / PR-CI-rollout gate                                       | open    | partial | partial | `parent-runtime-core`                  | `production-domain`, `parent-domain`                      |

**Goal**

- Own parent shell packaging, local service connection, update/release boundaries, and packaged runtime diagnostics.

**Code Written**

- Read-only/mobile bridge/update/signing/diagnostics/capability/release slices already exist and are checked.

**Test List Done**

- Observed existing coverage for the checked package/runtime support slices.

**Test List Required**

- Tauri shell tests.
- Local service connection tests.
- LAN controller tests.
- Installer/preview tests.
- Launch smoke/build script tests.
- Checklist/rollout gate tests.

**Reason / Blocker / Deferred**

- Cross-platform packaging and shell runtime still depend on later environment/runtime passes.

### AI Plan

- plan status: **in-progress**
- primary Rust crates: `child-ai-core`, `screen-ai-core`, `ocentra-eventing`
- primary TS domains/apps: `ai-domain`, `event-domain`, `evidence-domain`, `agent-protocol-domain`
- read if working this plan: [AGENTS](ai-plan/AGENTS.md), [PLAN_STATE](ai-plan/PLAN_STATE.md), [NEXT_ACTIONS](ai-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](ai-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `ai-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                           | status  | code    | test    | location crate                      | location domain/app                                                                  |
| ------------------------------------------------------------------------------------- | ------- | ------- | ------- | ----------------------------------- | ------------------------------------------------------------------------------------ |
| 01 source index and repo reconciliation                                               | checked | done    | covered | `child-ai-core`                     | `ai-domain`, `event-domain`                                                          |
| 02-08 AI contracts/provider/runtime/queue/routing                                     | open    | partial | partial | `child-ai-core`, `ocentra-eventing` | `ai-domain`, `event-domain`, `agent-protocol-domain`                                 |
| 09-18 evidence context/prompt/classifier/execution/validator/degraded handling        | open    | partial | partial | `child-ai-core`                     | `ai-domain`, `evidence-domain`, `policy-domain`                                      |
| 19-25 result journal/read-model/memory/graph contracts                                | open    | partial | partial | `child-ai-core`, `ocentra-evidence` | `ai-domain`, `evidence-domain`                                                       |
| 26-39 reuse candidates and worker lanes across browser/app/tracking/screen/device fit | open    | partial | partial | `child-ai-core`, `screen-ai-core`   | `ai-domain`, `browser-domain`, `app-game-domain`, `tracking-domain`, `screen-domain` |
| 40-48 packaging/governance/portal surface/auth/security/perf/rollout                  | open    | partial | partial | `child-ai-core`, `screen-ai-core`   | `ai-domain`, `portal-domain`, `apps/portal`                                          |

**Goal**

- Finish AI as a canonical external analysis/runtime lane: provider/runtime/queue/context/result validation/memory/reuse/governance/portal surface.

**Code Written**

- `child-ai-core`, `screen-ai-core`, and `ai-domain` already exist and are partially consumed by downstream plans.

**Test List Done**

- Observed checked source-reconciliation and downstream gate coverage exist, but the dedicated AI plan remains largely open.

**Test List Required**

- Contract/provider/runtime/queue tests.
- Prompt/context/parser/validator/degraded-path tests.
- Journal/memory/graph tests.
- Reuse-lane tests across browser/app/tracking/screen.
- Packaging/security/performance/rollout tests.

**Reason / Blocker / Deferred**

- Many downstream e2e flows should stay deferred until AI core ownership is finished.

### V0.8 Enforcement Control Plan

- plan status: **in-progress**
- primary Rust crates: `child-enforcement-core`, `child-policy-core`, `app-core`, `app-game-core`, `browser-core`
- primary TS domains/apps: `enforcement-domain`, `policy-domain`, `browser-domain`, `app-game-domain`, `capability-domain`, `apps/portal`
- read if working this plan: [AGENTS](v0-8-enforcement-control-plan/AGENTS.md), [PLAN_STATE](v0-8-enforcement-control-plan/PLAN_STATE.md), [NEXT_ACTIONS](v0-8-enforcement-control-plan/NEXT_ACTIONS.md), [WORKPACK_INDEX](v0-8-enforcement-control-plan/WORKPACK_INDEX.md)
- test/proof route for this plan: selected `v0-8-enforcement-control-plan/workpacks/*.md`, then later-phase [TEST_PROOF_DECISION_MATRIX](../agent/TEST_PROOF_DECISION_MATRIX.md)

| workpack(s)                                                                                          | status  | code    | test    | location crate                                                        | location domain/app                                                        |
| ---------------------------------------------------------------------------------------------------- | ------- | ------- | ------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| 01-06 contracts/evidence/adapter matrix/owned-process/session/browser control                        | open    | partial | partial | `child-enforcement-core`, `app-core`, `app-game-core`, `browser-core` | `enforcement-domain`, `policy-domain`, `browser-domain`, `app-game-domain` |
| 07 unmanaged browser fallback                                                                        | checked | done    | covered | `child-enforcement-core`, `browser-core`                              | `enforcement-domain`, `browser-domain`                                     |
| 08 network/domain report-only boundary                                                               | open    | partial | partial | `child-enforcement-core`, `network-core`                              | `enforcement-domain`, `network-domain`                                     |
| 09 timer recovery and rollback                                                                       | checked | done    | covered | `child-enforcement-core`, `policy-control-core`                       | `enforcement-domain`, `policy-domain`                                      |
| 10-20 approvals/audit/child-facing states/service read-model/integrity/tamper/platform/proof/rollout | open    | partial | partial | `child-enforcement-core`, `child-policy-core`, `policy-control-core`  | `enforcement-domain`, `policy-domain`, `portal-domain`, `apps/portal`      |

**Goal**

- Make enforcement a clean authority layer with explicit approvals, audit, child-facing state, integrity, tamper handling, and rollout rules.

**Code Written**

- Unmanaged browser fallback and timer rollback slices are already checked.
- Core enforcement ownership exists in `child-enforcement-core` but most plan rows remain partial.

**Test List Done**

- Observed coverage exists for unmanaged-browser fallback and timer rollback slices.

**Test List Required**

- Policy decision evidence tests.
- Adapter capability/app-game/browser control tests.
- Parent approval/audit/service read-model tests.
- Child-facing status/integrity/tamper/unavailable-state tests.
- UI proof and rollout gate tests.

**Reason / Blocker / Deferred**

- Depends on policy/network/browser/app/app-game/platform proof authority settling first.

## Previous Write Chunk

- current focus: `policy-control-plane-plan` `WP05` ask-parent overrides slice
- current strategy:
  - keep the work inside the existing policy request owner seam: `policy-control-core` Rust contract + tests plus the child-policy and child-notification handoff seams that consume it
  - keep Rust aligned on replay rejection, parent confirmation, assistant preview-only gating, bonus-time expiry, audited overrides, and notification handoff
  - keep validation focused to the touched slice: targeted Rust tests and architecture lint only, not full `npm validate`
- current files in this chunk:
  - `crates/policy-control-core/tests/unit/policy_request.rs`
  - `crates/child-policy-core/tests/unit/child_domain_policy.rs`
  - `docs/proof/policy-control-plane-plan/05-ask-parent-overrides-proof.md`
  - `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - `docs/plans/policy-control-plane-plan/NEXT_ACTIONS.md`
  - `docs/plans/policy-control-plane-plan/WORKPACK_INDEX.md`
  - `docs/plans/currentstatus.md`
- current result:
  - `policy-control-core` request tests now cover replay rejection when a reused approval id changes decision
  - `child-policy-core` request handoff, delivery handoff, and child-notification-core policy-control notification tests pass for the ask-parent override path
  - the parent-runtime-core ask-parent update-flow unit suite still carries unrelated import debt, so it is noted as a validation gap rather than part of the WP05 close claim

- current focus: `policy-control-plane-plan` `WP04` delivery/ack/audit slice
- current strategy:
  - keep the work inside the existing policy delivery owner seam: `policy-control-core` Rust contract + tests and the policy-control proof/plan docs
  - keep Rust aligned on per-device/domain delivery, ack, offline degradation, replay safety, rollback linkage, and redacted logs
  - keep validation focused to the touched slice: targeted Rust tests and architecture lint only, not full `npm validate`
- current files in this chunk:
  - `crates/policy-control-core/tests/unit/policy_delivery.rs`
  - `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
  - `docs/proof/policy-control-plane-plan/04-delivery-ack-audit-proof.md`
  - `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - `docs/plans/policy-control-plane-plan/NEXT_ACTIONS.md`
  - `docs/plans/policy-control-plane-plan/WORKPACK_INDEX.md`
  - `docs/plans/currentstatus.md`
- current result:
  - `policy-control-core` delivery tests now cover acknowledged/offline parent-visible state, redacted log output, and explicit WP04 version-skew round-trips
  - focused architecture lint passed for the touched Rust test files, and the WP04 proof/plan pointers were updated

- current focus: `policy-control-plane-plan` `WP08` policy-event model slice
- current strategy:
  - keep the work inside the existing policy owner seams: `policy-control-core` Rust contract + tests, `policy-domain` TS contract + tests, and the policy-control proof/plan docs
  - keep Rust and TS aligned on the same event-family registry, deterministic aggregate/idempotency keys, replay safety, rollback linkage, dead-letter/manual-required visibility, and redacted summaries
  - keep validation focused to the touched slice: targeted Rust tests, `policy-domain` type-check and unit test, and focused architecture lint only, not full `npm validate`
- current files in this chunk:
  - `crates/policy-control-core/src/lib.rs`
  - `crates/policy-control-core/src/policy_event.rs`
  - `crates/policy-control-core/tests/unit.rs`
  - `crates/policy-control-core/tests/unit/policy_event.rs`
  - `crates/policy-control-core/tests/version_skew.rs`
  - `crates/policy-control-core/tests/version-skew/policy_event.rs`
  - `packages/policy-domain/package.json`
  - `packages/policy-domain/src/policy-event.ts`
  - `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `docs/proof/policy-control-plane-plan/08-event-family-registry-proof.md`
  - `docs/proof/policy-control-plane-plan/08-event-idempotency-proof.md`
  - `docs/proof/policy-control-plane-plan/08-event-replay-ordering-proof.md`
  - `docs/proof/policy-control-plane-plan/08-rollback-event-linkage-proof.md`
  - `docs/proof/policy-control-plane-plan/08-event-redaction-proof.md`
  - `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
  - `docs/plans/policy-control-plane-plan/NEXT_ACTIONS.md`
  - `docs/plans/policy-control-plane-plan/WORKPACK_INDEX.md`
  - `docs/plans/currentstatus.md`
- current result:
  - `policy-control-core` event-family registry, idempotency, replay, rollback linkage, manual-required/dead-letter handling, and redacted-summary tests all pass
  - `policy-domain` type-check and targeted Vitest now pass on the mirrored TS contract
  - focused architecture lint passed for the touched Rust and TS files, and the WP08 proof/plan pointers were updated
  - `packages/portal-domain/src/details.ts`
  - `apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx`
  - `apps/portal/tests/live-activity-network-flow.test.ts`
  - `docs/plans/currentstatus.md`
- current result:
  - `packages/child-runtime-domain/src/child-runtime-gates.ts` now mirrors the setup owner seam with nested `provisioningDecision` install/service/overall/blocker semantics, while `packages/child-runtime-domain/tests/unit/child-runtime-gates.test.ts` covers ready, installed-not-started, offline/degraded, reinstall-required, and mismatch rejection paths
  - superseded on 2026-08-16: the network product-path producer/payload/pipeline was synthetic and has been deleted; orphaned protocol fields and parser branches are scheduled for production-contract cleanup
  - the portal drawer keeps only real observation/runtime-delivery projection; analyzer/detection/risk-budget/policy/action/custody facets remain unavailable until authoritative shipped owners exist

- current focus: `tracking-plan` residual `WP16` broader schedule-matrix and tolerance-boundary closure
- current strategy:
  - stay inside the existing TS/Rust owner seams and extend the real `tests/unit` buckets instead of inventing plan-specific folders
  - close the remaining schedule-engine behavior breadth with explicit school/activity/calendar fixtures and a direct tolerance-boundary case
- current files in this chunk:
  - `crates/tracking-core/tests/unit/expected_place.rs`
  - `packages/tracking-domain/tests/unit/tracking.test.ts`
  - `docs/plans/currentstatus.md`
- current result:
  - `crates/tracking-core/tests/unit/expected_place.rs` now covers distinct school, activity, and calendar-backed expected-place cases in the Rust owner bucket
  - `packages/tracking-domain/tests/unit/tracking.test.ts` now mirrors that broader schedule matrix and adds a direct expected-place tolerance-boundary case at the geofence edge
  - the narrow residual WP16 list from the last scout is now closed in actual unit tests without creating any fake folder structure

- current focus: `account-identity-family-plan` `WP05` TypeScript controller-lease enforcement
- current strategy:
  - keep the change in the existing `family-domain` authority owner seam rather than spreading lease gating across callers first
  - add the missing remote-sensitive lease gate and prove it in the existing `tests/unit` bucket
- current files in this chunk:
  - `packages/family-domain/src/household-authority.ts`
  - `packages/family-domain/tests/unit/household-authority.test.ts`
  - `docs/plans/currentstatus.md`
- current result:
  - `family-domain` household authority now rejects remote-sensitive actions when the controller lease is missing, expired, or revoked
  - `packages/family-domain/tests/unit/household-authority.test.ts` now covers active, required, expired, and revoked controller-lease outcomes in the real unit bucket
  - the Rust mirror is now also landed in `crates/family-identity-core/src/household_authority.rs` and `crates/family-identity-core/tests/unit/household_authority.rs`, keeping the remote-sensitive controller-lease gate aligned across TS and Rust

- current focus: `policy-control-plane-plan` `WP03` compiler artifact contract closure
- current strategy:
  - keep the work inside the existing compiler owner seam rather than inventing a second policy-cache or proof-only artifact shape
  - add missing contract fields and negative cases in the real owner files, then strengthen only the existing `tests/unit` and `tests/version-skew` buckets
  - close the highest-signal compiler gaps first: evidence/custody propagation, domain-cache not-source-truth rejection, screen fixture coverage, tracking/geofence fixture breadth, and determinism checks
- current files in this chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_compiler.rs`
  - `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
  - `docs/plans/currentstatus.md`
- current result:
  - compiled policy artifacts now preserve explicit evidence/custody requirements from the source document instead of dropping retention/export/delete/sync ownership at the compiler boundary
  - the compiler now rejects `domain-cache` source-surface documents on the owner path, so a compiled artifact cannot be fed back in as canonical source truth
  - the unit bucket now covers explicit screen-domain status mapping, broader tracking/location-geofence target coverage, a cross-domain determinism matrix, and the new domain-cache negative case without creating any fake test folders
  - the version-skew bucket now serializes the new custody field and screen-domain status strings instead of staying browser-only

- current focus: `policy-control-plane-plan` `WP07` time-budget/reset/carryover/offline-recovery TS owner slice
- current strategy:
  - keep the new authoring/runtime schedule-budget contract in the existing `policy-domain` owner seam instead of inventing a second proof-only or portal-only shape
  - add the missing cases to the existing `tests/unit` bucket files rather than creating workpack-specific folders
  - tighten bonus-time approval requests so they require schedule budget context, but leave broader Rust/source-compiler parity as the next follow-on slice
- current files in this chunk:
  - `packages/policy-domain/src/policy.ts`
  - `packages/policy-domain/src/authority.ts`
  - `packages/policy-domain/tests/unit/policy.test.ts`
  - `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
  - `packages/policy-domain/tests/unit/policy-approval-override.test.ts`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - schedules in `policy-domain` now require explicit time-budget reset/carryover/grace/effective-window/clock-source/offline-recovery semantics
  - schedule boundaries can now surface runtime budget state, active bonus-time expiry, and offline timer recovery state instead of leaving those WP07 outcomes implicit
  - bonus-time approvals now reject requests that carry minutes without schedule budget context
  - focused `lint:architecture` and `prettier --check` pass for the touched TS owner files
  - package-local `policy-domain` `type-check`, `build`, and `test` execution now pass after the repo-root `tsconfig.base.json` landed, so the next live gap is downstream consumer validation and any remaining non-owner package debt rather than the policy-domain bootstrap itself
  - the WP07 proof bundle now exists under `docs/proof/policy-control-plane-plan/07-*.md` and is backed by the focused `policy-control-core` Rust unit/version-skew runs plus the `policy-domain` package test run in this checkout

- current focus: `policy-control-plane-plan` `WP07` timezone-boundary/manualRequired conflict slice
- current strategy:
  - keep the change inside the Rust owner seams that actually define schedule/conflict truth: `policy_conflict` for explicit conflict output and `policy_preview` for parent-visible pre-save behavior
  - preserve source version, audit refs, and rollback refs on conflict records so rollback-aware review data survives instead of being lost at the conflict boundary
  - use the existing crate-owned `tests/unit/policy_conflict.rs` and `tests/unit/policy_preview.rs` buckets rather than creating proof-only or plan-specific test folders
- current files in this chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_conflict.rs`
  - `crates/policy-control-core/tests/unit/policy_conflict.rs`
  - `crates/policy-control-core/tests/unit/policy_preview.rs`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - nonexistent-local-time, ambiguous-local-time, and manual clock-skew schedules no longer disappear from the conflict owner as if they were unrelated windows; they now surface as blocking/manualRequired conflicts with explicit reason codes
  - the same source-of-truth lifecycle context now survives into conflict records, so review/rollback surfaces can still see the source policy version, audit refs, and rollback ref that produced the conflict
  - the parent preview seam now has direct test coverage for nonexistent-local-time, ambiguous-local-time, and manual clock-skew before save, instead of only generic overlap or unsupported/manualRequired target states
  - plain `ChildDevice` clock sources no longer fabricate clock-skew findings in preview or conflict output
  - the WP07 proof bundle now exists for the current schedule/time-budget/conflict slices, and the next live gap is broader downstream parent/runtime consumers rather than missing proof artifacts in the owner seams

- current focus: `policy-control-plane-plan` `WP03-WP04-WP05` lifecycle-ref propagation and downstream handoff slice
- current strategy:
  - keep the lifecycle metadata in the owning `policy_source` / `policy_compiler` seam and propagate it through the existing child-policy, child-notification, child-runtime, and parent-runtime paths instead of inventing another parallel delivery-only truth
  - put the missing coverage into the existing crate-owned `tests/unit`, `tests/version-skew`, and `tests/integration` buckets rather than creating plan-specific test folders
  - fix only the minimum adjacent compile drift needed for the shared child-runtime integration target to execute the real policy-control slice
- current files in this chunk:
  - `crates/policy-control-core/src/policy_source.rs`
  - `crates/policy-control-core/src/policy_compiler.rs`
  - `crates/policy-control-core/tests/unit/policy_source.rs`
  - `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `crates/policy-control-core/tests/unit/policy_preview.rs`
  - `crates/policy-control-core/tests/unit/policy_delivery.rs`
  - `crates/policy-control-core/tests/unit/policy_conflict.rs`
  - `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `crates/policy-control-core/tests/version-skew/policy_source_migration.rs`
  - `crates/policy-control-core/tests/version-skew/policy_preview.rs`
  - `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
  - `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
  - `crates/child-policy-core/tests/unit/policy_control_delivery_handoff.rs`
  - `crates/child-policy-core/tests/unit/tracking_policy.rs`
  - `crates/child-notification-core/Cargo.toml`
  - `crates/child-notification-core/tests/unit/policy_control_notification.rs`
  - `crates/child-runtime/src/tracking_runtime_flow.rs`
  - `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`
  - `crates/child-runtime/tests/integration/policy_control_runtime_flow_intent.rs`
  - `crates/parent-runtime-core/tests/unit/policy_control_dispatch.rs`
  - `crates/parent-runtime-core/tests/unit/policy_control_update_flow.rs`
  - `crates/agent-protocol/src/constants/tracking_runtime.rs`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - the `ParentPolicySourceDocument` lifecycle fields now exist in the real downstream fixtures, so the new source/compiler lifecycle metadata is exercised by actual code and tests rather than being stranded in the owner crate alone
  - `policy-control-core` now preserves audit refs plus supersede/rollback refs through both the older compile helper and the domain compiler seam
  - child-policy delivery handoff, child-notification parent notification, child-runtime policy-control integration, and parent-runtime dispatch/update flows all execute green against the propagated lifecycle metadata
  - the shared child-runtime integration target needed one adjacent eventing metadata fix plus tracking contract-drift alignment before the policy-control integration slice could run, and that supportive work now lands in this checkout
  - the next live policy-control gap has shifted upward from lifecycle propagation into broader parent authoring/preview plus schedule/conflict/time-budget fixture breadth
  - the WP02 proof bundle now exists under `docs/proof/policy-control-plane-plan/02-*.md` and is backed by the focused `policy-control-core` preview run plus the `policy-domain` package test run in this checkout

- current focus: `policy-control-plane-plan` `WP03` compiler support-matrix, delivery-boundary provenance, and TS owner-contract slice
- current strategy:
  - keep compiler support ownership first-class by writing explicit support-matrix and per-rule capability-state data into the Rust compiled artifact instead of deriving that state only from hard-coded target matches
  - keep the first delivery consumer honest by preserving source-side compiler provenance on the queued delivery record instead of pretending queue-time transition metadata is enough
  - add the missing TypeScript owner contract in `policy-domain` without adopting barrel exports or inventing plan-specific test folders
  - keep this pass to core code + tests only; do not broaden into runtime/provider rollout proof or full package validation yet
- current files in this chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_compiler.rs`
  - `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `crates/policy-control-core/tests/version-skew/policy_compiler.rs`
  - `crates/policy-control-core/src/policy_delivery.rs`
  - `crates/policy-control-core/tests/unit/policy_delivery.rs`
  - `crates/policy-control-core/tests/version-skew/policy_delivery.rs`
  - `crates/child-policy-core/tests/unit/policy_control_delivery_handoff.rs`
  - `crates/child-runtime/tests/integration/policy_control_runtime_flow_intent.rs`
  - `packages/policy-domain/src/policy-compiler.ts`
  - `packages/policy-domain/tests/unit/policy-compiler.test.ts`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - `policy-control-core` compiled artifacts now carry explicit `support_matrix` rows plus per-rule `capability_state`, and the compiler owner seam now has a dedicated override-capable support-matrix input instead of leaving support ownership implicit in hard-coded domain/target matches
- `PolicyDeliveryRecord` now preserves `source_audit_reference_ids`, `source_superseded_by_policy_version`, and `source_rollback_ref` separately from delivery-transition audit/state fields, so WP03 source/compiler provenance no longer disappears at the first queue boundary
- WP03 proof artifacts now exist under `docs/proof/policy-control-plane-plan/03-*.md` and are backed by the focused Rust and TS validation already executed in this checkout
  - the Rust unit/version-skew coverage written for this slice now asserts both the new support-matrix/capability-state artifact fields and the queued-delivery source metadata fields, including superseded and rolled-back source-document cases
  - the Rust compiler tests now also prove the domain-override semantics where `enforcement` and `notification-ask-parent` keep supported matrix rows while specific rules still downgrade to `manual-required` with the right reason-code semantics
  - child-policy and child-runtime queue/handoff tests now also prove the queued delivery still exposes the preserved source compiler metadata at the first real downstream consumer seams
  - `packages/policy-domain/src/policy-compiler.ts` now exists as a shared WP03 owner contract for compiler domains, capability states, support matrices, rule statuses, delivery targets, evidence-custody flags, no-claim labels, rollback/supersede refs, and compiled artifact parsing
  - `packages/policy-domain/tests/unit/policy-compiler.test.ts` now gives that TS owner seam real coverage for deterministic parsing, capability-state/status alignment, manual-required/unsupported reason-code enforcement, no-claim-set integrity, support-matrix ownership, and rollback-vs-supersede exclusivity
  - `packages/app-game-domain/src/app-game-policy-target-compiler.ts` and `packages/app-game-domain/src/app-game-policy-target-compiler-rules.ts` now consume the shared `PolicyCompilerCapabilityStateSchema` / `PolicyCompilerCapabilityState` owner contract instead of a local duplicate capability-state enum, and manual-required or unsupported capability refs can no longer parse as `dry-run-ready` compiled output
  - `packages/app-game-domain/tests/unit/app-game-policy-target-compiler.test.ts` and `packages/app-game-domain/tests/unit/app-game-policy-preview-handoff-fixtures.ts` now keep the real app-game unit seam on the shared capability-state vocabulary and prove both manual-required and unsupported capability refs stay explicit instead of silently compiling as ready
  - `packages/tracking-domain/src/tracking-policy-compiler-runtime-proof.ts` now requires a shared `PolicyCompiledArtifact` input at the tracking runtime-proof consumer boundary, and rejects wrong-domain artifacts, source-policy-version mismatches, and missing source-rule coverage before the local tracking runtime-proof seam can treat a free-floating rule as sufficient input
  - `packages/tracking-domain/tests/contract/tracking-policy-compiler-runtime-proof.test.ts` now covers the new shared-artifact happy path plus wrong-domain, source-version-mismatch, and missing-source-rule negatives, and `packages/tracking-domain/package.json` now carries the honest `@ocentra-parent/policy-domain` dependency for that import path
- `packages/browser-domain/src/browser-control-coverage-matrix.ts` now also emits canonical `PolicyCompilerCapabilityState` values alongside the local browser-specific capability-state labels, so the browser compiler support-matrix seam now speaks the shared supported/manual-required/unsupported vocabulary instead of only browser-local coverage wording
- `packages/browser-domain/tests/unit/browser-control-coverage-matrix.test.ts` now proves the browser control coverage matrix keeps direct-control, capability-backed, and documentation-only rows honest while also keeping the shared compiler capability state explicit on every row
- focused browser-domain build/test/lint now pass after correcting the shared policy/capability package exports to the real `dist/src` artifacts and clearing the stale unused browser-domain imports that were left behind by the old boundary shape
- `packages/screen-domain` now builds and passes focused tests after the explicit `ai-domain`, `enforcement-domain`, and `notification-domain` export entries landed and the forbidden ai-runtime barrel export was removed, so the WP03 consumer seam no longer depends on path-map hacks or a full repo-wide validation run
- `packages/browser-domain/src/browser-game-policy-compiler.ts` and `packages/browser-domain/src/social-policy-compiler.ts` now also emit canonical `PolicyCompilerCapabilityState` values on compiled decision candidates, so the browser-domain policy compiler seams now carry the shared supported/manual-required/unsupported vocabulary instead of only local compiler-mode wording
- `packages/browser-domain/tests/unit/browser-game-policy-compiler.test.ts` and `packages/browser-domain/tests/unit/social-policy-compiler.test.ts` now prove those browser policy compiler candidates preserve the shared capability-state contract alongside their existing deterministic compiler-mode and fallback semantics
- `packages/parent-domain/src/browser-game-policy-compiler.ts`, `packages/parent-domain/src/browser-game-policy-compiler-values.ts`, `packages/parent-domain/src/social-policy-compiler.ts`, `packages/parent-domain/src/social-policy-compiler-values.ts`, and `packages/parent-domain/src/tracking-policy-compiler-runtime-proof.ts` now forward the shared compiler entrypoints through local aliases instead of barrel re-exports, so the parent-domain public surface keeps the WP03 contract without violating the no-reexport gate
- `packages/parent-domain/tests/unit/browser-policy-compiler.test.ts` now covers the browser-game and social wrapper value schemas in addition to the compiler candidates, so the parent-facing compiler entrypoints stay wired through the parent package surface
- the WP03 route/proof closure is complete in this checkout

- current focus: `tracking-plan` `WP32` portal active-summary consumer slice
- current strategy:
  - stay inside one workpack and one consumer seam: `trackingReadModel` additive fields -> `portal-domain` summary/coverage mapping -> existing `apps/portal` tracking test
  - keep the proof/test ownership in the existing real consumer test instead of creating plan-specific folders or fake harnesses
  - use the additive active-summary fields when present, but preserve a legacy fallback path so older service payloads still render
  - keep tombstone coverage in the service-data card and stop deleted-history refs from leaking back into the narrow live summary
- current files in this chunk:
  - `packages/portal-domain/src/tracking-status-panel.ts`
  - `apps/portal/tests/tracking-status-panel.test.ts`
  - `docs/plans/currentstatus.md`
  - `docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md`
- current result:
  - the portal tracking live summary now prefers `latestActiveEventId` and `latestActiveObservedAt` instead of recomputing from the tombstone-latest surface
  - the portal tracking service-data coverage card now consumes the additive active kind/device/capability count buckets when present and falls back to legacy row-derived values when they are absent
  - active evidence refs now stay on the live summary/coverage side, while tombstone deleted-evidence refs remain isolated to the tombstone/citation coverage path
  - the focused `apps/portal` tracking status test fixture now proves the newer tombstone-latest payload shape plus the legacy additive-field-absent fallback in real assertions, and the shared `portal-domain` owner now has a direct `tests/unit/tracking-status-panel.test.ts` execution path for the same slice
  - focused app-test execution is still blocked before test execution by the same pre-existing `@ocentra-parent/social-domain/social-alert-report-intent` workspace resolution failure through `agent-protocol-domain`, but the scoped architecture gate passed for the touched files

- focus: `tracking-plan` `WP26` tracking alert/notification read-model slice
- strategy used:
  - stay inside one workpack and one boundary seam: protocol activity kind -> tracking read-model selector -> service-backed read-model tests
  - keep the new checks in the existing crate-owned `tests/unit` and `tests/contract` buckets instead of creating plan-specific folders
  - add only the smallest missing live-projection slice for alerts/notifications instead of expanding into provider delivery or portal preference UI
  - remove the new `parent-runtime-core` root re-export surface touched by this slice so the runtime path stays aligned with the repo’s no-reexport rule
- files actively moved in the last chunk:
  - actual files in this chunk: `crates/agent-protocol/src/activity.rs`, `crates/agent-protocol/src/activity_conversions.rs`, `crates/agent-protocol/src/constants/activity_event_kind.rs`, `crates/agent-protocol/src/constants/sqlite.rs`, `crates/agent-protocol/src/activity_tests.rs`, `crates/agent-protocol/tests/contract/tracking_read_model.rs`, `crates/tracking-core/src/read_model_rows.rs`, `crates/tracking-core/tests/unit.rs`, `crates/tracking-core/tests/unit/read_model.rs`, `crates/agent-service/src/tracking_read_model_service_tests.rs`, `docs/plans/currentstatus.md`
  - `crates/agent-protocol/src/constants/tracking_runtime.rs`
  - `crates/agent-protocol/src/tracking/runtime_event.rs`
  - `crates/agent-protocol/tests/contract/tracking_retention_settings_write_command.rs`
  - `crates/parent-runtime-core/src/tracking_dispatch.rs`
  - `crates/parent-runtime-core/src/tracking_child_check_in_request_flow.rs`
  - `crates/parent-runtime-core/src/lib.rs`
  - `crates/parent-runtime-core/src/tracking_config_update_flow.rs`
  - `crates/parent-runtime-core/src/policy_control_update_flow.rs`
  - `crates/parent-runtime-core/tests/unit.rs`
  - `crates/parent-runtime-core/tests/unit/runtime_dispatch.rs`
  - `crates/parent-runtime-core/tests/unit/tracking_child_check_in_request_flow.rs`
  - `crates/parent-runtime-core/tests/unit/tracking_config_update_flow.rs`
  - `crates/parent-runtime-core/tests/unit/policy_control_dispatch.rs`
  - `crates/parent-runtime-core/tests/unit/policy_control_update_flow.rs`
  - `crates/child-runtime/src/tracking_runtime_flow.rs`
  - `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`
  - `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`
  - `docs/plans/currentstatus.md`
- result of last chunk:
  - actual result of this chunk: the tracking activity protocol now names `activity.tracking.alert.evaluated` and `activity.tracking.parent-notification.requested`, the tracking read-model selector now includes those activity rows, and the tracking/service contract tests now cover that live projection instead of leaving WP26 alert/notification rows stranded outside the service-backed read model
  - `tracking-core` now has crate-owned unit coverage proving alert and parent-notification rows appear in the tracking projection while non-tracking rows remain excluded
  - `agent-protocol` now has explicit protocol coverage for the new tracking activity kinds and their read-model row serialization
  - `agent-service` now seeds and expects the alert/parent-notification rows in the service-backed tracking read model counts/latest-active event surface
  - touched-file validation for this slice should be recorded separately from the older `policy-control-core` blocker that still affects focused `tracking-core` execution
  - follow-on WP30 portal slice: `packages/portal-domain/src/tracking-notification-parent-surface-hosted-ui-proof.ts` now maps notification parent-surface rows from structured read-model input, and `packages/portal-domain/tests/unit/tracking-notification-parent-surface-hosted-ui-proof.test.ts` now covers row mapping, the manual-action quiet-hours branch, and the invalid-input fallback
  - focused package-local validation passed for that portal slice, while `apps/portal/tests/tracking-status-panel.test.ts` is still blocked before execution by the pre-existing `@ocentra-parent/social-domain/social-alert-report-intent` workspace resolution failure through `agent-protocol-domain`
  - follow-on WP16 schedule-engine slice: `packages/tracking-domain` now owns explicit holiday-mode/trip-exception schedule suppression with surfaced exception state/audit refs, `crates/tracking-core` now mirrors the same suppression with dedicated reason codes, `cmd /c npx vitest run tests/unit/tracking.test.ts` passed, and the focused Rust command is still blocked before `tracking-core` runs by the unchanged `policy-control-core` import failure

- focus: `tracking-plan` `WP16` decision citation and boundary slice
- strategy used:
  - stay inside the existing `packages/tracking-domain` owner seam instead of opening a new plan-wide runtime front
  - make the decision contract carry schedule-rule/tolerance/grace citations directly so downstream read-model/UI consumers do not have to reconstruct them from adjacent schedule inputs
  - add the missing low-accuracy ambiguous and DST-spanning window tests in the real `tests/unit` bucket rather than creating one-off plan folders
- files actively moved in this chunk:
  - `packages/tracking-domain/src/tracking-geofence.ts`
  - `packages/tracking-domain/src/tracking-runtime.ts`
  - `packages/tracking-domain/tests/unit/tracking.test.ts`
  - `packages/tracking-domain/tests/unit/tracking-fixtures.ts`
  - `docs/plans/tracking-plan/workpacks/16-expected-place-schedule-engine.md`
  - `docs/plans/currentstatus.md`
- result of this chunk:
  - `TrackingExpectedPlaceScheduleSchema` and `TrackingExpectedPlaceDecisionSchema` now keep optional `ruleId` and `distanceToleranceMeters` plus decision-level `lateGraceSeconds` and `earlyExitGraceSeconds`, and the runtime helper preserves those citations on evaluated expected-place decisions
  - `packages/tracking-domain/tests/unit/tracking.test.ts` now proves those citations survive evaluation, low-accuracy geofence input stays `unknown`/ambiguous instead of drifting into a late-arrival or exit accusation, and a DST-spanning encoded UTC window still evaluates inside the active expected-place window
  - focused validation passed for the touched TS owner seam with `cmd /c npx vitest run tests/unit/tracking.test.ts` and `cmd /c npm run lint:architecture -- packages/tracking-domain/src/tracking-geofence.ts packages/tracking-domain/src/tracking-runtime.ts packages/tracking-domain/tests/unit/tracking.test.ts packages/tracking-domain/tests/unit/tracking-fixtures.ts`

- focus: `tracking-plan` `WP16` Rust event citation parity slice
- strategy used:
  - keep the existing Rust event identity fields and extend them with the missing tolerance/grace/exception citations instead of introducing a second parallel rule wrapper
  - prove the event JSON shape in both the protocol owner crate and the tracking-core contract bucket
  - accept the unchanged `policy-control-core` compile failure as an external blocker for focused `tracking-core` execution, but still land code/tests that compile through the independent protocol crate
- files actively moved in this chunk:
  - `crates/agent-protocol/src/constants/tracking_runtime.rs`
  - `crates/agent-protocol/src/tracking/runtime_event.rs`
  - `crates/agent-protocol/tests/contract.rs`
  - `crates/agent-protocol/tests/contract/tracking_expected_place_state_evaluated_event.rs`
  - `crates/tracking-core/src/expected_place.rs`
  - `crates/tracking-core/tests/unit/expected_place.rs`
  - `crates/tracking-core/tests/contract/runtime_events.rs`
  - `docs/plans/tracking-plan/workpacks/16-expected-place-schedule-engine.md`
  - `docs/plans/currentstatus.md`
- result of this chunk:
  - `TrackingExpectedPlaceStateEvaluatedEvent` now keeps `distanceToleranceMeters`, `lateGraceSeconds`, `earlyExitGraceSeconds`, and `exceptionState` alongside the existing `scheduleId` and `expectedPlaceRef`, and the tracking-core event builder populates those citations from the expected-place evaluation
  - `crates/agent-protocol/tests/contract/tracking_expected_place_state_evaluated_event.rs` now proves the expected-place event contract/idempotency path plus the serialized grace/tolerance/exception citation payload
  - `crates/tracking-core/tests/unit/expected_place.rs` and `crates/tracking-core/tests/contract/runtime_events.rs` now assert the new Rust-side citation fields, and those focused `tracking-core` cargo executions now run green after the `policy-control-core` import fix

- focus: `policy-control-plane-plan` `WP01` source-of-truth supersede/versioning slice
- strategy used:
  - stay inside the owning `policy_source` seam instead of leaking source lifecycle state into delivery-only helpers or portal callers
  - mirror the delivery seam's "replacement policy version must be newer" invariant at the source-of-truth boundary so `superseded` becomes a real typed transition instead of only a status enum label
  - require a fresh supersede audit ref on the source record so the lifecycle doc's "audit trail missing for supersede" failure stays enforced in code and tests
- files actively moved in the last chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_source.rs`
  - `crates/policy-control-core/tests/unit/policy_source.rs`
  - `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - `policy-control-core` now has an explicit `supersede_parent_policy_source_document(...)` helper, so a source document cannot move to `superseded` unless the replacement policy version is strictly newer and the supersede event carries a new audit ref
  - `crates/policy-control-core/tests/unit/policy_source.rs` now covers the same-version rejection, duplicate-audit-ref rejection, and successful supersede path in the existing real unit bucket
  - `crates/policy-control-core/tests/version-skew/policy_source.rs` now proves the source supersede helper rejects non-newer replacement versions as part of the WP01 versioning boundary
  - `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_source.rs crates/policy-control-core/tests/unit/policy_source.rs crates/policy-control-core/tests/version-skew/policy_source.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_source -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew policy_source -- --test-threads=1` all pass for this slice
  - the policy-control plane closeout is complete in this checkout, so the remaining coordination is PR/handoff rather than further WP01/WP07/WP02 implementation

- focus: `policy-control-plane-plan` `WP01` source-of-truth rollback-ref slice
- strategy used:
  - stay inside the same `policy_source` seam and add the missing rollback artifact there instead of pretending delivery-only rollback state is enough for source truth
  - make rollback carry an explicit prior-version reference so the lifecycle negative "rollback loses prior version reference" is enforced in code rather than left to proof prose
  - keep the new lifecycle and authz negatives in the existing real `policy_source` unit/version-skew buckets instead of creating plan-specific test folders
- files actively moved in the last chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_source.rs`
  - `crates/policy-control-core/tests/unit/policy_source.rs`
  - `crates/policy-control-core/tests/version-skew/policy_source.rs`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - `policy-control-core` now has a typed `PolicyRollbackRef` plus `rollback_parent_policy_source_document(...)`, so a source document cannot move to `rolledBack` unless it points at the rolled-back document/version, cites an older restored version, names a distinct restored document, and records a fresh audit ref
  - `crates/policy-control-core/tests/unit/policy_source.rs` now covers mismatched-actor and mismatched-role authority rejection plus the rollback prior-version/audit-ref negatives and successful rollback path
  - `crates/policy-control-core/tests/version-skew/policy_source.rs` now proves the rollback helper rejects non-older restored policy versions as part of the WP01 lifecycle boundary
  - `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_source.rs crates/policy-control-core/tests/unit/policy_source.rs crates/policy-control-core/tests/version-skew/policy_source.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_source -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew policy_source -- --test-threads=1` all pass for this slice
  - the policy-control plane closeout is complete in this checkout, so the remaining coordination is PR/handoff rather than further WP01/WP07/WP02 implementation

- focus: `policy-control-plane-plan` `WP03` pre-confirmation compiler gate
- strategy used:
  - stay inside the owning `policy_compiler` seam instead of pushing compile-candidate rules into portal callers or delivery code
  - enforce the lifecycle rule that confirmed source policy must exist before compile, but keep the change narrow by rejecting only `draft` and `preview` source states first
  - keep the proof in the existing real compiler unit bucket so the compiler contract gains a concrete negative without creating another plan-specific test tree
- files actively moved in the last chunk:
  - `crates/agent-protocol/src/constants/policy_control.rs`
  - `crates/policy-control-core/src/policy_compiler.rs`
  - `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `docs/plans/currentstatus.md`
  - `docs/plans/policy-control-plane-plan/PLAN_STATE.md`
- current result:
  - `policy-control-core` now rejects `draft` and `preview` source documents before domain compilation, so compiler artifacts cannot be generated from pre-confirmation portal state
  - `crates/policy-control-core/tests/unit/policy_compiler.rs` now proves the new compile gate while keeping the existing domain-fixture coverage green
  - `cmd /c npm run lint:architecture -- crates/agent-protocol/src/constants/policy_control.rs crates/policy-control-core/src/policy_compiler.rs crates/policy-control-core/tests/unit/policy_compiler.rs`, `cargo test -p ocentra-policy-control-core --test unit policy_compiler -- --test-threads=1`, and `cargo test -p ocentra-policy-control-core --test version_skew policy_compiler -- --test-threads=1` all pass for this slice
  - the policy-control plane closeout is complete in this checkout, so the remaining coordination is PR/handoff rather than further WP03/WP07/WP02 implementation

Everything else above remains a hand-written inspection matrix for current ownership and current gap shape, not an execution proof.

## Next Write Order

1. `policy-control-plane-plan` is closed; hand off to the coordinator for the next assigned plan.
