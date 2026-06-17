# Workpack 09: Cross-Plan Route Gate

Purpose: sync adjacent plan routes, feature routes, and proof gates.

## Owns

- Route references in adjacent plan AGENTS files.
- PLAN_INDEX row for this plan.
- FEATURE_ROUTE_INDEX references to this plan.
- Proof pointer conventions outside the plan folder.

## Exit condition

- Adjacent plans reference this plan where appropriate.
- Plan and feature indexes stay aligned.
- Proof locations are pointed to, not stored in the plan folder.

## Proof target

- `output/device-trust-bootstrap-plan-proof/09-*`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Plan-local route tests currently prove document and route alignment only; adjacent plan and feature routes still need truthful proof-backed sync as the runtime workpacks land.

## Negative cases

- No stale route claims.
- No proof stored inside this plan folder.
- No adjacent plan claims ownership of device-trust bootstrap slices that belong here.
