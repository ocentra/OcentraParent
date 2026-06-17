# 03 Pairing And Route Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `03 Pairing And Route Proof`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md),
[LAN pairing expectations](../../expectations/lan-pairing.md).
Assumes WP01 contracts and WP02 SQLite tables exist.

## Where We Are

The V0.9 LAN spine now wires `crates/agent-service` to a service-backed read
model that consumes LAN slot/parser fixture states for signed discovery, route
custody, relay/cache unavailable, manual-proof, and parent-decision fields
(feature doc checklist row `[ ]`). Live B-lane browser proof exists at:
`output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`.

What is **not** proved yet:

- Revocation and recovery flow. Typed `DeviceRevocationState` exists in read model but **parent recovery UX** — the flow that lets a parent re-pair after losing a controller device — has no portal flow, no service command, and no audit event.
- Stale/offline detection triggers a read-model label but no typed `RecoveryState` (from WP01) is written to SQLite and returned in the household read model.
- Wrong-device command rejection is implemented in the LAN spine (existing lan-plan work) but the household layer (WP02 new tables) has not integrated the same origin-check path.
- `SetupAuditEvent` (WP01) is not persisted or returned. The existing LAN audit rows live in the lan-plan proof but are not aggregated into household-level audit history.

## Where We Want To Be

Prove that the household pairing and route layer enforces:

1. Revocation: once revoked, the device rejects all commands until a fresh pairing is issued.
2. Recovery: parent can initiate a typed `StartRecovery` command that invalidates the old pairing, creates a new `RecoveryState` record, and waits for fresh pairing proof.
3. Stale/offline: the household read model labels a device as stale after a configurable TTL and does not claim it is reachable.
4. Wrong-device rejection: commands addressed to device A that arrive at device B's service endpoint are rejected with `WrongDevice` and an audit event.
5. Audit: every pairing, revocation, recovery, and wrong-device event produces a `SetupAuditEvent` record in the SQLite `setup_audit_events` table.

## Scope

- Add `crates/agent-service/src/commands/household_commands.rs`: handlers for `StartRecovery`, `RevokeDevice`, `ConfirmPairing`.
- In `crates/agent-service/src/db/household.rs` (from WP02): add `setup_audit_events` table; write audit record in every command handler.
- In `crates/agent-service/src/lan/` (already exists, owned by lan-plan): read the household `controller_leases` table to validate the lease before routing a command — do not duplicate LAN origin-check logic, only add household-layer lease validity check.
- Add `RecoveryState` insertion in `StartRecovery` handler; read it back in `HouseholdReadModel`.
- Add stale-TTL logic: if a device's last heartbeat timestamp from the LAN spine is older than `stale_ttl_seconds` (config), set `routeState = "stale"` in the read model response.
- Rust parity test for `SetupAuditEvent` serde (shape defined in WP01 `crates/agent-protocol/src/household.rs`).

## Touched Paths

- `crates/agent-service/src/commands/household_commands.rs` (new)
- `crates/agent-service/src/commands/mod.rs` (add `household_commands`)
- `crates/agent-service/src/db/household.rs` (add `setup_audit_events` table)
- `crates/agent-service/src/read_models/household_read_model.rs` (add stale-TTL, RecoveryState)
- `crates/agent-protocol/src/household.rs` (SetupAuditEvent serde tests)
- `scripts/test/family-setup-pairing-and-route-proof.mjs` (new integration test script)

## Tests And Proof

- [ ] Rust unit test: `RevokeDevice` command sets `DeviceRegistration.trustState = "revoked"` in SQLite and subsequent commands return `DeviceRevoked`.
- [ ] Rust unit test: `StartRecovery` command inserts a `RecoveryState` row and invalidates the old `ParentControllerLease`.
- [ ] Rust unit test: stale-TTL logic — simulate heartbeat timestamp 2× stale_ttl in the past; assert read model returns `routeState = "stale"`.
- [ ] Rust unit test: wrong-device command addressed to device A, handled by device B's service — returns `WrongDevice`; `SetupAuditEvent` written.
- [ ] Rust unit test: `ConfirmPairing` after `StartRecovery` creates a new valid lease and clears the `RecoveryState`.
- [ ] `SetupAuditEvent` serde round-trip test in `crates/agent-protocol/src/household.rs`.
- [ ] Integration test script `scripts/test/family-setup-pairing-and-route-proof.mjs`: start local service, execute revoke → reject → recover → re-pair cycle, assert audit log contains all events, output `output/lan-plan-proof/03-pairing-and-route-proof/03-pairing-cycle-proof.json`.
- Manual-required: two-device physical LAN proof. CI runs single-machine only. Record manual-required before claiming household multi-device readiness.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md), [LAN pairing expectations](../../expectations/lan-pairing.md), [current PLAN_STATE](../PLAN_STATE.md), and this workpack.
- [ ] Confirmed WP01 contracts and WP02 tables exist before adding command handlers.
- [ ] lan-plan origin-check path inspected: household lease check added without duplicating LAN origin-check logic.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] `RevokeDevice` test written and passes.
- [ ] `StartRecovery` and `ConfirmPairing` round-trip test written and passes.
- [ ] Stale-TTL read model test written and passes.
- [ ] Wrong-device rejection test written and passes; audit event confirmed in SQLite.
- [ ] `SetupAuditEvent` serde test in `crates/agent-protocol` passes.
- [ ] Integration script `family-setup-pairing-and-route-proof.mjs` runs and outputs proof JSON.
- [ ] Physical two-device LAN proof recorded as manual-required in [main checklist](../implementation-checklist.md).
- [ ] Proof artifacts saved to `output/lan-plan-proof/03-pairing-and-route-proof/`.
- [ ] [main checklist](../implementation-checklist.md) rows 03 updated.

## Manual-Required Gaps

Physical two-device LAN proof: a parent host and a child host on distinct IP addresses with a real router between them, signed hello/heartbeat, and physical revoke/re-pair cycle. This cannot run in CI. Record as `manual-required` with the following evidence requirement: packet capture log, service log from both hosts, and `03-manual-two-device-pairing-proof.md` in the proof folder.
