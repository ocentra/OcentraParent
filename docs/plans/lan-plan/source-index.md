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

- Authoritative execution workpacks: `01-25`
- Active open follow-on workpacks: `23`, `25`
- Current slice evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`

`21-25` are active `lan-plan` follow-on workpacks. Rows `21`, `22`, and `24`
are locally complete with their own proof; rows `23` and `25` remain
partial/manual. No row counts as PR-ready without its own current Rust-first
proof, organized tests where applicable, and row truth.

## Product Docs

- Owning feature: [remote-lan-mobile-platforms.md](../../features/remote-lan-mobile-platforms.md)
- Adjacent feature: [family-setup-device-roles.md](../../features/family-setup-device-roles.md)
- Main expectation: [lan-pairing.md](../../expectations/lan-pairing.md)
- Adjacent expectation: [family-setup.md](../../expectations/family-setup.md)

## Authoritative Source Ownership

| Surface | Authoritative paths | Truth rule |
| --- | --- | --- |
| Rust shared contracts and LAN core | `crates/schema`, `crates/lan-core`, `crates/agent-protocol` | Shared LAN contracts, protocol shapes, read-model DTOs, and discovery/runtime truth live in Rust. This is the authoritative contract and business-logic boundary for current `lan-plan` work. |
| Rust service/runtime | `crates/agent-service/src/lan_pairing_browser_add_device_state/source_matrix.rs`, `crates/agent-service/src/lan_pairing_household_device_spine.rs`, `crates/agent-service/src/lan_pairing.rs`, `crates/agent-service/src/lan_pairing_status.rs`, `crates/parent-runtime-core` | Service-backed read models, route/device state, runtime decisions, and proof truth are Rust-owned. |
| Historical TS package surfaces | `packages/lan-domain/src/lan-discovery-source-matrix.ts`, `packages/lan-domain/src/household-device-spine.ts`, `packages/lan-domain/src/lan-production-household-proof.ts`, `packages/lan-domain/src/lan-signed-discovery-relay-spine.ts`, `packages/lan-domain/src/lan-pairing-device.ts`, `packages/lan-domain/src/lan-pairing-control.ts`, `packages/lan-domain/src/lan-pairing-product-proof.ts`, `packages/lan-domain/src/v0-9-production-discovery-household-proof.ts`, `packages/agent-protocol-domain/src/lan-discovery-source-matrix.ts`, `packages/agent-protocol-domain/src/lan-pairing-browser-add-device-state.ts`, `packages/agent-protocol-domain/src/lan-pairing-browser-runtime.ts`, `packages/agent-protocol-domain/src/lan-pairing-challenge.ts`, `packages/parent-domain/src/lan-*.ts` | These paths are migration debt, compatibility seams, or historical references only. They are not authoritative LAN contract, business-logic, runtime, or proof owners. |
| Portal presentation and proof consumers | `apps/portal/tests/transport-lan-target.test.ts`, `apps/portal/tests/live-activity-network-flow.test.ts`, `apps/portal/e2e/portal-ui.spec.ts` | TS is presentation only here: portal/UI rendering, fixture-backed proof consumption, and bridge-facing tests. Portal is not the source of LAN truth. |

## Validation And Proof Entry Points

- `cargo check -p ocentra-lan-core --tests`
- `cargo check -p ocentra-parent-agent-service --tests`
- `cargo check -p ocentra-parent-runtime-core --tests`
- missing as of the 2026-08-15 code-first audit; restore or replace:
  `node scripts/test/v0-9-production-lan-household-proof.mjs`
- missing as of the 2026-08-15 code-first audit; restore or replace:
  `node scripts/test/v0-9-household-lan-proof-readiness.mjs`
- missing as of the 2026-08-15 code-first audit; restore or replace:
  `node scripts/test/v0-9-household-lan-product-proof.mjs`
- missing as of the 2026-08-15 code-first audit; restore or replace:
  `node scripts/test/v0-9-production-discovery-household-proof.mjs`
- missing as of the 2026-08-15 code-first audit; restore or replace before use:
  `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`
- missing as of the 2026-08-15 code-first audit; restore or replace before use:
  `node scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`

## Routing Rules

- Do not treat historical TS package surfaces as active LAN ownership while Rust owns contracts, runtime, read models, and proof truth.
- Use organized crate/app test folders only; placeholder trees, fake tests, mock-only closure, and inline source-owned tests do not count.
- Do not mark any follow-on row complete without its own current proof
  artifacts, organized tests where applicable, and row truth.
- Do not cite proof artifacts that are missing on disk. Missing artifacts must be marked open, manual-required, or not yet regenerated.
