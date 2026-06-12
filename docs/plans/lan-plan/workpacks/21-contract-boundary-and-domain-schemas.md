# 01 Contract Boundary And Domain Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `01 Contract Boundary And Domain Schemas`
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

## Where We Are

`packages/parent-domain` owns the add-device read model, household device row
contract, and parent decision fields (assign/rename/ignore/restore/trust/revoke).
`packages/agent-protocol-domain` owns LAN route commands and discovery-state
shapes. `crates/agent-protocol` mirrors some of these in Rust. The V0.9 LAN
spine carries signed discovery, route custody, and stale/offline labels.

What does **not** exist yet: TypeScript schemas for `HouseholdProfile`,
`ChildProfile`, `ParentMember`, `ParentControllerLease`, `ObserverPermission`,
`SetupInvite`, `RecoveryState`, and `SetupAuditEvent`. The expectation doc at
`docs/expectations/family-setup.md` lists these as required contract families.
No Rust parity exists for household membership shapes. The read model carries
LAN-specific device rows but lacks a first-class household identity contract
that ties household id → parent members → child profiles → device registrations.

## Where We Want To Be

Every household and role concept must live as a typed Effect Schema contract in
the owning domain package before any runtime code, service handler, or portal
surface claims to display or enforce it. Rust protocol shapes must mirror the
TypeScript contracts before `crates/agent-service` accepts or emits the
payloads.

## Scope

- `HouseholdProfile` schema: `householdId`, `displayName`, `createdAt`, `parentMemberIds[]`, `childProfileIds[]`.
- `ParentMember` schema: `memberId`, `householdId`, `role` (`controller` | `co-parent` | `observer`), `inviteState`, `joinedAt`.
- `ChildProfile` schema: `childId`, `householdId`, `displayName`, `deviceIds[]`, `custodyLabel`.
- `DeviceRegistration` schema: `deviceId`, `childId`, `householdId`, `trustState`, `roleLabel`, `routeState` (`local` | `lan` | `relay` | `cache` | `unavailable`), `staleSince?`.
- `ParentControllerLease` schema: `leaseId`, `parentMemberId`, `deviceId`, `issuedAt`, `expiresAt`, `revocationState`.
- `ObserverPermission` schema: `permId`, `parentMemberId`, `householdId`, `grantedScopes[]`, `isWriteBlocked: true`.
- `SetupInvite` schema: `inviteId`, `householdId`, `inviteeEmail`, `role`, `expiresAt`, `acceptedAt?`, `revokedAt?`.
- `RecoveryState` schema: `recoveryId`, `deviceId`, `reason` (`stale` | `revoked` | `offline` | `lost`), `parentActionRequired`.
- `SetupAuditEvent` schema: `eventId`, `householdId`, `actorMemberId`, `targetId`, `action`, `timestamp`, `evidenceRef?`.
- Rust parity in `crates/agent-protocol/src/household.rs` for `HouseholdProfile`, `ChildProfile`, `DeviceRegistration`, and `ParentControllerLease`.
- Invalid-state rejection tests: observer with write scope must fail; device in wrong household must fail; revoked lease issued as valid must fail.

## Touched Paths

- `packages/parent-domain/src/household-profile.ts` (new)
- `packages/parent-domain/src/child-profile.ts` (new)
- `packages/parent-domain/src/parent-member.ts` (new)
- `packages/parent-domain/src/device-registration.ts` (new)
- `packages/parent-domain/src/controller-lease.ts` (new)
- `packages/parent-domain/src/observer-permission.ts` (new)
- `packages/parent-domain/src/setup-invite.ts` (new)
- `packages/parent-domain/src/recovery-state.ts` (new)
- `packages/parent-domain/src/setup-audit-event.ts` (new)
- `packages/parent-domain/src/index.ts` (re-export all above)
- `crates/agent-protocol/src/household.rs` (new)
- `crates/agent-protocol/src/lib.rs` (add `pub mod household;`)

## Tests And Proof

- TypeScript Effect Schema parse/reject tests for every shape in `packages/parent-domain/src/*.test.ts`.
- Rust serde round-trip and invalid-state tests in `crates/agent-protocol/src/household.rs` test module.
- No raw `string` annotations in domain types; all identifiers use opaque branded types or newtypes.
- Proof artifact: `output/lan-plan-proof/01-contract-boundary-and-domain-schemas/01-schema-validation-proof.log`.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [feature doc](../../features/family-setup-device-roles.md), [family setup expectations](../../expectations/family-setup.md), [LAN pairing expectations](../../expectations/lan-pairing.md), [current PLAN_STATE](../PLAN_STATE.md), and this workpack.
- [ ] Check enhancement overlap with adjacent plans: `lan-plan` (route/discovery shapes), `portal-ux-household-surfaces-plan` (read model consumption).
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no duplicate household truth created alongside existing add-device read model.
- [ ] Before-state snapshot recorded in `output/lan-plan-proof/01-contract-boundary-and-domain-schemas/00-source-snapshot.md`.
- [ ] Contracts written first; no Rust/service/portal changes until TypeScript contracts exist and pass schema tests.
- [ ] Rust parity added in `crates/agent-protocol/src/household.rs` after TypeScript contracts pass.
- [ ] No runtime read model, service handler, or portal UI changed by this workpack; schema-only slice.
- [ ] Invalid-state rejection tests written: observer write-scope rejected, wrong-household device rejected, revoked-lease re-issue rejected.
- [ ] Tests/proof listed above are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in `output/lan-plan-proof/01-contract-boundary-and-domain-schemas/` and summarized in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No runtime or UI claim is created by contracts alone. `SetupInvite` delivery via
email or push notification is out of scope for this workpack; record as deferred.
Physical multi-device LAN proof is manual-required and belongs to workpack 03.
