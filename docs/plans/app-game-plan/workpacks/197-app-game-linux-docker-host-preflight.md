# WP197 App/game Linux Docker host preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP197 App/game Linux Docker host preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Represent Docker host visibility as parent-safe Linux platform evidence in the
shared app/game platform proof spine.

This workpack records whether the local Windows host can see Docker CLI,
daemon, context, image, and container inventory boundaries, but it stores only
counts and readiness states. It does not store Docker context names, image
names, container ids, raw paths, or private daemon diagnostics.

## Non-Goals

- No Docker container policy execution.
- No Linux AppArmor, SELinux, package, Flatpak, Snap, or container restriction
  proof.
- No launch blocking, rollback, audit, adapter dispatch, platform enforcement,
  provider delivery, or child-device delivery claim.
- No raw Docker context, image, or container identifier custody.

## Files

- `packages/parent-domain/src/app-game-linux-docker-host-preflight.ts`
- `packages/parent-domain/tests/app-game-linux-docker-host-preflight.test.ts`
- `packages/parent-domain/src/app-game-platform-proof-status.ts`
- `packages/parent-domain/tests/app-game-platform-proof-status.test.ts`
- `scripts/test/app-game-linux-docker-host-preflight-proof.mjs`
- `scripts/test/app-game-platform-proof-status-proof.mjs`

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-linux-docker-host-preflight app-game-platform-proof-status`
- `cmd /c node --check scripts/test/app-game-linux-docker-host-preflight-proof.mjs`
- `cmd /c node scripts/test/app-game-linux-docker-host-preflight-proof.mjs`
- `cmd /c node scripts/test/app-game-platform-proof-status-proof.mjs`

## Done Criteria

- Docker CLI/daemon/context/image/container visibility is represented as typed
  parent-domain readiness.
- Context, image, and container details are redacted to counts only.
- Linux platform proof status can carry `linux-docker-host-preflight-ref`.
- Container policy, platform enforcement, adapter dispatch, and child delivery
  remain false.
