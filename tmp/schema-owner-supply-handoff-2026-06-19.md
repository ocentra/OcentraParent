# Schema Owner Supply Handoff — 2026-06-19

Branch: `codex/schema-owner-consolidation-web-20260619`
Base checkpoint: `e8a68bf64577b514bb85991a9d077b7bc57dd2bf`

## Strategy

This branch is a schema-supply branch, not a completed import-rewire branch.

Goal:

- Create canonical TypeScript contract/schema surfaces under `packages/schema-domain`.
- Do not depend on peer domain packages from `schema-domain`.
- Let local Codex validation mechanically replace old imports, remove local schema owners, and delete frontages.

Do not restore local schema definitions in domain packages.

## Canonical TypeScript surfaces added or extended

### Family

- `@ocentra-parent/schema-domain/family-reference-primitives`
- `@ocentra-parent/schema-domain/family-references`
- `@ocentra-parent/schema-domain/family-child-profile`
- `@ocentra-parent/schema-domain/family-legal`
- `@ocentra-parent/schema-domain/family-session`
- `@ocentra-parent/schema-domain/family-setup`
- `@ocentra-parent/schema-domain/family-restore`

Source files:

- `packages/schema-domain/src/family-reference-primitives.ts`
- `packages/schema-domain/src/family-references.ts`
- `packages/schema-domain/src/family-child-profile.ts`
- `packages/schema-domain/src/family-household-authority.ts`
- `packages/schema-domain/src/family-session-lifecycle.ts`
- `packages/schema-domain/src/family-setup-invite.ts`
- `packages/schema-domain/src/family-restore-lifecycle.ts`

### Evidence

- `@ocentra-parent/schema-domain/evidence-primitives`
- `@ocentra-parent/schema-domain/evidence-kinds`
- `@ocentra-parent/schema-domain/evidence-contracts`
- `@ocentra-parent/schema-domain/evidence-custody`

Source files:

- `packages/schema-domain/src/evidence-primitives.ts`
- `packages/schema-domain/src/evidence-kinds.ts`
- `packages/schema-domain/src/evidence-contracts.ts`
- `packages/schema-domain/src/evidence-custody-contracts.ts`

### App-game

- `@ocentra-parent/schema-domain/app-game-primitives`
- `@ocentra-parent/schema-domain/app-game-identity`
- `@ocentra-parent/schema-domain/app-game-inventory`
- `@ocentra-parent/schema-domain/app-game-category`
- `@ocentra-parent/schema-domain/app-game-session`
- `@ocentra-parent/schema-domain/app-game-launcher`
- existing: `@ocentra-parent/schema-domain/app-game-child-runtime-transport-receipt`

Source files:

- `packages/schema-domain/src/app-game-primitives.ts`
- `packages/schema-domain/src/app-game-identity-primitives.ts`
- `packages/schema-domain/src/app-game-inventory-primitives.ts`
- `packages/schema-domain/src/app-game-category-risk-primitives.ts`
- `packages/schema-domain/src/app-game-session-primitives.ts`
- `packages/schema-domain/src/app-game-launcher.ts`
- `packages/schema-domain/src/app-game-child-runtime-transport-receipt.ts`

## Local domain package state

### `packages/family-domain`

Schema definitions were drained or neutralized from the high-signal family files.

- `src/references.ts` moved/renamed into `schema-domain/src/family-references.ts`.
- `src/reference-primitives.ts` could not be physically deleted by the connector, so it was neutralized to `export {};`. Codex should delete it after validation confirms no imports remain.
- `src/child-profile.ts` now keeps helper behavior and consumes `schema-domain` schemas.
- `src/household-authority.ts` now keeps behavior and consumes `schema-domain/family-legal`.
- `src/session-lifecycle.ts` now keeps behavior and consumes `schema-domain/family-session`.
- `src/setup-lifecycle.ts` now keeps behavior and consumes `schema-domain/family-setup` and `schema-domain/family-restore`.

Known local fix needed:

- In `packages/family-domain/src/setup-lifecycle.ts`, restore the original parse in `deviceTrustStateForRecoveryState`:
  - change `const parsedState = state;`
  - to `const parsedState = RecoveryStateSchema.parse(state);`
  - import `RecoveryStateSchema` from `@ocentra-parent/schema-domain/family-restore`.

### `packages/evidence-domain`

Do not keep evidence schema ownership here long term. Replace imports to:

- `@ocentra-parent/schema-domain/evidence-primitives`
- `@ocentra-parent/schema-domain/evidence-kinds`
- `@ocentra-parent/schema-domain/evidence-contracts`
- `@ocentra-parent/schema-domain/evidence-custody`

Then delete or neutralize local schema owner files after validation.

### `packages/app-game-domain`

Do not keep app-game schema ownership here long term. Replace imports to:

- `@ocentra-parent/schema-domain/app-game-primitives`
- `@ocentra-parent/schema-domain/app-game-identity`
- `@ocentra-parent/schema-domain/app-game-inventory`
- `@ocentra-parent/schema-domain/app-game-category`
- `@ocentra-parent/schema-domain/app-game-session`
- `@ocentra-parent/schema-domain/app-game-launcher`
- `@ocentra-parent/schema-domain/app-game-child-runtime-transport-receipt`

Then delete or neutralize local schema owner files after validation.

## Connector-blocked local cleanup

Some source paths were blocked by the connector. Codex should handle locally:

- Delete `packages/family-domain/src/reference-primitives.ts` if no imports remain.
- Clean remaining parent-domain one-hop re-export frontages.
- The connector blocked some parent-domain files with app-game/control/notification/status wording.

## Validation commands

Run narrow validation first:

```powershell
npm run build --workspace @ocentra-parent/schema-domain
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/evidence-domain
npm run build --workspace @ocentra-parent/app-game-domain
npm run lint:architecture -- --files packages/schema-domain packages/family-domain packages/evidence-domain packages/app-game-domain packages/parent-domain
```

Then continue packetized import rewiring and deletion.

## Non-goals for this branch

- Do not claim green validation.
- Do not move behavior into `schema-domain`.
- Do not introduce barrel/re-export shims.
- Do not make `schema-domain` depend on peer domain packages.
