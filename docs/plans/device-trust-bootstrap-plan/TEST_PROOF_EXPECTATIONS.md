# Test and Proof Expectations

Status: architecture-open.

## Purpose
This file tracks the validation shape for each workpack. It does not store proof artifacts.

## Required flow
- [ ] Write or update the trust model doc for the selected slice.
- [ ] Write or update tests or proof harnesses for the selected slice.
- [ ] Run the lightest compile, lint, or validation check that covers the touched boundary.
- [ ] Run the targeted tests or proof harness.
- [ ] Run the broader package or route validation required by the touched risk.
- [ ] Collect proof in the designated local artifact path or crate-local proof folder.
- [ ] Record the proof pointer outside the plan folder.

## Expectations by workpack

| Workpack | Expected validation | Proof target | Notes |
| --- | --- | --- | --- |
| 01-device-trust-source-of-truth | Trust state table checks, boundary review, route sync check. | `docs/proof/device-trust-bootstrap-plan/01-*` | No login-versus-trust conflation. |
| 02-local-key-sealing | Platform store round-trip tests and wrong-user / wrong-device negatives. | `docs/proof/device-trust-bootstrap-plan/02-*` | No plaintext trust keys. |
| 03-parent-step-up-auth | Step-up ceremony tests and replay / expiry negatives. | `docs/proof/device-trust-bootstrap-plan/03-*` | Use native OS prompts or passkeys. |
| 04-phone-qr-approval-bridge | QR challenge binding tests and one-time approval negatives. | `docs/proof/device-trust-bootstrap-plan/04-*` | Desktop-bound and action-bound. |
| 05-entitlement-device-license | Signature, expiry, revocation, and offline-grace tests. | `docs/proof/device-trust-bootstrap-plan/05-*` | License alone never unlocks behavior. |
| 06-recovery-reset-re-pair | Encrypted bundle, wrong-household, wrong-key, and re-pair tests. | `docs/proof/device-trust-bootstrap-plan/06-*` | Recovery is not account login. |
| 07-child-tamper-uninstall | Tamper detection, uninstall authorization, and revocation tests. | `docs/proof/device-trust-bootstrap-plan/07-*` | Child cannot self-authorize trust removal. |
| 08-open-source-dependency-adoption | License, maintenance, security, platform, and replaceability review. | `docs/proof/device-trust-bootstrap-plan/08-*` | Adopt, research-only, or reject must be explicit. |
| 09-cross-plan-route-gate | Plan-index and feature-route sync checks plus diff hygiene. | `docs/proof/device-trust-bootstrap-plan/09-*` | No route drift. |

## Proof storage

Proof artifacts live in the designated local artifact path for the workpack or crate, not in this plan folder.

## Failure conditions

- Do not mark DONE or PR_READY until the selected workpack's validation, negative cases, and proof pointer are complete.
- Do not store proof inventories inside the plan folder.