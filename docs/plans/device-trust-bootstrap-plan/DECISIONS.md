# Decisions

This document records the core architecture choices for device trust bootstrap.

| ID | Decision | Status | Why it matters |
| --- | --- | --- | --- |
| DTB-001 | Use WebAuthn/passkeys for parent presence proof and step-up. | Proposed | Gives a standards-backed, phishing-resistant trust root. |
| DTB-002 | Use platform-backed local key sealing for trust material. | Proposed | Prevents plaintext trust keys and avoids app-managed crypto roots. |
| DTB-003 | Use QR or hybrid cross-device approval for desktop-to-phone action approval. | Proposed | Matches the user mental model for phone approval from desktop. |
| DTB-004 | Keep biometrics inside the OS or authenticator; do not store custom biometric data. | Proposed | Prevents the app from owning face/retina storage or recognition. |
| DTB-005 | Keep device trust separate from account login and from billing entitlement. | Proposed | Avoids login-success implying trust-success. |
| DTB-006 | Use signed entitlement snapshots for device-bound unlock. | Proposed | Makes copied binaries/config useless without signed trust state. |
| DTB-007 | Use encrypted recovery bundles for reset and re-pair. | Proposed | Supports recovery without a universal decrypt key. |
| DTB-008 | Make child uninstall and tamper response parent-controlled, not child-controlled. | Proposed | Preserves the parent-owned trust contract. |
| DTB-009 | Treat Play Integrity as supporting signal only on Android. | Proposed | Keeps Google-tied attestation out of the trust root. |
| DTB-010 | Treat RustDesk as research/reference only. | Proposed | Avoids AGPL/full-stack remote-desktop code as the trust root. |
| DTB-011 | Prefer adopt/research-only/reject decisions for every external dependency. | Proposed | Keeps library risk explicit and reviewable. |
| DTB-012 | Store proof outside the plan folder. | Proposed | Keeps docs clean and keeps evidence in designated artifact paths. |

## Open questions

- Exact fallback behavior when a platform store is unavailable or degraded.
- Exact QR/phone approval UX when the phone is unavailable.
- Exact recovery authority split between parent device, household backup, and support recovery.
- Exact anti-tamper response when the child device is rooted, jailbroken, or otherwise compromised.