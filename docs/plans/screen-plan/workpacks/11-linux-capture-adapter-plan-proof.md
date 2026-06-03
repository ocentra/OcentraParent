# 11 Linux Capture Adapter Plan Proof

## Target State

X11 capture, Wayland/PipeWire portal capture, compositor-specific status, and manual-required states are represented honestly.

## Current State

The shared Rust desktop adapter path now targets Linux through a real X11
command backend using `xwininfo`, `xwd`, and ImageMagick. WSLg selected-window
proof exists with captured pixels, encrypted custody, and raw deletion. Native
Wayland/PipeWire portal proof, root-display proof, and compositor parity remain
open.

## Checklist

- [x] Define X11 path.
- [~] Define Wayland portal/PipeWire path.
- [ ] Define GNOME/KDE/wlroots states.
- [ ] Define unsupported compositor state.
- [ ] Prove protected/permission-limited states.
- [~] Prove local OCR and deletion where capture works.

## Proof

- `output/screen-plan-proof/linux/`.
- Separate proof per compositor/session.
- `output/screen-plan-proof/linux-wslg/proof-summary.json`.
