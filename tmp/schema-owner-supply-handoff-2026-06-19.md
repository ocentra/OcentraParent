# Schema Owner Supply Handoff — 2026-06-19

Branch: `codex/schema-contracts-web-20260619`
Base checkpoint: `e8a68bf64577b514bb85991a9d077b7bc57dd2bf`

## Scope

This is a clean schema-supply branch.

It intentionally contains only:

- canonical TypeScript schema additions under `packages/schema-domain`;
- `packages/schema-domain/package.json` export-map additions;
- this handoff document.

It intentionally does not contain package-local import rewires, local schema deletions, or frontage cleanup. Codex should do those after validating this branch.

## Current coverage status

This branch does **not** yet prove full coverage for all 65 targets.

It supplies canonical replacements for the domains listed below. Remaining domains must be audited and supplied before global delete/replace work is attempted.

## Canonical TypeScript schema surfaces supplied

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

### Activity

- `@ocentra-parent/schema-domain/activity-journal`
- `@ocentra-parent/schema-domain/activity-capture`

Source files:

- `packages/schema-domain/src/activity-journal-primitives.ts`
- `packages/schema-domain/src/activity-capture.ts`

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

## Domains still needing supply audit

Audit these before claiming global coverage:

- `packages/agent-protocol-domain`
- `packages/ai-domain`
- `packages/browser-domain`
- `packages/capability-domain`
- `packages/child-runtime-domain`
- `packages/enforcement-domain`
- `packages/event-domain`
- `packages/lan-domain`
- `packages/logging-domain`
- `packages/network-domain`
- `packages/notification-domain`
- `packages/parent-domain`
- `packages/policy-domain`
- `packages/portal-domain`
- `packages/production-domain`
- `packages/remote-access-domain`
- `packages/screen-domain`
- apps/tools if they own Effect Schema contracts directly

## Known high-priority missing TypeScript surface

`packages/agent-protocol-domain` still owns or aggregates many local schema surfaces, including at least:

- `browser-runtime-events`
- `lan-pairing-browser-runtime`
- `lan-pairing-browser-add-device-state`
- `lan-signed-discovery-relay-spine`
- `lan-discovery-source-matrix`
- `security`
- network read-model/status contracts
- app-game adapter/read-model/status contracts
- social read-model/custody contracts

Those must be supplied in `schema-domain` before Codex can do a clean global replacement.

## Rust status

Rust mirror work is not supplied on this branch yet.

Required next Rust owner pass:

- add shared wire/protocol DTO mirrors to `crates/agent-protocol`;
- do not rewire consumers yet;
- do not add Rust `pub use` frontages;
- preserve exact TS/Rust encoded field names, discriminants, nullability, and schema versions.

## Codex execution order

1. Validate `schema-domain` only:

```powershell
npm run build --workspace @ocentra-parent/schema-domain
```

2. Continue schema supply for missing TypeScript packages before deleting local owners.

3. Add Rust mirrors to `crates/agent-protocol`.

4. Only after central supply exists, mechanically replace local imports package-by-package.

5. Delete local schema owner files only after their consumers compile against the canonical surfaces.

## Non-goals for this branch

- Do not claim green validation.
- Do not move behavior into `schema-domain`.
- Do not introduce barrel/re-export shims.
- Do not make `schema-domain` depend on peer domain packages.
- Do not globally delete local schema files until replacements compile.
