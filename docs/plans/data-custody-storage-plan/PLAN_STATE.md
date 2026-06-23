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

## Current ownership interpretation

```text
schema-domain:
  Canonical shared custody/export/sync/restore/report/query/assistant-citation/provider/retention/tombstone/parent-storage-setting shapes when they cross package, crate, app, or plan boundaries.

storage-custody-core:
  Rust generic custody/delete/export decision logic and custody action-plan events.

ocentra-evidence:
  Evidence references, evidence identity, and evidence custody ref semantics.

ocentra-eventing:
  Event journal/replay/idempotency spine. This plan consumes eventing contracts; it does not re-own bus implementation.

production-domain:
  Legacy package identity unless a selected public export is named. Current parent-owned sync/export contract proof routes through schema-domain.

portal-domain and apps/portal:
  Parent-visible custody projection, storage settings, preview, confirmation, and status UI only.

Account, device-trust, Cloudflare, payment, setup, remote, LAN, notification, report producers, and AI plans:
  Adjacent sibling owners or handoff consumers. They must not re-own data custody truth.
```

## Current coupling risks

```text
- Active proof roots are under output/data-custody-storage-plan-proof/<workpack>/; legacy docs/proof/data-custody-storage-plan references are stale and must not raise status.
- `packages/production-domain/src/parent-owned-sync-export.ts` is stale as a source-of-truth path; current contract proof points at `packages/schema-domain/src/parent-owned-sync-export.ts` and `@ocentra-parent/schema-domain/parent-owned-sync-export`.
- Contract/schema proof is not runtime custody proof.
- Sync manifest proof is not provider OAuth/upload/delete runtime proof.
- Export proof is not restore/apply proof.
- Report/query proof is not assistant-safe citation proof.
- Parent storage settings UI proof is not applied custody state.
- Eventing internals, portal UI internals, account authority, device trust material, Cloudflare runtime, payment semantics, setup journey, remote transport, notification delivery, report rendering, and AI runtime must stay in owning plans unless a selected handoff explicitly touches them.
```

## Current proof interpretation

```text
Source presence is not custody readiness.
Schema/domain contract proof is not storage runtime proof.
Provider status proof is not readable-payload or key-access proof.
Delete proof is not tombstone propagation, idempotency, or offline replay proof unless selected proof root proves it.
Import preview is non-mutating and cannot claim restore/apply.
Restore/apply proof must prove tombstone preservation and reject resurrection.
Report/query proof must prove source refs, citation allowlists, redaction, deletion/expiry behavior, and rate-limit/misuse boundaries before assistant/report safety claims.
WP07 can aggregate only accepted proof roots plus exact carried blockers.
```

## Current Route Status

- Status: execution-grade route established; no product completion claim is made.
- Default action: choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then choose required proof from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
- Current limitation: this plan defines ownership, expected proof, and handoff boundaries. It does not claim implementation is complete.

## What Is Already Present

- `crates/storage-custody-core` already owns generic custody/delete/export decision logic.
- `crates/ocentra-evidence` already carries custody-scoped evidence reference semantics.
- `crates/ocentra-eventing` already provides the journal/replay building blocks this plan must not duplicate.
- `packages/schema-domain/src/parent-owned-sync-export.ts` and `scripts/test/parent-owned-sync-export-manifest-proof.mjs` establish the current manifest/connector/status contract boundary.

## Open Product Gaps

- Zero-knowledge versus recoverable support mode is still a product decision.
- Parent-owned cloud default, provider choice defaults, and visible versus app-specific folder policy are still open.
- Provider sync runtime, restore/apply-back runtime, and tombstone propagation runtime remain open.
- Report/query/AI custody, support diagnostics, and parent storage settings/apply flow remain open.
- Proof artifacts must be created by implementation work; this plan only defines expected proof.
- Adjacent implementation plans must be updated only when their workpack is selected.

## No-Read Boundary

Do not read adjacent plans or source trees until a workpack names the exact handoff.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear; do not use it as permission to scan a whole family.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md) execution slices, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Active proof-root route:
  - use `output/data-custody-storage-plan-proof/<workpack-file-stem>/` plus the selected workpack's required artifacts from [PROOF_INDEX.md](PROOF_INDEX.md).
  - legacy `docs/proof/data-custody-storage-plan/` references are stale for new proof and should be removed as touched rather than treated as current proof truth.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof artifact under the selected output proof root,
  - negative cases, no-claim language, and manual-required notes where applicable.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, deletion/tombstone, and rollback/teardown proofs are present or carried as exact blockers for the assigned slice.

## Execution Blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack proof rows.
