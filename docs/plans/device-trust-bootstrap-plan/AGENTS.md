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
Out of scope: account provider implementation, child capture adapters, data storage implementation, package build mechanics, remote relay implementation, policy authoring UX, and custom biometric/face/retina storage.

## High-Density Execution Contract

- Route first from [PLAN_STATE.md](PLAN_STATE.md).
- Work one workpack at a time and the exact proof/checklist rows named by that workpack.
- Proof lives outside this plan folder; the plan only indexes proof locations.
- Do not turn login success into trust success.
- Do not treat copied binaries or copied config as license or trust proof.
- No DONE or PR_READY claim may skip the device-trust, recovery, tamper, and route-sync boundaries.

## Research Gate

Before DONE or PR_READY, inspect existing repo code/docs for the touched slice, map current behavior versus missing behavior, and record unresolved product or architecture choices with Sujan. Research direction already points toward WebAuthn/passkeys for parent presence proof, platform-backed local key sealing, age/rage-style encrypted bundles, and RustDesk only as architecture reference material.

## Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. One assigned workpack only
5. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
6. [PROOF_INDEX.md](PROOF_INDEX.md) only for proof claims

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
- `parent-client-runtime-distribution-plan` owns package build, signing, update, rollback, and installer mechanics.

## Failure Conditions

- Do not store biometric templates, face scans, retina scans, or custom recognition data.
- Do not create plaintext trust keys or universal decrypt keys.
- Do not let login alone imply device trust.
- Do not let license state alone unlock product behavior.
- Do not let a child device control, disable, or silently remove trust.
- Do not claim recovery without encrypted bundle handling and wrong-household / wrong-device negative cases.
- Do not claim PR-ready without proof pointers outside this folder.
