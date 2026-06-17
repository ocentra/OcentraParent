# LAN Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Source Index`
> Kind: source ownership index.
> Read when: Only when the authoritative LAN owner, proof surface, or cross-plan boundary is unclear.
> Stop rule: Use only the named owning paths. Do not fan out into unrelated packages or plans.
> Proves: ownership routing and current executable source surface only.
> Does not prove: full workpack completion, physical proof, or sibling plan completion.
> Proof rule: if ownership or status changes here, update `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `implementation-checklist.md`, and the current proof root.

<!-- /agent-capsule -->

This file is truth-synced to the executable `lan-plan` model on 2026-06-17.

## Authoritative Plan Model

- Authoritative execution workpacks: `01-20`
- Frozen follow-on only: `21-25`
- Current slice evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`

`21-25` remain in the folder as draft follow-on material. They do not drive current completion claims for `lan-plan`.

## Product Docs

- Owning feature: [remote-lan-mobile-platforms.md](../../features/remote-lan-mobile-platforms.md)
- Adjacent feature: [family-setup-device-roles.md](../../features/family-setup-device-roles.md)
- Main expectation: [lan-pairing.md](../../expectations/lan-pairing.md)
- Adjacent expectation: [family-setup.md](../../expectations/family-setup.md)

## Authoritative Source Ownership

| Surface | Authoritative paths | Truth rule |
| --- | --- | --- |
| TypeScript LAN domain | `packages/lan-domain/src/lan-discovery-source-matrix.ts`, `packages/lan-domain/src/household-device-spine.ts`, `packages/lan-domain/src/lan-production-household-proof.ts`, `packages/lan-domain/src/lan-signed-discovery-relay-spine.ts`, `packages/lan-domain/src/lan-pairing-device.ts`, `packages/lan-domain/src/lan-pairing-control.ts`, `packages/lan-domain/src/lan-pairing-product-proof.ts`, `packages/lan-domain/src/v0-9-production-discovery-household-proof.ts` | This package is the current TypeScript owner for executable `lan-plan` work. |
| Legacy compatibility shims | `packages/parent-domain/src/lan-*.ts` | These paths are legacy compatibility or stale references only. They are not authoritative ownership for current `lan-plan` completion. |
| TypeScript protocol/domain | `packages/agent-protocol-domain/src/lan-discovery-source-matrix.ts`, `packages/agent-protocol-domain/src/lan-pairing-browser-add-device-state.ts`, `packages/agent-protocol-domain/src/lan-pairing-browser-runtime.ts`, `packages/agent-protocol-domain/src/lan-pairing-challenge.ts` | Shared typed protocol and browser-runtime adapters. |
| Rust service/runtime | `crates/agent-service/src/lan_pairing_browser_add_device_state/source_matrix.rs`, `crates/agent-service/src/lan_pairing_household_device_spine.rs`, `crates/agent-service/src/lan_pairing.rs`, `crates/agent-service/src/lan_pairing_status.rs` | Service-backed read models and route/device state. |
| Portal proof surfaces | `apps/portal/tests/transport-lan-target.test.ts`, `apps/portal/tests/live-activity-network-flow.test.ts`, `apps/portal/e2e/portal-ui.spec.ts` | Rendering and proof consumers only. Portal is not the source of LAN truth. |

## Validation And Proof Entry Points

- `npx vitest run tests/unit/v0-9-production-discovery-household-proof.test.ts`
- `npm run test --workspace @ocentra-parent/lan-domain`
- `npm run build --workspace @ocentra-parent/lan-domain`
- `npm run lint:architecture -- --files packages/lan-domain`
- `node scripts/test/v0-9-production-lan-household-proof.mjs`
- `node scripts/test/v0-9-household-lan-proof-readiness.mjs`
- `node scripts/test/v0-9-household-lan-product-proof.mjs`
- `node scripts/test/v0-9-production-discovery-household-proof.mjs`
- `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`

## Routing Rules

- Do not treat `packages/parent-domain/src/lan-*` as active ownership while `packages/lan-domain` is the executable source.
- Do not use `21-25` for current completion claims.
- Do not cite proof artifacts that are missing on disk. Missing artifacts must be marked open, manual-required, or not yet regenerated.
