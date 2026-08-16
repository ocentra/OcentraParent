# 28 Live View Optional Mode

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `28 Live View Optional Mode`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns local live-view preflight, platform prompt proof requirements, no-frame-retention contract, local loopback/session/runtime proof, relay-cache harness boundary, worker startup gate, and product-blocked status.
remote-access-plan owns relay-backed remote live-access/session authority, standing grants, and remote product proof.
data-custody-storage-plan owns retention/export/delete/privacy policy for frames/artifacts.
privacy/legal review is external unless a signed-off artifact is provided.
```

## Target State

Separate live view mode exists only with transport, relay/cache, viewer audit, retention, platform proof, and remote access boundary.

## Current State

Live view is a separate planned/gap feature and not part of default local summary evidence.
`ScreenEvidenceRemoteBoundarySettingSchema` now records the current default as `liveViewMode: disabled`, and the retention proof rejects relay/live-view mode values from the local screen-summary boundary.
`ScreenLiveViewOptInSettingSchema` and `scripts/test/screen-optional-retention-live-preflight-proof.mjs` now define and prove the separate optional live-view preflight contract for disabled, LAN-only, and relay-backed modes. The contract requires explicit parent approval, viewer audit, platform proof ref, LAN mutual-auth or relay end-to-end-encrypted transport label, custody label, no frame retention, no session recording, and no remote input control. This is contract/preflight proof only; real live transport, relay/cache execution, platform permission prompts, service persistence, parent UI, and privacy/legal approval remain separate gates.
Android MediaProjection proof records explicit OS capture consent for the Android child-agent capture adapter, but it is not live-view permission-prompt proof and does not close this workpack's live-view runtime gate.
`ScreenLiveViewPlatformPermissionGateSchema` and `scripts/test/screen-live-view-platform-permission-proof.mjs` now add a fail-closed platform-permission gate. The proof consumes the existing real Android MediaProjection capture-consent artifact and records it as `screen-capture-only`, proving it cannot mark live view product-ready without a live-view permission prompt proof, viewer audit, live transport proof, no frame retention, and no remote input. This closes the missing gate-artifact slot but does not implement or claim real live transport, relay/cache execution, service live-view sessions, platform screenshots, or privacy/legal approval.
`ScreenLiveViewParentUiPersistenceProofSchema` and `scripts/test/screen-live-view-parent-ui-persistence-proof.mjs` now prove parent Settings command/readiness evidence can be carried as persisted live-view opt-in state into the service-session and Rust runtime decision proofs while product live view stays false. This closes parent UI persistence as a proof input only; production worker startup, real live-view prompt screenshots, relay/cache execution, physical-device parity, and privacy/legal approval remain open.
`scripts/test/screen-live-view-worker-startup-proof.mjs` now proves the Rust service worker startup gate exists behind the runtime decision boundary and stays stopped unless runtime readiness, a real live-view prompt artifact, relay/cache execution when needed, physical-device parity, and privacy/legal approval are all proved. This closes the worker-startup gate artifact only; actual production worker start, real platform prompt screenshots, relay/cache execution, physical-device parity, and privacy/legal approval remain open.
`scripts/test/screen-live-view-session-transport-proof.mjs`, `scripts/test/screen-live-view-service-session-proof.mjs`, and `scripts/test/screen-live-view-runtime-proof.mjs` also prove the local loopback transport/session/runtime boundaries: a real local capture artifact is queued, transported through a LAN mutual-auth loopback proof with viewer audit, raw frame cache/session recording/remote input stay false, and the raw temp frame is deleted. These proofs close the local transport/runtime evidence bundle, but they do not satisfy platform live-view permission-prompt screenshots, relay/cache execution, physical-device parity, or privacy/legal approval.
`scripts/test/screen-live-view-relay-cache-proof.mjs` now proves the relay-backed transport/cache item with a real captured frame: it writes an end-to-end encrypted relay envelope to an ephemeral local relay cache, verifies the frame digest after parent-side decryption, then deletes both the relay cache and raw temp frame. This closes relay/cache execution proof as a local forced relay/cache harness only; real platform prompt screenshots, physical-device parity, privacy/legal approval, hosted relay infrastructure, and product live view remain open.

## Required proof fields

The selected proof must name, at minimum:

```text
product_decision_state
mode_state
parent_approval_state
viewer_audit_state
platform_prompt_state
platform_proof_ref_state
lan_transport_state
relay_transport_state
relay_cache_state
frame_retention_state
session_recording_state
remote_input_state
parent_ui_persistence_state
service_session_state
runtime_decision_state
worker_startup_state
local_loopback_state
physical_device_parity_state
privacy_legal_state
remote_access_boundary_state
product_live_view_state
no_remote_access_claim
no_product_ready_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Record product decision.
- [ ] Define LAN-only vs relay-backed mode.
- [ ] Define consent/disclosure.
- [ ] Define transport/custody.
- [ ] Define retention/no-retention behavior.
- [ ] Define viewer audit.
- [ ] Add separate contract/preflight proof.
- [ ] Add fail-closed platform permission gate proof.
- [ ] Add parent UI persistence carry-forward proof.
- [ ] Add fail-closed Rust service worker startup gate proof.
- [ ] Add local loopback live-view transport/session/runtime proof.
- [ ] Add real live-view platform prompt proof.
- [ ] Add relay/cache execution proof for relay-backed mode.
- [ ] Add physical-device parity and privacy/legal approval proof.

## Proof

- Updated live-view feature doc.
- Tests proving local-summary opt-in does not enable live view.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
- `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`.
- `output/screen-plan-proof/live-view-platform-permission/proof-summary.json`.
- `output/screen-plan-proof/live-view-session-transport/proof-summary.json`.
- `output/screen-plan-proof/live-view-service-session/proof-summary.json`.
- `output/screen-plan-proof/live-view-runtime/proof-summary.json`.
- `output/screen-plan-proof/live-view-parent-ui-persistence/proof-summary.json`.
- `output/screen-plan-proof/live-view-worker-startup/proof-summary.json`.
- `output/screen-plan-proof/live-view-relay-cache/proof-summary.json`.
- Capture-adapter platform consent reference: `output/screen-plan-proof/android-mediaprojection/proof-summary.json`.

## Failure conditions

- Do not claim product live-view readiness without real prompt screenshots, physical parity, privacy/legal approval, and production worker/start proof.
- Do not claim remote-access readiness from screen live-view proof.
- Do not allow frame retention, session recording, or remote input unless the selected proof explicitly opens and proves that future scope.
- Do not treat local loopback or relay-cache harness proof as hosted relay product proof.
