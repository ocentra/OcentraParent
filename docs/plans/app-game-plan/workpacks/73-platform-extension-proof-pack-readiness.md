# WP73 App/Game Platform Extension Proof-Pack Readiness

## Scope

Add a parent-domain proof-pack readiness contract for the non-Windows native
app/game platform extension checklist rows.

This work keeps native apps and native games as separate product meanings on
top of the shared app/game evidence spine. It does not implement platform
adapters, adapter dispatch, broad installed-app blocking, privileged mobile
controls, store or MDM provider execution, or child-device delivery.

## Owned Proof

- Contract: `packages/parent-domain/src/app-game-platform-extension-proof-pack-readiness.ts`
- Tests: `packages/parent-domain/tests/app-game-platform-extension-proof-pack-readiness.test.ts`
- Harness: `scripts/test/app-game-platform-extension-proof-pack-readiness.mjs`
- Evidence:
  `output/app-game-plan-proof/73-platform-extension-proof-pack-readiness`
  and `output/app-plan-proof/73-platform-extension-proof-pack-readiness`

## Acceptance

- macOS, iOS, Android, and Linux each have exactly one proof-pack readiness row.
- Every row carries both `native-app` and `native-game` product meanings.
- Every row names checklist refs and required proof refs.
- Adapter execution, broad blocking, privileged mobile, store/MDM provider
  execution, and child delivery claims remain false.
- Focused parent-domain tests and the proof harness pass.

## Non-Goals

- No live platform adapter implementation.
- No package export while `packages/parent-domain/package.json` is owned by
  another lane.
- No central product capability checklist status move.
