# WP183 App/Game Platform Proof Status Surface

## Scope

Surface the existing Android physical-device proof and Linux WSL runtime proof as one parent-safe app/game platform proof status read model and portal-domain intent.

This work keeps native app and native game product meaning unified over the same low-level evidence spine. It does not promote Android, Linux, macOS, or iOS into runtime enforcement support.

## Implementation

- Added `packages/parent-domain/src/app-game-platform-proof-status.ts`.
- Added `packages/parent-domain/tests/app-game-platform-proof-status.test.ts`.
- Added `packages/portal-domain/src/app-game-platform-proof-status-panel.ts`.
- Added `packages/portal-domain/tests/app-game-platform-proof-status-panel.test.ts`.
- Added `scripts/test/app-game-platform-proof-status-proof.mjs`.

The parent-domain read model consumes:

- `AppGameAndroidPhysicalDeviceProof`
- `AppGameLinuxWslRuntimeProof`

It emits one Android status row and one Linux status row with:

- parent-safe proof state;
- parent-safe package/runtime visibility counts;
- explicit authority state;
- proof refs;
- open platform gaps;
- adapter dispatch, broad blocking, platform enforcement, and child delivery fixed to unclaimed.

The portal-domain intent renders that status as review-only platform visibility.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-platform-proof-status app-game-android-physical-device-proof app-game-linux-wsl-runtime-proof
cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-platform-proof-status-panel
node scripts/test/app-game-platform-proof-status-proof.mjs
```

## Proof

- `test-results/app-game-platform-proof-status-proof/proof.json`
- `output/app-game-plan-proof/183-app-game-platform-proof-status-surface/proof.json`

## Boundaries

Proved:

- Android physical-device proof can be summarized as parent-visible app/game platform evidence.
- Linux WSL runtime proof can be summarized as parent-visible app/game platform evidence.
- Portal-domain can render both platform rows without raw package names, raw process names, raw distro names, raw device serials, or private host paths.

Not proved:

- Android Device Owner/Profile Owner authority.
- Android UsageEvents replay, Accessibility overlay, hide/suspend/uninstall-block, lock task, or managed configuration execution.
- Linux X11/Wayland foreground proof.
- Linux AppArmor/SELinux policy, package/Flatpak/Snap restriction, terminate/block-launch adapters, rollback, or audit.
- macOS Screen Time, Endpoint Security, MDM, or local runtime proof from this Windows host.
- iOS Family Controls, DeviceActivity, ManagedSettings, MDM, or supervised restriction proof from this Windows host.
- Provider delivery, child-device delivery, broad installed-app blocking, non-scoped platform enforcement, raw private rows/targets, or private diagnostics.
