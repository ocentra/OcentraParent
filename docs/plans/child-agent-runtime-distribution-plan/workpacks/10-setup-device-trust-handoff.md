# Workpack 10 - Setup Device Trust Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `10-setup-device-trust-handoff`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the setup-device-trust request/response contract that hands off into child distribution.

## Owns

- setup-device-trust request and response shape
- typed handoff from setup into child install state
- separation from parent bootstrap and parent-client package proof
- explicit route sync with setup-install-provisioning-plan
- no-claim boundary between setup success, trust bootstrap, package artifact, install state, and runtime readiness

## Ownership boundary

```text
setup-install-provisioning-plan owns setup journey and UI flow.
device-trust-bootstrap-plan owns trusted-device bootstrap, sealed/local trust material, pairing/bootstrap artifacts, and trust revocation semantics.
child-agent-runtime-distribution-plan owns the typed handoff consumed by child package/install distribution proof.
schema-domain owns shared setup-trust-handoff shapes when the shape crosses package, crate, app, or plan boundaries.
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
