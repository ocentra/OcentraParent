<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: package readiness, signing readiness, store readiness, setup readiness, release readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Workpack Families

Use this file to classify a selected workpack before opening source. The historical folder path is `parent-desktop-runtime-package-plan`, but the canonical scope is parent client runtime distribution across web, desktop, Android parent, and iOS parent artifacts.

## Scope and route boundary family

```text
Workpacks:
WP01 Parent Client Scope And Route Boundary

Owners:
parent-client-runtime-distribution-plan route docs
setup-install-provisioning-plan for setup journey/readiness
child-agent-runtime-distribution-plan for child packages/runtime
portal-ux-household-surfaces-plan for generic household shell UX

Rule:
Scope proof must separate parent client artifacts from setup journey, device trust, portal shell UX, and child runtime distribution. Route docs are not package proof.
```

## Parent web distribution family

```text
Workpacks:
WP02 Parent Web Portal Distribution

Owners:
apps/portal for web build/source surface
portal-domain for public portal contracts/projections when selected
Cloudflare control-plane only when hosting/deploy path is selected

Rule:
Web build proof must name build output, route, auth/cache/env separation, hosting/deploy state, and no child-agent execution claim. Web build is not production account portal readiness.
```

## Parent desktop shell and package family

```text
Workpacks:
WP03 Parent Desktop Shell Package

Owners:
scripts/dev for dev:desktop launch proof
scripts/release for selected desktop package artifacts
apps/portal for the embedded parent surface when selected
local-service route owner only through typed handoff

Rule:
Launch smoke proves launch/degrade state only. It is not desktop product readiness, signing readiness, update readiness, or mobile parity.
```

## Parent Android package family

```text
Workpacks:
WP04 Parent Android Package

Owners:
scripts/release/parent-android and selected Android packaging proof helpers
setup/device-trust owners through explicit handoff only

Rule:
Android scaffold or source artifact proof is not parent Android support. Device, signing, store, service bridge, and manual-required states must stay explicit.
```

## Parent iOS package family

```text
Workpacks:
WP05 Parent iOS Package

Owners:
scripts/release/parent-ios and selected iOS packaging proof helpers
setup/device-trust owners through explicit handoff only

Rule:
iOS simulator/scaffold proof is not App Store/TestFlight readiness. Signing, provisioning, device, store, and manual-required states must stay explicit.
```

## Parent local-service route bridge family

```text
Workpacks:
WP06 Parent Local Service Route Bridge

Owners:
parent-client-runtime-distribution-plan for parent client route bridge contract
agent-service/local-service owners only through selected public route proof
setup-install-provisioning-plan for setup journey and readiness state
child-agent-runtime-distribution-plan for child runtime execution authority

Rule:
Route bridge proof is not setup completion and not child-agent execution authority. It must name input/output state, degraded/offline state, and no-claim boundaries.
```

## Signing, store, and notarization matrix family

```text
Workpacks:
WP07 Parent Client Signing Store Matrix

Owners:
selected artifact package owner per platform
platform store/signing documentation only when the selected artifact names it

Rule:
Signing/store/notarization proof is per artifact and per platform. Preview output, scaffold output, or CI success cannot imply signing or store readiness.
```

## Update and rollback family

```text
Workpacks:
WP08 Parent Client Update Rollback

Owners:
selected artifact package owner per platform
release/update helpers when selected

Rule:
Update proof must include channel, version, checksum, SBOM, rollback or teardown path, stale update rejection, and manual-required states. Update channel without rollback/checksum/SBOM proof is blocked.
```

## Launch smoke matrix family

```text
Workpacks:
WP09 Parent Client Launch Smoke Matrix

Owners:
selected parent client artifact owner per platform
manual platform proof owner when the host cannot run the artifact

Rule:
Launch smoke proves only launch/degrade behavior for that artifact/platform. It is not product readiness, setup completion, signing readiness, or store readiness.
```

## Setup handoff family

```text
Workpacks:
WP10 Setup Handoff Contracts

Owners:
parent-client-runtime-distribution-plan for package-to-setup handoff contract
setup-install-provisioning-plan for setup journey, install state, and readiness
account/device-trust plans for account/trust prerequisites

Rule:
Setup handoff proof names request/response state only. Package creation is not setup completion.
```

## Proof, CI, and release gate family

```text
Workpacks:
WP11 Proof CI Release Gate

Owners:
selected proof roots under output/parent-client-runtime-distribution-plan-proof/<workpack>/
PLAN_STATE, WORKPACK_INDEX, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, and release docs when status changes

Rule:
Release gate proof may aggregate only accepted roots or exact carried blockers. CI success is not a substitute for artifact proof, negative cases, teardown/rollback, manual-required state, and no-claim boundaries.
```
