# Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

Status: open.

All items begin unchecked. Proof is collected outside the plan folder.

WP01 review refresh (2026-07-19): DTB-01 remains unchecked. A partial Rust parent-presence slice is visible and focused tests are green on Windows, but Unix production custody is unavailable, a specifically authorized sealing action is absent, and no platform key sealing exists. The implemented boundary is durable local delivery into a hash-chained `ocentra-eventing` journal; subscriber delivery and a broader event-bus runtime remain unimplemented. Generated output is local-only and untracked.

| ID | Checklist | Workpack | Status | Proof target |
| --- | --- | --- | --- | --- |
| DTB-01 | Trust source of truth defined | 01-device-trust-source-of-truth | [ ] partial runtime slice only | local untracked `output/device-trust-bootstrap-plan-proof/01-*` plus visible source/tests |
| DTB-02 | Platform key sealing matrix defined | 02-local-key-sealing | [ ] partial: the merged Windows-only slice has custody source and dispatch safety only. The live parent desktop command rejects an unstaged/consumed opaque handle; external/runtime callers cannot mint a ceremony from authority flags because no ceremony issuer is exported until a record-backed household-authority owner exists. The adapter source includes DPAPI ciphertext plus external registry-epoch restored-record rejection, but end-to-end sealing/restored-record execution is unclaimed without an issuer; focused custody proof is limited to idempotent unissued revocation. Production ceremony custody, Android/Linux/iOS/macOS, recovery/re-pair, and merged CI proof remain open | local untracked `output/device-trust-bootstrap-plan-proof/02-*` plus visible source/tests and current CI after merge |
| DTB-03 | Parent step-up policy defined | 03-parent-step-up-auth | [ ] | `output/device-trust-bootstrap-plan-proof/03-*` |
| DTB-04 | QR approval bridge defined | 04-phone-qr-approval-bridge | [ ] | `output/device-trust-bootstrap-plan-proof/04-*` |
| DTB-05 | Device-bound entitlement model defined | 05-entitlement-device-license | [ ] | `output/device-trust-bootstrap-plan-proof/05-*` |
| DTB-06 | Recovery and re-pair model defined | 06-recovery-reset-re-pair | [ ] | `output/device-trust-bootstrap-plan-proof/06-*` |
| DTB-07 | Child tamper and uninstall model defined | 07-child-tamper-uninstall | [ ] | `output/device-trust-bootstrap-plan-proof/07-*` |
| DTB-08 | Dependency adoption decisions recorded | 08-open-source-dependency-adoption | [ ] | `output/device-trust-bootstrap-plan-proof/08-*` |
| DTB-09 | Route gate and sync checks defined | 09-cross-plan-route-gate | [ ] | `output/device-trust-bootstrap-plan-proof/09-*` |

## Selected slice: WP08 dependency adoption (2026-08-09)

- [x] Replayed the dependency adoption boundary test (1/1).
- [x] Ran the scoped architecture and Enforcer guard checks.
- [x] Recorded the durable manifest at `docs/proof/device-trust-bootstrap-plan/slice-08-dependency-adoption.md`.
- [ ] Runtime adoption, platform ceremony, key sealing, recovery execution,
  CI, review, and main-merge gates remain open.
