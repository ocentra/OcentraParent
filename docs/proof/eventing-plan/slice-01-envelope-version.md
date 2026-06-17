# Slice 01 Envelope Version

## Scope

Envelope schema/version, event identifier newtypes, and the shared TypeScript
contract mirror.

## Evidence

- `cargo test -p ocentra-eventing --test unit`
- `cargo test -p ocentra-eventing --tests`
- `npm run test --workspace @ocentra-parent/event-domain`
- `npm run type-check --workspace @ocentra-parent/event-domain`

## What This Proves

- Reusable Rust envelope/id/version surfaces are still exercised in this
  checkout.
- The shared `@ocentra-parent/event-domain` mirror still passes its focused test
  and type-check surface.
- WP13 cleanup did not regress the reusable crate test harness layout.

## Negative / Not Proved

- This slice alone does not prove the full WP11 hardening surface; see the
  restored WP11 proof roots plus the scoped
  `@ocentra-parent/agent-protocol-domain` package validation for that closure.

## Remaining Gaps

- WP10 remains open.
- No `PR_READY` or full-plan `DONE` claim follows from this slice.
