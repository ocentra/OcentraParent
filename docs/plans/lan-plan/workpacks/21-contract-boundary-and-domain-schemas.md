# 21 Rust-Owned Contract Boundary And Domain Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `21 Rust-Owned Contract Boundary And Domain Schemas`
> Kind: assigned active workpack; read only when this exact workpack is selected.
> Read when: Only when this exact workpack is explicitly selected from `WORKPACK_INDEX.md`.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack's own proof rows and tests support the claim.
> Proves: only this workpack's current boundary scope and any progress explicitly recorded here.
> Does not prove: current completion of sibling workpacks or broad LAN readiness.
> Proof rule: Rewrite or discard any stale historical assumptions before using this file for execution claims.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[family setup expectations](../../../expectations/family-setup.md),
[LAN pairing expectations](../../../expectations/lan-pairing.md).

## Active scope status

This workpack is part of the authoritative `01-25` LAN execution model. It is
locally complete for the current Rust-owned contract-boundary slice.

Historical TS-first notes in older copies of this draft are stale. Current
direction for this workpack is:

- Rust owns the contracts and business logic.
- `crates/schema` or another Rust-owned shared boundary owns cross-surface
  household/role shapes.
- `crates/agent-protocol` and `crates/agent-service` own protocol/runtime parity.
- TS may consume generated bridge artifacts only at the presentation edge. TS
  does not become the contract authority, runtime owner, read-model owner, or
  proof owner.

## Where We Are

The current LAN spine already has Rust-owned discovery, route-custody,
stale/offline, and signed-discovery labels. This locally complete workpack
records the household/role contract family closure for:

- `HouseholdProfile`
- `ChildProfile`
- `ParentMember`
- `ParentControllerLease`
- `ObserverPermission`
- `SetupInvite`
- `RecoveryState`
- `SetupAuditEvent`

Those shapes are now represented in the Rust-owned household/setup contract
family under `crates/family-identity-core`, including the record-shaped
`family_identity::RecoveryState`. They are execution truth only for this
contract slice; downstream runtime, portal, invite delivery, recovery UX, and
physical LAN behavior still need their own selected proof.

## Where We Want To Be

Every household and role concept lives as a Rust-owned shared contract before
any runtime code, service handler, or portal surface claims to display or
enforce it.

UI-facing bridge artifacts may be generated from those Rust contracts, but the
UI remains presentation only.

## Active Scope

- Define the household/role contract family in the owning Rust schema boundary.
- Add Rust protocol/runtime parity only after the shared Rust contracts are
  stable.
- Generate UI edge types only where the UI must render or dispatch through the
  host bridge.
- Keep this slice schema-focused; do not smuggle runtime/UI ownership into the
  contract packet.

## Contract family inventory

The remaining shape family for this workpack includes:

- `HouseholdProfile`: `householdId`, `displayName`, `createdAt`,
  `parentMemberIds[]`, `childProfileIds[]`
- `ParentMember`: `memberId`, `householdId`, `role`, `inviteState`, `joinedAt`
- `ChildProfile`: `childId`, `householdId`, `displayName`, `deviceIds[]`,
  `custodyLabel`
- `DeviceRegistration`: `deviceId`, `childId`, `householdId`, `trustState`,
  `roleLabel`, `routeState`, `staleSince?`
- `ParentControllerLease`: `leaseId`, `parentMemberId`, `deviceId`, `issuedAt`,
  `expiresAt`, `revocationState`
- `ObserverPermission`: `permId`, `parentMemberId`, `householdId`,
  `grantedScopes[]`, `isWriteBlocked`
- `SetupInvite`: `inviteId`, `householdId`, `inviteeEmail`, `role`, `expiresAt`
- `RecoveryState`: `recoveryId`, `deviceId`, `reason`, `parentActionRequired`
- `SetupAuditEvent`: `eventId`, `householdId`, `actorMemberId`, `targetId`,
  `action`, `timestamp`, `evidenceRef?`

## Tests And Proof

- Rust contract parse/reject tests for every new shape in the owning schema or
  protocol crate test trees.
- Rust serde round-trip and invalid-state tests for every household/role shape.
- Generated bridge drift checks may exist only as supporting interface sanity
  where the UI consumes the shape. They do not create LAN proof closure.
- Test proof must live in real organized Rust crate test folders. Inline
  source-owned tests, placeholder directories, `.gitkeep` trees, fake
  coverage, or mock-only readiness do not count.
- Proof artifact: `output/lan-plan-proof/21-contract-boundary-and-domain-schemas/`
- Current proof: `output/lan-plan-proof/21-contract-boundary-and-domain-schemas/01-local-validation.md`
- Current validation: `cargo test -p ocentra-family-identity-core -- --nocapture`
  and `cargo lint-architecture crates/family-identity-core`, both green for
  this slice.

## AI Worker Checklist

- [x] Confirm WP21 is the assigned active workpack.
- [x] Rewrite any stale TS-first wording still present in this file before code moves.
- [x] Confirm the owning Rust schema boundary before implementation starts.
- [x] Keep TS out of contract ownership; generated UI edge types only.
- [x] No runtime read model, service handler, or portal UI should claim truth
      that the Rust contract layer does not already own.
- [x] Invalid-state rejection tests written for observer write-scope, wrong
      household device, and revoked-lease reuse.
- [x] Tests/proof listed above are implemented or explicitly marked
      manual-required with reason.

## Manual-Required Gaps

Contracts alone do not create runtime or UI truth. Any eventual invite
delivery, recovery UX, or multi-device LAN proof remains separate from this
contract slice unless a later proof packet proves otherwise.
