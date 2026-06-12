# WP178 App/Game Adapter Host Capability State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP178 App/Game Adapter Host Capability State`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Make local host capability status and parent-safe probe references explicit in
the existing app/game adapter execution readiness and dispatch preflight read
models. Windows stays the only execution-capable scoped adapter path; Android
ADB and Linux WSL/Docker are reported as local host visibility signals only.

## Implementation

- Add `hostCapabilityState`, `hostCapabilityEvidenceRefs`, and
  `hostCapabilityProbeRefs` to the Rust protocol, TypeScript protocol parser,
  parent-domain readiness contract, agent-service read models, and portal-domain
  panel intents.
- Derive host capability state from the service host:
  - `available` for Windows scoped/manual/artifact capability rows, with
    `windows-host-local-probe-ref`;
  - `available` for Android when `adb` is visible through `PATH`,
    `ANDROID_HOME`, or `ANDROID_SDK_ROOT`, with separate ADB path/SDK probe
    refs;
  - `available` for Linux when `wsl` or `docker` is visible on `PATH`, with
    separate WSL/Docker path probe refs;
  - `not-detected` for Android/Linux when those host tools are absent;
  - `not-applicable` for macOS/iOS in the Windows-local service host path.
- Keep probe refs as opaque `*-probe-ref` values only; do not expose raw paths,
  device serials, WSL distro names, Docker daemon diagnostics, or private
  adapter logs.
- Preserve the no-claim boundary: Android/Linux host capability signals do not
  make dispatch eligible, and macOS/iOS remain manual/CI-required platform rows.

## Validation

- `cargo test -p ocentra-parent-agent-protocol app_game_adapter`
- `cargo test -p ocentra-parent-agent-service app_game_adapter`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-adapter-dispatch-preflight app-game-adapter-execution-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-adapter-execution-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-adapter-dispatch-preflight-panel app-game-adapter-execution-readiness-panel`
- `cmd /c npm run build:contracts`
- `cmd /c node scripts/test/app-game-adapter-execution-readiness-proof.mjs`
- `cmd /c node scripts/test/app-game-adapter-dispatch-preflight-live-handoff-proof.mjs`

Additional focused probe-ref validation:

- `cargo test -p ocentra-parent-agent-protocol app_game_adapter`
- `cargo test -p ocentra-parent-agent-service app_game_adapter`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-adapter-dispatch-preflight app-game-adapter-execution-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-adapter-execution-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-adapter-dispatch-preflight-panel app-game-adapter-execution-readiness-panel`
- `cmd /c npm run build:contracts`

## No-Claim Boundaries

- Does not enable Android package suspend/hide, Device Owner/Profile Owner
  control, or UsageStats proof.
- Does not add Linux package, cgroup, desktop session, AppArmor, SELinux, or
  systemd enforcement.
- Does not claim macOS or iOS runtime execution on Windows.
- Does not add broad installed-app blocking.
- Does not add provider delivery, child-device delivery, platform enforcement,
  raw private source rows, raw target values, or private diagnostics.
