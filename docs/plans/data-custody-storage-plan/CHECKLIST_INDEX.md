<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Data Custody Storage Plan Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark storage/sync/export/delete/report rows complete from docs-only work unless the selected workpack allows docs-only proof.
- Do not mark PR_READY until WP07 aggregates proof from all required earlier workpacks.

## WP01 Custody Source Of Truth

- [ ] Data classification matrix defined.
- [ ] Source-of-truth matrix defined.
- [ ] Account/control-plane separation documented.
- [ ] Redaction boundary documented.
- [ ] No default hosted private activity store proof exists.
- [ ] Current repo owners mapped.
- [ ] Adjacent owner handoffs recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.
- [ ] No broad custody claim made.

## WP02 Encryption Key Custody

- [x] Key custody model defined. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/00-key-custody-model-proof.md`
- [x] Platform key wrapper matrix defined. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/01-platform-key-wrapper-matrix-proof.md`
- [x] Wrong-key negative proof exists. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/02-wrong-key-negative-proof.md`
- [x] Revoked-device negative proof exists. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/03-revoked-device-negative-proof.md`
- [x] No universal Ocentra key proof exists. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/04-no-universal-ocentra-key-proof.md`
- [x] Recovery/manual-required modes defined. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/05-recovery-mode-proof.md`
- [x] Platform limitations recorded. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/01-platform-key-wrapper-matrix-proof.md`
- [x] Required proof artifacts written. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/`
- [x] Focused commands pass or blocker recorded. Proof: `output/data-custody-storage-plan-proof/02-encryption-key-custody/16-validation-commands.log`
- [x] Workpack completion section filled. Proof: `docs/plans/data-custody-storage-plan/workpacks/02-encryption-key-custody.md`
- [x] Adjacent device-trust handoff recorded. Proof: `docs/plans/data-custody-storage-plan/workpacks/02-encryption-key-custody.md`
- [x] No key-readiness overclaim made. Proof: `docs/plans/data-custody-storage-plan/workpacks/02-encryption-key-custody.md`

## WP03 Parent Owned Cloud Sync

- [ ] Provider capability matrix defined.
- [ ] Encryption-before-upload proof exists.
- [ ] Provider revoked state exists.
- [ ] Quota/conflict/corruption states defined.
- [ ] Offline retry and partial outage states defined.
- [ ] Tombstone propagation defined.
- [ ] No automatic Ocentra fallback store proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] Provider disconnect behavior recorded.
- [ ] No sync-ready overclaim made.
- [ ] Parent-visible state requirements recorded.

## WP04 Retention Delete Tombstone

- [x] Retention matrix defined. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/00-retention-matrix-proof.md`
- [x] Delete state machine defined. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/01-delete-state-machine-proof.md`
- [x] Tombstone idempotency proof exists. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/02-tombstone-idempotency-proof.md`
- [x] Offline retry behavior defined. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/03-offline-retry-proof.md`
- [x] Derived-output boundary proof exists. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/04-derived-output-boundary-proof.md`
- [x] Wrong-role denial proof exists. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/05-wrong-role-denied-proof.md`
- [x] Expiry boundary proof exists. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/06-retention-expiry-boundary-proof.md`
- [x] Restore cannot revive deleted state proof exists. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/07-restore-cannot-revive-deleted-state-proof.md`
- [x] Required proof artifacts written. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/`
- [x] Focused commands pass or blocker recorded. Proof: `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/16-validation-commands.log`
- [x] Workpack completion section filled. Proof: `docs/plans/data-custody-storage-plan/workpacks/04-retention-delete-tombstone.md`
- [x] No delete-ready overclaim made. Proof: `docs/plans/data-custody-storage-plan/workpacks/04-retention-delete-tombstone.md`
- [x] Adjacent eventing/data owner handoffs recorded. Proof: `docs/plans/data-custody-storage-plan/workpacks/04-retention-delete-tombstone.md`

## WP05 Export Import Backup Recovery

- [x] Export bundle contract defined. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/00-export-bundle-contract-proof.md`
- [x] Encrypted payload proof exists. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/01-encrypted-payload-proof.md`
- [x] Import preview is non-mutating proof exists. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/02-import-preview-non-mutating-proof.md`
- [x] Wrong household/key/bundle negative proof exists. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/03-wrong-household-key-bundle-proof.md`
- [x] Tombstone preserved proof exists. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/04-tombstone-preserved-proof.md`
- [x] Restore/apply idempotency proof exists. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/05-restore-apply-idempotent-proof.md`
- [x] Partial restore state defined. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/06-partial-restore-proof.md`
- [x] Support recovery limits recorded. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/01-encrypted-payload-proof.md`
- [x] Required proof artifacts written. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/16-validation-commands.log`
- [x] Focused commands pass or blocker recorded. Proof: `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/16-validation-commands.log`
- [x] Workpack completion section filled. Proof: `docs/plans/data-custody-storage-plan/workpacks/05-export-import-backup-recovery.md`
- [x] No restore-ready overclaim made. Proof: `docs/plans/data-custody-storage-plan/workpacks/05-export-import-backup-recovery.md`

WP05 checklist rows above cover the bounded bundle/preflight/base contract
packet and retained historical proof only. Durable runtime composition/custody
mounting, external Account/key/provider/producer handoffs, and its expected
integration test are tracked separately under WP11 below; these rows do not
close WP05 runtime reachability or plan completion.

## WP06 Report Query Custody

- [x] Derived source matrix defined. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/00-derived-source-matrix-proof.md`
- [x] Deleted/expired data not returned proof exists. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/01-deleted-expired-not-returned-proof.md`
- [x] Query cursor/pagination proof exists. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/02-query-cursor-pagination-proof.md`
- [x] Query rate-limit/misuse boundary defined. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/03-query-rate-limit-proof.md`
- [x] Notification payload allow/deny matrix defined. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/04-notification-payload-allow-deny-proof.md`
- [x] Portal cache custody proof exists. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/05-portal-cache-custody-proof.md`
- [x] Assistant allowed-citation proof exists. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/06-assistant-allowed-citation-proof.md`
- [x] Stale/conflict state proof exists. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/07-stale-conflict-state-proof.md`
- [x] Required proof artifacts written. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/`
- [x] Focused commands pass or blocker recorded. Proof: `output/data-custody-storage-plan-proof/06-report-query-custody/16-validation-commands.log`
- [x] Workpack completion section filled. Proof: `docs/plans/data-custody-storage-plan/workpacks/06-report-query-custody.md`
- [x] No report/query custody overclaim made. Proof: `docs/plans/data-custody-storage-plan/workpacks/06-report-query-custody.md`
- [x] Adjacent AI/notification owner handoffs recorded. Proof: `docs/plans/data-custody-storage-plan/workpacks/06-report-query-custody.md`

## WP08 Parent Storage Settings Apply Flow

- [ ] Parent storage choice state machine defined.
- [ ] Export status proof exists.
- [ ] Import preview proof exists.
- [ ] Apply confirmation proof exists.
- [ ] Provider disconnect proof exists.
- [ ] Provider delete proof exists.
- [ ] No automatic fallback proof exists.
- [ ] Portal cache status proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No settings-ready overclaim made.

## WP09 Parent Local Bundle Provider Runtime

- [ ] Production source owner and shipped caller are mapped and reviewed.
- [ ] Parent-local encrypted bundle byte persistence and retrieval are implemented.
- [ ] Exact-byte hash/signature, household, version, and key-context verification is implemented.
- [ ] Atomic write, recovery, and corruption quarantine behavior is implemented.
- [ ] Manual and scheduled job retry, restart, and idempotency custody is implemented.
- [ ] Provider-neutral adapter behavior preserves explicit no-provider, no-fallback, and manual-required states.
- [ ] Required positive, negative, interruption, replay, and redaction test source is written and reviewed.
- [ ] Focused tests and focused architecture/Enforcer gates pass or an exact blocker is recorded.
- [ ] Required proof artifacts are written under `output/data-custody-storage-plan-proof/09-parent-local-bundle-provider-runtime/`.
- [ ] Workpack completion section and plan indexes are synchronized without a custody-ready overclaim.

## WP10 Restore Orchestration And Producer Handoffs

- [ ] Production source owner and shipped caller are mapped and reviewed.
- [ ] Durable preflight, apply, migration, rollback, and idempotency orchestration state is implemented.
- [ ] Trusted Account authority and confirmation are bound to household, operation, expiry, and replay state.
- [ ] Data-class producer/consumer handoffs return owner-derived outcomes before receipt advancement.
- [ ] Tombstone, retention, and no-resurrection constraints are enforced across restore and migration.
- [ ] Crash/restart, partial failure, retry, rollback, and idempotency behavior is implemented.
- [ ] Required positive, negative, interruption, replay, and no-fake-receipt test source is written and reviewed.
- [ ] Focused tests and focused architecture/Enforcer gates pass or an exact blocker is recorded.
- [ ] Required proof artifacts are written under `output/data-custody-storage-plan-proof/10-restore-orchestration-and-producer-handoffs/`.
- [ ] Workpack completion section and plan indexes are synchronized without a restore-ready overclaim.

## WP11 Runtime Composition And Custody Mount

- [ ] WP05 base ledgers, reconciliation, Eventing/outbox, and manual-required gates are mounted without a second ledger.
- [ ] Account WP05 base authority transaction/CAS consumer seam is mounted without assigning it coordinator ownership.
- [ ] Account WP05A durable multi-owner coordinator/recovery typed Data handoff is owner-resolved.
- [ ] Key/import custody and integrity binding are owner-resolved.
- [ ] Producer-owned sealed artifact custody is bound to the WP05 operation.
- [ ] WP09 provider operation capability and opaque outcome are mounted.
- [ ] WP10 owner-derived producer outcomes, partial, and manual-required states are mounted.
- [ ] Missing, stale, revoked, ambiguous, no-fake-success, and no-resurrection negatives are covered.
- [ ] Private mount traits remain private and request/JSON authority selectors are rejected.
- [ ] Expected integration test source is written and reviewed at `crates/parent-runtime-core/tests/integration/data_custody_runtime_composition.rs`.
- [ ] Focused test, architecture, generated, single-source, and Enforcer gates pass or an exact blocker is recorded.
- [ ] Required proof artifacts are written under `output/data-custody-storage-plan-proof/11-runtime-composition-and-custody-mount/`.
- [ ] Workpack completion section and plan indexes are synchronized without a runtime-complete overclaim.

## WP07 Rollout Proof And Route Gate

- [ ] WP01 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP02 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP03 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP04 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP05 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP06 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP08 proof root accepted by a clean-checkout aggregate gate. Blocker: historic `output/` evidence is ignored and absent.
- [ ] WP09 proof root accepted by a clean-checkout aggregate gate. Blocker: production source and expected tests are not implemented.
- [ ] WP10 proof root accepted by a clean-checkout aggregate gate. Blocker: production source and expected tests are not implemented.
- [ ] Route/index aggregate proof published from a clean checkout.
- [ ] Privacy language review published from a clean checkout.
- [ ] Manual-required gap register published from a clean checkout.
- [ ] Adjacent handoff proof published from a clean checkout.
- [ ] Current focused retention/custody lifecycle validation passes. Blocker: the durable store moved to `crates/child-runtime`, while `crates/storage-custody-core/tests/unit/retention_delete_tombstone_store.rs` and `crates/child-runtime/tests/unit/runtime_gate.rs` still import the deleted core module. Migrate the expected tests first, then execute the focused child-runtime/storage-custody matrix. The older `docs/proof` pointer records a previous surface and is not current acceptance.
- [x] PLAN_STATE and WORKPACK_INDEX reflect the limited lifecycle proof and aggregate blocker.
- [x] No PR_READY claim is made without accepted aggregate proof roots.
