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

Canonical custody note (2026-08-18): the reviewed Account/Data handoff is the
typed `crates/family-identity-core/src/household_authority_runtime_authorization.rs`
boundary from packets `1d2e3d98c`/`b3c47fd3d`; it is not an external producer or
runtime caller. Local-only Data WP05 branch `bfb85f51` is fully superseded and is
not integration evidence. Provider, key, producer, caller, expected-test,
focused-validation, proof, and DONE gaps remain open.

Route status: execution-grade architecture and an integrated production-source wave now exist. Current source closes the WP02 cross-scope decrypt-authority gap, WP03 manifest-custody gap, WP05's bounded schema/storage/parent-runtime source packet, WP06's full Rust request/row plus generated TypeScript custody boundary, and places the WP04/WP07 durable effect/tombstone lifecycle in its child-runtime owner. WP06 now derives an opaque query-source capability from current Account authority, enforces session expiry, exact role binding, bounded page size, and cursor/source/stable-sort continuity; no report/query runtime consumer reaches it. WP05's external Account/key/provider/producer composition, production caller reachability, expected tests, focused execution, proof, precommit/CI, and PR remain open; unavailable provider-backed paths stay manual-required. This source wave deliberately did not write or run tests, regenerate proof, run precommit/CI, or claim DONE or PR_READY. Older checked rows and ignored `output/` roots remain historical until the later test/proof waves re-accept them from a clean checkout.

## Current ownership interpretation

```text
crates/schema:
  Canonical shared custody/export/sync/restore/report/query/assistant-citation/provider/retention/tombstone/parent-storage-setting shapes when they cross package, crate, app, or plan boundaries.

storage-custody-core:
  Rust generic custody/delete/export decision logic, WP05 bundle/preflight
  binding, backup/restore/migration plans, fail-closed no-resurrection, and
  partial-write compensation. It owns pure decisions/orchestration only.

parent-runtime-core:
  WP05 durable backup scheduler/job ledger, restore/migration ledger, restart
  reconciliation, executor/rollback mounting, and real Eventing journal/outbox
  composition. It consumes only opaque Account/family authority, key/decrypt
  capability, provider-neutral adapter, and producer ports; it does not mint
  any of them.

child-runtime:
  Child-service command dispatch, opaque custody-authority consumption, durable effect ledger, tombstone outbox, startup recovery, reconciliation, and delivery-owned terminal acknowledgement.

ocentra-evidence:
  Evidence references, evidence identity, and evidence custody ref semantics.

ocentra-eventing:
  Event journal/replay/idempotency spine. This plan consumes eventing contracts; it does not re-own bus implementation.

production-domain:
  Legacy package identity unless a selected public export is named. Current parent-owned sync/export contract proof routes through the Rust owner and generated TS edge surfaces.

portal-domain and apps/portal:
  Parent-visible custody projection, storage settings, preview, confirmation, and status UI only.

Account, device-trust, Cloudflare, payment, setup, remote, LAN, notification, report producers, and AI plans:
  Adjacent sibling owners or handoff consumers. They must not re-own data custody truth.
```

## Current coupling risks

```text
- Active proof roots are under output/data-custody-storage-plan-proof/<workpack>/; legacy docs/proof/data-custody-storage-plan references are stale and must not raise status.
- `packages/production-domain/src/parent-owned-sync-export.ts` is stale as a source-of-truth path; WP03 now routes the canonical contract through `crates/schema/src/parent_owned_sync_export.rs`, runtime/read-model truth through `crates/storage-custody-core/src/parent_owned_sync_export.rs`, and only thin/generated edge validation through `packages/schema-domain/src/parent-owned-sync-export.ts`.
- Contract/schema proof is not runtime custody proof.
- Sync manifest proof is not provider OAuth/upload/delete runtime proof.
- WP05 export/import/restore proof now covers the shared bundle contract and restore/apply state machine only; it is not provider adapter runtime proof.
- WP06's expected report/query proof root covers assistant/report citation allowlists at the shared contract boundary; no current WP06 proof was regenerated in this source-only routing refresh, and it is not AI runtime answer proof.
- Parent storage settings UI proof is not applied custody state.
- Eventing internals, portal UI internals, account authority, device trust material, Cloudflare runtime, payment semantics, setup journey, remote transport, notification delivery, report rendering, and AI runtime must stay in owning plans unless a selected handoff explicitly touches them.
```

## Current proof interpretation

```text
Source presence is not custody readiness.
Schema/domain contract proof is not storage runtime proof.
Provider status proof is not readable-payload or key-access proof.
WP02 key-custody proof root now covers the shared key hierarchy contract, explicit platform decrypt authority, wrong-household/wrong-device/revoked-key/lost-key fail-closed states, linux manual-required state, mobile proof-gated limits, and no universal decrypt root boundary at the shared contract/runtime layer.
WP04 retention/delete proof now covers the retention matrix, delete state machine, tombstone idempotency, offline replay protection, explicit expiry failure, and restore-no-revival boundary at the shared contract/runtime layer; it is not provider-runtime execution proof.
WP05 export/import proof root now covers the versioned bundle manifest, per-class encrypted payload sections, manifest/payload integrity checks, redacted human summary, non-mutating preview, version/household/key/tombstone/duplicate/migration preflight, partial restore, fail-closed negatives, idempotent apply state, and no default support decrypt path at the shared contract/runtime layer.
Import preview remains non-mutating; WP05 restore/apply proof covers only the shared bundle/preflight/apply state machine and not provider-side retrieval or child-device filesystem execution.
Restore/apply proof must prove tombstone preservation and reject resurrection.
WP06 report/query proof root now proves source refs, citation allowlists, redaction, deletion/expiry behavior, stable pagination, stale/conflict honesty, and rate-limit boundaries at the shared contract/runtime layer.
WP08 parent storage settings/apply flow proof root now proves explicit storage mode labels, preview-before-apply, separate disconnect/delete states, manual-required visibility, and no-claim portal/provider-runtime boundaries at the shared contract/runtime layer; it is not final portal rendering, host wiring, or provider execution proof.
WP07 can aggregate only accepted proof roots plus exact carried blockers. The
child service now has an internal command path from
`ChildAgentIngress::submit_storage_custody_action` through dynamic Device Trust
readiness, `ChildStorageCustodyRuntime::execute`, the durable effect ledger,
and delete/tombstone recovery. Default composition remains manual-required
because no Account/family trusted custody-authority adapter or external caller
supplies the opaque handle. Test migration, restart behavior, aggregate
publication, and downstream delivery proof remain open; source reachability
alone is not lifecycle completion.
```

## Current Route Status

- Status: execution-grade route established; no product completion claim is made.
- Default action: choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then choose required proof from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
- Current limitation: this plan defines ownership, expected proof, and handoff boundaries. It does not claim implementation is complete.

## Production-code reachability audit (2026-08-17, source checkpoint `7a1e1c389`)

This pass inspected the integrated production owners and non-test callers:

```text
WP01: crates/schema source-of-truth contract plus generated edge; no shipped runtime consumer found.
WP02: decrypt-scope authority now fails closed against the selected platform row; real platform/provider key consumers remain adjacent-owner work.
WP03: sync manifest custody is validated before claim-safe state derivation; provider SDK/OAuth/upload/delete/retrieval execution remains adjacent-owner work.
WP04: generic retention/delete derivation remains in storage-custody-core; the durable tombstone/effect owner is now child-runtime and is reached by its internal custody command path.
WP05: dishonest import bundles are rejected before derivation, and the bounded schema/storage/parent-runtime backup, restore, migration, rollback, reconciliation, and Eventing source packet is present. Provider/filesystem execution, external authority/key/provider/producer composition, production callers, tests, focused validation, and proof remain open.
WP06: current Account-authority-derived opaque query-source capability, request/row authority and citation binding, session expiry, exact actor-role binding, page-size, and cursor/source/stable-sort continuity now fail closed at the Rust boundary with generated TypeScript parity. No report/query/notification/AI/portal runtime consumer reaches the boundary; stale Rust tests and the unwritten TypeScript contract test remain open.
WP07: submit_storage_custody_action -> PublishStorageCustody -> dynamic Device Trust gate -> ChildStorageCustodyRuntime::execute -> durable effect/journal/tombstone lifecycle is real source. Default custody authority is manual-required and no trusted Account/family adapter or external caller supplies the opaque handle.
WP08: schema/storage-custody-core parent-storage settings/apply derivation plus generated edge; no portal/desktop host/provider apply caller found.
Migrated Data And AI UI: source-only and not executable custody scope.
```

The bounded WP05 source route is now present in its schema,
storage-custody-core, and parent-runtime-core ownership layers. The next legal
work is to mount the separate trusted Account/family, key/decrypt,
provider-neutral, and producer owners and a real production caller without
accepting caller-selected authority; absent external owners remain
manual-required or blocked. The parent runtime retains private traits, opaque
handles, generation/currentness checks, and the independent Device Trust gate.
WP09 and WP10 remain downstream source-only handoffs. WP06's shared source edge
is integrated, but no report/query runtime consumer is routed to it; expected
tests, focused execution, proof, precommit, CI, and PR are intentionally
deferred to their later phases.

Global engineering-graph regeneration is deferred to the owning coordinator;
this plan-local source/status checkpoint does not write global graph files.
Graph state remains evidence-derived; topology presence does not promote a
workpack to DONE.

## What Is Already Present

- `crates/storage-custody-core` already owns generic custody/delete/export decision logic.
- `crates/ocentra-evidence` already carries custody-scoped evidence reference semantics.
- `crates/ocentra-eventing` already provides the journal/replay building blocks this plan must not duplicate.
- WP01 source-of-truth contracts exist in `crates/schema` and the generated TypeScript edge; current runtime consumption and clean-checkout proof acceptance remain open.
- WP02 source now includes explicit decrypt-scope authority validation in `crates/storage-custody-core`; platform consumers and the new expected-test matrix remain open.
- WP03 source now includes manifest-custody validation in `crates/storage-custody-core`; provider execution and current tests/proof remain open.
- WP04 generic retention/delete derivation remains in `storage-custody-core`, while the durable child-side tombstone/effect lifecycle now lives in and is reached through `child-runtime`; moved-store tests remain stale.
- WP05 source now rejects dishonest import bundles and includes the explicit
  schema durable backup/schedule/job/migration/rollback contracts, pure
  `storage-custody-core` decisions/orchestration, and the durable
  `parent-runtime-core` scheduler/job and restore/migration runtime. Its
  external mounts, production caller reachability, expected tests, focused
  validation, and proof remain open.
- WP06's Rust request/row authority boundary and generated TypeScript edge are source-present with current-authority, expiry, role, page, and cursor/source continuity enforcement; no downstream report/query consumer is routed, and the expected Rust/TypeScript tests remain stale or unwritten.
- WP07 has a real internal child-service command/effect/journal/tombstone path and startup recovery, but default authority is manual-required and no trusted external composition supplies it.
- WP08 parent-storage settings/apply contracts remain source-present; real portal/desktop/provider apply composition and current validation/proof remain open.
- All older `output/data-custody-storage-plan-proof/...` references are historical until the later proof wave publishes durable clean-checkout evidence.

## Open Product Gaps

- Zero-knowledge versus recoverable support mode is still a product decision beyond WP02's explicit manual-required and no-universal-key boundary.
- Parent-owned cloud default, provider choice defaults, and visible versus app-specific folder policy are still open.
- Provider sync runtime and provider/file retrieval execution remain open. WP03 has a claim-safe shared contract/manifest source boundary but no current test/proof acceptance or provider-side OAuth/upload/delete/retrieval execution.
- Trusted Account/family custody-authority composition and an external upstream child-service caller remain open; the present runtime correctly returns manual-required.
- WP05 backup cadence/manual backup and migration execution/rollback source are
  present in the bounded schema/storage/parent-runtime packet, including the
  scheduler/job ledger, durable restore/migration ledger, restart
  reconciliation, executor/rollback mount, and Eventing/outbox composition.
  Account/key/provider/producer owners, production caller reachability, tests,
  focused validation, and proof remain open; no SDK or fake adapter is in
  scope.
- WP06 downstream report/query runtime consumption, expected tests, focused validation, and proof refresh remain open; the shared generated edge is source-present.
- AI runtime custody and support diagnostics remain open.
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

## 2026-08-18 runtime ownership correction

The bounded source packet now supplies WP05's legal schema, pure
storage-custody-core, and parent-runtime-core ownership layers. It does not
mount a production local/provider writer or retriever, byte-level provider
verifier, external authority/key/provider/producer composer, or upstream
caller. Child-runtime owns local data/tombstone durability; Account owns
authority. The WP05 route remains split as follows:

- `crates/schema` owns durable backup cadence/schedule/job lifecycle,
  idempotency/execution refs, provider-neutral operation refs, and
  bundle/plan-bound migration apply/rollback/reconciliation results/receipts.
- `crates/storage-custody-core` owns pure backup scheduling/job-state,
  restore-execution, migration/rollback, bundle/preflight binding, and
  fail-closed compensation decisions.
- `crates/parent-runtime-core` owns durable scheduler/job persistence,
  restore/migration ledger, restart reconciliation, executor/rollback mount,
  and real Eventing journal/outbox composition.

WP09 and WP10 remain downstream source-only routes for pure provider-neutral
byte and producer-handoff orchestration and cannot duplicate the parent
runtime owner. The expected tests, focused validation, proof, and completion
remain open. No provider fake, caller-supplied authority, or source
completion is implied.
