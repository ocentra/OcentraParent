# WP22 Deletion And Retention Proof

plan: screen-plan
workpack: 22 Deletion And Retention Proof
owner: screen-plan
artifact_shape: proof-pack
proof_tier: p3-local-runtime
platform: windows
queue_state: deleted
image_custody_state: deleted
retention_state: no-raw-retention
portal_state: not-claimed
run_id: 8HIf9RVzwxgAAAAAAAAAAA

## Runtime evidence

`cargo run -p ocentra-parent-screen-capture-adapter --example screen_capture_real_proof -- output/screen-plan-proof/22-deletion-and-retention-proof/runtime-capture`

exit: 0
result: pass
artifact: `output/screen-plan-proof/22-deletion-and-retention-proof/runtime-capture/04-deletion-proof.json`

The redacted JSON records `existedBeforeEncryption: true`,
`existsAfterDelete: false`, `rawImageDeleted: true`, and
`encryptedQueueContainsRawDigest: false`. It contains no image bytes, OCR text,
or private window/app values.

## Focused validation

- `cargo test -p ocentra-parent-agent-core screen_evidence_queue -- --nocapture` — pass; 4 queue tests.
- `cargo test -p ocentra-parent-agent-service screen_ai_retention_sweeper -- --nocapture` — pass; 4 sweeper/deletion-event tests.
- `cargo test -p ocentra-parent-agent-protocol screen_evidence -- --nocapture` — pass; 4 contract/serialization tests.
- `npm run test --workspace @ocentra-parent/portal -- screen` — pass; 37 files and 144 tests.
- `cargo build --manifest-path crates/parent-dev-bridge/Cargo.toml` — pass.
- `SCREEN_PARENT_PORTAL_SUMMARY_UI_PROOF=1 OCENTRA_PARENT_PORTAL_PORT=4478 npm run test:e2e --workspace @ocentra-parent/portal -- tests/e2e/screen-summary-ui-proof.spec.ts` — blocked: the shared runner executes unrelated network E2E coverage and times out there before screen screenshot artifacts are emitted.

## Outcome mapping

| WP22 outcome | Evidence | State |
| --- | --- | --- |
| raw image after success | P3 real capture deletion JSON | proved |
| raw image after expiry | agent-service sweeper test | proved |
| durable delete-failed state | agent-protocol contract test | proved |
| durable deletion proof ref | queue/sweeper tests | proved |
| sanitized portal-visible state | portal unit test / Playwright screenshot | open: shared E2E runner timeout before screenshot |
| no default long-term raw retention | protocol serialization and P3 capture | proved |

## No-claim boundary

This proves only local screen-image deletion and local encrypted queue custody.
It does not prove product retention policy, parent raw-retention opt-in, remote
raw upload, AI quality, policy/enforcement behavior, privacy/legal approval,
or portal/product completion. The local output artifact is evidence only; this
tracked manifest is the durable reference.
