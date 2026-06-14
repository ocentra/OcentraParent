<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `KEY_CUSTODY_MODEL.md`
> Kind: key custody model.
> Read when: When a workpack needs the decrypt authority model and recovery boundary.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Key custody claims must match the platform matrix and bundle protocol.

<!-- /agent-capsule -->

# Data Custody Storage Plan Key Custody Model

## Key classes

| Key or secret | Default holder | Unlock purpose | May decrypt child evidence | May decrypt parent exports | Notes |
| --- | --- | --- | --- | --- | --- |
| Child device local key | Child device / child service | Unlock local evidence and journal material | Yes, on that device | No by default | Device-bound and role-bound |
| Parent desktop key | Parent desktop | Unlock parent cache, reports, and local parent-owned bundles | Only if custody model allows it | Yes | Parent-owned authority |
| Parent mobile key | Parent mobile app | Approve provider access and inspect status | Only if custody model allows it | Yes, for parent-owned bundles | View and approval role, not sole truth |
| Household recovery key | Household recovery path | Recover authorized household bundles when product supports recovery | Only if explicitly designed | Yes | Recovery is a product decision, not a default |
| Provider auth token | Provider connection | Access provider APIs for sync and delete | No | No | Token is not a decrypt key |
| Support secret or diagnostic token | Support flow | Limited support diagnostics only | No | No | Never a universal decrypt path |

## Custody rules

- Household-owned data is readable only by authorized household devices or components with valid role, pairing, and key material.
- A provider connection never becomes a decrypt authority just because it can store files.
- Parent access and child device access are separate roles even when they share a household.
- Revocation removes future access and must not silently preserve old decrypt power.
- Lost-key handling must be explicit. If recovery is not enabled, the product must fall back to manual-required.
- Support cannot be assumed to recover encrypted child activity. If recovery exists, it must be a deliberate parent-owned flow.

## Recovery and loss states

| State | Meaning | Expected behavior |
| --- | --- | --- |
| `keyAvailable` | Required key material exists | Normal flow may continue |
| `keyUnavailable` | Key material is missing | Mark manual-required |
| `keyRevoked` | Access was intentionally removed | Reject decrypt and restore |
| `wrongHousehold` | Bundle or key does not match household binding | Reject |
| `wrongDevice` | Device binding does not match | Reject or downgrade to preview only |
| `reinstallRequired` | Device reinstalled or profile lost | Re-provision or reject based on policy |
| `recoveryAvailable` | A deliberate recovery path exists | Follow recovery flow with proof |
| `recoveryNotSupported` | Product does not allow recovery | Manual-required remains true |

## Platform decision summary

- Windows is the first proof target.
- Android and iOS remain manual-required until device proof exists.
- Linux remains manual-required until a real secret-store decision exists.
- Web or hosted portal is not the decrypt root.
- Parent desktop is the primary near-term parent custody surface.
- Child service owns child-device local evidence.

## Proof anchors

- `data-custody.keys.hierarchy-contract`
- `data-custody.keys.platform-custody-matrix`
- `data-custody.keys.wrong-household-negative`
- `data-custody.keys.revocation-negative`
- `data-custody.keys.loss-manual-required`

