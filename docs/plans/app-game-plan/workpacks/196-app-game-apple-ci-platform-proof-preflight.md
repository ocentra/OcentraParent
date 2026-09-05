# WP196 App/game Apple CI platform proof preflight

> **Current status (2026-08-29):** bounded Rust/service/parent/portal status
> source and real behavior tests are present. Apple execution remains an
> external CI/device/provisioning proof boundary; no macOS or iOS enforcement
> adapter is claimed.

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP196 App/game Apple CI platform proof preflight`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Represent macOS and iOS app/game control as Apple-platform CI-required proof
rows in the shared app/game evidence spine.

This does not claim that the Windows local host can prove macOS or iOS runtime
behavior. It converts the existing macOS/iOS manual artifact gates into a typed
parent-domain preflight that names the CI runner, entitlement, device,
rollback, and audit artifacts required before Apple platform support can move
from manual-required to implemented.

## Non-Goals

- No macOS local runtime proof from Windows.
- No iOS runtime proof from Windows.
- No macOS MDM, Endpoint Security, System Extension, LaunchDaemon, rollback, or
  audit execution claim.
- No iOS FamilyControls, DeviceActivity, ManagedSettings, shield UI,
  TestFlight/device install, MDM, or supervised-device execution claim.
- No adapter dispatch, broad blocking, platform enforcement, provider delivery,
  or child-device delivery claim.

## Files

The historical `packages/parent-domain` and `scripts/test/app-game-*` paths
below are retired. Current ownership is the Rust platform-proof contract,
agent-service host-capability/status payloads, parent-runtime bridge, portal
status panel, and their checked-in Rust/TypeScript behavior tests. Those files
represent fail-closed Apple rows; they do not perform Apple runtime work.

- `packages/parent-domain/src/app-game-apple-ci-platform-proof-preflight.ts`
- `packages/parent-domain/tests/app-game-apple-ci-platform-proof-preflight.test.ts`
- `packages/parent-domain/src/app-game-platform-proof-status.ts`
- `packages/parent-domain/tests/app-game-platform-proof-status.test.ts`
- `scripts/test/app-game-apple-ci-platform-proof-preflight-proof.mjs`
- `scripts/test/app-game-platform-proof-status-proof.mjs`

## Validation

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-apple-ci-platform-proof-preflight`
- `cmd /c node --check scripts/test/app-game-apple-ci-platform-proof-preflight-proof.mjs`
- `cmd /c node scripts/test/app-game-apple-ci-platform-proof-preflight-proof.mjs`
- `cmd /c node scripts/test/app-game-platform-proof-status-proof.mjs`

## Done Criteria

- macOS and iOS rows are present in the parent-domain preflight.
- macOS and iOS rows are present in shared platform proof status when the
  preflight is supplied.
- Windows-local proof remains false for Apple platforms.
- Adapter dispatch and platform enforcement remain false.
- The proof names the CI/device artifacts that would be required later.
