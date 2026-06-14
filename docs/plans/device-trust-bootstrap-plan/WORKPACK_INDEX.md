<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `WORKPACK_INDEX.md`
> Kind: workpack chooser; do not read all workpacks.
> Read when: After PLAN_STATE.md and NEXT_ACTIONS.md and before opening any workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this index changes, update PLAN_STATE.md and route sync docs.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Workpack Index

T00 route/no-overclaim proof is handled in [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md) and [PROOF_AND_TEST_INVENTORY.md](PROOF_AND_TEST_INVENTORY.md), not as a workpack.

| Workpack | Purpose | Status |
| --- | --- | --- |
| [01-device-trust-source-of-truth](workpacks/01-device-trust-source-of-truth.md) | Define trust ownership, trust states, bootstrap lifecycle, and cross-plan boundaries. | Planned |
| [02-local-key-sealing](workpacks/02-local-key-sealing.md) | Define platform-backed key sealing, fallback behavior, and wrong-device negative cases. | Planned |
| [03-parent-step-up-auth](workpacks/03-parent-step-up-auth.md) | Define parent step-up auth with passkeys, biometrics, and OS-native approval. | Planned |
| [04-phone-qr-approval-bridge](workpacks/04-phone-qr-approval-bridge.md) | Define desktop QR approval, phone approval, and replay-resistant action binding. | Planned |
| [05-entitlement-device-license](workpacks/05-entitlement-device-license.md) | Define signed entitlement snapshots and device-bound license unlock. | Planned |
| [06-recovery-reset-re-pair](workpacks/06-recovery-reset-re-pair.md) | Define encrypted recovery bundles, reset, revoke, and re-pair flows. | Planned |
| [07-child-tamper-uninstall](workpacks/07-child-tamper-uninstall.md) | Define child tamper, uninstall, and anti-tamper boundaries. | Planned |
| [08-open-source-dependency-adoption](workpacks/08-open-source-dependency-adoption.md) | Evaluate WebAuthn, keyring, and encrypted-bundle dependencies for adoption. | Planned |
| [09-cross-plan-route-gate](workpacks/09-cross-plan-route-gate.md) | Sync adjacent plan routes, feature routes, and proof gates. | Planned |