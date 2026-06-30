# Workpack 01 - Child Agent Scope and Route Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `01-child-agent-scope-and-route-boundary`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: lock the canonical child-agent scope, the setup-device-trust handoff boundary, and the compatibility note for the historical parent-client path.

## Owns

- canonical child-agent scope correction
- child package route separation from parent-client distribution
- setup-device-trust handoff contract
- compatibility note for the historical parent-client folder path
- package/runtime/setup/trust no-claim boundary for this plan

## Ownership boundary

```text
crates/schema or the owning Rust crate owns shared child package/runtime/setup-trust-handoff shapes. `schema-domain` is temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
child-runtime-domain owns package-boundary metadata/helpers only.
this plan owns child distribution proof routing and selected artifact/platform proof expectations.
parent-client-runtime-distribution-plan owns parent client packages.
setup-install-provisioning-plan owns setup journey flow.
device-trust-bootstrap-plan owns trusted-device bootstrap and sealed/local trust material.
child local-service/runtime owners own runtime behavior; this plan packages and proves distribution boundaries.
```

## Allowed inputs

```text
PLAN_INDEX / FEATURE_ROUTE_INDEX route entries
package scripts under scripts/release when package proof is selected
child-runtime-domain package metadata/helper public exports
Rust-owned child package/runtime/setup-trust-handoff contracts plus generated DTOs or temporary edge decoders
selected setup/device-trust handoff references
selected proof roots under output/child-agent-runtime-distribution-plan-proof/
```

## Forbidden scope upgrades

```text
parent-client proof closing child distribution rows
setup UI completion closing package or runtime rows
package build/checksum proof closing install/runtime/respawn/uninstall rows
Android debug APK proof closing device-owner or managed-profile rows
iOS simulator/provisioning proof closing background-service or supervision parity rows
platform scaffold/manual-required rows being counted as READY
```

## Must prove

- the plan owns child Windows, macOS, Linux, Android, and iOS distribution only
- parent-client distribution stays in `parent-client-runtime-distribution-plan`
- setup-device-trust is a handoff, not package proof
- the route bridge names the real input and output state
- canonical shared shapes route through `crates/schema` or another neutral Rust-owned boundary; use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete
- `child-runtime-domain` is metadata/helper scope, not the shared-contract source of truth
- proof pointers stay outside the plan folder
- no-claim text distinguishes package build, install, runtime health, respawn, uninstall/revocation, setup trust, and release readiness

## Execution truth

- WP01 closed as a docs/proof route-correction slice; no runtime code or shared-contract source change was required.
- Canonical shared ownership stays Rust-first: `crates/schema` or another neutral Rust-owned crate owns shared child package/runtime/setup-trust-handoff shapes when they cross package, crate, app, or plan boundaries.
- `schema-domain` remains temporary generated-validation or edge-decoder scope only where TypeScript still consumes Rust-owned contracts during migration.
- `child-runtime-domain` remains metadata/helper scope only and does not become the shared-contract source of truth.
- The setup-device-trust path remains a handoff boundary into child distribution rather than package/install/runtime proof.
- The historical parent-client folder path remains compatibility-only: `docs/plans/parent-desktop-runtime-package-plan/`. Canonical parent distribution ownership stays in `parent-client-runtime-distribution-plan`.
- Proof for this scope correction is stored only under `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/`.

## Required proof files

```text
output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/00-scope-summary.md
output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/01-negative-case-proof.md
output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/02-no-claim-boundary.md
output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/16-validation-commands.log
```

## Failure conditions

- child runtime leaks into parent-client scope
- setup completion is claimed from package metadata alone
- manual-required platform gaps are hidden
- proof is stored in the plan folder instead of the designated artifact path
- shared handoff shapes are duplicated in a sibling feature owner instead of shared through the canonical boundary
- package-script, checksum, or debug artifact proof is used as broad package/runtime readiness
