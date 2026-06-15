# Workpack 06 - Parent Local-Service Route Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `06-parent-local-service-route-bridge`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the typed route bridge between parent client launch/readiness and local-service state.

## Must prove

- bridge inputs and outputs use the canonical contract shape
- route state stays separate from setup ownership
- service reachability and degradation are explicit
- the bridge does not absorb child runtime distribution claims

## Failure conditions

- bridge state becomes a setup completion claim
- route bridge and package claims are merged
- missing service state is reported as healthy
