<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Tracking Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/tracking-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.
Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership And Schema Authority Contract

This plan owns tracking evidence, tracking runtime contracts, location/geofence/expected-place/nearby-place semantics, tracking read models, tracking proof roots, and tracking no-claim boundaries. It does not own generic schema authority, generic event-bus mechanics, generic journal/replay mechanics, custody policy, account or household authority, policy authority, notification provider runtime, AI provider runtime, OS/platform behavior, broad portal shell completion, or real-device proof outside selected workpacks.

Current Rust-first authority comes from
`../../agent/RUST_FIRST_PARENT_ARCHITECTURE.md`. Older tracking docs that name
`schema-domain`, `tracking-domain`, `agent-protocol-domain`, WebSocket, or Vite
as product authority are migration/debt inventory, not current architecture
truth.

Canonical schema rule:

```text
crates/schema owns cross-boundary tracking DTOs, route snapshots, actions,
results, generated bridge artifacts, and encoded-shape drift proof.
The owning Rust tracking/runtime crates own tracking behavior, projections, read
models, state machines, and policy/activity handoff logic.
tracking-domain, schema-domain, and agent-protocol-domain are transitional TS
edge/helper surfaces only until Rust/generated consumers replace them.
TypeScript must not invent divergent canonical tracking schemas.
```

A product shape must be promoted to `crates/schema` or the owning Rust
domain/runtime crate when it crosses package, crate, plan, protocol, event,
portal DTO, runtime command, policy input, notification input, custody/export,
read-model, or proof-metadata boundaries.

Tracking-local schemas are allowed only when private, implementation-local, and not used as public contracts.

Required Rust-owned schema/domain families:

```text
location evidence
device status
permission and capability state
geofence rule and transition
expected-place schedule and status
nearby-place analysis and ambiguity
tracking retention and custody refs
tracking policy refs and decision inputs
notification intent and dispatch result refs
escalation intent and result refs
temporary live tracking mode
missing-device mode
tracking event payloads and protocol constants
tracking read-model DTOs
proof metadata and evidence refs
```

Forbidden claims and shortcuts:

```text
local tracking schema used as public protocol shape
local event string invented where schema/protocol/eventing owns the type
private event bus, journal, replay, provider-status, custody, notification, AI-provider, LAN, network, browser, or app-game mechanics duplicated inside tracking-specific source
LAN/IP/Wi-Fi hint upgraded into GPS/current-location proof
low-accuracy nearby-place evidence upgraded into exact-place or accusation copy
AI output upgraded into household authority, policy decision, notification, escalation, or enforcement
fixture/proof file upgraded into real-device, provider-runtime, production-worker, or product-ready proof
portal screenshot upgraded into runtime delivery or product UI proof
```

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
5. Assigned workpack under `workpacks/`, if any.
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
7. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
8. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

## Local decision tree

- If the hub assignment names a workpack, open only that workpack, then use `TEST_PROOF_EXPECTATIONS.md` to choose expected tests/proof for that work type.
- If owner/proof family is unclear, use `WORKPACK_FAMILIES.md`; do not scan every family.
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

[location-geofence-device-status.md](../../features/location-geofence-device-status.md), [location-geofence.md](../../expectations/location-geofence.md), [platforms.md](../../expectations/platforms.md)

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

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and `../../agent/VALIDATION_FLOW.md`.
