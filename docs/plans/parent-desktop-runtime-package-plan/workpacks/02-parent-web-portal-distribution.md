# Workpack 02 - Parent Web Portal Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Workpack: `02-parent-web-portal-distribution`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: prove the hosted parent portal as a distribution target with its own build, route, auth, cache, and environment separation.

## Current source truth (2026-08-25)

Status: `REPAIR/BLOCKED` for the production-source phase.

The reachable `apps/portal` and `portal-domain` implementation is a
presentation-only route/query resolver. It does not provide the Rust hosted
distribution owner/source, an immutable asset/version/digest manifest, a
publisher or hosted-artifact owner, a production deployment binding, a real
desktop/service/runtime caller, install/upgrade/rollback lifecycle, or a
fail-closed missing-package API. No source path is invented for those missing
owners here; the current code-map roots remain the existing presentation and
test files only.

This truth packet does not claim implementation completion, test execution,
retained proof, CI, PR readiness, READY, or DONE. Tests, proof, CI, packaging,
deployment, and release remain open and outside this production-source packet.

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
- Status: validation / repair-blocked. The current resolver is presentation-only; the real hosted distribution owner, immutable artifact custody, production binding, runtime caller, package lifecycle, and missing-package failure boundary are absent.
- Focused validation: no tests, proof, CI, packaging, deployment, or release commands are run or changed by this production-source packet.
- No-claim boundary: this workpack does not claim production publishing, immutable package/version/digest custody, setup readiness, desktop/mobile package readiness, desktop/service reachability, install/upgrade/rollback, child runtime authority, READY, or DONE.
