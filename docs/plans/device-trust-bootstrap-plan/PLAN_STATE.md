<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Use this to choose one next action; do not scan historical docs.
> Proves: current planning state only.
> Does not prove: product completion or implementation readiness.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan State

Status: blocked / audit-truth-synced / not complete.

## Current Truth

This plan owns the one-time trust bootstrap layer for parent and child devices. The product model is still: pair once, trust once, seal locally, and keep that trust until a parent revokes, removes, or resets the device.

Current direction from research and the pasted plan set:

- WebAuthn and passkeys are the right parent-presence proof foundation, including QR and hybrid cross-device flows.
- Biometric verification stays inside the authenticator or OS prompt; the relying party does not get biometric data.
- Local trust material must be sealed with platform-backed stores, not custom app-managed plaintext keys.
- Platform-backed local sealing is documented in `LOCAL_KEY_SEALING_MODEL.md` and `PLATFORM_KEY_CUSTODY_MATRIX.md`.
- Plan-local test folders now live under `tests/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- Recovery must use an encrypted bundle or equivalent sealed backup artifact; account recovery is not the same thing as data or device recovery.
- Device trust is separate from account login, subscription entitlement, policy delivery, and remote-access grant state.
- RustDesk is useful as architecture reference material for remote-desktop patterns, but not as embedded trust-root product code by default.
- Android Play Integrity is a supporting signal only; it is not the trust root.
- No proof roots currently exist on disk under `output/device-trust-bootstrap-plan-proof/` or `docs/proof/device-trust-bootstrap-plan/`.
- The current plan-local tests are mostly doc-shape and route-alignment checks, not runtime trust-bootstrap proof.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared trust state, device registration, parent step-up assertion, QR approval, recovery, entitlement binding, tamper/uninstall, and route-handoff shapes when they cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

family-domain:
  Household/role/action authorization helper surface consuming Rust-owned/generated contracts. It is trust-adjacent, not full device-trust runtime.

lan-domain:
  LAN pairing and selected-device proof consumer. It is transport/pairing-adjacent, not the trust root.

agent-protocol and agent-service:
  Protocol/service proof only when the selected workpack names runtime or wire behavior.

setup-install-provisioning-plan:
  Install/setup journey owner and first-run handoff producer.

account-identity-family-plan:
  Account, household, role, session, invite, and membership authority owner.

data-custody-storage-plan:
  Encrypted storage, export/import, restore, and recovery artifact custody after device trust exists.

payment-subscription-plan:
  Subscription entitlement policy and billing state owner.

parent/child runtime distribution plans:
  Package build, signing, update, rollback, installer, and child package mechanics owners.

remote-access-plan and policy-control-plane-plan:
  Standing access grant and policy delivery consumers after device trust exists.
```

## Current coupling risks

```text
- No execution-grade device-trust runtime module exists yet.
- `family-domain` contains trust-adjacent authority helpers but not platform key sealing, QR approval runtime, recovery bundle runtime, or trust-root state machine.
- `lan-domain` and LAN Rust seams contain pairing/selected-device proof consumers, but LAN pairing is not trust root proof.
- Current plan-local tests prove document and route shape only, not runtime trust.
- Login/session proof, LAN pairing proof, package install proof, and license proof are all insufficient for device trust.
- Platform key sealing is modelled but not proven at runtime.
- Recovery/reset/re-pair remains unproven without encrypted bundle handling and wrong-household/device/key negatives.
- Child tamper/uninstall remains unproven without parent-authorized revocation and package/runtime handoff proof.
```

## Current proof interpretation

```text
Document assertions are not runtime trust proof.
Route-alignment tests are not runtime trust proof.
WebAuthn/passkey schema proof is not platform ceremony proof.
QR challenge shape is not phone approval bridge proof.
Key-custody model proof is not platform-backed sealing proof.
Entitlement snapshot proof is not product unlock proof.
Recovery docs are not recovery execution proof.
Child uninstall/tamper docs are not parent-authorized uninstall proof.
WP09 can aggregate only accepted proof roots plus exact carried blockers.
```

## Proof Coverage

- Proof roots are planned under `output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/`, but they are absent on disk today.
- Device-trust tests now live under `tests/device-trust-bootstrap-plan/<major-category>/`.
- Current top-level categories are `unit`, `contract`, `integration`, `e2e`, and `security`.
- The legacy `docs/proof/device-trust-bootstrap-plan/*` path is also absent on disk.

## Verified implementation boundary

- `packages/family-domain` contains typed trust-adjacent authority and recovery contracts, including `DeviceTrustState`, privileged device actions, setup invite rules, and recovery authorization boundaries.
- `packages/lan-domain` plus the Rust LAN pairing runtime contain trusted-route and selected-device registry contracts, restart behavior, and explicit manual proof gaps for LAN pairing.
- `packages/parent-domain` is mostly frontage for this slice and currently fails the repo re-export architecture gate on the named LAN/tamper bridge files.
- No execution-grade device-trust state machine exists yet in repo code.
- No execution-grade local key sealing implementation exists yet in repo code.
- No execution-grade parent step-up, phone QR approval bridge, encrypted recovery bundle handling, entitlement-binding runtime, or child uninstall authorization runtime exists yet in repo code.
- Login alone does not create trust, child devices do not own the trust root, and revocation must win over stale state.

## Execution Gate

- Route and implementation continue from [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
- Update this plan only through the blueprint and the selected workpack.
- Do not mark this plan complete from checklist deltas alone.
- Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.
- Proof must be collected in the designated local artifact path or crate-local proof folder, not inside this plan folder.
- True completion remains blocked until the runtime ownership split is resolved across the actual source owners and real proof artifacts exist for the missing trust-bootstrap slices.
