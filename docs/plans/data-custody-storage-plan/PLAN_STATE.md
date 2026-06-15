<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If state changes, update NEXT_ACTIONS.md, WORKPACK_INDEX.md, CHECKLIST_INDEX.md, and feature/checklist rows as needed.

<!-- /agent-capsule -->

# Data Custody Storage Plan State

## Current Product Scope

This plan owns data custody guarantees, encrypted storage, evidence retention, export/import/restore, sync, deletion/tombstones, no-stolen-data boundaries, cloud/relay custody, report/query custody, and parent storage settings/apply flow.

Route status: execution-grade architecture, UI docs, and test/proof inventory now exist. Implementation and proof remain open until the selected workpacks close their required slices and proof artifacts.

## Current Route Status

- Status: execution-grade route established; no product completion claim is made.
- Default action: choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then choose required proof from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
- Current limitation: this plan defines ownership, expected proof, and handoff boundaries. It does not claim implementation is complete.

## What Is Already Present

- `crates/storage-custody-core` already owns generic custody/delete/export decision logic.
- `crates/ocentra-evidence` already carries custody-scoped evidence reference semantics.
- `crates/ocentra-eventing` already provides the journal/replay building blocks this plan must not duplicate.
- `packages/production-domain/src/parent-owned-sync-export.ts` and `scripts/test/parent-owned-sync-export-manifest-proof.mjs` already establish the current manifest/connector/status contract boundary.

## Open Product Gaps

- Zero-knowledge versus recoverable support mode is still a product decision.
- Parent-owned cloud default, provider choice defaults, and visible versus app-specific folder policy are still open.
- Provider sync runtime, restore/apply-back runtime, and tombstone propagation runtime remain open.
- Report/query/AI custody, support diagnostics, and parent storage settings/apply flow remain open.
- Proof artifacts must be created by implementation work; this plan only defines expected proof.
- Adjacent implementation plans must be updated only when their workpack is selected.

## No-Read Boundary

Do not read adjacent plans or source trees until a workpack names the exact handoff.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md) execution slices, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/data-custody-storage-plan/.
- Required proof manifest names:
  - docs/proof/data-custody-storage-plan/00-route-consistency-proof.md
  - docs/proof/data-custody-storage-plan/00-no-overclaim-proof.md
  - docs/proof/data-custody-storage-plan/01-data-classification-matrix-proof.md
  - docs/proof/data-custody-storage-plan/01-source-of-truth-proof.md
  - docs/proof/data-custody-storage-plan/01-no-default-ocentra-child-data-negative-proof.md
  - docs/proof/data-custody-storage-plan/01-account-control-plane-separation-proof.md
  - docs/proof/data-custody-storage-plan/02-key-custody-model-proof.md
  - docs/proof/data-custody-storage-plan/02-platform-key-wrapper-matrix-proof.md
  - docs/proof/data-custody-storage-plan/02-wrong-key-negative-proof.md
  - docs/proof/data-custody-storage-plan/02-revoked-device-negative-proof.md
  - docs/proof/data-custody-storage-plan/02-no-ocentra-universal-key-proof.md
  - docs/proof/data-custody-storage-plan/03-provider-capability-matrix-proof.md
  - docs/proof/data-custody-storage-plan/03-encrypted-before-upload-proof.md
  - docs/proof/data-custody-storage-plan/03-provider-revoked-negative-proof.md
  - docs/proof/data-custody-storage-plan/03-quota-conflict-corruption-proof.md
  - docs/proof/data-custody-storage-plan/03-offline-retry-partial-outage-proof.md
  - docs/proof/data-custody-storage-plan/03-tombstone-propagation-proof.md
  - docs/proof/data-custody-storage-plan/03-no-ocentra-fallback-proof.md
  - docs/proof/data-custody-storage-plan/04-retention-matrix-proof.md
  - docs/proof/data-custody-storage-plan/04-delete-state-machine-proof.md
  - docs/proof/data-custody-storage-plan/04-tombstone-idempotency-proof.md
  - docs/proof/data-custody-storage-plan/04-offline-replay-protection-proof.md
  - docs/proof/data-custody-storage-plan/04-report-export-ai-no-leak-proof.md
  - docs/proof/data-custody-storage-plan/04-wrong-role-denied-proof.md
  - docs/proof/data-custody-storage-plan/04-retention-expiry-boundary-proof.md
  - docs/proof/data-custody-storage-plan/04-restore-cannot-resurrect-proof.md
  - docs/proof/data-custody-storage-plan/05-export-bundle-contract-proof.md
  - docs/proof/data-custody-storage-plan/05-encrypted-payload-proof.md
  - docs/proof/data-custody-storage-plan/05-import-preview-non-mutating-proof.md
  - docs/proof/data-custody-storage-plan/05-wrong-household-negative-proof.md
  - docs/proof/data-custody-storage-plan/05-wrong-key-negative-proof.md
  - docs/proof/data-custody-storage-plan/05-corrupt-bundle-negative-proof.md
  - docs/proof/data-custody-storage-plan/05-tombstone-preserved-proof.md
  - docs/proof/data-custody-storage-plan/05-restore-apply-idempotent-proof.md
  - docs/proof/data-custody-storage-plan/05-partial-restore-proof.md
  - docs/proof/data-custody-storage-plan/06-report-derived-source-matrix-proof.md
  - docs/proof/data-custody-storage-plan/06-deleted-expired-no-leak-proof.md
  - docs/proof/data-custody-storage-plan/06-query-cursor-pagination-proof.md
  - docs/proof/data-custody-storage-plan/06-query-rate-limit-abuse-proof.md
  - docs/proof/data-custody-storage-plan/06-notification-payload-allow-deny-proof.md
  - docs/proof/data-custody-storage-plan/06-portal-cache-custody-proof.md
  - docs/proof/data-custody-storage-plan/06-assistant-allowed-citation-proof.md
  - docs/proof/data-custody-storage-plan/06-stale-conflict-state-proof.md
  - docs/proof/data-custody-storage-plan/07-rollout-proof-pack.md
  - docs/proof/data-custody-storage-plan/07-route-index-sync-proof.md
  - docs/proof/data-custody-storage-plan/07-privacy-language-review-proof.md
  - docs/proof/data-custody-storage-plan/07-manual-required-gap-register.md
  - docs/proof/data-custody-storage-plan/08-parent-storage-choice-state-machine-proof.md
  - docs/proof/data-custody-storage-plan/08-export-status-proof.md
  - docs/proof/data-custody-storage-plan/08-import-preview-proof.md
  - docs/proof/data-custody-storage-plan/08-apply-confirmation-proof.md
  - docs/proof/data-custody-storage-plan/08-provider-disconnect-proof.md
  - docs/proof/data-custody-storage-plan/08-provider-delete-proof.md
  - docs/proof/data-custody-storage-plan/08-no-ocentra-fallback-proof.md
  - docs/proof/data-custody-storage-plan/08-portal-cache-status-proof.md
  - docs/proof/data-custody-storage-plan/transport-auth-replay-proof.md
  - docs/proof/data-custody-storage-plan/event-idempotency-ordering-proof.md
  - docs/proof/data-custody-storage-plan/event-sensitive-log-redaction-proof.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## Execution Blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack proof rows.
