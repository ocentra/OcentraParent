# WP28 Temporary Live Tracking Mode

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP28 Temporary Live Tracking Mode`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Support parent-approved temporary live tracking with duration, cadence, permission, disclosure, battery, expiry, audit, and retention behavior.

## Central schema boundary

```text
schema-domain owns temporary live mode contracts that cross policy, runtime, portal, event, custody, or proof boundaries.
tracking-core may evaluate TTL/expiry/cadence readiness from canonical inputs.
tracking-domain may provide helper/proof adapters only.
platform/runtime owners own actual live/current/background execution.
```

## Source Inputs

- `docs/device-location-tracking-capability-guide.md`
- `docs/expectations/location-geofence.md`
- `docs/expectations/data-custody.md`

## Target State

Temporary live tracking is time-boxed, capability-gated, visible to parent, safe for disclosure requirements, and auto-expiring.

## Required proof fields

```text
canonical_schema_owner_state
parent_authorization_state
duration_state
cadence_state
max_duration_state
auto_stop_state
battery_degraded_state
permission_degraded_state
audit_state
retention_delete_state
platform_runtime_state
portal_live_map_state
provider_state
production_worker_state
no_product_ready_claim
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/28-temporary-live-tracking-mode/`

- `03-runtime-location-evidence.json`
- `09-policy-alert-proof.json`
- `14-retention-delete-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Require parent authorization and duration.
- [ ] Model cadence, max duration, and auto-stop reason.
- [ ] Preserve battery/permission degraded states.
- [ ] Audit start/update/degrade/stop.
- [ ] Test expiry and deletion/retention behavior.

## Where We Are

This workpack has focused P1 fixture proof from `codex/tracking-temporary-live-mode-proof` under the proof root below. The proof derives active-authorized, battery-degraded, permission-degraded, expired-auto-stopped, retention-delete-ready, and manual-required rows from existing temporary live tracking grants. Runtime, platform, provider, parent portal live-map, physical-device, and production worker behavior is not claimed beyond the proof state recorded in `proof.json` and the implementation checklist.

## Manual-Required Gaps

- Platform, provider, UI, live/current/background runtime, relay, physical-device, and production worker claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-temporary-live-mode-proof`.
- [ ] Validation commands and results: `node scripts/test/tracking-temporary-live-mode-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/28-temporary-live-tracking-mode/` and `test-results/tracking-temporary-live-mode-proof/`.
- [ ] Known gaps/manual-required states: live/current/background location runtime, parent portal live-map runtime, provider delivery, relay runtime, physical-device proof, production temporary-live workers, and production proof remain proof-gated.
