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

- `docs/proof/device-trust-bootstrap-plan/09-*`

## Negative cases

- No stale route claims.
- No proof stored inside this plan folder.
- No adjacent plan claims ownership of device-trust bootstrap slices that belong here.