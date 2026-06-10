# 11 Linux Capture Adapter Plan Proof

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

- [x] Define X11 path.
- [x] Define Wayland portal/PipeWire path.
- [x] Define GNOME/KDE/wlroots states.
- [x] Define unsupported compositor state.
- [x] Prove protected/permission-limited states.
- [~] Prove local OCR and deletion where capture works.

## Proof

- `output/screen-plan-proof/linux/`.
- `output/screen-plan-proof/linux/proof-summary.json`.
- Separate proof per compositor/session.
- `output/screen-plan-proof/linux-wslg/proof-summary.json`.
