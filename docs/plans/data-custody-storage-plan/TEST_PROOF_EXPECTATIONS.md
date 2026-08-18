<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Data Custody Storage Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Common command set

Use the subset relevant to the selected workpack:

```bash
# Rust custody/evidence/event proof scope
cargo test -p ocentra-storage-custody-core
cargo test -p ocentra-evidence
cargo test -p ocentra-eventing
cargo test -p ocentra-parent-runtime-core

# Shared custody/export/sync/restore/report/query schema scope
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- custody
npm run test --workspace @ocentra-parent/schema-domain -- parent-owned-sync-export

# Legacy/consumer package scope only when the selected workpack names it
npm run build --workspace @ocentra-parent/production-domain
npm run test --workspace @ocentra-parent/production-domain -- custody

# UI scope only when parent storage settings/report/query surfaces are selected
npm run test --workspace @ocentra-parent/portal -- storage

# Existing parent-owned sync/export proof anchor when WP03 is selected
node scripts/test/parent-owned-sync-export-manifest-proof.mjs

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files crates/schema crates/storage-custody-core crates/parent-runtime-core crates/ocentra-evidence crates/ocentra-eventing packages/schema-domain packages/production-domain apps/portal scripts/test docs/plans/data-custody-storage-plan
```

Run through `npm run agent:run --` when collecting proof if available.

## Command ownership notes

- `packages/schema-domain` owns canonical shared custody/export/sync/restore/report/query/assistant-citation/provider/retention/tombstone/parent-storage-setting shapes when contracts cross package/crate/app/plan boundaries.
- `crates/storage-custody-core` proves generic Rust custody/delete/export decisions and custody action-plan events only; WP05's backup/restore/migration modules remain pure decisions/orchestration and never persist a second job or receipt ledger.
- `crates/parent-runtime-core` owns the durable WP05 scheduler/job and restore/migration ledgers, restart reconciliation, executor/rollback mounting, and real Eventing journal/outbox composition. It consumes only opaque Account/family authority, key/decrypt capability, provider-neutral adapter, and producer ports; it must not mint or implement those external owners.
- `crates/ocentra-evidence` proves evidence refs and evidence identity only.
- `crates/ocentra-eventing` proves event/journal/idempotency primitives only; this plan must not re-own the event bus.
- `packages/production-domain` is legacy package identity unless a selected public export is named. Current parent-owned sync/export contract proof routes through schema-domain.
- Portal, account, device-trust, Cloudflare, payment, setup, remote, LAN, notification, report producer, and AI scopes run only when the selected workpack names the handoff.

## Data Custody E2E meaning

Do not use one proof family to claim the whole custody path. For this plan, E2E has separate meanings:

```text
custody source-of-truth E2E: data class -> owner/source-of-truth row -> allowed storage/query/export/retention policy.
key custody E2E: custody location -> key owner/wrapper -> wrong-key/revoked-device/no-universal-key negative proof.
sync/provider E2E: parent-owned provider choice -> encrypted manifest/bundle -> connector status/cursor/conflict/revocation/offline/tombstone state.
retention/delete/tombstone E2E: retention window/action -> delete request/result -> tombstone/idempotency/offline replay -> no restore resurrection.
export/import/restore E2E: export bundle -> encrypted payload -> import preview -> apply confirmation -> wrong household/key/corrupt/partial restore states.
report/query E2E: source refs -> query cursor/pagination -> deleted/expired exclusion -> redacted report/notification payloads.
assistant citation E2E: allowed evidence/report refs -> assistant citation boundary -> no private payload leakage.
parent settings/apply E2E: parent storage choice -> export/import preview -> confirmation -> provider disconnect/delete -> applied status or manual-required state.
rollout gate E2E: accepted proof roots + carried blockers -> route/index sync -> privacy language review -> manual-required gap register.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every custody proof slice must preserve product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact raw child evidence payloads, private report contents, provider account identifiers beyond opaque refs, key material, recovery material, device trust material, assistant private prompts/responses, notification bodies unless allowed, and support diagnostics not selected for proof
log data class, source-of-truth ref, custody location, key owner, encryption state, retention state, tombstone state, provider state, export/import/restore state, actor/role state, evidence refs, proof refs, manual-required notes, and no-claim boundaries when safe
separate custody policy, key custody, provider sync, retention/delete, export/import/restore, report/query, assistant citation, parent settings, and rollout states
never treat docs, schema proof, portal logs, provider status rows, or source presence as runtime custody proof without command output or exact blocker
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, data class, custody location, key owner, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## WP01 Custody Source Of Truth

Expected coverage:

```text
data class matrix
owner/source-of-truth matrix
account control-plane separation
redaction boundary
no default hosted private activity store
```

## WP02 Encryption Key Custody

Expected coverage:

```text
key custody model
platform key wrapper matrix
wrong-key negative case
revoked device negative case
no universal Ocentra key
recovery/manual-required modes
```

## WP03 Parent Owned Cloud Sync

Expected coverage:

```text
provider capability matrix
encryption-before-upload proof
provider revoked state
quota/conflict/corruption states
offline retry and partial outage state
tombstone propagation
no automatic Ocentra fallback store
provider OAuth/upload/delete runtime non-claims unless selected proof exists
```

## WP04 Retention Delete Tombstone

Expected coverage:

```text
retention matrix
delete state machine
tombstone idempotency
offline retry behavior
report/export/assistant derived-data boundaries
wrong role denied
expiry boundary
restore cannot revive deleted state
```

## WP05 Export Import Backup Recovery

Expected coverage:

```text
export bundle contract
encrypted payload proof
import preview is non-mutating
wrong household/key/bundle state rejected
retention/tombstone preserved
restore/apply idempotent
partial restore state
support recovery limits
schema-owned backup cadence/schedule/job lifecycle and operation refs
migration apply/rollback/reconciliation receipts bound to bundle/plan identity
storage-custody-core pure backup/restore/migration/preflight decisions
parent-runtime durable scheduler/job and restore/migration ledgers
restart reconciliation, executor/rollback mount, and Eventing/outbox seam
no caller-supplied authority, key, integrity, or provider identity
```

Expected source/test ownership roots (all deferred until the source packet is
complete):

```text
schema: crates/schema/src/export_import_backup_recovery.rs,
       crates/schema/src/export_import_backup_recovery/,
       crates/schema/tests/contract/export_import_backup_recovery_runtime.rs
storage decisions: crates/storage-custody-core/src/export_import_backup_recovery_backup_schedule.rs,
                   crates/storage-custody-core/src/export_import_backup_recovery_backup_job_state.rs,
                   crates/storage-custody-core/src/export_import_backup_recovery_restore_execution_plan.rs,
                   crates/storage-custody-core/src/export_import_backup_recovery_migration_execution.rs,
                   crates/storage-custody-core/src/export_import_backup_recovery_bundle_preflight_binding.rs,
                   crates/storage-custody-core/src/export_import_backup_recovery_compensation.rs,
                   crates/storage-custody-core/tests/unit/export_import_backup_recovery_runtime.rs
parent runtime: crates/parent-runtime-core/src/data_custody_backup_runtime.rs,
                crates/parent-runtime-core/src/data_custody_backup_runtime_schedule.rs,
                crates/parent-runtime-core/src/data_custody_backup_runtime_job_ledger.rs,
                crates/parent-runtime-core/src/data_custody_backup_runtime_reconciliation.rs,
                crates/parent-runtime-core/src/data_custody_runtime_eventing.rs,
                crates/parent-runtime-core/src/data_custody_restore_runtime.rs,
                crates/parent-runtime-core/src/data_custody_restore_runtime_ledger.rs,
                crates/parent-runtime-core/src/data_custody_restore_runtime_reconciliation.rs,
                crates/parent-runtime-core/src/data_custody_restore_runtime_executor.rs,
                crates/parent-runtime-core/src/data_custody_restore_runtime_rollback.rs,
                crates/parent-runtime-core/tests/unit/data_custody_backup_runtime.rs,
                crates/parent-runtime-core/tests/unit/data_custody_restore_runtime.rs,
                crates/parent-runtime-core/tests/integration/data_custody_runtime.rs
```

## WP06 Report Query Custody

Expected coverage:

```text
derived source matrix
deleted/expired data not returned
query cursor/pagination
query rate-limit/misuse boundary
notification payload allow/deny
portal cache custody
assistant citations restricted to allowed refs
stale/conflict state
```

## WP08 Parent Storage Settings Apply Flow

Expected coverage:

```text
storage choice state machine
export status
import preview
apply confirmation
provider disconnect/delete state
no automatic fallback store
portal cache status
visible manual-required state
```

## WP07 Rollout Proof And Route Gate

Expected aggregation:

```text
WP01 proof root exists or blocker recorded
WP02 proof root exists or blocker recorded
WP03 proof root exists or blocker recorded
WP04 proof root exists or blocker recorded
WP05 proof root exists or blocker recorded
WP06 proof root exists or blocker recorded
WP08 proof root exists or blocker recorded
route/index sync proof
privacy language review
manual-required gap register
accepted proof roots and carried blockers
adjacent handoff no-claim boundaries
```

Required negative states:

```text
storage claim without custody owner proof
sync claim without encryption/key proof
restore claim without preview/apply proof
delete claim without tombstone proof
report/query claim without source/citation proof
assistant/report output includes disallowed private payload
parent settings UI claim without apply confirmation proof
automatic Ocentra fallback store without explicit product decision and proof

## WP09 Parent Local Bundle Provider Runtime

Expected coverage:

```text
encrypted-before-persistence/provider handoff
exact byte hash/signature and household/key/source binding
atomic write/replace, interruption, restart, quarantine, and recovery
manual/scheduled retry, restart, and duplicate-job idempotency
provider-neutral opaque status and unsupported-provider/manual-required state
payload/key/provider/path redaction and no-fallback-store negative
```

WP09 expected tests cover only the downstream pure byte-custody/provider-port
boundary. Durable scheduler/job persistence and restart reconciliation are
tested through the WP05 parent-runtime roots above, not by a second WP09
ledger.

## WP10 Restore Orchestration And Producer Handoffs

Expected coverage:

```text
non-mutating preflight bound to bundle, household, authority, key, tombstone, schema, and migration
trusted confirmation expiry/replay/wrong-operation/wrong-household negatives
monotonic apply/migration/rollback/idempotency under retry and restart
missing/failed/partial data-class producer handoffs and manual-required outcomes
tombstone/no-resurrection and migration rollback boundaries
receipt provenance, owner result requirement, redaction, and no-fake-success
```

WP10 expected tests cover only downstream pure producer-handoff orchestration.
Durable restore/migration ledgers, restart reconciliation, executor/rollback
mounting, and Eventing/outbox composition are tested through the WP05
parent-runtime roots above.
```
