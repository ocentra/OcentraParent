# WP184 App/Game Platform Proof Status Service Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP184 App/Game Platform Proof Status Service Surface`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Promote the WP183 platform proof status surface from parent-domain proof
aggregation into a live service-backed app/game command, read model, and parent
portal route.

This keeps native apps and native games unified over one platform proof status
spine while preserving honest platform boundaries. Windows local service proof
can show scoped execution status, Android/Linux can show host visibility, and
macOS/iOS remain not locally provable from this Windows worker host.

## Implementation

- Added `packages/agent-protocol-domain/src/app-game-platform-proof-status.ts`.
- Added `crates/agent-protocol/src/app_game_platform_proof_status.rs`.
- Added `crates/agent-service/src/activity_api/app_game_platform_proof_status_payload.rs`.
- Routed `agent.activity.app-game.platform-proof-status.read-model.get` through
  the Rust WebSocket command handler.
- Added `apps/portal/src/AppGamePlatformProofStatusRoutePanel.tsx`.
- Added `apps/portal/src/app-game-platform-proof-status-panel.ts`.
- Updated `apps/portal/src/live-activity-state.ts` and
  `apps/portal/src/ParentPortalRoute.tsx`.
- Updated `packages/portal-domain/src/app-game-platform-proof-status-panel.ts`
  to consume the live service read model.
- Extended `scripts/test/app-game-platform-proof-status-proof.mjs` to cover
  the service/protocol/portal route path.

The service read model emits five parent-safe rows:

- Windows: scoped app/game owned-process execution proof only.
- Android: ADB host visibility or not-detected state.
- Linux: WSL/Docker host visibility or not-detected state.
- macOS: not locally provable from this Windows service host.
- iOS: not locally provable from this Windows service host.

WP187 extends the Android row with physical-device and UsageEvents foreground
proof refs, and extends the Linux row with WSLg display plus X11/Wayland socket
refs. Those refs remain visibility-only details and do not upgrade enforcement
claims.

All rows keep adapter dispatch, broad installed-app blocking, platform
enforcement, provider delivery, child-device delivery, and private diagnostics
fixed to unclaimed.

## Validation

Focused validation for this workpack:

```powershell
cargo test -p ocentra-parent-agent-protocol app_game_platform_proof_status
cargo test -p ocentra-parent-agent-service app_game_platform_proof_status
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-platform-proof-status
cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-platform-proof-status-panel
cmd /c npm exec --workspace @ocentra-parent/portal -- vitest run tests/app-game-platform-proof-status-route-panel.test.ts
node scripts/test/app-game-platform-proof-status-proof.mjs
```

## Proof

- `test-results/app-game-platform-proof-status-proof/proof.json`
- `output/app-game-plan-proof/183-app-game-platform-proof-status-surface/proof.json`
- `output/app-game-plan-proof/184-app-game-platform-proof-status-service-surface/proof.json`

## Boundaries

Proved:

- The Rust protocol exposes a typed app/game platform proof status command and
  reported event.
- The Rust service reports live host capability status using parent-safe proof
  refs and probe refs.
- The portal live activity state parses the reported service event.
- The App/Game Sessions route renders the platform proof status rows.

Not proved:

- Android Device Owner/Profile Owner authority or UsageEvents replay.
- Linux native foreground capture, policy mechanism, rollback, or audit.
- macOS Screen Time, Endpoint Security, MDM, or local runtime proof from this
  Windows host.
- iOS Family Controls, DeviceActivity, ManagedSettings, MDM, supervised
  restriction, signing, or entitlement proof from this Windows host.
- Broad installed-app blocking, platform enforcement, provider delivery,
  child-device delivery, raw private source rows, raw target values, or private
  diagnostics.
