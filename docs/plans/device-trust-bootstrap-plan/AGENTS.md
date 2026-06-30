<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: AGENTS
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after a global route selects it.
> Stop rule: Choose one workpack; do not inspect sibling plans unless the selected workpack names a handoff.
> Proves: local routing and ownership only.
> Does not prove: implementation completion, deployed product behavior, trust recovery, entitlement unlock, or PR readiness.
> Proof rule: Route changes must keep PLAN_STATE, WORKPACK_INDEX, TEST_PROOF_EXPECTATIONS, PLAN_INDEX, and FEATURE_ROUTE_INDEX aligned.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Agent Route

Task: define the parent and child device trust bootstrap layer that sits between account/household authority and high-risk product actions.
Context: this plan owns the "pair once, trust once, seal locally" boundary. It must prove parent presence, device sealing, QR/phone approval, signed entitlement unlock, recovery/reset, and anti-tamper expectations without inventing biometric storage or a custom identity provider.
Scope: trusted parent device, trusted child device, local sealed keys, passkey/platform approval, phone QR action approval, encrypted recovery bundle, signed entitlement snapshot, child uninstall authorization, recovery/reset/re-pair, and dependency adoption proof.
Out of scope: account provider implementation, child capture adapters, data storage implementation, package build mechanics, remote relay implementation, policy authoring UX, and custom biometric storage.

## High-Density Execution Contract

- Route first from [PLAN_STATE.md](PLAN_STATE.md).
- Work one workpack at a time and the exact proof/checklist rows named by that workpack.
- Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.
- Proof lives outside this plan folder; the plan only indexes proof locations.
- Do not turn login success into trust success.
- Do not treat copied binaries, copied config, LAN pairing, package install, or license state as license or trust proof.
- No DONE or PR_READY claim may skip the device-trust, recovery, tamper, and route-sync boundaries.

## Ownership, Import, And Boundary Contract

This plan owns device-trust bootstrap rules, proof, and cross-plan handoff boundaries. It does not own account identity, setup journey, LAN transport, package distribution, billing entitlement policy, data custody runtime, remote access grants, or policy UX.

Module roles:

```text
crates/schema or the owning Rust crate: canonical shared trust state, device registration, parent step-up assertion, QR approval, recovery, entitlement binding, tamper/uninstall, route-handoff, and route/action/read-model DTO shapes when those shapes cross package, crate, app, or plan boundaries.
schema-domain: temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
family-domain: household/role/action authorization helper surface that consumes Rust-owned/generated family contracts. It is not the full device-trust runtime.
lan-domain: LAN pairing and selected-device proof consumer. LAN pairing is not the device-trust root.
agent-protocol and agent-service: protocol/service proof only when the selected workpack names runtime or wire behavior.
setup-install-provisioning-plan: install/setup journey owner and first-run handoff producer.
account-identity-family-plan: account, household, role, session, invite, and membership authority owner.
data-custody-storage-plan: encrypted storage, export/import, restore, and recovery artifact custody after device trust exists.
payment-subscription-plan: subscription entitlement policy and billing state owner.
parent/child runtime distribution plans: package build, signing, update, rollback, installer, and child package mechanics owners.
remote-access-plan and policy-control-plane-plan: standing access grants and policy delivery consumers after device trust exists.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
Rust-owned canonical trust/device/step-up/recovery/entitlement/tamper handoff shapes plus generated DTOs or temporary edge decoders
family-domain public household authority helpers when actor/role proof is selected
lan-domain public package metadata or selected pairing helper surfaces when the selected workpack names LAN proof
agent-protocol/agent-service public surfaces only when runtime/protocol proof is selected
neutral event/evidence/logging/protocol primitives
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports and claims:

```text
account provider internals imported as trust proof
setup UI or package installer internals imported as trust proof
LAN transport or pairing internals upgraded into trust-root proof
payment entitlement state upgraded into product unlock proof
package install/copy/config proof upgraded into device trust proof
WebAuthn/passkey contract shape upgraded into platform ceremony proof
QR shape upgraded into phone approval bridge proof without action binding, expiry, replay rejection, and audit proof
local-key model upgraded into platform-backed sealing runtime proof
recovery docs upgraded into encrypted recovery execution proof without wrong-household, wrong-device, wrong-key, and replay negatives
child device runtime allowed to control, disable, or silently remove trust
```

If device-trust work needs setup, account, LAN, payment, data custody, runtime distribution, remote access, policy, portal, or child runtime behavior, it must use typed evidence, commands, events, requests, read models, artifact manifests, proof roots, and explicit handoffs. If a shape is used by multiple feature owners, place or consume it through `crates/schema` or another neutral Rust-owned boundary. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Research Gate

Before DONE or PR_READY, inspect only the selected slice of existing repo code/docs, map current behavior versus missing behavior, and record unresolved product or architecture choices with Sujan. Research direction already points toward WebAuthn/passkeys for parent presence proof, platform-backed local key sealing, encrypted recovery bundles, and RustDesk only as architecture reference material.

## Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear
5. One assigned workpack only
6. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
7. [PROOF_INDEX.md](PROOF_INDEX.md) only for proof claims

## Decision Tree

| If the task is about... | Open |
| --- | --- |
| Trust source, trust state, or ownership model | `workpacks/01-device-trust-source-of-truth.md` |
| Local key sealing, platform stores, recovery key material | `workpacks/02-local-key-sealing.md` |
| Parent step-up, passkeys, biometrics, OS auth | `workpacks/03-parent-step-up-auth.md` |
| Desktop QR approval, phone approval bridge, replay protection | `workpacks/04-phone-qr-approval-bridge.md` |
| Signed entitlement snapshots and device-bound license behavior | `workpacks/05-entitlement-device-license.md` |
| Recovery reset, re-pair, and encrypted recovery bundles | `workpacks/06-recovery-reset-re-pair.md` |
| Child tamper, uninstall, and anti-tamper boundaries | `workpacks/07-child-tamper-uninstall.md` |
| Dependency adoption and security review | `workpacks/08-open-source-dependency-adoption.md` |
| Cross-plan route sync and proof gate | `workpacks/09-cross-plan-route-gate.md` |

## Ownership Boundaries

- `setup-install-provisioning-plan` owns parent and child install journeys, first-run packaging handoff, and the public setup path that feeds device trust.
- `account-identity-family-plan` owns account, household, role, session, invite, and membership authority.
- `data-custody-storage-plan` owns encrypted storage, export/import, restore, and recovery artifacts after device trust exists.
- `remote-access-plan` owns standing live access grants after device trust exists.
- `payment-subscription-plan` owns subscription entitlement policy and billing state.
- `policy-control-plane-plan` owns policy authoring and delivery after device trust exists.
- `parent-client-runtime-distribution-plan` and child runtime distribution plans own package build, signing, update, rollback, and installer mechanics.

## Failure Conditions

- Do not store biometric templates or custom recognition data.
- Do not create plaintext trust keys or universal decrypt keys.
- Do not let login alone imply device trust.
- Do not let license state alone unlock product behavior.
- Do not let LAN pairing or package install imply device trust.
- Do not let a child device control, disable, or silently remove trust.
- Do not claim recovery without encrypted bundle handling and wrong-household / wrong-device / wrong-key negative cases.
- Do not claim PR-ready without proof pointers outside this folder.
