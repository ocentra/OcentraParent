# Parent Web Portal Distribution

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_WEB_PORTAL_DISTRIBUTION.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

The parent web portal is a distribution target, not just a page. It must prove:

- the portal builds from the real workspace;
- the route surface is parent-only;
- auth and cache boundaries do not leak child data;
- environment separation is explicit for preview, staging, and production;
- UI states distinguish download, launch, and manual-required handoff states.

The hosted portal route is parent-client only and does not claim child-agent execution or setup completion.

Rust-first ownership note:

- Rust owns contracts and runtime truth where parent-client distribution state crosses runtime boundaries.
- TypeScript in `apps/portal` stays presentation-only for the hosted parent web portal surface; it must not become the source of setup, child-runtime, or production-readiness truth.

## Boundary

- Owns the hosted parent portal surface.
- Does not own child runtime distribution, pairing protocol internals, or setup journey logic.
- Does not own desktop/mobile package readiness, route-bridge runtime authority, or production publishing truth.

## Validation anchors

- `npm run build --workspace @ocentra-parent/portal`
- `npm run test --workspace @ocentra-parent/portal`
- `npm run test:e2e --workspace @ocentra-parent/portal`

## Negative cases that must exist

- wrong route rejects or redirects without leaking child state
- stale cache does not present a fresh install claim
- missing auth does not expose parent-only distribution controls
- preview env does not claim production release status
