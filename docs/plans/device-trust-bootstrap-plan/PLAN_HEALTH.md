# Device Trust Bootstrap Plan Health

Status: blocked / not complete.

## Current health

- Architecture docs and route indexes exist, but they were overclaiming completion and proof presence before the audit truth sync.
- Device-trust tests live under `tests/device-trust-bootstrap-plan/` with major category folders, but the current suite is mostly document and route-shape coverage.
- No proof artifacts currently exist on disk under `output/device-trust-bootstrap-plan-proof/`.
- WP04 through WP07 had stale legacy `docs/proof/...` pointers before this sync.
- The plan is not complete at the implementation, proof, or validation level.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist/proof row.
- Update `PLAN_STATE.md` and `NEXT_ACTIONS.md` if current state changes.
- Update adjacent route docs only when a typed handoff claim changes.
- Do not claim READY from document creation or document tests alone.
- Do not claim READY from route-alignment tests alone.
- Do not claim READY from login/session proof.
- Do not claim READY from LAN pairing proof.
- Do not claim READY from package install, copied binary, or copied config proof.
- Do not claim READY from license or entitlement state alone.
- Do not claim READY from WebAuthn/passkey contract shape without platform ceremony proof when the claim requires it.
- Do not claim READY from QR challenge shape without replay, expiry, action-binding, and audit proof.
- Do not claim READY from local-key model without platform-backed sealing runtime proof.
- Do not claim READY from recovery docs without encrypted bundle handling and wrong-household/device/key negatives.
- Do not claim READY from child uninstall/tamper docs without parent-authorized revocation and package/runtime handoff proof.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
account login/session authority
device trust source-of-truth
local key sealing
parent step-up approval
phone QR approval bridge
entitlement-device binding
recovery reset and re-pair
child tamper/uninstall authorization
dependency adoption review
cross-plan route gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
WP01 trust source-of-truth proof
WP02 platform-backed local key sealing proof
WP03 parent step-up runtime proof
WP04 phone QR approval bridge runtime proof
WP05 signed entitlement/device binding proof
WP06 encrypted recovery/reset/re-pair proof
WP07 child tamper/uninstall authorization proof
WP08 dependency adoption proof
WP09 cross-plan route gate proof
```

## Rejection conditions

The plan is unhealthy if:

```text
login/session proof is used as trust proof
LAN pairing is used as trust-root proof
package install/copy is used as trust proof
license state is used as unlock proof
child device can disable, control, or silently remove trust
recovery can revive revoked trust without parent-approved re-pair
trust keys are modeled as plaintext or universal decrypt keys
platform unsupported/manual-required states are hidden
proof/checklist changed before source/tests for implementation work
```

## Agent route walkthrough

- Landing decision: root plan routing selects this plan for device-trust bootstrap, local sealed trust state, parent step-up, phone QR approval, entitlement-device binding, recovery/reset/re-pair, child tamper/uninstall authorization, dependency adoption, and route gate proof.
- Scope split: account identity, setup journey, LAN transport, package distribution, payment entitlement policy, data custody runtime, remote access grants, and policy UX stay in sibling plans unless the selected workpack names a typed handoff.
- Minimum read set: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, one workpack, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md` when validating proof.
- Test/proof decision: require trust source-of-truth, key sealing, parent step-up, phone QR approval, entitlement-device binding, recovery, child tamper/uninstall, dependency adoption, and route-gate tiers only where the selected workpack claims them.
- DONE blocker: no device-trust claim may move unless proof distinguishes account login, setup, LAN pairing, package install, license state, trust state, key sealing, approval, recovery, tamper/uninstall, and no-claim boundaries.

## PR-ready rule

The whole plan is PR-ready only when WP09 consumes or blocks every earlier proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, blocker classification, no-claim language, and remaining open workpacks listed.
