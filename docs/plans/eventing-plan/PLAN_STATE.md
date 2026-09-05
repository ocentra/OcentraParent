# Reusable Rust Eventing Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `eventing-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## WP02 code-and-test-source checkpoint — 2026-08-28

- Four reviewed reusable implementation roots own the typed ID/taxonomy,
  validation, live/stored envelope, and duplicate contract-registry boundaries.
  Real non-test callers remain present across child runtime, parent runtime,
  agent core/service, family identity, and app/game owners.
- Canonical `d23e2d60a` completes the five mapped real test roots with malformed
  taxonomy separators, strong-ID serde validation, full source metadata,
  current-schema round-trip, registry Rust-type identity, and typed envelope
  identity assertions.
- No test was executed in this code-first phase. Retained WP02 proof, matching
  checklist acceptance, runtime durability/currentness/authorization, transport,
  consumer behavior, CI, READY, and DONE remain open.

## Scope

This folder is the implementation plan for a reusable Rust event bus inspired by the Ocentra Games `@ocentra/eventing-domain` package, but rebuilt as a Rust-first crate that can be used by Ocentra Parent and other Ocentra Rust projects.

## Current ownership interpretation

```text
ocentra-eventing:
  Reusable Rust local event bus, typed envelopes, event ids, idempotency keys, aggregate ordering, queue/dead-letter semantics, request/response registry, journal/replay, topology/contract registry, local dispatch lifecycle, and testkit helpers.

schema-domain:
  Canonical shared event contract shapes when event contracts cross package, crate, app, or plan boundaries.

event-domain:
  Package-boundary metadata only. Shared event contracts live in schema-domain or the owning protocol package.

agent-protocol and agent-service:
  Protocol/service consumers when selected. They prove wire, service, and read-model delivery only for their own surfaces.

LAN and remote-access plans:
  Transport, mesh, relay, pairing, route authority, and cross-device delivery owners.

Network, AI, policy, enforcement, portal, data-custody, browser, app-game, screen, tracking, setup, payment, and account plans:
  Consumer owners. They may publish or consume typed events through handoffs, but they own their domain behavior.
```

## Current coupling risks

```text
- Local bus proof is not cross-device transport proof.
- Protocol shape proof is not service delivery proof.
- NDJSON journal/replay proof is not production durability, retention, deletion, export, or remote replication proof.
- Consumer read-model proof is not reusable eventing crate readiness.
- Eventing does not own AI classification, policy decisions, enforcement actions, portal rendering, data custody policy, LAN/remote relay, or adapter side effects.
- WP12 route proof and WP13 layout proof cannot close WP10 LAN household mesh consumer proof.
- Historical checked rows and historical proof references do not override current proof-root absence.
```

## Current proof interpretation

```text
output/eventing-plan-proof/<workpack>/ is the normal raw/generated implementation/workpack proof root.
docs/proof/eventing-plan/ contains the hand-authored WP06 durable manifest; the
WP12 generated route-proof bundle is absent.
crate tests can prove local reusable bus behavior only.
event-domain metadata does not prove shared event contract implementation.
agent-protocol/service tests prove protocol/service handoff only when selected.
consumer plan proof is required before claiming product behavior.
WP06 now retains its exact enforcement WP11 journal handoff, journal/topology
  proof, and compact validation log in its hand-authored durable manifest under
  `docs/proof/eventing-plan/`. WP10 remains
open until its proof roots and LAN/remote-access handoff verification exist.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-eventing-snapshot.md](current-eventing-snapshot.md)

## What is implemented / exercised in this checkout

- `crates/ocentra-eventing` and its external `unit`, `contract`, `journal_replay`, `integration`, and `version_skew` harnesses are present. This audit does not treat their historical pass wording as current retained proof.
- `packages/event-domain` and downstream mirror commands remain expected validation, not current closure evidence.
- 2026-08-18 WP08 hardening checkpoint: `agent-service` has a narrow typed
  parent-intent marker route. Missing/malformed requests are rejected; valid
  markers are manual-required because no authenticated session producer is
  mounted. The result explicitly leaves journal, Eventing publication, event
  id, and child transport unclaimed. This removes the prior caller-fabricated
  policy/tracking state, zero-consumer publish, and replay-redispatch path, but
  it is not functional parent-runtime integration. The planned
  `parent_runtime_intent_ingress.rs` owner and expected service/runtime tests
  remain absent.
- 2026-08-16 production-code pass: WP10 now has a code-drafted, unvalidated
  structural household-mesh validator and fail-closed runtime authorization
  boundary in `crates/agent-protocol/src/household_mesh.rs`,
  `crates/agent-protocol/src/household_mesh/household_mesh_bridge_input.rs`,
  `crates/agent-core/src/household_mesh_event_bridge.rs`, and
  `crates/agent-core/src/household_mesh_bridge_runtime_validation_import.rs`.
  Agent-core owns the private token and republish conversion; the resolver
  remains unavailable until LAN/account/device authority composition exists.
  Tests, validation, proof, checklist, and runtime integration remain
  deferred.
- 2026-08-16/17 WP09 production pass: exact-source capture ingestion now feeds
  deterministic durable Eventing publication/replay, startup and recurring
  reconciliation run before/after readiness as owned, persisted-row corruption
  fails closed, and read/stream APIs remain projection-only. The portal host
  bridge rejects all four enforcement mutation commands before Tauri/dev-web
  serialization while allowing enforcement read-model commands; the
  parent-assistant service router returns an assistant-answer event rather than
  an enforcement event. The eight focused local command families and portal
  command-boundary gates are green. Commit `4b7bf6e3f` has the normal
  pre-commit and is pushed; WP09 integration, CI, review, and merge remain
  open.
- WP13 test-folder layout code is present, but current validation and proof are
  open; its proof root is absent and the `contract` harness must be included.
- WP12 remains blocked: `scripts/test/eventing-rollout-proof.mjs` and its
  canonical route-proof root are absent, and WP09 integration acceptance, WP10
  authority/consumer handoff, and WP13 validation/proof remain incomplete.
- WP11 production source is independently accepted and integrated through
  `fa1230661`: live `EventEnvelope<E>` fields remain private; live decode,
  `store()`, and stored `decode()` revalidate contract, aggregate, and
  idempotency identity; pending requests bind their associated response type;
  journal event-id/phase idempotency fails closed when unsupported; and only a
  journal-created, non-cloneable, consuming `ReplayActionReport` can authorize
  action replay. The routed test-source packet is integrated at canonical
  `ac5d41322` across the three target harnesses, whose `--no-run` compilation is
  current. Actual test execution, retained proof, checklist rows 63-68, and
  completion review remain open; this is a validation transition only, with no
  DONE claim.

## Open gaps / missing product runtime

- WP08 cannot publish a parent intent until Account WP03 supplies opaque live
  session authority, Tracking WP40 and Policy WP03/WP04/WP08 supply canonical
  producers/consumers, Enforcement WP11 supplies durable result history, and
  Child Runtime WP10 supplies authenticated ingress/local republish. Existing
  caller-parameterized parent-runtime helpers are not production authority.
- Parent protocol event payloads and service read-model bridges still need their own feature proofs before they can claim full parent/controller or child-agent product eventing completion.
- Cross-process parent-to-child, external transport/relay, LAN/relay, and service transport delivery are not implemented by the reusable crate. They must publish into a local bus on each side after typed transport/API boundaries.
- Household AI Provider Mesh is consumer-layer work. The reusable eventing crate supplies local bus semantics, typed envelopes, idempotency, TTL, retry, dead-letter, aggregate ordering, request/response, journal/replay, and topology proof. It does not provide cross-device transport, peer discovery, provider trust, job authority, payload custody, policy behavior, or enforcement behavior.
- Network AI classification, policy decisions, enforcement commands, adapter side effects, audit storage, and portal rendering remain network/service/UI consumer work, not event bus responsibilities.
- External transport delivery currently proves local queue/idempotency/dead-letter semantics and route-decision requirements only. A live transport/relay delivery implementation remains a separate workpack.
- The NDJSON journal is the reusable append/replay proof layer. Production durability requirements such as fsync policy, SQLite projections, remote replication, or retention/deletion enforcement remain consumer/platform decisions.
- Current source audit at root `d1d39b437` found `NdjsonEventJournal::recover` calling `acquire_append_file_lock` across the `ndjson_io` child boundary while the helper was only `pub(super)`. The helper and guard are now visible only within `crate::journal::ndjson` (not crate-wide and not re-exported), preserving the journal module's ownership boundary. Its proof refresh remains deferred.
- The WP13 source-layout cleanup is code-complete: no eventing test modules
  remain under `crates/ocentra-eventing/src/`. Focused validation, including
  `cargo test -p ocentra-eventing --test contract`, and the fresh proof root
  remain open.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).
- Historical checklist rows remain marked complete in `implementation-checklist.md`, but those checkmarks are not current proof truth for this checkout.
- `CHECKLIST_INDEX.md` is the current tracker for WP06 closure and the open
  WP09-WP13 validation/proof work; WP11 checklist rows 63-68 remain open.

## Workpack summary

- Workpacks indexed: 13 route workpacks.
- Workpack source: `05-implementation-workpacks.md` rows split into focused files under `workpacks/`.
- Historical route docs describe prior closure for WP01-WP08 and WP11-WP13,
  but the cited proof bundle is not present in this checkout. WP06 is locally
  evidenced by its durable hand-authored manifest because enforcement WP11
  needs its exact generic handoff.
- WP09 is the active production-foundation packet. Its current source
  implements ingestion-time publication of the exact captured source
  observation, deterministic phase-scoped identity/idempotency, a
  network-owned `ProductionFileEventJournal` with recovery before listener
  readiness, startup/recurring reconciliation, projection-only read/stream
  consumers, and real AI/portal direct-command authority negatives. Phase 1
  production code and expected focused tests are written. Queue/request-response
  helpers are deliberately not wired into a fake product caller, and the
  workpack does not synthesize downstream AI/policy/enforcement/audit/portal
  phases. Focused Eventing, protocol, core, ActivityStore, service,
  parent-runtime, and portal families plus changed-file architecture/Enforcer
  gates pass locally.
- Local ignored evidence is regenerated at
  `output/eventing-plan-proof/09-network-consumer-event-chain/`, with a compact
  manifest and eight wrapped command families. Normal pre-commit and accepted
  commits through `4b7bf6e3f` are pushed. Rows 57-62 remain unchecked until
  whole-plan integration, CI, review, and merge are complete; ignored evidence
  is not retained acceptance by itself.
- Workpacks closed in the current selectable slice: WP06 only. WP08 is now
  explicitly blocked after its safe fail-closed source hardening. WP09 remains
  open on integration/CI/review/merge; WP10 is blocked on LAN WP26 and its
  authority composition; WP11 is in validation after production and routed
  test-source integration, with `--no-run` compilation current but execution,
  retained proof, checklist rows 63-68, and completion review open; WP12 is
  blocked on its route harness/root and WP09/WP10/WP13 prerequisites; WP13 is
  in validation with current validation/proof still open.

### Active/open workpacks

- [08 Parent Runtime Integration](workpacks/08-parent-runtime-integration.md)
  (safe reject/manual-required ingress integrated; functional runtime owner,
  dependency composition, expected tests, and proof remain open)
- [09 Network Consumer Event Chain](workpacks/09-network-consumer-event-chain.md)
  (Phase 1 and expected tests written; focused/local proof and normal pre-commit
  green; accepted commits pushed; whole-plan integration, CI, review, and merge
  open)
- [10 LAN Household Mesh Consumer](workpacks/10-lan-household-mesh-consumer.md)
- [11 Type Safety And Ownership Hardening](workpacks/11-type-safety-and-ownership-hardening.md)
  (production and routed test source integrated; three target `--no-run`
  compiles are current; execution, retained proof, checklist rows 63-68, and
  completion review remain open)

## Validation reality

- Focused reusable crate validation passes in this checkout: `cargo test -p ocentra-eventing --test unit`, `--test contract`, `--test journal_replay`, `--test integration`, and `--test version_skew`, plus `cargo lint-architecture crates/ocentra-eventing/src crates/ocentra-eventing/tests`.
- Post-WP13 cleanup validation is required but not current retained evidence;
  it must include `cargo test -p ocentra-eventing --test contract` and write
  `output/eventing-plan-proof/13-test-folder-layout-regression-audit/`.
- WP12 route-proof validation is blocked because
  `node scripts/test/eventing-rollout-proof.mjs` and
  `output/eventing-plan-proof/12-rollout-proof-and-pr-gate/` are absent.
- Shared TypeScript contract mirror validation passes in this checkout: `npm run test --workspace @ocentra-parent/event-domain` and `npm run type-check --workspace @ocentra-parent/event-domain`.
- Focused downstream mirror validation also passes: `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- network-runtime-events.test.ts contracts.test.ts` and `cargo test -p ocentra-parent-agent-protocol child_domain_runtime_events --quiet`.
- WP11 routed test-source migration is integrated at `ac5d41322`, and the three
  target harnesses compile with `--no-run`. Actual test execution, retained
  proof regeneration, checklist rows 63-68, and completion review remain open;
  the cited `63-type-safety-source-gate`, `66-76-source-safety`, `67-lock-await`,
  and `68-fixture-parity` roots are not retained here. The policy-control
  TypeScript checks are not Eventing WP11 closure evidence.
- WP06 focused proof now passes: journal/replay (22), topology manifest (4),
  lineage compatibility (3), runtime shutdown (5), and the scoped
  architecture gate are recorded in the durable manifest under
  `docs/proof/eventing-plan/`.
- WP09 local evidence now also records 41 journal, 10 protocol-runtime, 17
  protocol-flow, 43 core-runtime, 6 ActivityStore, 36 filtered service, 39 full
  service-bridge, and 2 parent-runtime passing tests. Portal rows 60-61 add 159
  passing portal tests/type-check and an 11-test parent-assistant target.
- Current interpretation: WP09 code and focused local evidence are present,
  but integration/CI/review/merge remain open. WP06 is the only closed
  selectable workpack. WP10 is blocked on LAN WP26; WP11-WP13 remain open for
  implementation/validation/proof reconciliation.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless the selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Active proof-root route:
  - use `output/eventing-plan-proof/<workpack-file-stem>/` for new raw/generated implementation/workpack output.
  - `docs/proof/eventing-plan/` contains the hand-authored WP06 durable manifest;
    it does not replace the absent WP12 generated route-proof bundle.
  - historical `docs/proof/eventing-plan/slice-*` references do not close runtime work by themselves.
- Before any checked update, attach:
  - a real test run log or explicit blocker from the assigned implementation boundary,
  - a proof artifact under the selected proof root,
  - negative cases, no-claim language, and manual-required notes where applicable.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, consumer handoff, and rollback/teardown proofs are present or carried as exact blockers for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
