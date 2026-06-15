# Parent Android Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_ANDROID_DISTRIBUTION.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

The parent Android package is a parent client artifact with device and store proof separated from setup and child runtime ownership.

## Boundary

- Owns the parent Android package, device install proof, signer state, and store/manual-required state.
- Does not own child Android runtime distribution, device-owner policy, or setup journey logic.

## Validation anchors

- `npm run release:package:parent-android`
- `npm run test:parent-mobile-shell-runtime-proof`
- `npm run test:parent-mobile-package-source-artifact-proof`
- `npm run test:parent-mobile-service-bridge`
- `npm run test:parent-mobile-controller-observer-handoff`

## Negative cases that must exist

- scaffold-only package remains manual-required
- missing device proof does not become store proof
- parent Android install does not imply child Android runtime distribution
- package-source proof does not claim device-owner policy
