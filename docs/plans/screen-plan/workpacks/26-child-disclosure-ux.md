# 26 Child Disclosure UX

## Target State

Child-visible/local disclosure, parent-enabled status, calm wording, and no hidden capture are implemented.

## Current State

`ScreenChildDisclosureUxProofSchema` defines local child-visible disclosure
states for disabled, paused, ready, capture-active, and protected-surface
conditions. Shared text-domain copy renders in the parent Settings route proof
panel, and `scripts/test/screen-child-disclosure-ux-proof.mjs` runs the real
Rust agent plus Vite portal on the B-lane ports to prove the visible copy,
custody labels, local-only status, audit references, and deleted-image
non-retention states.

This is local dev portal proof. It does not claim a production child app, OS
notification, tray integration, foreground overlay, service-persisted child
disclosure state, raw retention enablement, live view, or remote screenshot
upload.

## Checklist

- [x] Define child-visible status.
- [x] Define local disclosure copy.
- [x] Define paused/disabled states.
- [x] Define capture-active state where platform permits.
- [x] Avoid hidden capture.
- [x] Add screenshots/proof.

## Proof

- Contract/tests:
  `packages/activity-domain/tests/screen-child-disclosure-ux.test.ts` and
  `packages/text-domain/tests/screen-child-disclosure-ux-text.test.ts`.
- Executable proof:
  `scripts/test/screen-child-disclosure-ux-proof.mjs`.
- Proof summary:
  `output/screen-plan-proof/26-child-disclosure-ux/proof-summary.json`.
- UI screenshots:
  `output/screen-plan-proof/26-child-disclosure-ux/10-ui-snapshots/screen-child-disclosure-active-card.png`
  and
  `output/screen-plan-proof/26-child-disclosure-ux/10-ui-snapshots/screen-child-disclosure-settings-route.png`.
