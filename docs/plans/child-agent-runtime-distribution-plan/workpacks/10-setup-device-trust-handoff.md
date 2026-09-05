# Workpack 10 - Setup Device Trust Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `10-setup-device-trust-handoff`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: own the child-side setup/device-trust handoff consumption, trusted child startup, authenticated ingress, and external health boundary without taking Setup or Device Trust ownership.

## Owns

- setup-device-trust request and response shape
- typed handoff from setup into child install state
- separation from parent bootstrap and parent-client package proof
- explicit route sync with setup-install-provisioning-plan
- no-claim boundary between setup success, trust bootstrap, package artifact, install state, and runtime readiness
- shipped adapter from Device Trust current-state authority into child startup
- authenticated product ingress and externally reachable child health ownership
- durable handoff delivery/replay state and live updater consumption on the child side

## Ownership boundary

```text
setup-install-provisioning-plan owns setup journey and UI flow.
device-trust-bootstrap-plan owns trusted-device bootstrap, sealed/local trust material, pairing/bootstrap artifacts, and trust revocation semantics.
child-agent-runtime-distribution-plan owns the typed handoff consumed by child package/install distribution proof.
crates/schema or the owning Rust crate owns shared setup-trust-handoff shapes when the shape crosses package, crate, app, or plan boundaries. `schema-domain` is temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
parent-client-runtime-distribution-plan owns parent client artifact state and must not close this handoff.
```

## Handoff contract shape requirements

The handoff proof must name, at minimum:

```text
handoff_id
household_ref
child_profile_ref
target_device_ref
setup_session_ref
trust_bootstrap_ref
child_package_target_ref
platform
artifact_requirement
install_precondition_state
manual_required_state
expiry_or_replay_guard_ref
handoff_status
no_claim
```

These are field requirements for proof routing, not an implementation code prescription.

## Must prove

- the setup handoff is a typed contract, not a loose UI transition
- the request/response names the real setup state and the target package
- parent bootstrap and child pairing codes are not conflated
- setup journey success is not package readiness
- trust bootstrap is not install/runtime readiness
- package artifact proof is not setup or trust proof
- the handoff proof points to the external artifact path
- route sync with setup-install-provisioning-plan and device-trust-bootstrap-plan is named when those plans are touched

## Execution truth

- `crates/schema/src/setup_device_trust_handoff.rs` now owns the canonical request, response, and proof DTO shape for this cross-plan handoff.
- The Rust-owned handoff proof names `handoff_id`, `household_ref`, `child_profile_ref`, `target_device_ref`, `setup_session_ref`, `trust_bootstrap_ref`, `child_package_target_ref`, `platform`, `artifact_requirement`, `install_precondition_state`, `manual_required_state`, `expiry_or_replay_guard_ref`, `handoff_status`, and `no_claim`.
- The proof carries `artifact_requirement.external_artifact_path` as an external artifact pointer into the Windows package proof root and keeps that pointer separate from package/install/runtime readiness claims.
- The proof names route sync with `setup-install-provisioning-plan` and `device-trust-bootstrap-plan` through explicit route-sync rows rather than absorbing those plans' ownership.
- No `schema-domain` edge was added in this workpack because no live TypeScript consumer currently needs this handoff shape; Rust/shared ownership stays canonical and TypeScript remains optional/thin only if a consumer appears later.
- `crates/agent-updater/src/handoff.rs` now owns the package/update consumer projection. It consumes the canonical response together with `UpdateOutcome`, retains typed handoff identity/artifact/platform fields and the response no-claim list, and maps current, dry-run, completed, reboot-required, and failed updater states without collapsing manual-required handoff states.
- The consumer fails closed when `manual_required_state` is set, setup/trust is manual or expired, or a ready/install-precondition response is inconsistent with the required trust-bootstrap state; inconsistent responses remain explicitly rejected/manual rather than becoming update-ready.
- The consumer is an explicit Rust API/composition port; no setup producer, durable delivery/replay owner, platform transport, install callback, or live runtime wiring exists in this slice.
- `consume_setup_device_trust_handoff` has no production caller. The updater CLI invokes update execution without this projection.
- Desktop startup calls `ChildAgentService::initialize()`, and Android JNI constructs `ChildAgentServicePaths::from_root`; both paths omit `ChildAgentTrustBindingSource`, so readiness remains fail-closed/manual-required.
- `ChildAgentIngress` is an in-process queue, not authenticated product ingress. Health is available only through local Rust/Android calls and is not a shipped external endpoint.
- Status is source partial / first runtime source packet. It does not claim setup journey completion, package readiness, install success, service health, respawn, uninstall/revocation parity, transport, or parent-client parity.

## Required production source outcome

- consume Device Trust WP01's current child-device binding at shipped desktop and Android startup without caller-supplied identity strings;
- expose authenticated child command ingress and an external health/readiness boundary tied to the same current binding;
- persist child-side handoff receipt/replay/expiry state and connect it to the updater/installer outcome;
- preserve Setup ownership of the producer/UI journey and avoid a reverse dependency on Setup WP07, which already depends on Child WP10;
- keep trust, artifact, install, runtime health, update, and setup-journey states separate.

Implementation dependency: Device Trust WP01 reviewed implementation. This implementation-only edge may order source work but does not promote normal READY/DONE.

## Expected test-source gap

- current, missing, stale, revoked, wrong-child, wrong-device, and recovered trust binding at startup;
- authenticated ingress acceptance plus unauthenticated/stale/replayed request rejection;
- external health/readiness in ready, manual-required, recovery-pending, tamper, and revoked states;
- durable handoff receipt/replay/expiry and crash/restart recovery;
- live updater callback and installer outcomes without collapsing setup/trust/runtime states;
- graph/route cycle protection: no Child WP10 dependency on Setup WP07.

## Required proof files

```text
output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/00-scope-summary.md
output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/01-negative-case-proof.md
output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/02-no-claim-boundary.md
output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/16-validation-commands.log
```

## Failure conditions

- setup success is claimed from package metadata alone
- parent bootstrap and child pairing codes are merged into one concept
- route sync with setup-install-provisioning-plan is missing
- route sync with device-trust-bootstrap-plan is missing when trust material is claimed
- proof is stored inside the plan folder
- setup-device-trust handoff is used to claim install, service health, respawn, uninstall/revocation, or release readiness without the selected distribution proof
