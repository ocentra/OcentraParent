# WP180 App/Game Host Capability Summary Counts

## Scope

Expose aggregate host capability counts in the service-backed app/game adapter
execution readiness and dispatch preflight read models. This makes Android ADB
and Linux WSL/Docker host visibility visible at the parent summary level while
keeping platform execution blocked until platform-specific proof exists.

## Implementation

- Add aggregate fields to the Rust protocol and TypeScript protocol parsers:
  - `hostCapabilityAvailableCount`
  - `hostCapabilityNotDetectedCount`
  - `hostCapabilityNotApplicableCount`
  - `hostCapabilityProbeRefCount`
- Derive those counts from the existing service rows instead of creating a
  second source of truth.
- Reject protocol payloads where aggregate counts do not match row state.
- Render the host capability counts in the portal-domain execution readiness
  and dispatch preflight summary intents.

## Validation

- `cargo test -p ocentra-parent-agent-protocol app_game_adapter`
- `cargo test -p ocentra-parent-agent-service app_game_adapter`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-adapter-dispatch-preflight app-game-adapter-execution-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-adapter-dispatch-preflight-panel app-game-adapter-execution-readiness-panel`
- `cmd /c npm run build:contracts`

## No-Claim Boundaries

- Does not make Android rows dispatch-eligible or claim UsageStats,
  Accessibility, VPN/DNS, Device Owner, Profile Owner, hide/suspend,
  uninstall-block, lock-task, managed configuration, or Play policy proof.
- Does not make Linux rows dispatch-eligible or claim package manager, Flatpak,
  Snap, AppImage, procfs, cgroup/systemd, X11/Wayland, AppArmor, SELinux, or
  package restriction proof.
- Does not claim macOS or iOS runtime execution from Windows.
- Does not add broad installed-app blocking, platform enforcement, provider
  delivery, child-device delivery, raw private source rows, raw target values,
  or private diagnostics.
