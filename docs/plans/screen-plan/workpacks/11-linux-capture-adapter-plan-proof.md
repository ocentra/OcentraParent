# 11 Linux Capture Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `11 Linux Capture Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

X11 capture, Wayland/PipeWire portal capture, compositor-specific status, and manual-required states are represented honestly.

## Current State

The shared Rust desktop adapter path now targets Linux through a real X11
command backend using `xwininfo`, `xwd`, and ImageMagick. WSLg selected-window
proof exists with captured pixels, encrypted custody, and raw deletion. Native
Wayland/PipeWire portal proof, root-display proof, and compositor parity remain
open.
`ScreenLinuxCaptureCapabilityProofSchema` and
`scripts/test/screen-linux-capture-capability-proof.mjs` now consume the
existing WSLg/X11 proof, verify current XDG Desktop Portal ScreenCast/PipeWire
and X11 source boundaries, and keep native X11 root-display, native Wayland
GNOME/KDE/wlroots, unknown Wayland, unsupported compositor, raw remote upload,
and raw-retention-by-default rows manual-required before Linux product-ready
capture claims.

## Checklist

- [ ] Define X11 path.
- [ ] Define Wayland portal/PipeWire path.
- [ ] Define GNOME/KDE/wlroots states.
- [ ] Define unsupported compositor state.
- [ ] Prove protected/permission-limited states.
- [ ] Prove local OCR and deletion where capture works.

## Proof

- `output/screen-plan-proof/linux/`.
- `output/screen-plan-proof/linux/proof-summary.json`.
- Separate proof per compositor/session.
- `output/screen-plan-proof/linux-wslg/proof-summary.json`.
