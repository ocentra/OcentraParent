# 02 Current State And Gap Map

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `02 Current State And Gap Map`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md),
[family setup expectations](../../expectations/family-setup.md),
[LAN pairing expectations](../../expectations/lan-pairing.md).
Assumes workpack 01 domain schemas exist before this workpack executes.

## Where We Are

The V0.9 LAN spine produces a service-backed add-device read model with:

- Household device rows: trusted registry, route custody, stale/offline selected-device.
- Signed discovery/relay spine rows: signed hello/heartbeat manual-required.
- LAN source-matrix rows: 20 plan workpacks and discovery sources visible.
- Parent decision fields: assign/rename/ignore/restore/trust/revoke.
- Live B-lane Playwright proof: `output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`.

What is **missing from the current read model and runtime**:

1. `HouseholdProfile` and `ChildProfile` are not stored, persisted, or served by `crates/agent-service`. The add-device model has device rows but no parent-tier household grouping.
2. `ParentControllerLease` is conceptually present (controller assigns exist) but no typed lease id, expiry time, or revocation state is stored in SQLite or returned in the read model.
3. `ObserverPermission` does not exist. Co-parent and observer are not distinguishable in the current model.
4. Portal tests cover LAN slot/parser fixture states but have no tests for full setup, recovery, or degraded first-run states (feature doc checklist row open).
5. Parent recovery UX is absent — no typed flow for "my parent device was lost/replaced, re-pair to the household."
6. Source labels `local`, `lan`, `relay`, `cache`, `parent-owned storage`, `unavailable` exist as read-model labels but are not enforced as a closed enum in the TypeScript domain; raw strings can be passed.

## Where We Want To Be

Map every gap to a specific file/crate change, tag it as: **implement now** (contracts exist from WP01), **deferred to later workpack**, or **manual-required**. A future agent must be able to read this workpack, see the gap table, and know exactly which file to open first.

## Scope

Gap table — implement all "implement now" rows in this workpack:

| Gap                                           | File/Crate                                                           | Action                                                                                                          | Target WP       |
| --------------------------------------------- | -------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------- |
| `HouseholdProfile` not stored in SQLite       | `crates/agent-service/src/db/household.rs` (new)                     | Add SQLite table `households`, `child_profiles`, `parent_members` and CRUD handlers                             | now             |
| `HouseholdProfile` not in read model response | `crates/agent-service/src/read_models/household_read_model.rs` (new) | Emit `HouseholdReadModel` on `agent.household.read-model.get` command                                           | now             |
| `ParentControllerLease` missing expiry/revoke | `crates/agent-service/src/db/leases.rs` (new)                        | Add `controller_leases` table with `issued_at`, `expires_at`, `revocation_state`; check expiry on every command | now             |
| `ObserverPermission` not modelled             | `crates/agent-service/src/db/permissions.rs` (new)                   | Add `observer_permissions` table; reject write commands from observer role                                      | now             |
| Route label closed enum missing               | `packages/parent-domain/src/device-registration.ts`                  | Replace `routeState: string` with `RouteState` Effect Schema enum                                               | now             |
| Portal missing full setup/recovery tests      | `packages/portal/tests/family-setup.playwright.ts` (new)             | Add Playwright tests for: create household, add child profile, assign device, revoke device, recovery flow      | WP04            |
| Physical two-device LAN proof                 | manual                                                               | Record manual-required; cannot be automated in CI                                                               | manual-required |

For the "implement now" rows: implement SQLite tables and read model handlers in Rust, then write integration tests confirming the read model returns typed household data when the service is queried.

## Touched Paths

- `crates/agent-service/src/db/household.rs` (new)
- `crates/agent-service/src/db/leases.rs` (new)
- `crates/agent-service/src/db/permissions.rs` (new)
- `crates/agent-service/src/db/mod.rs` (add new modules)
- `crates/agent-service/src/read_models/household_read_model.rs` (new)
- `crates/agent-service/src/read_models/mod.rs` (add `household_read_model`)
- `packages/parent-domain/src/device-registration.ts` (fix `routeState` enum)

## Tests And Proof

- Rust integration test: insert a `HouseholdProfile` via the db layer, query the read model handler, assert the response JSON matches the WP01 TypeScript schema shape.
- Rust negative test: command from an observer role that requests a write action must return `PermissionDenied`.
- Rust negative test: controller lease past `expires_at` must be rejected by the command handler with `LeaseExpired`.
- Rust negative test: wrong-device command (device id mismatch) must return `WrongDevice`.
- SQLite migration test: run schema up-migration, verify all tables created, run rollback, verify clean.
- TypeScript test: `RouteState` enum rejects raw string `"foo"` and accepts only `"local" | "lan" | "relay" | "cache" | "unavailable"`.
- Proof artifact: `output/lan-plan-proof/02-current-state-and-gap-map/02-gap-map-integration-proof.log`.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md), [family setup expectations](../../expectations/family-setup.md), [LAN pairing expectations](../../expectations/lan-pairing.md), [current PLAN_STATE](../PLAN_STATE.md), and this workpack.
- [ ] Confirmed WP01 contracts exist in `packages/parent-domain/src/` before starting Rust implementation.
- [ ] Check enhancement overlap: `lan-plan` owns discovery/heartbeat rows; do not duplicate them in household tables.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing `crates/agent-service/src/db/` inspected; no parallel table for household truth that conflicts with existing LAN slot tables.
- [ ] SQLite migration tested: up and rollback verified.
- [ ] Read model handler tested with typed Rust integration test using WP01 schema shape.
- [ ] Observer write-command rejection test written and passes.
- [ ] Lease expiry rejection test written and passes.
- [ ] `RouteState` enum fix in TypeScript tested; raw string rejected.
- [ ] Proof command logs saved to `output/lan-plan-proof/02-current-state-and-gap-map/`.
- [ ] [main checklist](../implementation-checklist.md) rows 02 updated.
- [ ] Deferred items (Playwright tests, physical LAN proof) recorded in [main checklist](../implementation-checklist.md) as open.

## Manual-Required Gaps

Physical two-device LAN proof cannot be automated in CI. The SQLite tables and
read model can be integration-tested with a single-machine local service, but
real household multi-device proof requires two distinct hosts running signed
hello/heartbeat. Mark `signed hello/heartbeat` rows manual-required in the
checklist; they belong to workpack 03.
