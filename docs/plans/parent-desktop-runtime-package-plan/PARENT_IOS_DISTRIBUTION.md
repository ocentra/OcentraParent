# Parent iOS Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_IOS_DISTRIBUTION.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

The parent iOS package is a parent client artifact with provisioning and store proof separated from setup and child runtime ownership.

## Boundary

- Owns the parent iOS package, device install proof, provisioning state, and store/manual-required state.
- Does not own child iOS runtime distribution, device-owner policy, or setup journey logic.

## Validation anchors

- `npm run release:package:parent-ios`
- `npm run test:parent-mobile-shell-runtime-proof`
- `npm run test:parent-mobile-package-source-artifact-proof`
- `npm run test:parent-mobile-service-bridge`
- `npm run test:parent-mobile-controller-observer-handoff`

## Negative cases that must exist

- scaffold-only package remains manual-required
- missing provisioning does not become store proof
- parent iOS install does not imply child iOS runtime distribution
- package-source proof does not claim privileged background capability
