# Parent Desktop Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_DESKTOP_DISTRIBUTION.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

The parent desktop shell/package proves the desktop distribution boundary, not full product readiness.

## Boundary

- Owns the desktop shell, package preview, local-service bridge, launch smoke, update boundary, and signing claims for the parent desktop artifact.
- Does not own child-agent runtime distribution, setup journey ownership, policy behavior, or billing behavior.

## Validation anchors

- `npm run dev:desktop`
- `npm run dev:desktop:lan`
- `npm run test:parent-desktop-release-support-proof`

## Negative cases that must exist

- launch with no local service proves the shell degrades honestly
- stale service route does not claim controller authority
- unsigned preview does not claim store or notarization status
- launch smoke does not imply parent readiness or child runtime parity
