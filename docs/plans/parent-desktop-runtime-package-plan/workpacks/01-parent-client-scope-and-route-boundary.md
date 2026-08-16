# Workpack 01 - Parent Client Scope and Route Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `01-parent-client-scope-and-route-boundary`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: lock the canonical scope, the setup handoff boundary, and the compatibility note for the historical folder path.

## Owns

- canonical scope correction
- parent/client route bridge separation from setup
- compatibility note for `docs/plans/parent-desktop-runtime-package-plan/`
- no-claim boundaries between parent client artifacts, setup, child runtime, device trust, and portal UX

## Ownership boundary

```text
parent-client-runtime-distribution-plan owns parent web, desktop, Android parent, and iOS parent distribution proof.
setup-install-provisioning-plan owns setup journey and readiness state.
child-agent-runtime-distribution-plan owns child package/runtime distribution.
device-trust-bootstrap-plan owns trusted-device bootstrap and local sealed trust.
portal-ux-household-surfaces-plan owns generic household shell UX.
```

## Must prove

- the plan owns parent web, desktop, Android, and iOS distribution only
- child-agent runtime distribution is separate
- setup is a handoff, not package proof
- the route bridge contract names the real input/output state
- the historical folder path is compatibility-only and the canonical plan scope is parent client runtime distribution

## Required proof fields

The selected proof must name, at minimum:

```text
canonical_plan_name
historical_folder_path
parent_client_artifact_scope
setup_handoff_state
child_runtime_boundary_state
device_trust_boundary_state
portal_ux_boundary_state
route_bridge_contract_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Failure conditions

- child runtime leaks into parent scope
- setup completion is claimed from package metadata alone
- manual-required gaps are hidden
- portal shell UX is used as distribution proof
- route bridge state is used as child-agent authority

## Completion

- Proof root: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/`
- Status: complete. Proof artifacts, checklist rows, and plan/index state are now aligned.
- Focused validation: route/proof boundary content checks passed and `git diff --check` passed on the touched WP01 route/proof files.
- Notes: canonical parent-client scope, Rust-first route-bridge ownership, setup-install handoff boundary, portal shell UX handoff, and compatibility-only historical folder path are documented in the plan-local route docs and proof root.
