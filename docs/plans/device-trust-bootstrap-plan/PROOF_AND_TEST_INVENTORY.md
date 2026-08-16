# Proof and Test Inventory

Purpose: enumerate the proof families and the validation shape expected for each trust slice. This document does not store proof artifacts.

## Inventory

| Workpack | Test shape | Proof artifact shape | Manual-required notes |
| --- | --- | --- | --- |
| 01-device-trust-source-of-truth | State-machine and boundary checks. | Trust model note plus route-sync proof. | Keep login separate from trust. |
| 02-local-key-sealing | Platform store round-trip and negative-device tests. | Platform sealing matrix plus wrong-device proof. | Never store plaintext trust keys. |
| 03-parent-step-up-auth | Step-up and replay/expiry tests. | Step-up policy proof. | Use OS-native prompts or passkeys. |
| 04-phone-qr-approval-bridge | QR challenge binding and replay tests. | One-time approval proof. | Action-bound, household-bound, target-bound. |
| 05-entitlement-device-license | Signature and revocation tests. | Signed entitlement snapshot proof. | License alone must not unlock behavior. |
| 06-recovery-reset-re-pair | Wrong-household, wrong-key, and re-pair tests. | Recovery bundle proof. | Recovery is not account login. |
| 07-child-tamper-uninstall | Tamper and uninstall denial tests. | Anti-tamper proof. | Child cannot self-authorize removal. |
| 08-open-source-dependency-adoption | Review matrix checks for each dependency. | Adoption decision proof. | Adopt, research-only, or reject. |
| 09-cross-plan-route-gate | Route and index consistency checks. | Route sync proof. | No stale route claims. |

## Current real validation owners

- Plan-local `tests/device-trust-bootstrap-plan/` currently proves document and route alignment.
- `packages/family-domain` currently owns the strongest real typed coverage for trust-adjacent authority and recovery.
- `packages/lan-domain` plus Rust LAN pairing tests currently own the strongest real typed coverage for trusted-device registry, selection, route recovery, and explicit manual-proof gaps.
- `packages/parent-domain` is not a reliable proof owner for this slice until the current re-export architecture violations are resolved.

## Platform expectations by slice

- WP02 local key sealing: Windows, Android, and Linux proof are expected when implementation lands; iOS/macOS real proof is an external-platform constraint from this host.
- WP03 parent step-up: Windows contract/integration proof is expected; Android proof is expected for phone-backed or mobile-adjacent approval paths; iOS/macOS real proof is an external-platform constraint from this host.
- WP04 phone QR approval: Windows desktop plus Android emulator/device proof is expected where relevant; iOS phone proof is an external-platform constraint from this host.
- WP05 entitlement device license: Windows and Linux proof are expected where the runtime slice lands; Android proof is expected if child-device entitlement consumption is touched.
- WP06 recovery/reset/re-pair: Windows and Linux proof are expected for bundle and restore behavior; Android proof is expected when mobile or child-device restore paths are touched; iOS/macOS real proof is an external-platform constraint from this host.
- WP07 child tamper/uninstall: Windows proof is expected where Windows runtime paths exist; Android proof is expected where Android child uninstall or tamper paths exist; iOS/macOS real proof is an external-platform constraint from this host.

## Blocker reporting rule

Every proof note that records blocked validation must separate:

1. real dependency blockers
2. external platform constraints
3. avoidable local execution gaps

## Storage rule

Collect proof in the designated local artifact path or crate-local proof folder. Do not store proof inventories inside the plan folder.
