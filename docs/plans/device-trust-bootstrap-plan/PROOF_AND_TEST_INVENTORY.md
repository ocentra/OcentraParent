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

## Storage rule

Collect proof in the designated local artifact path or crate-local proof folder. Do not store proof inventories inside the plan folder.