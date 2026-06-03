# 09 Windows Capture Adapter Plan Proof

## Target State

Windows.Graphics.Capture path, picker/consent/border, display/window capture, and protected/degraded states are proved before Windows support is claimed.

## Current State

Windows is the preferred first desktop proof target. Implementation proof is open.

## Checklist

- [ ] Verify current Microsoft official capture docs.
- [ ] Add Windows capability probe.
- [ ] Prove display capture.
- [ ] Prove app/window capture.
- [ ] Prove managed browser window capture.
- [ ] Prove protected-surface skip/degraded state.
- [ ] Prove queue write and deletion.

## Proof

- `output/screen-plan-proof/windows/`.
- Local Windows capture logs/screenshots.
