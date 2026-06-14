# Parent Client Route Bridge Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_CLIENT_ROUTE_BRIDGE_MODEL.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

This model keeps the parent client route bridge separate from setup and package claims.

## Contract shape

- `parentInstallPackage(platform, channel, accountRef, householdRef, installCodeRef)`
- `parentClientLaunch(platform, installState)`
- `parentClientReadiness(platform, deviceTrustState, packageState, routeState)`

## Allowed outputs

- `artifactState`
- `packageVersion`
- `channel`
- `signingState`
- `storeState`
- `launchTarget`
- `installProofState`
- `updateState`
- `rollbackState`
- `manualRequiredGaps`

## Rules

- The route bridge may report readiness and handoff state.
- The route bridge may not claim setup completion.
- The route bridge may not claim child runtime distribution.
- The route bridge may not blur parent web, desktop, Android, or iOS artifact states together.

## Negative cases that must exist

- unsupported platform returns manual-required gaps
- stale route state does not become launch readiness
- missing signing/store state does not become production status
