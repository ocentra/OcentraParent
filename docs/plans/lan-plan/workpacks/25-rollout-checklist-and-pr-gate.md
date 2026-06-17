# 05 Rollout Checklist And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `05 Rollout Checklist And PR Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[family setup expectations](../../../expectations/family-setup.md),
[LAN pairing expectations](../../../expectations/lan-pairing.md),
[PR/DONE flow](../../../agent/PR_DONE_FLOW.md).
Assumes WP01–04 all complete.

## Where We Are

At the start of WP05 all prior workpacks must be done:

- WP01: Domain schemas in `packages/parent-domain/src/` and Rust parity in `crates/agent-protocol/src/household.rs`.
- WP02: SQLite tables `households`, `child_profiles`, `parent_members`, `controller_leases`, `observer_permissions`; `HouseholdReadModel` handler; `RouteState` closed enum.
- WP03: `household_commands.rs` with `RevokeDevice`, `StartRecovery`, `ConfirmPairing`; `setup_audit_events` table; stale-TTL read model; wrong-device rejection; `family-setup-pairing-and-route-proof.mjs` proof script.
- WP04: `/setup` wizard routes, `DeviceRecoveryPanel`, `DeviceRouteLabel`, Playwright tests covering setup/recovery/degraded/observer-role states.

## PR Gate: Required Before Any PR_READY Claim

All of the following must be true. If any is false, keep the plan state open.

### Automated gates (must pass in CI)

| Check               | Command                                                                 | Expected Output                                                                                      |
| ------------------- | ----------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| TypeScript compile  | `pnpm tsc --noEmit -p packages/parent-domain/tsconfig.json`             | No errors                                                                                            |
| Parent-domain tests | `pnpm test --filter parent-domain`                                      | All household/child/lease/invite schemas pass                                                        |
| Rust compile        | `cargo build -p agent-protocol -p agent-service`                        | No errors                                                                                            |
| Rust tests          | `cargo test -p agent-protocol -p agent-service -- household`            | All household tests pass                                                                             |
| Portal Playwright   | `pnpm playwright test packages/portal/tests/family-setup.playwright.ts` | All 5 tests pass; screenshots saved                                                                  |
| Integration script  | `node scripts/test/family-setup-pairing-and-route-proof.mjs`            | Proof JSON written to `output/lan-plan-proof/03-pairing-and-route-proof/03-pairing-cycle-proof.json` |

### Proof artifact checklist

- [ ] `output/lan-plan-proof/01-contract-boundary-and-domain-schemas/01-schema-validation-proof.log` exists and shows 0 failures.
- [ ] `output/lan-plan-proof/02-current-state-and-gap-map/02-gap-map-integration-proof.log` exists; SQLite migration up/rollback shown.
- [ ] `output/lan-plan-proof/03-pairing-and-route-proof/03-pairing-cycle-proof.json` exists; revoke→reject→recover→re-pair cycle shown.
- [ ] `output/lan-plan-proof/04-portal-ux-and-first-run-handoff/06-ui-snapshots/` contains screenshots for: setup-wizard-step-1, setup-wizard-step-3-device, setup-summary, recovery-panel-stale, recovery-panel-confirmed, degraded-service-offline, source-label-relay, source-label-cache, observer-role-readonly.
- [ ] `output/lan-plan-proof/05-rollout-checklist-and-pr-gate/05-pr-gate-validation.log` exists; all CI commands recorded with pass/fail output.

### Security no-claim checks

- [ ] No portal surface claims a device is "protected" before `capabilityStatus` confirms the child agent is active and capable.
- [ ] Observer role cannot issue `AssignDevice`, `RenameDevice`, `TrustDevice`, `RevokeDevice`, `StartRecovery`, or `ConfirmPairing` commands — confirmed by Rust test.
- [ ] Revoked device command rejection confirmed by Rust test; audit event confirmed in SQLite.
- [ ] Wrong-device rejection confirmed by Rust test; audit event confirmed in SQLite.
- [ ] `SetupInvite` delivery (email/SMS) is explicitly not claimed; deferred item recorded.

### Feature doc checklist update

Update `docs/features/family-setup-device-roles.md` to move the following rows from `[ ]` to `[ ]` only after proof artifacts exist:

- "Household profile contract" → check after WP01 + WP02 proof.
- "Child profile contract and UI" → check after WP01 + WP04 Playwright proof.
- "Parent-controller and parent-observer role UI" → check after WP04 observer-role test.
- "First-run add-device UX" → check after WP04 Playwright wizard proof.
- "Revocation and recovery flow" → check after WP03 revoke + WP04 recovery panel proof.
- "Source labels: local, LAN, relay, cache, parent-owned storage, unavailable" → check after WP04 source-label tooltip proof.
- "Portal tests for full setup, recovery, and degraded first-run states" → check after WP04 all Playwright tests pass.

Do **not** check "Real LAN proof before claiming multi-device household readiness" — this row remains manual-required until two physical hosts are proven.

### Manual-required proof before broad household LAN readiness claim

If the team wants to claim V0.9 household multi-device readiness:

- Two distinct physical hosts (parent host + child host on different IPs).
- Both running `crates/agent-service` with signed hello/heartbeat.
- Full pair → assign → revoke → recover cycle executed on real hardware.
- Evidence: `output/lan-plan-proof/05-rollout-checklist-and-pr-gate/05-manual-two-device-lan-proof.md` containing:
  - Service logs from both hosts showing pairing handshake.
  - Packet capture excerpt showing signed advertisement.
  - Screenshot of portal Devices/LAN showing two distinct hosts with `lan` route label.
  - Date, hostnames, network topology (e.g., same subnet, distinct IPs).

## Touched Paths

- `output/lan-plan-proof/05-rollout-checklist-and-pr-gate/` (new — proof logs)
- `docs/features/family-setup-device-roles.md` (update checklist rows after proof)
- `docs/plans/account-identity-family-plan/PLAN_STATE.md` (update family/account workpack summary when proof changes shared household readiness)
- `docs/product-capability-checklist.md` (update household readiness row after proof)

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md), [LAN pairing expectations](../../../expectations/lan-pairing.md), [PR/DONE flow](../../../agent/PR_DONE_FLOW.md), [current PLAN_STATE](../PLAN_STATE.md), and this workpack.
- [ ] All WP01–04 checklist rows confirmed complete; no open items in prior workpacks.
- [ ] All automated CI commands run and outputs saved to proof folder.
- [ ] All proof artifact files exist at the paths listed above.
- [ ] Security no-claim checks confirmed; no false "protected" claim in portal.
- [ ] Feature doc checklist rows updated (only provable rows; LAN multi-device row stays open).
- [ ] `PLAN_STATE.md` checklist summary updated to reflect current checked/unchecked counts.
- [ ] `docs/product-capability-checklist.md` row updated only for claims with proof artifacts.
- [ ] If manual two-device LAN proof is complete: `05-manual-two-device-lan-proof.md` exists in proof folder.
- [ ] If manual proof is not yet complete: row stays open; no claim of multi-device household readiness.

## Manual-Required Gaps

Two-device physical LAN pairing remains the final gap before production household
multi-device readiness can be claimed. All automated proof can be complete while
this row stays open. Do not close the feature doc LAN readiness row until
`05-manual-two-device-lan-proof.md` exists.

Co-parent invite email delivery, push notification routing, and iOS/Android
child agent pairing all remain deferred and must not be claimed by this plan.
