<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Reusable Rust Eventing Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Reusable Rust Eventing Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/eventing-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.

## Ownership, Import, And Boundary Contract

This plan owns reusable local eventing semantics and proof. It does not own every product feature that publishes, consumes, transports, displays, stores, or reacts to events.

Module roles:

```text
ocentra-eventing: reusable Rust local event bus, typed envelopes, event ids, idempotency keys, aggregate ordering, queue/dead-letter semantics, request/response registry, journal/replay, topology/contract registry, local dispatch lifecycle, and testkit helpers.
schema-domain: canonical shared event contract shapes when event contracts cross package, crate, app, or plan boundaries.
event-domain: package-boundary metadata only; shared event contracts live in schema-domain or the owning protocol package.
agent-protocol and agent-service: protocol/service consumers when selected. They prove wire/service/read-model delivery only for their own surfaces.
LAN and remote-access plans: transport, mesh, relay, pairing, route authority, and cross-device delivery owners.
network, AI, policy, enforcement, portal, data-custody, browser, app-game, screen, tracking, setup, payment, and account plans: consumer owners that may publish or consume events through typed handoffs; they own their domain behavior.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
ocentra-eventing public modules for local event bus, envelope, ids, queue, journal, replay, request/response, topology, contract registry, and testkit proof
schema-domain shared event shapes when a contract is cross-boundary
agent-protocol/agent-service public protocol surfaces only when selected by the workpack
consumer-plan public contract surfaces only when the selected workpack names a consumer handoff
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports and claims:

```text
consumer feature runtime internals imported into the eventing crate
local bus proof upgraded into cross-device transport, LAN mesh, relay, or remote delivery proof
NDJSON journal/replay proof upgraded into production durability, retention, deletion, export, or remote replication proof
protocol shape proof upgraded into service delivery proof
consumer read-model proof upgraded into eventing crate readiness
AI, policy, enforcement, portal, storage, account, payment, setup, browser, screen, app-game, tracking, LAN, or remote behavior claimed as eventing behavior
provider or peer devices allowed to publish policy/enforcement events directly through the eventing plan
```

If eventing work needs LAN, remote, network, AI, policy, enforcement, portal, data custody, browser, app-game, screen, tracking, setup, payment, or account behavior, it must use typed evidence, commands, events, requests, read models, proof roots, and explicit handoffs. If a shape is used by multiple feature owners, place or consume it through `schema-domain` or another neutral shared boundary. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.
5. Assigned workpack under `workpacks/`, if any.
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
7. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
8. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

## Local decision tree

- If the hub assignment names a workpack, open only that workpack, then use `TEST_PROOF_EXPECTATIONS.md` to choose expected tests/proof for that work type.
- If the assignment names a checklist row but no workpack, use `CHECKLIST_INDEX.md` to locate the row, then use `TEST_PROOF_EXPECTATIONS.md` for required tests/proof.
- If the assignment is docs/status only, use `DOC_INDEX.md` and the docs/status rows in `TEST_PROOF_EXPECTATIONS.md`; do not inspect source or sibling plans unless the row names them.
- If the assignment touches source, contracts, runtime, UI, AI, platform, security, persistence, or observability, read `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the local workpack is known.
- If the assignment is PR_READY or DONE, read `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, `PLAN_HEALTH.md` only for broad claims, then `../../agent/PR_DONE_FLOW.md`.
- If `TEST_PROOF_EXPECTATIONS.md` says a required test/proof is missing, keep the row open and report the missing test/proof instead of claiming completion.

## Local work loop

1. Read only the route files above and the assigned workpack/checklist row.
2. Identify the intended implementation crate/package or current owning package/crate if the per-plan implementation crate is not created yet.
3. Make the narrow code/doc change.
4. Run the lightest relevant compile/lint/type/schema check for the touched area before expanding scope.
5. Add or update the tests named by `TEST_PROOF_EXPECTATIONS.md`; if the expected test folder/crate does not exist yet, record the missing location and keep the row open.
6. Run the focused tests/proof commands, then run broader validation only when `VALIDATION_FLOW.md` or PR_READY scope requires it.
7. Update workpack/checklist/proof docs with exact test names, command logs, proof artifacts, skipped checks, and remaining gaps.

## Product docs for this plan

[evidence-store-query.md](../../features/evidence-store-query.md), [reports-notifications-sync.md](../../features/reports-notifications-sync.md), [evidence-storage.md](../../expectations/evidence-storage.md), [data-custody.md](../../expectations/data-custody.md)

## Validation and proof choice

After the assigned workpack is known, use [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) first, then [TEST_PROOF_DECISION_MATRIX.md](../../agent/TEST_PROOF_DECISION_MATRIX.md) only for global risk escalation. Record the selected rows in DONE/PR_READY. Do not read unrelated proof docs, and do not close checklist rows with happy-path-only proof when auth, protocol, persistence, UI, AI, platform, security, performance, or observability risk is touched.

## Do not read by default

- `implementation-checklist.md` as a whole.
- all `workpacks/*.md`.
- `README_FULL_ORIGINAL.md`.
- `source-index.md` or pasted-content audits unless source ownership is unclear.
- sibling plan folders.
- global checkpoints unless `PROOF_INDEX.md` names them for your proof.

## Before DONE / PR_READY

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and
feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and
`../../agent/VALIDATION_FLOW.md`.
