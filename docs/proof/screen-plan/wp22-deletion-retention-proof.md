# WP22 Deletion And Retention Proof — Invalidated

review_status: partial-current-head
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

## Review outcome

Do not use this manifest or its prior artifacts as WP22 completion evidence.
Independent review found disabled default sweeping, malformed-expiry indefinite
retention, unlocked in-place queue rewrites, non-transactional deletion state,
capture-path leakage, fabricated portal final-row seeding, and proof-custody
leakage. Replacement implementation and fresh production-path proof are
required before any checkbox is restored.

## Current-head transactional outbox addendum (2026-07-20)

`7847f2df2` introduces a Rust-owned encrypted-queue deletion outbox. Expired
entries are persisted to that outbox before the queue rewrite, and each later
sweep reloads the outbox even after the queue is empty. A publication failure
therefore leaves a redacted deletion intent available for restart replay rather
than silently losing the queue entry.

The current runtime acknowledges only successful terminal publications. The
same focused restart test proves acknowledgement removes one pending entry,
repeating that acknowledgement removes zero, and the next sweep is queue-empty.
This prevents both silent loss and endless duplicate terminal publication.

Validation at `b2a5caee6c493fd2605a367853170b6c324b3c61`:

- `CARGO_TARGET_DIR=E:\\OcentraBuild\\screen-wp22 cargo test -p ocentra-parent-agent-service --test screen_ai_runtime screen_retention_sweeper --no-fail-fast` — pass; 4 tests, including injected missing-store publication failure followed by restart/outbox replay and terminal deletion-event publication.
- `npm run lint:architecture -- --files crates/agent-core/src/screen_evidence_queue.rs crates/agent-core/src/screen_evidence_queue_sweep.rs crates/agent-service/tests/unit/screen_ai_retention_sweeper_runtime_tests.rs` — pass.

This is focused custody/runtime proof only. It does not restore the invalidated
portal or product-retention claims, and WP22 remains open pending the complete
accepted proof pack.

## PR 574 review-repair addendum (2026-07-23)

The current branch hardens the local Rust deletion and custody path without
restoring the invalidated WP22 completion claim:

- a successful image deletion is not relabeled `deleteFailed` when only durable
  outbox acknowledgement fails;
- the screen-analysis query, rather than the browser query, owns deletion-state
  tie-breaking for equal timestamps;
- analysis leases reject already-expired queue rows, quarantine malformed lease
  records, and renew while a scheduled provider job is waiting or running;
- each deletion publication returns only its operation-local event journal;
- malformed deletion-outbox rows remain in the canonical outbox until their
  failure projection succeeds and is explicitly acknowledged;
- storage-custody tombstone replacement syncs the owning directory on platforms
  that support directory fsync.

Focused validation:

- `cargo test -p ocentra-parent-agent-service --test screen_ai_runtime -- --test-threads=1` — pass; 41 tests.
- `cargo test -p ocentra-parent-agent-core --test unit screen_evidence_queue_ -- --test-threads=1` — pass; 15 tests.
- `cargo test -p ocentra-parent-agent-core --test unit activity_store_screen_evidence_tests -- --test-threads=1` — pass; 6 tests.
- `cargo test -p ocentra-parent-agent-core --test unit activity_store_browser_tests -- --test-threads=1` — pass; 5 tests.
- `cargo test -p ocentra-storage-custody-core --test unit retention_delete_tombstone_store -- --test-threads=1` — pass; 4 tests.
- focused `npm run lint:architecture -- --files ...` for the touched Rust source — pass.

The screen service suite is run serially because two unrelated capture tests
share process-global fixture state in parallel mode; each also passes
individually. This addendum proves the eight review repairs only. It does not
prove product retention policy, portal completion, remote deletion, or broad
WP22 completion.

## Accepted end-to-end receipt (2026-07-20)

The accepted ignored artifact is
`output/screen-plan-proof/22-deletion-and-retention-proof/accepted-end-to-end-deletion-proof.md`.
Its tracked receipt is this section: protocol contract `screen_evidence` passed
4/4 (including delete-failed serialization and raw-payload exclusion), service
bridge passed 6/6 (including committed deletion projection and raw-retention
rejection), and Chromium `screen-summary-ui-proof.spec.ts` passed 1/1 against
the Rust service at `OCENTRA_PARENT_AGENT_PORT=4499` in 18.7 seconds. The
portal assertion records a service-backed deleted state and no raw pixels.

Artifact receipt digest: `wp22-e2e-37dff7246-protocol4-service6-portal1`.
This compact receipt intentionally contains no screenshot bytes, raw paths,
OCR text, or private screen content.

## Superseded runtime evidence

`cargo run -p ocentra-parent-screen-capture-adapter --example screen_capture_real_proof -- output/screen-plan-proof/22-deletion-and-retention-proof/runtime-capture`

exit: 0
result: pass
artifact: `output/screen-plan-proof/22-deletion-and-retention-proof/runtime-capture/04-deletion-proof.json`

The redacted JSON records `existedBeforeEncryption: true`,
`existsAfterDelete: false`, `rawImageDeleted: true`, and
`encryptedQueueContainsRawDigest: false`. It contains no image bytes, OCR text,
or private window/app values.

artifact_sha256: `232e14d8b613890c2b616d77eafb905656362da73dd6e11d3babf22877bde174`
artifact_commit: `29f34dbc0` (proof run was executed from this checkpoint)

The artifact is ignored local evidence, not shipped content. The digest and the
four redacted boolean outcomes above are the tracked verification anchor; the
manifest deliberately omits its raw temporary path and any image content.

## Focused validation

- `cargo test -p ocentra-parent-agent-core screen_evidence_queue -- --nocapture` — pass; 4 queue tests.
- `cargo test -p ocentra-parent-agent-service screen_ai_retention_sweeper -- --nocapture` — pass; 4 sweeper/deletion-event tests.
- `cargo test -p ocentra-parent-agent-protocol screen_evidence -- --nocapture` — pass; 4 contract/serialization tests.
- `npm run test --workspace @ocentra-parent/portal -- screen` — pass; 37 files and 144 tests.
- `cargo build --manifest-path crates/parent-dev-bridge/Cargo.toml` — pass.
- `SCREEN_PARENT_PORTAL_SUMMARY_UI_PROOF=1 OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC=tests/e2e/screen-summary-ui-proof.spec.ts npm run test:e2e --workspace @ocentra-parent/portal` — pass; exact Chromium spec, Rust dev bridge, local portal, and log bridge.

## Rendered portal evidence

route: `#/screen-analysis`
project: `chromium`
service_context: Rust agent service plus parent dev bridge
sanitized_state_assertion: named screen-analysis region is visible; deleted
custody/evidence state is rendered; raw screenshot text has count zero; no raw
image path, bytes, OCR text, or private window/app fields are recorded here.

The ignored screenshot artifacts are verified by these tracked digests:

| Artifact | Viewport | SHA-256 |
| --- | --- | --- |
| `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-desktop.png` | 1280x720 | `044068fe2abe75ea5131f762528f74073b92dbab33713fc40c907e58e32a8b2c` |
| `output/screen-plan-proof/screen-parent-portal-summary-ui/screenshots/screen-analysis-route-mobile.png` | 390x844 | `e655caec9b4e9fa52e9227b9c81d2da0e2c713ad3fb80e382a72cf87f8afb067` |

Command context: `SCREEN_PARENT_PORTAL_SUMMARY_UI_PROOF=1`
and `OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC=tests/e2e/screen-summary-ui-proof.spec.ts`.
The matching ignored accessibility summary records desktop/mobile screenshot
assertions, a named region, deleted evidence, and raw-screenshot absence.

## Outcome mapping

| WP22 outcome | Evidence | State |
| --- | --- | --- |
| raw image after success | P3 real capture deletion JSON | proved |
| raw image after expiry | agent-service sweeper test | proved |
| durable delete-failed state | agent-protocol contract test | proved |
| durable deletion proof ref | queue/sweeper tests | proved |
| sanitized portal-visible state | exact-spec Playwright screenshots and accessibility summary | proved |
| no default long-term raw retention | protocol serialization and P3 capture | proved |

## No-claim boundary

This proves only local screen-image deletion and local encrypted queue custody.
It does not prove product retention policy, parent raw-retention opt-in, remote
raw upload, AI quality, policy/enforcement behavior, privacy/legal approval,
or portal/product completion. The local output artifact is evidence only; this
tracked manifest is the durable reference.
