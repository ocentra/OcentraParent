# 11 Linux Capture Adapter Plan Proof

## Target State

X11 capture, Wayland/PipeWire portal capture, compositor-specific status, and manual-required states are represented honestly.

## Current State

Linux support is desktop-session-specific and must not be claimed universally.

## Checklist

- [ ] Define X11 path.
- [ ] Define Wayland portal/PipeWire path.
- [ ] Define GNOME/KDE/wlroots states.
- [ ] Define unsupported compositor state.
- [ ] Prove protected/permission-limited states.
- [ ] Prove local OCR and deletion where capture works.

## Proof

- `output/screen-plan-proof/linux/`.
- Separate proof per compositor/session.
