# Workpack 09: Cross-Plan Route Gate

Purpose: sync adjacent plan routes, feature routes, and proof gates.

## Owns

- Route references in adjacent plan AGENTS files.
- PLAN_INDEX row for this plan.
- FEATURE_ROUTE_INDEX references to this plan.
- Proof pointer conventions outside the plan folder.
- Aggregation of accepted proof roots and exact carried blockers.
- Manual-required gap register and no-claim boundary for device-trust readiness.

## Ownership boundary

```text
WP09 aggregates device-trust-bootstrap-plan proof roots only.
Adjacent plans own their own implementation and may be referenced only by typed handoff proof.
WP09 cannot convert blockers, manual-required rows, document tests, route tests, login state, LAN pairing, package install, or license state into broad readiness.
```

## Required route-gate artifact fields

The route-gate artifact must name, at minimum:

```text
route_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
manual_required_gaps
adjacent_handoff_refs
claims_allowed
claims_blocked
plan_index_sync_state
feature_route_sync_state
adjacent_route_sync_state
proof_pointer_state
platform_constraint_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Exit condition

- Adjacent plans reference this plan where appropriate.
- Plan and feature indexes stay aligned.
- Proof locations are pointed to, not stored in the plan folder.
- Accepted proof roots and carried blockers are explicit.
- Product claims allowed and product claims blocked are explicit.
- Manual-required platform gaps are visible.

## Proof target

- `output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/`

## Required proof files

```text
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/00-scope-summary.md
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/01-negative-case-proof.md
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/02-no-claim-boundary.md
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/03-platform-proof-status.md
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/16-validation-commands.log
output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/17-blockers.md
```

## Current audit state

- No proof root currently exists on disk for this workpack.
- Plan-local route tests currently prove document and route alignment only; adjacent plan and feature routes still need truthful proof-backed sync as the runtime workpacks land.

## Negative cases

- No stale route claims.
- No proof stored inside this plan folder.
- No adjacent plan claims ownership of device-trust bootstrap slices that belong here.
- No PR_READY from document tests or route tests alone.
- No trust readiness from login/session, LAN pairing, package install, license state, or entitlement snapshot presence.
- No adjacent plan completion inferred without typed handoff and no-claim boundary.
