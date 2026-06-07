# WP107 Adapter Capability Status Handoff

## Scope

Project the existing app/game adapter proof spine into parent-domain status rows
that keep native app and native game product meanings separate for Windows,
macOS, Linux, Android, and iOS.

## Implementation

- Add `app-game-adapter-capability-status-handoff` in `parent-domain`.
- Consume the existing V0.8 supported adapter runtime proof and
  cross-platform enforcement capability proof.
- Expose ten rows: native app and native game for each supported platform.
- Keep Windows ready only for owned-process time-limit runtime proof.
- Keep broad installed-app blocking manual-required.
- Keep macOS scaffold/manual-required, Linux unavailable, and Android/iOS
  manual-required until platform artifacts exist.

## Validation

- `cmd /c node scripts/test/app-game-adapter-capability-status-handoff-proof.mjs`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-adapter-capability-status-handoff v0-8-supported-adapter-runtime-proof v0-8-cross-platform-enforcement-capability-proof`
- `cmd /c node --check scripts/test/app-game-adapter-capability-status-handoff-proof.mjs`
- `git diff --check HEAD`
- `git diff --check origin/main...HEAD`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`
- `cmd /c node scripts/check-no-test-doubles.mjs`
- `cmd /c node scripts/check-source-shape.mjs`

## Non-Claims

- No broad installed-app blocking.
- No macOS/Linux/Android/iOS adapter execution.
- No privileged mobile enforcement.
- No portal rendering.
- No child delivery.
- No adapter dispatch.
