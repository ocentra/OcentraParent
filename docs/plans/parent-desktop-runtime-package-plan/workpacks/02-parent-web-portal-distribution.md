# Workpack 02 - Parent Web Portal Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `02-parent-web-portal-distribution`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the hosted parent portal as a distribution target with its own build, route, auth, cache, and environment separation.

## Must prove

- parent portal build succeeds from the real workspace
- route/auth/cache boundaries are parent-only
- preview/staging/production states are distinguishable
- negative route and stale-cache cases fail honestly

## Failure conditions

- child data leaks through the portal route
- preview state is presented as production release
- portal proof is used to claim desktop or mobile parity

## Completion

- Proof root: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/`
- Status: complete. Real build, hosted route/auth/cache/env proof, checklist rows, and plan/index state are aligned.
- Focused validation: portal build passed, portal vitest passed, and the focused hosted Playwright spec passed after rerouting off a foreign process on default port `4490`.
- No-claim boundary: this workpack does not claim production publishing, setup readiness, desktop/mobile package readiness, or child runtime authority.
