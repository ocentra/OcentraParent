# WP187 App/Game Platform Proof Status Android/Linux Detail Refs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP187 App/Game Platform Proof Status Android/Linux Detail Refs`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Carry the real Android and Linux proof details gathered by WP185 and WP186 into
the live service-backed platform proof status read model and portal intent.

This is a visibility upgrade only. It does not convert host proof into platform
authority, blocking, provider delivery, or child-device delivery.

## Implementation

- Added stable Rust proof refs for Android physical-device proof, Android
  UsageEvents foreground evidence, Linux WSLg display, Linux X11 socket, and
  Linux Wayland socket readiness.
- Added explicit open gaps for Android durable UsageEvents replay and Linux
  active foreground capture.
- Updated the Rust service platform proof status rows so Android and Linux
  detail refs are visible through the existing `proofRefs` array.
- Updated Rust service, TypeScript protocol, portal-domain, and proof harness
  assertions so the detail refs stay parent-visible and non-promoting.

## Validation

Focused validation for this workpack:

```powershell
cargo test -p ocentra-parent-agent-protocol app_game_platform_proof_status
cargo test -p ocentra-parent-agent-service app_game_platform_proof_status
cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-platform-proof-status
cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-platform-proof-status-panel
cmd /c node scripts/test/app-game-platform-proof-status-proof.mjs
```

## Proof

- `test-results/app-game-platform-proof-status-proof/proof.json`
- `output/app-game-plan-proof/184-app-game-platform-proof-status-service-surface/proof.json`

## Boundaries

Proved:

- Android physical-device and UsageEvents foreground proof refs are present in
  the service-backed platform proof status row.
- Linux WSLg display and X11/Wayland socket proof refs are present in the
  service-backed platform proof status row.
- The parent portal intent renders those refs without upgrading enforcement.

Not proved:

- Android durable UsageEvents replay, Device Owner/Profile Owner authority,
  hide/suspend/uninstall block, lock task, managed configuration, or Play policy
  authority.
- Linux active foreground capture, native service management, AppArmor/SELinux,
  package/Flatpak/Snap restriction, launch blocking, rollback, or audit.
- Adapter dispatch, broad installed-app blocking, platform enforcement,
  provider delivery, child-device delivery, raw private source rows, raw target
  values, or private diagnostics.
